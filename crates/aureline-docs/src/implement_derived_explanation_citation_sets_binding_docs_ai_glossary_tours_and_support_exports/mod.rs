//! Derived-explanation citation sets that bind generated prose to its evidence.
//!
//! Every claimed M5 derived explanation — a docs-browser explanation, an AI
//! answer, a glossary card, a guided-tour step, an architecture explainer, or a
//! support-export note — must attach exactly one [`DerivedExplanationCitationSet`]
//! or explicitly label itself an inference. A citation set binds one explanation
//! to the concrete evidence it actually depended on: the cited files, the cited
//! symbols, the cited docs nodes, the code-graph epoch, the locale, and the
//! derivation tool/version that produced it. The set reuses the canonical
//! source-class, trust-class, freshness, version-match, and locale vocabularies
//! frozen by the docs-contracts matrix rather than minting parallel tokens, so
//! AI, onboarding, glossary, docs, and support surfaces project one citation
//! object instead of inventing prose-only private explanation state.
//!
//! The lane enforces three product invariants:
//!
//! * **Prose never outruns its basis.** A [`CitationBasis::DirectCitation`] set
//!   must name at least one cited file, symbol, or docs node; a
//!   [`CitationBasis::LabeledInference`] set must carry an explicit inference
//!   label naming why no direct citation exists and what it was inferred from,
//!   and may never claim primary authority — its trust class stays
//!   [`DocsContractTrustClass::DerivedInferenceOnly`].
//! * **Derived explanations never outlive their citation sets.** A citation set
//!   keeps a stable export-safe identity bound to its explanation; the basis
//!   survives offline, mirrored, localized, and support-export flows even when
//!   raw content is redacted or omitted, because the citation refs, graph epoch,
//!   and derivation tool/version are metadata, not content.
//! * **Every surface reuses the same object.** Consumer projections record that
//!   each claimed surface reuses the shared citation set, preserves the inference
//!   label, and preserves the citation basis on export — and the support-export
//!   projection must cover every citation set so an export never silently drops a
//!   derived explanation's evidence basis.
//!
//! [`DerivedExplanationCitationPacket::materialize`] computes the validation
//! findings and the promotion state (`stable`, `narrowed_below_stable`, or
//! `blocks_stable`) from the input, so an explanation that cites nothing, an
//! inference that hides behind authoritative trust, a redaction that strips the
//! basis, or a support export that drops a citation set automatically narrows or
//! blocks before it reaches a consumer surface. The packet is an inspectable,
//! serde-serializable truth packet: it carries no raw document bodies, raw source
//! files, raw URLs, raw provider payloads, prompt text, or credentials — only
//! metadata, citation refs, derivation identity, and contract references.
//!
//! The boundary schema is
//! [`schemas/docs/implement-derived-explanation-citation-sets-binding-docs-ai-glossary-tours-and-support-exports.schema.json`](../../../../schemas/docs/implement-derived-explanation-citation-sets-binding-docs-ai-glossary-tours-and-support-exports.schema.json).
//! The contract doc is
//! [`docs/docs/m5/implement_derived_explanation_citation_sets_binding_docs_ai_glossary_tours_and_support_exports.md`](../../../../docs/docs/m5/implement_derived_explanation_citation_sets_binding_docs_ai_glossary_tours_and_support_exports.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/implement_derived_explanation_citation_sets_binding_docs_ai_glossary_tours_and_support_exports/`](../../../../fixtures/docs/m5/implement_derived_explanation_citation_sets_binding_docs_ai_glossary_tours_and_support_exports/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    DocsContractFreshnessState, DocsContractLocaleMatch, DocsContractSourceClass,
    DocsContractTrustClass, DocsContractVersionMatchState,
};

/// Stable record-kind tag carried by [`DerivedExplanationCitationPacket`].
pub const DERIVED_EXPLANATION_CITATION_RECORD_KIND: &str =
    "derived_explanation_citation_sets_packet";

/// Stable record-kind tag carried by [`DerivedExplanationCitationSupportExport`].
pub const DERIVED_EXPLANATION_CITATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "derived_explanation_citation_sets_support_export";

/// Schema version for derived-explanation citation-set records.
pub const DERIVED_EXPLANATION_CITATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DERIVED_EXPLANATION_CITATION_SCHEMA_REF: &str =
    "schemas/docs/implement-derived-explanation-citation-sets-binding-docs-ai-glossary-tours-and-support-exports.schema.json";

/// Repo-relative path of the contract doc.
pub const DERIVED_EXPLANATION_CITATION_DOC_REF: &str =
    "docs/docs/m5/implement_derived_explanation_citation_sets_binding_docs_ai_glossary_tours_and_support_exports.md";

/// Repo-relative path of the checked support-export artifact.
pub const DERIVED_EXPLANATION_CITATION_ARTIFACT_REF: &str =
    "artifacts/docs/m5/implement_derived_explanation_citation_sets_binding_docs_ai_glossary_tours_and_support_exports/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const DERIVED_EXPLANATION_CITATION_SUMMARY_REF: &str =
    "artifacts/docs/m5/implement_derived_explanation_citation_sets_binding_docs_ai_glossary_tours_and_support_exports.md";

/// Repo-relative path of the protected fixture directory.
pub const DERIVED_EXPLANATION_CITATION_FIXTURE_DIR: &str =
    "fixtures/docs/m5/implement_derived_explanation_citation_sets_binding_docs_ai_glossary_tours_and_support_exports";

/// Repo-relative path of the frozen docs-contracts matrix the lane consumes.
pub const DERIVED_EXPLANATION_CITATION_MATRIX_CONTRACT_REF: &str =
    "schemas/docs/freeze-the-m5-docs-source-result-pack-version-match-citation-set-and-browser-handoff-matrix.schema.json";

/// A claimed M5 surface that produces derived explanations and must attach a
/// citation set.
///
/// These are exactly the surfaces named by the lane: each one publishes prose
/// that must trace back to one citation set or carry an explicit inference label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DerivedExplanationSurface {
    /// A docs-browser-derived explanation (peek/hover/explainer prose).
    DocsBrowserExplanation,
    /// An AI answer rendered in the assistant surface.
    AiAnswer,
    /// A glossary card surfaced in learning/onboarding packs.
    GlossaryCard,
    /// A guided-tour step.
    GuidedTourStep,
    /// An architecture / topology explainer card.
    ArchitectureExplainer,
    /// A support-export note carried into a redacted support packet.
    SupportExportNote,
}

impl DerivedExplanationSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DocsBrowserExplanation,
        Self::AiAnswer,
        Self::GlossaryCard,
        Self::GuidedTourStep,
        Self::ArchitectureExplainer,
        Self::SupportExportNote,
    ];

    /// Surfaces that MUST each carry at least one citation set and one
    /// projection in a stable packet.
    pub const REQUIRED: [Self; 6] = Self::ALL;

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowserExplanation => "docs_browser_explanation",
            Self::AiAnswer => "ai_answer",
            Self::GlossaryCard => "glossary_card",
            Self::GuidedTourStep => "guided_tour_step",
            Self::ArchitectureExplainer => "architecture_explainer",
            Self::SupportExportNote => "support_export_note",
        }
    }
}

/// Whether a derived explanation rests on a direct citation or a labeled
/// inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationBasis {
    /// The explanation cites at least one concrete file, symbol, or docs node.
    DirectCitation,
    /// No direct citation exists; the explanation is an explicitly labeled
    /// inference and never claims primary authority.
    LabeledInference,
}

impl CitationBasis {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectCitation => "direct_citation",
            Self::LabeledInference => "labeled_inference",
        }
    }
}

/// How much cited content the packet carries for a citation set.
///
/// Redaction never strips the citation *basis* — the cited refs, graph epoch,
/// and derivation identity stay so the explanation remains traceable even when
/// the underlying content is withheld.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationRedactionState {
    /// Cited refs are carried with no content withheld (refs are metadata only).
    ContentInlinePreserved,
    /// Cited content is redacted but the citation basis is preserved.
    ContentRedactedBasisPreserved,
    /// Cited content is omitted from this packet but the citation basis is
    /// preserved so the source corpus is not forced into every export.
    ContentOmittedBasisPreserved,
}

impl CitationRedactionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContentInlinePreserved => "content_inline_preserved",
            Self::ContentRedactedBasisPreserved => "content_redacted_basis_preserved",
            Self::ContentOmittedBasisPreserved => "content_omitted_basis_preserved",
        }
    }

    /// True when content is withheld and only the basis is preserved.
    pub const fn withholds_content(self) -> bool {
        matches!(
            self,
            Self::ContentRedactedBasisPreserved | Self::ContentOmittedBasisPreserved
        )
    }
}

/// Confidence carried by a labeled inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InferenceConfidence {
    /// Inference closely tied to the cited graph epoch and surrounding evidence.
    Grounded,
    /// Pattern-based inference; correctness is plausible but unverified.
    Heuristic,
    /// Low-confidence inference that must read as speculative.
    Speculative,
}

impl InferenceConfidence {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Grounded => "grounded",
            Self::Heuristic => "heuristic",
            Self::Speculative => "speculative",
        }
    }
}

/// One cited workspace file.
///
/// Carries a path ref and a content-digest ref — never a raw file body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitedFileRef {
    /// Workspace-relative path ref of the cited file.
    pub file_path_ref: String,
    /// Stable content-digest ref pinning the cited revision (no raw body).
    pub content_digest_ref: String,
    /// Optional cited line span ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_span_ref: Option<String>,
    /// Source class of the cited file.
    pub source_class: DocsContractSourceClass,
}

impl CitedFileRef {
    /// True when required identity fields are present.
    fn is_well_formed(&self) -> bool {
        !self.file_path_ref.trim().is_empty() && !self.content_digest_ref.trim().is_empty()
    }
}

/// One cited code symbol.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitedSymbolRef {
    /// Stable symbol ref.
    pub symbol_ref: String,
    /// Optional container (module/type) ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_ref: Option<String>,
    /// Graph node ref the symbol resolves to in the cited epoch.
    pub graph_node_ref: String,
    /// Short symbol-kind label (e.g. `function`, `type`).
    pub kind_label: String,
}

impl CitedSymbolRef {
    /// True when required identity fields are present.
    fn is_well_formed(&self) -> bool {
        !self.symbol_ref.trim().is_empty()
            && !self.graph_node_ref.trim().is_empty()
            && !self.kind_label.trim().is_empty()
    }
}

/// One cited docs node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitedDocRef {
    /// Stable docs-node ref.
    pub doc_node_ref: String,
    /// Source class of the cited docs node.
    pub source_class: DocsContractSourceClass,
    /// Version-match state of the cited docs node against the active build.
    pub version_match: DocsContractVersionMatchState,
    /// Freshness state of the cited docs node.
    pub freshness: DocsContractFreshnessState,
    /// Locale-match state of the cited docs node.
    pub locale: DocsContractLocaleMatch,
    /// Trust class of the cited docs node.
    pub trust_class: DocsContractTrustClass,
}

impl CitedDocRef {
    /// True when required identity fields are present.
    fn is_well_formed(&self) -> bool {
        !self.doc_node_ref.trim().is_empty()
    }
}

/// The code-graph epoch a derived explanation was produced against.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphEpochRef {
    /// Stable code-graph epoch ref.
    pub epoch_ref: String,
    /// Workspace revision ref the epoch was built from.
    pub workspace_revision_ref: String,
    /// RFC 3339 timestamp the epoch was captured.
    pub captured_at: String,
}

impl GraphEpochRef {
    /// True when required identity fields are present.
    fn is_well_formed(&self) -> bool {
        !self.epoch_ref.trim().is_empty()
            && !self.workspace_revision_ref.trim().is_empty()
            && !self.captured_at.trim().is_empty()
    }
}

/// The derivation tool and version that produced a derived explanation.
///
/// Carries identity only — never prompt text or raw provider payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivationTool {
    /// Stable derivation-tool ref.
    pub tool_ref: String,
    /// Derivation-tool version ref.
    pub tool_version_ref: String,
    /// Optional model identity ref (no prompt text or payloads).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_ref: Option<String>,
}

impl DerivationTool {
    /// True when required identity fields are present.
    fn is_well_formed(&self) -> bool {
        !self.tool_ref.trim().is_empty() && !self.tool_version_ref.trim().is_empty()
    }
}

/// The explicit inference label a [`CitationBasis::LabeledInference`] set carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InferenceLabel {
    /// Why no direct citation exists.
    pub reason: String,
    /// Short summary of what the inference was drawn from.
    pub inferred_from_summary: String,
    /// Inference confidence.
    pub confidence: InferenceConfidence,
}

impl InferenceLabel {
    /// True when required identity fields are present.
    fn is_well_formed(&self) -> bool {
        !self.reason.trim().is_empty() && !self.inferred_from_summary.trim().is_empty()
    }
}

/// One derived-explanation citation set binding a generated explanation to the
/// evidence it depended on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedExplanationCitationSet {
    /// Stable, export-safe citation-set id.
    pub citation_set_id: String,
    /// Stable id of the derived explanation this set binds.
    pub explanation_id: String,
    /// Surface the bound explanation appears on.
    pub explanation_surface: DerivedExplanationSurface,
    /// Human-readable explanation label.
    pub explanation_label: String,
    /// Whether the explanation rests on a direct citation or labeled inference.
    pub basis: CitationBasis,
    /// Effective source class of the citation basis.
    pub source_class: DocsContractSourceClass,
    /// Effective trust class of the citation basis.
    pub trust_class: DocsContractTrustClass,
    /// Effective freshness of the citation basis.
    pub freshness: DocsContractFreshnessState,
    /// Locale of the bound explanation.
    pub locale: DocsContractLocaleMatch,
    /// Cited workspace files.
    #[serde(default)]
    pub cited_files: Vec<CitedFileRef>,
    /// Cited code symbols.
    #[serde(default)]
    pub cited_symbols: Vec<CitedSymbolRef>,
    /// Cited docs nodes.
    #[serde(default)]
    pub cited_docs: Vec<CitedDocRef>,
    /// Code-graph epoch the explanation was produced against.
    pub graph_epoch: GraphEpochRef,
    /// Derivation tool and version that produced the explanation.
    pub derivation: DerivationTool,
    /// Inference label, present when [`Self::basis`] is
    /// [`CitationBasis::LabeledInference`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference_label: Option<InferenceLabel>,
    /// How much cited content the packet carries for this set.
    pub redaction: CitationRedactionState,
    /// True when raw boundary material is excluded from this set.
    pub raw_boundary_material_excluded: bool,
}

impl DerivedExplanationCitationSet {
    /// True when the set names at least one direct citation.
    pub fn has_any_citation(&self) -> bool {
        !self.cited_files.is_empty()
            || !self.cited_symbols.is_empty()
            || !self.cited_docs.is_empty()
    }

    /// True when every required identity field is present and the cited refs are
    /// well formed.
    pub fn is_well_formed(&self) -> bool {
        !self.citation_set_id.trim().is_empty()
            && !self.explanation_id.trim().is_empty()
            && !self.explanation_label.trim().is_empty()
            && self.graph_epoch.is_well_formed()
            && self.derivation.is_well_formed()
            && self.cited_files.iter().all(CitedFileRef::is_well_formed)
            && self
                .cited_symbols
                .iter()
                .all(CitedSymbolRef::is_well_formed)
            && self.cited_docs.iter().all(CitedDocRef::is_well_formed)
    }

    /// True when the basis matches the citations and inference label.
    pub fn basis_consistent(&self) -> bool {
        match self.basis {
            CitationBasis::DirectCitation => {
                self.has_any_citation() && self.inference_label.is_none()
            }
            CitationBasis::LabeledInference => {
                !self.has_any_citation()
                    && self
                        .inference_label
                        .as_ref()
                        .is_some_and(InferenceLabel::is_well_formed)
            }
        }
    }

    /// True when the trust and source class match the basis.
    ///
    /// A labeled inference never claims primary authority; a direct citation is
    /// never recorded as derived-inference-only.
    pub fn trust_consistent(&self) -> bool {
        match self.basis {
            CitationBasis::DirectCitation => {
                self.trust_class != DocsContractTrustClass::DerivedInferenceOnly
                    && self.source_class != DocsContractSourceClass::DerivedExplanation
            }
            CitationBasis::LabeledInference => {
                self.trust_class == DocsContractTrustClass::DerivedInferenceOnly
                    && self.source_class == DocsContractSourceClass::DerivedExplanation
            }
        }
    }

    /// True when the citation basis survives the declared redaction state.
    ///
    /// When content is withheld the set must still name its basis (cited refs or
    /// an inference label) plus a well-formed graph epoch and derivation tool.
    pub fn basis_preserved_through_redaction(&self) -> bool {
        if !self.redaction.withholds_content() {
            return true;
        }
        let names_basis = self.has_any_citation() || self.inference_label.is_some();
        names_basis && self.graph_epoch.is_well_formed() && self.derivation.is_well_formed()
    }

    /// True when this set is a labeled inference flagged speculative.
    fn is_speculative_inference(&self) -> bool {
        self.basis == CitationBasis::LabeledInference
            && self
                .inference_label
                .as_ref()
                .is_some_and(|label| label.confidence == InferenceConfidence::Speculative)
    }

    /// True when this direct citation rests on stale or unverified freshness.
    fn is_stale_direct_citation(&self) -> bool {
        self.basis == CitationBasis::DirectCitation
            && matches!(
                self.freshness,
                DocsContractFreshnessState::Stale | DocsContractFreshnessState::Unverified
            )
    }
}

/// One per-surface projection asserting a surface reuses the shared citation
/// objects rather than inventing prose-only private state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationConsumerProjection {
    /// Consumer surface.
    pub surface: DerivedExplanationSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Packet id this projection belongs to.
    pub packet_id_ref: String,
    /// True when the surface reuses the shared citation set objects.
    pub reuses_shared_citation_object: bool,
    /// True when the surface preserves the inference label.
    pub preserves_inference_label: bool,
    /// True when the surface preserves the citation basis through export.
    pub preserves_citation_basis_on_export: bool,
    /// Citation-set ids this surface projects.
    pub citation_set_id_refs: Vec<String>,
}

impl CitationConsumerProjection {
    /// True when the projection preserves every required flag.
    pub fn preserves_required_flags(&self) -> bool {
        self.reuses_shared_citation_object
            && self.preserves_inference_label
            && self.preserves_citation_basis_on_export
    }
}

/// Derived promotion state of a citation packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DerivedExplanationCitationPromotionState {
    /// All invariants hold; the packet certifies a clean stable claim.
    Stable,
    /// A non-fatal narrowing applies (e.g. a stale citation or speculative
    /// inference); the claim is narrowed below stable.
    NarrowedBelowStable,
    /// A blocking invariant failed; the packet may not claim stable.
    BlocksStable,
}

impl DerivedExplanationCitationPromotionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity of a validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DerivedExplanationCitationValidationSeverity {
    /// Blocks the stable claim.
    Blocker,
    /// Narrows the claim below stable.
    Warning,
}

impl DerivedExplanationCitationValidationSeverity {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocker => "blocker",
            Self::Warning => "warning",
        }
    }
}

/// Closed set of validation finding kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DerivedExplanationCitationValidationKind {
    /// Record kind does not match the contract.
    WrongRecordKind,
    /// Schema version does not match the contract.
    WrongSchemaVersion,
    /// Packet identity is incomplete.
    MissingPacketIdentity,
    /// Source contract refs omit the schema or contract doc.
    MissingSourceContracts,
    /// Packet declares no citation sets.
    MissingCitationSets,
    /// A citation set drops a required identity field.
    CitationSetIncomplete,
    /// Two citation sets share an id.
    DuplicateCitationSetId,
    /// Two citation sets bind the same explanation id.
    DuplicateExplanationBinding,
    /// A direct-citation set cites nothing.
    CitationBasisMissing,
    /// A labeled-inference set drops its inference label.
    InferenceLabelMissing,
    /// Trust or source class disagrees with the basis.
    BasisTrustInconsistent,
    /// A citation set drops its graph epoch.
    GraphEpochMissing,
    /// A citation set drops its derivation tool/version.
    DerivationToolMissing,
    /// A redacted or omitted set lost its citation basis.
    RedactionDropsCitationBasis,
    /// A required surface has no citation set.
    SurfaceCoverageMissing,
    /// A required surface has no consumer projection.
    RequiredSurfaceProjectionMissing,
    /// A projection references a different packet.
    ConsumerProjectionPacketIdMismatch,
    /// A projection drops a required preservation flag.
    ConsumerProjectionDropsReuse,
    /// A projection references an unknown citation set.
    ConsumerProjectionOrphanCitationRef,
    /// The support-export projection does not cover every citation set.
    SupportExportDropsCitationBasis,
    /// Raw boundary material is present in the export.
    RawBoundaryMaterialPresent,
    /// A direct citation rests on stale or unverified freshness.
    CitationFreshnessNarrowed,
    /// A labeled inference is flagged speculative.
    SpeculativeInferenceNarrowed,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl DerivedExplanationCitationValidationKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingPacketIdentity => "missing_packet_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::MissingCitationSets => "missing_citation_sets",
            Self::CitationSetIncomplete => "citation_set_incomplete",
            Self::DuplicateCitationSetId => "duplicate_citation_set_id",
            Self::DuplicateExplanationBinding => "duplicate_explanation_binding",
            Self::CitationBasisMissing => "citation_basis_missing",
            Self::InferenceLabelMissing => "inference_label_missing",
            Self::BasisTrustInconsistent => "basis_trust_inconsistent",
            Self::GraphEpochMissing => "graph_epoch_missing",
            Self::DerivationToolMissing => "derivation_tool_missing",
            Self::RedactionDropsCitationBasis => "redaction_drops_citation_basis",
            Self::SurfaceCoverageMissing => "surface_coverage_missing",
            Self::RequiredSurfaceProjectionMissing => "required_surface_projection_missing",
            Self::ConsumerProjectionPacketIdMismatch => "consumer_projection_packet_id_mismatch",
            Self::ConsumerProjectionDropsReuse => "consumer_projection_drops_reuse",
            Self::ConsumerProjectionOrphanCitationRef => "consumer_projection_orphan_citation_ref",
            Self::SupportExportDropsCitationBasis => "support_export_drops_citation_basis",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
            Self::CitationFreshnessNarrowed => "citation_freshness_narrowed",
            Self::SpeculativeInferenceNarrowed => "speculative_inference_narrowed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the citation-set validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedExplanationCitationValidationFinding {
    /// Closed finding kind.
    pub finding_kind: DerivedExplanationCitationValidationKind,
    /// Finding severity.
    pub severity: DerivedExplanationCitationValidationSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl DerivedExplanationCitationValidationFinding {
    fn blocker(
        finding_kind: DerivedExplanationCitationValidationKind,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity: DerivedExplanationCitationValidationSeverity::Blocker,
            summary: summary.into(),
        }
    }

    fn warning(
        finding_kind: DerivedExplanationCitationValidationKind,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity: DerivedExplanationCitationValidationSeverity::Warning,
            summary: summary.into(),
        }
    }
}

/// Constructor input for [`DerivedExplanationCitationPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedExplanationCitationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface or workflow label.
    pub surface_label: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Derived-explanation citation sets.
    pub citation_sets: Vec<DerivedExplanationCitationSet>,
    /// Per-surface projections.
    pub consumer_projections: Vec<CitationConsumerProjection>,
    /// Source contract refs.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
}

/// Export-safe derived-explanation citation-set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedExplanationCitationPacket {
    /// Record kind; must equal [`DERIVED_EXPLANATION_CITATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DERIVED_EXPLANATION_CITATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface or workflow label.
    pub surface_label: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Derived-explanation citation sets.
    pub citation_sets: Vec<DerivedExplanationCitationSet>,
    /// Per-surface projections.
    pub consumer_projections: Vec<CitationConsumerProjection>,
    /// Source contract refs.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Derived promotion state.
    pub promotion_state: DerivedExplanationCitationPromotionState,
    /// Validation findings.
    #[serde(default)]
    pub validation_findings: Vec<DerivedExplanationCitationValidationFinding>,
}

impl DerivedExplanationCitationPacket {
    /// Materializes the packet and records its derived findings and promotion
    /// state.
    pub fn materialize(input: DerivedExplanationCitationPacketInput) -> Self {
        let mut packet = Self {
            record_kind: DERIVED_EXPLANATION_CITATION_RECORD_KIND.to_owned(),
            schema_version: DERIVED_EXPLANATION_CITATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            generated_at: input.generated_at,
            citation_sets: input.citation_sets,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            promotion_state: DerivedExplanationCitationPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet's invariants, including the stored promotion
    /// state.
    pub fn validate(&self) -> Vec<DerivedExplanationCitationValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when no blocker validation findings exist.
    pub fn is_stable(&self) -> bool {
        !self.validate().iter().any(|finding| {
            finding.severity == DerivedExplanationCitationValidationSeverity::Blocker
        })
    }

    /// Returns true when the packet certifies the clean stable claim.
    pub fn is_clean_stable(&self) -> bool {
        self.promotion_state == DerivedExplanationCitationPromotionState::Stable
            && self.validate().is_empty()
    }

    /// Returns the surfaces covered by at least one citation set.
    pub fn covered_surfaces(&self) -> Vec<DerivedExplanationSurface> {
        let mut set = BTreeSet::new();
        for citation_set in &self.citation_sets {
            set.insert(citation_set.explanation_surface);
        }
        set.into_iter().collect()
    }

    /// Returns true when at least one projection preserves this packet for
    /// `surface`.
    pub fn has_projection_for(&self, surface: DerivedExplanationSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.surface == surface
                && projection.packet_id_ref == self.packet_id
                && projection.preserves_required_flags()
        })
    }

    /// Returns the citation set with the given id, if present.
    pub fn citation_set(&self, citation_set_id: &str) -> Option<&DerivedExplanationCitationSet> {
        self.citation_sets
            .iter()
            .find(|set| set.citation_set_id == citation_set_id)
    }

    /// Wraps the packet in an export-safe support export.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> DerivedExplanationCitationSupportExport {
        DerivedExplanationCitationSupportExport {
            record_kind: DERIVED_EXPLANATION_CITATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DERIVED_EXPLANATION_CITATION_SCHEMA_VERSION,
            export_id: export_id.into(),
            export_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            citation_basis_preserved: true,
            export_packet: self.clone(),
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("derived explanation citation packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Derived-Explanation Citation Sets\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Promotion: `{}` ({} validation findings)\n",
            self.promotion_state.as_str(),
            self.validation_findings.len()
        ));
        out.push_str(&format!(
            "- Citation sets: {} / Surfaces: {}\n",
            self.citation_sets.len(),
            self.consumer_projections.len()
        ));
        out.push_str("\n## Citation sets\n\n");
        for citation_set in &self.citation_sets {
            out.push_str(&format!(
                "- **{}** (`{}`): surface `{}` / basis `{}`\n",
                citation_set.explanation_label,
                citation_set.citation_set_id,
                citation_set.explanation_surface.as_str(),
                citation_set.basis.as_str(),
            ));
            out.push_str(&format!(
                "   - source `{}` / trust `{}` / freshness `{}` / locale `{}`\n",
                citation_set.source_class.as_str(),
                citation_set.trust_class.as_str(),
                citation_set.freshness.as_str(),
                citation_set.locale.as_str(),
            ));
            out.push_str(&format!(
                "   - cited: {} files / {} symbols / {} docs; epoch `{}`; redaction `{}`\n",
                citation_set.cited_files.len(),
                citation_set.cited_symbols.len(),
                citation_set.cited_docs.len(),
                citation_set.graph_epoch.epoch_ref,
                citation_set.redaction.as_str(),
            ));
            if let Some(label) = &citation_set.inference_label {
                out.push_str(&format!(
                    "   - inference (`{}`): {}\n",
                    label.confidence.as_str(),
                    label.reason
                ));
            }
        }
        out
    }

    fn derived_findings(
        &self,
        check_promotion: bool,
    ) -> Vec<DerivedExplanationCitationValidationFinding> {
        let mut findings = Vec::new();

        if self.record_kind != DERIVED_EXPLANATION_CITATION_RECORD_KIND {
            findings.push(DerivedExplanationCitationValidationFinding::blocker(
                DerivedExplanationCitationValidationKind::WrongRecordKind,
                "record kind does not match the derived-explanation citation contract",
            ));
        }
        if self.schema_version != DERIVED_EXPLANATION_CITATION_SCHEMA_VERSION {
            findings.push(DerivedExplanationCitationValidationFinding::blocker(
                DerivedExplanationCitationValidationKind::WrongSchemaVersion,
                "schema version does not match the derived-explanation citation contract",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.generated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
        {
            findings.push(DerivedExplanationCitationValidationFinding::blocker(
                DerivedExplanationCitationValidationKind::MissingPacketIdentity,
                "packet identity is incomplete",
            ));
        }

        self.validate_source_contracts(&mut findings);
        self.validate_citation_sets(&mut findings);
        self.validate_surface_coverage(&mut findings);
        self.validate_projections(&mut findings);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("derived explanation citation packet serializes"),
        ) {
            findings.push(DerivedExplanationCitationValidationFinding::blocker(
                DerivedExplanationCitationValidationKind::RawBoundaryMaterialPresent,
                "export contains forbidden raw boundary material",
            ));
        }

        if check_promotion {
            let derived = promotion_state_for(&findings);
            if self.promotion_state != derived {
                findings.push(DerivedExplanationCitationValidationFinding::blocker(
                    DerivedExplanationCitationValidationKind::PromotionStateMismatch,
                    "stored promotion state disagrees with derived findings",
                ));
            }
        }

        findings
    }

    fn validate_source_contracts(
        &self,
        findings: &mut Vec<DerivedExplanationCitationValidationFinding>,
    ) {
        let refs: BTreeSet<&str> = self
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(DERIVED_EXPLANATION_CITATION_SCHEMA_REF)
            || !refs.contains(DERIVED_EXPLANATION_CITATION_DOC_REF)
        {
            findings.push(DerivedExplanationCitationValidationFinding::blocker(
                DerivedExplanationCitationValidationKind::MissingSourceContracts,
                "source contract refs omit the schema or contract doc",
            ));
        }
    }

    fn validate_citation_sets(
        &self,
        findings: &mut Vec<DerivedExplanationCitationValidationFinding>,
    ) {
        if self.citation_sets.is_empty() {
            findings.push(DerivedExplanationCitationValidationFinding::blocker(
                DerivedExplanationCitationValidationKind::MissingCitationSets,
                "packet must declare at least one citation set",
            ));
        }

        let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
        let mut seen_explanations: BTreeSet<&str> = BTreeSet::new();
        for citation_set in &self.citation_sets {
            if !citation_set.is_well_formed() {
                findings.push(DerivedExplanationCitationValidationFinding::blocker(
                    DerivedExplanationCitationValidationKind::CitationSetIncomplete,
                    format!(
                        "citation set {} drops a required identity field",
                        citation_set.citation_set_id
                    ),
                ));
            }
            if !citation_set.graph_epoch.is_well_formed() {
                findings.push(DerivedExplanationCitationValidationFinding::blocker(
                    DerivedExplanationCitationValidationKind::GraphEpochMissing,
                    format!(
                        "citation set {} drops its graph epoch",
                        citation_set.citation_set_id
                    ),
                ));
            }
            if !citation_set.derivation.is_well_formed() {
                findings.push(DerivedExplanationCitationValidationFinding::blocker(
                    DerivedExplanationCitationValidationKind::DerivationToolMissing,
                    format!(
                        "citation set {} drops its derivation tool/version",
                        citation_set.citation_set_id
                    ),
                ));
            }
            if !citation_set.citation_set_id.trim().is_empty()
                && !seen_ids.insert(citation_set.citation_set_id.as_str())
            {
                findings.push(DerivedExplanationCitationValidationFinding::blocker(
                    DerivedExplanationCitationValidationKind::DuplicateCitationSetId,
                    format!("duplicate citation set id {}", citation_set.citation_set_id),
                ));
            }
            if !citation_set.explanation_id.trim().is_empty()
                && !seen_explanations.insert(citation_set.explanation_id.as_str())
            {
                findings.push(DerivedExplanationCitationValidationFinding::blocker(
                    DerivedExplanationCitationValidationKind::DuplicateExplanationBinding,
                    format!(
                        "two citation sets bind explanation {}",
                        citation_set.explanation_id
                    ),
                ));
            }

            self.validate_citation_set_basis(citation_set, findings);
        }
    }

    fn validate_citation_set_basis(
        &self,
        citation_set: &DerivedExplanationCitationSet,
        findings: &mut Vec<DerivedExplanationCitationValidationFinding>,
    ) {
        match citation_set.basis {
            CitationBasis::DirectCitation if !citation_set.has_any_citation() => {
                findings.push(DerivedExplanationCitationValidationFinding::blocker(
                    DerivedExplanationCitationValidationKind::CitationBasisMissing,
                    format!(
                        "direct-citation set {} cites no file, symbol, or docs node",
                        citation_set.citation_set_id
                    ),
                ));
            }
            CitationBasis::LabeledInference
                if citation_set
                    .inference_label
                    .as_ref()
                    .map_or(true, |label| !label.is_well_formed()) =>
            {
                findings.push(DerivedExplanationCitationValidationFinding::blocker(
                    DerivedExplanationCitationValidationKind::InferenceLabelMissing,
                    format!(
                        "labeled-inference set {} drops its inference label or reason",
                        citation_set.citation_set_id
                    ),
                ));
            }
            _ => {}
        }

        if !citation_set.basis_consistent() {
            findings.push(DerivedExplanationCitationValidationFinding::blocker(
                DerivedExplanationCitationValidationKind::CitationBasisMissing,
                format!(
                    "citation set {} basis {} disagrees with its citations and label",
                    citation_set.citation_set_id,
                    citation_set.basis.as_str()
                ),
            ));
        }
        if !citation_set.trust_consistent() {
            findings.push(DerivedExplanationCitationValidationFinding::blocker(
                DerivedExplanationCitationValidationKind::BasisTrustInconsistent,
                format!(
                    "citation set {} trust/source class disagrees with basis {}",
                    citation_set.citation_set_id,
                    citation_set.basis.as_str()
                ),
            ));
        }
        if !citation_set.basis_preserved_through_redaction() {
            findings.push(DerivedExplanationCitationValidationFinding::blocker(
                DerivedExplanationCitationValidationKind::RedactionDropsCitationBasis,
                format!(
                    "redacted citation set {} lost its citation basis",
                    citation_set.citation_set_id
                ),
            ));
        }
        if !citation_set.raw_boundary_material_excluded {
            findings.push(DerivedExplanationCitationValidationFinding::blocker(
                DerivedExplanationCitationValidationKind::RawBoundaryMaterialPresent,
                format!(
                    "citation set {} retains raw boundary material",
                    citation_set.citation_set_id
                ),
            ));
        }
        if citation_set.is_stale_direct_citation() {
            findings.push(DerivedExplanationCitationValidationFinding::warning(
                DerivedExplanationCitationValidationKind::CitationFreshnessNarrowed,
                format!(
                    "direct-citation set {} rests on {} freshness and narrows below stable",
                    citation_set.citation_set_id,
                    citation_set.freshness.as_str()
                ),
            ));
        }
        if citation_set.is_speculative_inference() {
            findings.push(DerivedExplanationCitationValidationFinding::warning(
                DerivedExplanationCitationValidationKind::SpeculativeInferenceNarrowed,
                format!(
                    "labeled-inference set {} is speculative and narrows below stable",
                    citation_set.citation_set_id
                ),
            ));
        }
    }

    fn validate_surface_coverage(
        &self,
        findings: &mut Vec<DerivedExplanationCitationValidationFinding>,
    ) {
        let covered: BTreeSet<DerivedExplanationSurface> = self
            .citation_sets
            .iter()
            .map(|set| set.explanation_surface)
            .collect();
        for required in DerivedExplanationSurface::REQUIRED {
            if !covered.contains(&required) {
                findings.push(DerivedExplanationCitationValidationFinding::blocker(
                    DerivedExplanationCitationValidationKind::SurfaceCoverageMissing,
                    format!(
                        "no citation set binds an explanation on the {} surface",
                        required.as_str()
                    ),
                ));
                break;
            }
        }
    }

    fn validate_projections(
        &self,
        findings: &mut Vec<DerivedExplanationCitationValidationFinding>,
    ) {
        let present: BTreeSet<DerivedExplanationSurface> = self
            .consumer_projections
            .iter()
            .map(|projection| projection.surface)
            .collect();
        for required in DerivedExplanationSurface::REQUIRED {
            if !present.contains(&required) {
                findings.push(DerivedExplanationCitationValidationFinding::blocker(
                    DerivedExplanationCitationValidationKind::RequiredSurfaceProjectionMissing,
                    format!(
                        "no projection reuses the packet on the {} surface",
                        required.as_str()
                    ),
                ));
                break;
            }
        }

        let known_ids: BTreeSet<&str> = self
            .citation_sets
            .iter()
            .map(|set| set.citation_set_id.as_str())
            .collect();

        for projection in &self.consumer_projections {
            if projection.packet_id_ref != self.packet_id {
                findings.push(DerivedExplanationCitationValidationFinding::blocker(
                    DerivedExplanationCitationValidationKind::ConsumerProjectionPacketIdMismatch,
                    format!(
                        "surface {} references packet {}",
                        projection.surface.as_str(),
                        projection.packet_id_ref
                    ),
                ));
            }
            if !projection.preserves_required_flags() {
                findings.push(DerivedExplanationCitationValidationFinding::blocker(
                    DerivedExplanationCitationValidationKind::ConsumerProjectionDropsReuse,
                    format!(
                        "surface {} drops a required citation-reuse flag",
                        projection.surface.as_str()
                    ),
                ));
            }
            if projection.citation_set_id_refs.is_empty() {
                findings.push(DerivedExplanationCitationValidationFinding::blocker(
                    DerivedExplanationCitationValidationKind::ConsumerProjectionDropsReuse,
                    format!(
                        "surface {} reuses no shared citation set",
                        projection.surface.as_str()
                    ),
                ));
            }
            for set_ref in &projection.citation_set_id_refs {
                if !known_ids.contains(set_ref.as_str()) {
                    findings.push(DerivedExplanationCitationValidationFinding::blocker(
                        DerivedExplanationCitationValidationKind::ConsumerProjectionOrphanCitationRef,
                        format!(
                            "surface {} references unknown citation set {}",
                            projection.surface.as_str(),
                            set_ref
                        ),
                    ));
                }
            }
        }

        self.validate_support_export_coverage(&known_ids, findings);
    }

    fn validate_support_export_coverage(
        &self,
        known_ids: &BTreeSet<&str>,
        findings: &mut Vec<DerivedExplanationCitationValidationFinding>,
    ) {
        let Some(projection) = self
            .consumer_projections
            .iter()
            .find(|projection| projection.surface == DerivedExplanationSurface::SupportExportNote)
        else {
            return;
        };
        let exported: BTreeSet<&str> = projection
            .citation_set_id_refs
            .iter()
            .map(String::as_str)
            .collect();
        for set_id in known_ids {
            if !exported.contains(set_id) {
                findings.push(DerivedExplanationCitationValidationFinding::blocker(
                    DerivedExplanationCitationValidationKind::SupportExportDropsCitationBasis,
                    format!("support export drops the citation basis for citation set {set_id}"),
                ));
                break;
            }
        }
    }
}

/// Support-export wrapper preserving the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedExplanationCitationSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Exported packet id.
    pub export_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// True when the citation basis is preserved across the export boundary.
    pub citation_basis_preserved: bool,
    /// Exact packet preserved by the export.
    pub export_packet: DerivedExplanationCitationPacket,
}

impl DerivedExplanationCitationSupportExport {
    /// Returns true when the export preserves the same packet safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == DERIVED_EXPLANATION_CITATION_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == DERIVED_EXPLANATION_CITATION_SCHEMA_VERSION
            && self.export_packet_id_ref == self.export_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.citation_basis_preserved
            && self.export_packet.validate().is_empty()
    }
}

/// Errors emitted while reading the checked-in citation-set export.
#[derive(Debug)]
pub enum DerivedExplanationCitationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export's packet failed validation.
    Validation(Vec<DerivedExplanationCitationValidationFinding>),
    /// Support export wrapper is not export-safe.
    NotExportSafe,
}

impl fmt::Display for DerivedExplanationCitationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "derived explanation citation export parse failed: {error}"
                )
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "derived explanation citation export failed validation: {tokens}"
                )
            }
            Self::NotExportSafe => {
                write!(
                    formatter,
                    "derived explanation citation export wrapper is not export-safe"
                )
            }
        }
    }
}

impl Error for DerivedExplanationCitationArtifactError {}

/// Returns the seeded stable citation-set packet input.
pub fn seeded_stable_derived_explanation_citation_input() -> DerivedExplanationCitationPacketInput {
    seed::seeded_input()
}

/// Materializes the checked-in stable citation-set packet.
///
/// # Errors
///
/// Returns an error when the seeded packet fails its own stable invariants.
pub fn current_stable_derived_explanation_citation_packet(
) -> Result<DerivedExplanationCitationPacket, DerivedExplanationCitationArtifactError> {
    let packet = DerivedExplanationCitationPacket::materialize(
        seeded_stable_derived_explanation_citation_input(),
    );
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(DerivedExplanationCitationArtifactError::Validation(
            findings,
        ))
    }
}

/// Reads and validates the checked-in stable support export.
///
/// # Errors
///
/// Returns an error when the checked artifact fails to parse, is not
/// export-safe, or its packet fails validation.
pub fn current_stable_derived_explanation_citation_export(
) -> Result<DerivedExplanationCitationSupportExport, DerivedExplanationCitationArtifactError> {
    let export: DerivedExplanationCitationSupportExport = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/docs/m5/implement_derived_explanation_citation_sets_binding_docs_ai_glossary_tours_and_support_exports/support_export.json"
        )
    ))
    .map_err(DerivedExplanationCitationArtifactError::SupportExport)?;
    let findings = export.export_packet.validate();
    if !findings.is_empty() {
        return Err(DerivedExplanationCitationArtifactError::Validation(
            findings,
        ));
    }
    if !export.is_export_safe() {
        return Err(DerivedExplanationCitationArtifactError::NotExportSafe);
    }
    Ok(export)
}

fn promotion_state_for(
    validation: &[DerivedExplanationCitationValidationFinding],
) -> DerivedExplanationCitationPromotionState {
    if validation
        .iter()
        .any(|finding| finding.severity == DerivedExplanationCitationValidationSeverity::Blocker)
    {
        return DerivedExplanationCitationPromotionState::BlocksStable;
    }
    if validation
        .iter()
        .any(|finding| finding.severity == DerivedExplanationCitationValidationSeverity::Warning)
    {
        return DerivedExplanationCitationPromotionState::NarrowedBelowStable;
    }
    DerivedExplanationCitationPromotionState::Stable
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => {
            let lower = text.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("raw_body:")
                || lower.contains("prompt_text:")
        }
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

mod seed {
    use super::*;

    pub(super) const PACKET_ID: &str = "packet:derived_explanation_citation_sets:001";

    const EPOCH_REF: &str = "graph-epoch:workspace@2026-06-26T00:00:00Z";
    const WORKSPACE_REVISION_REF: &str = "workspace-revision:main@a1b2c3d";
    const CAPTURED_AT: &str = "2026-06-26T00:00:00Z";

    fn epoch() -> GraphEpochRef {
        GraphEpochRef {
            epoch_ref: EPOCH_REF.to_owned(),
            workspace_revision_ref: WORKSPACE_REVISION_REF.to_owned(),
            captured_at: CAPTURED_AT.to_owned(),
        }
    }

    fn tool(model_ref: Option<&str>) -> DerivationTool {
        DerivationTool {
            tool_ref: "derivation-tool:aureline-explainer".to_owned(),
            tool_version_ref: "explainer@2026.06".to_owned(),
            model_ref: model_ref.map(str::to_owned),
        }
    }

    fn file_ref(
        path: &str,
        digest: &str,
        line_span: Option<&str>,
        source_class: DocsContractSourceClass,
    ) -> CitedFileRef {
        CitedFileRef {
            file_path_ref: path.to_owned(),
            content_digest_ref: digest.to_owned(),
            line_span_ref: line_span.map(str::to_owned),
            source_class,
        }
    }

    fn symbol_ref(
        symbol: &str,
        container: Option<&str>,
        graph_node: &str,
        kind: &str,
    ) -> CitedSymbolRef {
        CitedSymbolRef {
            symbol_ref: symbol.to_owned(),
            container_ref: container.map(str::to_owned),
            graph_node_ref: graph_node.to_owned(),
            kind_label: kind.to_owned(),
        }
    }

    fn doc_ref(
        doc_node: &str,
        source_class: DocsContractSourceClass,
        version_match: DocsContractVersionMatchState,
        freshness: DocsContractFreshnessState,
        trust_class: DocsContractTrustClass,
    ) -> CitedDocRef {
        CitedDocRef {
            doc_node_ref: doc_node.to_owned(),
            source_class,
            version_match,
            freshness,
            locale: DocsContractLocaleMatch::SourceLanguageOriginal,
            trust_class,
        }
    }

    fn docs_browser_set() -> DerivedExplanationCitationSet {
        DerivedExplanationCitationSet {
            citation_set_id: "citation-set:docs_browser:tokio-spawn".to_owned(),
            explanation_id: "explanation:docs_browser:tokio-spawn-peek".to_owned(),
            explanation_surface: DerivedExplanationSurface::DocsBrowserExplanation,
            explanation_label: "tokio::spawn peek explanation".to_owned(),
            basis: CitationBasis::DirectCitation,
            source_class: DocsContractSourceClass::MirroredOfficialDocs,
            trust_class: DocsContractTrustClass::SignedMirrorVerified,
            freshness: DocsContractFreshnessState::WarmCached,
            locale: DocsContractLocaleMatch::SourceLanguageOriginal,
            cited_files: Vec::new(),
            cited_symbols: vec![symbol_ref(
                "symbol:tokio::runtime::Runtime::spawn",
                Some("container:tokio::runtime::Runtime"),
                "graph-node:tokio::runtime::Runtime::spawn",
                "function",
            )],
            cited_docs: vec![doc_ref(
                "docnode:mirror:tokio/runtime#spawn",
                DocsContractSourceClass::MirroredOfficialDocs,
                DocsContractVersionMatchState::ExactBuildMatch,
                DocsContractFreshnessState::WarmCached,
                DocsContractTrustClass::SignedMirrorVerified,
            )],
            graph_epoch: epoch(),
            derivation: tool(None),
            inference_label: None,
            redaction: CitationRedactionState::ContentInlinePreserved,
            raw_boundary_material_excluded: true,
        }
    }

    fn ai_answer_set() -> DerivedExplanationCitationSet {
        DerivedExplanationCitationSet {
            citation_set_id: "citation-set:ai_answer:workspace-runtime".to_owned(),
            explanation_id: "explanation:ai_answer:where-is-the-runtime-built".to_owned(),
            explanation_surface: DerivedExplanationSurface::AiAnswer,
            explanation_label: "AI answer: where the runtime is built".to_owned(),
            basis: CitationBasis::DirectCitation,
            source_class: DocsContractSourceClass::ProjectDocs,
            trust_class: DocsContractTrustClass::FirstPartyAuthoritative,
            freshness: DocsContractFreshnessState::AuthoritativeLive,
            locale: DocsContractLocaleMatch::SourceLanguageOriginal,
            cited_files: vec![
                file_ref(
                    "crates/aureline-runtime/src/lib.rs",
                    "digest:sha256:runtime-lib@a1b2c3d",
                    Some("L40-L96"),
                    DocsContractSourceClass::ProjectDocs,
                ),
                file_ref(
                    "crates/aureline-runtime/src/builder/mod.rs",
                    "digest:sha256:runtime-builder@a1b2c3d",
                    Some("L12-L58"),
                    DocsContractSourceClass::ProjectDocs,
                ),
            ],
            cited_symbols: vec![symbol_ref(
                "symbol:aureline_runtime::builder::RuntimeBuilder::build",
                Some("container:aureline_runtime::builder::RuntimeBuilder"),
                "graph-node:aureline_runtime::builder::RuntimeBuilder::build",
                "function",
            )],
            cited_docs: Vec::new(),
            graph_epoch: epoch(),
            derivation: tool(Some("model:assistant@m5")),
            inference_label: None,
            redaction: CitationRedactionState::ContentInlinePreserved,
            raw_boundary_material_excluded: true,
        }
    }

    fn glossary_set() -> DerivedExplanationCitationSet {
        DerivedExplanationCitationSet {
            citation_set_id: "citation-set:glossary:truth-packet".to_owned(),
            explanation_id: "explanation:glossary:truth-packet-card".to_owned(),
            explanation_surface: DerivedExplanationSurface::GlossaryCard,
            explanation_label: "Glossary card: truth packet".to_owned(),
            basis: CitationBasis::DirectCitation,
            source_class: DocsContractSourceClass::CuratedKnowledgePack,
            trust_class: DocsContractTrustClass::FirstPartyAuthoritative,
            freshness: DocsContractFreshnessState::AuthoritativeLive,
            locale: DocsContractLocaleMatch::SourceLanguageOriginal,
            cited_files: Vec::new(),
            cited_symbols: Vec::new(),
            cited_docs: vec![doc_ref(
                "docnode:knowledge-pack:glossary/truth-packet",
                DocsContractSourceClass::CuratedKnowledgePack,
                DocsContractVersionMatchState::ExactBuildMatch,
                DocsContractFreshnessState::AuthoritativeLive,
                DocsContractTrustClass::FirstPartyAuthoritative,
            )],
            graph_epoch: epoch(),
            derivation: tool(None),
            inference_label: None,
            redaction: CitationRedactionState::ContentInlinePreserved,
            raw_boundary_material_excluded: true,
        }
    }

    fn guided_tour_set() -> DerivedExplanationCitationSet {
        DerivedExplanationCitationSet {
            citation_set_id: "citation-set:guided_tour:open-workspace".to_owned(),
            explanation_id: "explanation:guided_tour:open-workspace-step".to_owned(),
            explanation_surface: DerivedExplanationSurface::GuidedTourStep,
            explanation_label: "Guided tour: open a workspace".to_owned(),
            basis: CitationBasis::DirectCitation,
            source_class: DocsContractSourceClass::ProjectDocs,
            trust_class: DocsContractTrustClass::FirstPartyAuthoritative,
            freshness: DocsContractFreshnessState::AuthoritativeLive,
            locale: DocsContractLocaleMatch::TranslatedComplete,
            cited_files: Vec::new(),
            cited_symbols: vec![symbol_ref(
                "symbol:aureline_shell::commands::open_workspace",
                Some("container:aureline_shell::commands"),
                "graph-node:aureline_shell::commands::open_workspace",
                "function",
            )],
            cited_docs: vec![doc_ref(
                "docnode:project-docs:onboarding/open-workspace",
                DocsContractSourceClass::ProjectDocs,
                DocsContractVersionMatchState::ExactBuildMatch,
                DocsContractFreshnessState::AuthoritativeLive,
                DocsContractTrustClass::FirstPartyAuthoritative,
            )],
            graph_epoch: epoch(),
            derivation: tool(None),
            inference_label: None,
            redaction: CitationRedactionState::ContentInlinePreserved,
            raw_boundary_material_excluded: true,
        }
    }

    fn architecture_set() -> DerivedExplanationCitationSet {
        DerivedExplanationCitationSet {
            citation_set_id: "citation-set:architecture:docs-cone".to_owned(),
            explanation_id: "explanation:architecture:docs-dependency-cone".to_owned(),
            explanation_surface: DerivedExplanationSurface::ArchitectureExplainer,
            explanation_label: "Architecture explainer: docs dependency cone".to_owned(),
            basis: CitationBasis::LabeledInference,
            source_class: DocsContractSourceClass::DerivedExplanation,
            trust_class: DocsContractTrustClass::DerivedInferenceOnly,
            freshness: DocsContractFreshnessState::AuthoritativeLive,
            locale: DocsContractLocaleMatch::SourceLanguageOriginal,
            cited_files: Vec::new(),
            cited_symbols: Vec::new(),
            cited_docs: Vec::new(),
            graph_epoch: epoch(),
            derivation: tool(Some("model:assistant@m5")),
            inference_label: Some(InferenceLabel {
                reason: "no single authored doc describes the full dependency cone; the shape is inferred from the package graph".to_owned(),
                inferred_from_summary: "module dependency edges and ownership metadata over the cited graph epoch".to_owned(),
                confidence: InferenceConfidence::Grounded,
            }),
            redaction: CitationRedactionState::ContentInlinePreserved,
            raw_boundary_material_excluded: true,
        }
    }

    fn support_export_set() -> DerivedExplanationCitationSet {
        DerivedExplanationCitationSet {
            citation_set_id: "citation-set:support_export:redacted-finding".to_owned(),
            explanation_id: "explanation:support_export:redacted-finding-note".to_owned(),
            explanation_surface: DerivedExplanationSurface::SupportExportNote,
            explanation_label: "Support export note: redacted finding".to_owned(),
            basis: CitationBasis::DirectCitation,
            source_class: DocsContractSourceClass::ProjectDocs,
            trust_class: DocsContractTrustClass::FirstPartyAuthoritative,
            freshness: DocsContractFreshnessState::WarmCached,
            locale: DocsContractLocaleMatch::SourceLanguageOriginal,
            cited_files: vec![file_ref(
                "crates/aureline-support/src/redacted/mod.rs",
                "digest:sha256:support-redacted@a1b2c3d",
                Some("L1-L20"),
                DocsContractSourceClass::ProjectDocs,
            )],
            cited_symbols: Vec::new(),
            cited_docs: Vec::new(),
            graph_epoch: epoch(),
            derivation: tool(None),
            inference_label: None,
            redaction: CitationRedactionState::ContentRedactedBasisPreserved,
            raw_boundary_material_excluded: true,
        }
    }

    pub(super) fn citation_sets() -> Vec<DerivedExplanationCitationSet> {
        vec![
            docs_browser_set(),
            ai_answer_set(),
            glossary_set(),
            guided_tour_set(),
            architecture_set(),
            support_export_set(),
        ]
    }

    fn projection(
        surface: DerivedExplanationSurface,
        citation_set_id_refs: Vec<String>,
    ) -> CitationConsumerProjection {
        CitationConsumerProjection {
            surface,
            projection_ref: format!("projection:{}:{}", PACKET_ID, surface.as_str()),
            packet_id_ref: PACKET_ID.to_owned(),
            reuses_shared_citation_object: true,
            preserves_inference_label: true,
            preserves_citation_basis_on_export: true,
            citation_set_id_refs,
        }
    }

    pub(super) fn projections() -> Vec<CitationConsumerProjection> {
        let all_ids: Vec<String> = citation_sets()
            .iter()
            .map(|set| set.citation_set_id.clone())
            .collect();
        vec![
            projection(
                DerivedExplanationSurface::DocsBrowserExplanation,
                vec!["citation-set:docs_browser:tokio-spawn".to_owned()],
            ),
            projection(
                DerivedExplanationSurface::AiAnswer,
                vec!["citation-set:ai_answer:workspace-runtime".to_owned()],
            ),
            projection(
                DerivedExplanationSurface::GlossaryCard,
                vec!["citation-set:glossary:truth-packet".to_owned()],
            ),
            projection(
                DerivedExplanationSurface::GuidedTourStep,
                vec!["citation-set:guided_tour:open-workspace".to_owned()],
            ),
            projection(
                DerivedExplanationSurface::ArchitectureExplainer,
                vec!["citation-set:architecture:docs-cone".to_owned()],
            ),
            // The support export reuses every citation set so an export never
            // silently drops a derived explanation's evidence basis.
            projection(DerivedExplanationSurface::SupportExportNote, all_ids),
        ]
    }

    pub(super) fn seeded_input() -> DerivedExplanationCitationPacketInput {
        DerivedExplanationCitationPacketInput {
            packet_id: PACKET_ID.to_owned(),
            surface_label:
                "workflow:derived_explanation_citation_sets_binding_docs_ai_glossary_tours_and_support_exports:stable"
                    .to_owned(),
            generated_at: "2026-06-26T00:00:00Z".to_owned(),
            citation_sets: citation_sets(),
            consumer_projections: projections(),
            source_contract_refs: vec![
                DERIVED_EXPLANATION_CITATION_SCHEMA_REF.to_owned(),
                DERIVED_EXPLANATION_CITATION_DOC_REF.to_owned(),
                DERIVED_EXPLANATION_CITATION_ARTIFACT_REF.to_owned(),
                DERIVED_EXPLANATION_CITATION_SUMMARY_REF.to_owned(),
                DERIVED_EXPLANATION_CITATION_MATRIX_CONTRACT_REF.to_owned(),
            ],
            redaction_class_token: "metadata_safe_default".to_owned(),
        }
    }
}
