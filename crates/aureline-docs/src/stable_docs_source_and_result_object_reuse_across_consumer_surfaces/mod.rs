//! Canonical docs-source descriptors and docs-result objects, reused across the
//! docs search, symbol-linked reference card, hover/peek docs, AI citation,
//! glossary card, and support-export surfaces.
//!
//! This module materializes the two foundational documentation objects that the
//! M5 docs object model is built from — the [`DocsSourceDescriptor`] and the
//! [`DocsResult`] — and proves that the *same* objects are reused, with their
//! identity preserved, across every consuming surface. The matrix lane freezes
//! *which* governed objects exist and the governance around them; this lane
//! materializes concrete object instances and binds them to per-surface
//! projections so docs search, symbol-linked reference cards, hover/peek docs,
//! AI citations, glossary cards, and support exports share one typed description
//! of documentation truth instead of re-deriving source/version/freshness state
//! ad hoc.
//!
//! Each source descriptor carries its source class, provider or pack identity,
//! BCP-47 locale, trust class, browser-handoff capability, mirror/offline
//! posture, version-match state, and freshness state. Each result carries a
//! stable result id, a title, a ref to its source descriptor, the version-match
//! and freshness state it observed, the symbol refs or citation anchors that
//! back it, snippet metadata that never forces full-content export, and a
//! support/export-safe identity. A [`DocsObjectSurfaceProjection`] records, per
//! consuming surface, that the source class, version match, freshness, trust
//! class, and symbol/citation linkage stay visible and that result identity is
//! preserved without forcing full content.
//!
//! The [`DocsObjectReusePacket`] validates the cross-cutting invariants: project
//! documentation, mirrored official docs, extension-contributed docs, live
//! external docs, and derived explanations stay distinguishable; project docs
//! never masquerade as vendor docs; derived explanations never claim primary
//! authority; live external docs always require an explicit browser handoff; a
//! result never silently upgrades the version-match or freshness state of its
//! source; and no surface (including support export) forces full-content export.
//! The packet reuses the canonical source-class, version-match, freshness,
//! mirror/offline, external-open, and inference-marker vocabularies already
//! owned by this crate rather than minting parallel tokens. Raw document bodies,
//! raw source files, raw URLs, raw provider payloads, and credentials never
//! cross this boundary.
//!
//! The boundary schema is
//! [`schemas/docs/stable-docs-source-and-result-object-reuse-across-consumer-surfaces.schema.json`](../../../../schemas/docs/stable-docs-source-and-result-object-reuse-across-consumer-surfaces.schema.json).
//! The contract doc is
//! [`docs/docs/m5/stable_docs_source_and_result_object_reuse_across_consumer_surfaces.md`](../../../../docs/docs/m5/stable_docs_source_and_result_object_reuse_across_consumer_surfaces.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/stable_docs_source_and_result_object_reuse_across_consumer_surfaces/`](../../../../fixtures/docs/m5/stable_docs_source_and_result_object_reuse_across_consumer_surfaces/).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    CitationInferenceMarker, CitationSourceClass, DocsExternalOpenState, DocsFreshnessClass,
    DocsMirrorOfflinePosture, SourcePrecedenceClass, VersionMatchState,
};

/// Stable record-kind tag carried by [`DocsObjectReusePacket`].
pub const DOCS_SOURCE_RESULT_REUSE_RECORD_KIND: &str =
    "stable_docs_source_and_result_object_reuse_packet";

/// Stable record-kind tag carried by [`DocsObjectReuseSupportExport`].
pub const DOCS_SOURCE_RESULT_REUSE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "stable_docs_source_and_result_object_reuse_support_export";

/// Schema version for docs-source/result reuse records.
pub const DOCS_SOURCE_RESULT_REUSE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DOCS_SOURCE_RESULT_REUSE_SCHEMA_REF: &str =
    "schemas/docs/stable-docs-source-and-result-object-reuse-across-consumer-surfaces.schema.json";

/// Repo-relative path of the contract doc.
pub const DOCS_SOURCE_RESULT_REUSE_DOC_REF: &str =
    "docs/docs/m5/stable_docs_source_and_result_object_reuse_across_consumer_surfaces.md";

/// Repo-relative path of the checked support-export artifact.
pub const DOCS_SOURCE_RESULT_REUSE_ARTIFACT_REF: &str =
    "artifacts/docs/m5/stable_docs_source_and_result_object_reuse_across_consumer_surfaces/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const DOCS_SOURCE_RESULT_REUSE_SUMMARY_REF: &str =
    "artifacts/docs/m5/stable_docs_source_and_result_object_reuse_across_consumer_surfaces.md";

/// Repo-relative path of the protected fixture directory.
pub const DOCS_SOURCE_RESULT_REUSE_FIXTURE_DIR: &str =
    "fixtures/docs/m5/stable_docs_source_and_result_object_reuse_across_consumer_surfaces";

/// Controlled trust class for a documentation source.
///
/// The trust class must stay consistent with the [`CitationSourceClass`] of the
/// source so that project documentation can never be relabeled with a vendor or
/// provider trust class, and a derived explanation can never claim a first-party
/// trust class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsObjectTrustClass {
    /// First-party authoritative source (workspace-owned project docs or
    /// reference generated against the running build).
    FirstPartyAuthoritative,
    /// Mirror signed and verified against the published upstream source.
    SignedMirrorVerified,
    /// Extension-contributed pack signed by a verified publisher.
    ExtensionPackSigned,
    /// Curated knowledge pack with a declared owner and support commitment.
    CuratedSupported,
    /// Live provider source resolved through an explicit browser handoff.
    LiveProviderHandoff,
    /// Derived inference; never primary authority.
    DerivedInferenceOnly,
}

impl DocsObjectTrustClass {
    /// Every trust class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FirstPartyAuthoritative,
        Self::SignedMirrorVerified,
        Self::ExtensionPackSigned,
        Self::CuratedSupported,
        Self::LiveProviderHandoff,
        Self::DerivedInferenceOnly,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyAuthoritative => "first_party_authoritative",
            Self::SignedMirrorVerified => "signed_mirror_verified",
            Self::ExtensionPackSigned => "extension_pack_signed",
            Self::CuratedSupported => "curated_supported",
            Self::LiveProviderHandoff => "live_provider_handoff",
            Self::DerivedInferenceOnly => "derived_inference_only",
        }
    }

    /// Returns true when this trust class is admissible for `source_class`.
    ///
    /// This is the invariant that keeps project docs from masquerading as vendor
    /// docs and derived explanations from claiming a first-party trust class.
    pub fn is_admissible_for(self, source_class: CitationSourceClass) -> bool {
        match source_class {
            CitationSourceClass::ProjectDocs
            | CitationSourceClass::GeneratedReference
            | CitationSourceClass::SupportRunbook => self == Self::FirstPartyAuthoritative,
            CitationSourceClass::MirroredOfficialDocs => self == Self::SignedMirrorVerified,
            CitationSourceClass::CuratedKnowledgePack => {
                matches!(self, Self::ExtensionPackSigned | Self::CuratedSupported)
            }
            CitationSourceClass::VendorProviderDocs => self == Self::LiveProviderHandoff,
            CitationSourceClass::DerivedExplanation => self == Self::DerivedInferenceOnly,
        }
    }
}

/// Consuming surface that must reuse the shared source/result objects.
///
/// These are exactly the surfaces the docs object model must keep in parity:
/// docs search, symbol-linked reference cards, hover/peek docs, AI citations,
/// glossary cards, and support exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsObjectConsumerSurface {
    /// Docs and code search result list.
    DocsSearch,
    /// Symbol-linked reference card (hover or peek over a symbol).
    SymbolReferenceCard,
    /// Hover/peek documentation popover.
    HoverPeekDocs,
    /// AI explanation citation drawer.
    AiCitation,
    /// Glossary / learning card.
    GlossaryCard,
    /// Support bundle export.
    SupportExport,
}

impl DocsObjectConsumerSurface {
    /// Every required consuming surface, in declaration order.
    pub const REQUIRED: [Self; 6] = [
        Self::DocsSearch,
        Self::SymbolReferenceCard,
        Self::HoverPeekDocs,
        Self::AiCitation,
        Self::GlossaryCard,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsSearch => "docs_search",
            Self::SymbolReferenceCard => "symbol_reference_card",
            Self::HoverPeekDocs => "hover_peek_docs",
            Self::AiCitation => "ai_citation",
            Self::GlossaryCard => "glossary_card",
            Self::SupportExport => "support_export",
        }
    }
}

/// Snippet metadata for a docs result.
///
/// The metadata locates the previewed excerpt without ever carrying the full
/// document body; [`DocsSnippetMeta::full_content_excluded`] must stay true so
/// support export and browser handoff preserve result identity without forcing
/// full-content export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSnippetMeta {
    /// Stable anchor the snippet renders against.
    pub snippet_anchor_ref: String,
    /// First line of the previewed excerpt (1-based).
    pub start_line: u32,
    /// Last line of the previewed excerpt (1-based, inclusive).
    pub end_line: u32,
    /// Upper bound on the rendered excerpt size in characters.
    pub excerpt_char_budget: u32,
    /// True when the full document body is excluded from this metadata.
    pub full_content_excluded: bool,
    /// Disclosure note for a redacted or truncated excerpt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redaction_note: Option<String>,
}

impl DocsSnippetMeta {
    fn is_well_formed(&self) -> bool {
        !self.snippet_anchor_ref.trim().is_empty() && self.end_line >= self.start_line
    }
}

/// Canonical docs-source descriptor reused by every consuming surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSourceDescriptor {
    /// Stable source id used by results, projections, and citations.
    pub source_id: String,
    /// Canonical source class.
    pub source_class: CitationSourceClass,
    /// Provider or pack id that owns this source.
    pub provider_or_pack_id: String,
    /// Provider or pack revision the descriptor was minted against.
    pub provider_or_pack_revision_ref: String,
    /// BCP-47 locale of the source content.
    pub locale: String,
    /// Trust class; must stay admissible for [`DocsSourceDescriptor::source_class`].
    pub trust_class: DocsObjectTrustClass,
    /// Browser or external-open posture.
    pub browser_handoff: DocsExternalOpenState,
    /// Handoff packet ref when an external open is available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Mirror/offline posture.
    pub mirror_offline_posture: DocsMirrorOfflinePosture,
    /// Precedence class used when several sources answer the same subject.
    pub precedence_class: SourcePrecedenceClass,
    /// Version-match state at mint time.
    pub version_match_state: VersionMatchState,
    /// Freshness state at mint time.
    pub freshness_state: DocsFreshnessClass,
    /// Pack manifest ref when the source resolves through a docs pack.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pack_manifest_ref: Option<String>,
    /// Disclosure note for derived, stale, drifted, handoff, or mirror posture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_note: Option<String>,
    /// True when raw bodies, raw URLs, secrets, and provider payloads are excluded.
    pub raw_boundary_material_excluded: bool,
}

impl DocsSourceDescriptor {
    fn is_well_formed(&self) -> bool {
        !self.source_id.trim().is_empty()
            && !self.provider_or_pack_id.trim().is_empty()
            && !self.provider_or_pack_revision_ref.trim().is_empty()
            && !self.locale.trim().is_empty()
    }

    fn is_derived_explanation(&self) -> bool {
        self.source_class == CitationSourceClass::DerivedExplanation
    }

    fn is_live_external(&self) -> bool {
        self.source_class == CitationSourceClass::VendorProviderDocs
    }

    /// Whether the descriptor must carry a disclosure note.
    fn requires_disclosure(&self) -> bool {
        self.browser_handoff.requires_disclosure()
            || self.freshness_state.lowers_certainty()
            || self.version_match_state != VersionMatchState::ExactBuildMatch
            || self.is_derived_explanation()
            || self.precedence_class == SourcePrecedenceClass::ProjectVendorDisagreementInspectable
    }

    fn has_disclosure(&self) -> bool {
        self.disclosure_note
            .as_deref()
            .map(|note| !note.trim().is_empty())
            .unwrap_or(false)
    }

    /// Whether a derived-explanation source keeps its inference-only posture.
    fn derived_guardrail_ok(&self) -> bool {
        self.trust_class == DocsObjectTrustClass::DerivedInferenceOnly
            && self.precedence_class == SourcePrecedenceClass::NotApplicable
            && self.has_disclosure()
    }

    /// Whether a live-external source resolves only through an explicit handoff.
    fn live_external_handoff_ok(&self) -> bool {
        self.browser_handoff == DocsExternalOpenState::Available
            && self.mirror_offline_posture == DocsMirrorOfflinePosture::LiveOnline
    }
}

/// Canonical docs-result object reused by every consuming surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsResult {
    /// Stable result id.
    pub result_id: String,
    /// User-visible title.
    pub title: String,
    /// Ref to the [`DocsSourceDescriptor`] that answered this result.
    pub docs_source_ref: String,
    /// Source class carried for projection; must match the referenced source.
    pub source_class: CitationSourceClass,
    /// Trust class carried for projection; must match the referenced source.
    pub trust_class: DocsObjectTrustClass,
    /// Version-match state; must match the referenced source (never upgraded).
    pub version_match_state: VersionMatchState,
    /// Freshness state; must match the referenced source (never upgraded).
    pub freshness_state: DocsFreshnessClass,
    /// Symbol refs backing this result, where present.
    #[serde(default)]
    pub symbol_refs: Vec<String>,
    /// Citation or docs anchor refs backing this result.
    #[serde(default)]
    pub citation_anchor_refs: Vec<String>,
    /// Snippet metadata for preview.
    pub snippet: DocsSnippetMeta,
    /// Inference markers shown in drawers and exports.
    #[serde(default)]
    pub inference_markers: Vec<CitationInferenceMarker>,
    /// Stable identity preserved by support export and browser handoff.
    pub support_export_safe_id: String,
    /// Disclosure note for a derived or degraded result.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_note: Option<String>,
    /// True when raw boundary material is excluded.
    pub raw_boundary_material_excluded: bool,
}

impl DocsResult {
    fn is_well_formed(&self) -> bool {
        !self.result_id.trim().is_empty()
            && !self.title.trim().is_empty()
            && !self.docs_source_ref.trim().is_empty()
            && !self.support_export_safe_id.trim().is_empty()
            && (!self.symbol_refs.is_empty() || !self.citation_anchor_refs.is_empty())
            && self.snippet.is_well_formed()
    }

    /// Whether the result agrees with its source on class, trust, version, and
    /// freshness so no surface can read a different truth for the same object.
    fn agrees_with(&self, source: &DocsSourceDescriptor) -> bool {
        self.source_class == source.source_class
            && self.trust_class == source.trust_class
            && self.version_match_state == source.version_match_state
            && self.freshness_state == source.freshness_state
    }
}

/// Per-surface projection proving a source/result pair is reused without drift.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsObjectSurfaceProjection {
    /// Consuming surface.
    pub consumer_surface: DocsObjectConsumerSurface,
    /// Stable projection id.
    pub projection_id: String,
    /// Result ref projected onto the surface.
    pub result_id_ref: String,
    /// Source ref projected onto the surface.
    pub source_id_ref: String,
    /// True when the surface shows the source class label.
    pub shows_source_class: bool,
    /// True when the surface shows the version-match state.
    pub shows_version_match: bool,
    /// True when the surface shows the freshness state.
    pub shows_freshness: bool,
    /// True when the surface shows the trust class.
    pub shows_trust_class: bool,
    /// True when the surface preserves the result's symbol/citation linkage.
    pub preserves_symbol_or_citation_refs: bool,
    /// True when the surface preserves result identity without re-minting it.
    pub preserves_result_identity: bool,
    /// True when the surface excludes full content (export-safe identity only).
    pub full_content_excluded: bool,
    /// True when the surface mints a private badge vocabulary instead of reusing
    /// the shared one; this must stay false.
    pub local_badge_vocabulary_used: bool,
}

impl DocsObjectSurfaceProjection {
    /// Whether the projection preserves the shared object model without drift.
    fn is_faithful(&self) -> bool {
        self.shows_source_class
            && self.shows_version_match
            && self.shows_freshness
            && self.shows_trust_class
            && self.preserves_symbol_or_citation_refs
            && self.preserves_result_identity
            && self.full_content_excluded
            && !self.local_badge_vocabulary_used
    }
}

/// Promotion state derived from the packet's validation findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsObjectPromotionState {
    /// Packet certifies the stable claim.
    Stable,
    /// Packet must remain narrowed below stable.
    NarrowedBelowStable,
    /// Packet blocks stable publication.
    BlocksStable,
}

impl DocsObjectPromotionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity for one validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsObjectFindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding.
    Warning,
    /// Blocker finding.
    Blocker,
}

/// Finding vocabulary for the docs-source/result reuse packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsObjectFindingKind {
    /// Record kind is wrong.
    WrongRecordKind,
    /// Schema version is wrong.
    WrongSchemaVersion,
    /// Packet identity is incomplete.
    MissingPacketIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A source descriptor is incomplete.
    SourceDescriptorIncomplete,
    /// A source descriptor's trust class is inadmissible for its source class.
    SourceTrustClassMismatch,
    /// The required distinguishable source classes are not all represented.
    SourceClassDistinguishabilityMissing,
    /// A derived explanation claims primary authority.
    DerivedExplanationMasqueradesAsPrimary,
    /// A live external source does not require an explicit handoff.
    LiveExternalDocsHandoffMissing,
    /// A required mirror/offline or handoff disclosure is missing.
    BoundaryDisclosureMissing,
    /// A result object is incomplete.
    ResultObjectIncomplete,
    /// A result references a source that is not present in the packet.
    ResultSourceRefUnresolved,
    /// A result disagrees with its source on class, trust, version, or freshness.
    SourceResultTruthMismatch,
    /// A result's snippet metadata would force full-content export.
    SnippetForcesFullContent,
    /// A required consuming surface has no projection.
    MissingConsumerSurface,
    /// A projection references a result or source not present in the packet.
    ProjectionRefUnresolved,
    /// A projection dropped shared object-model truth.
    ConsumerSurfaceProjectionDrift,
    /// Raw boundary material is present in the export.
    RawBoundaryMaterialPresent,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl DocsObjectFindingKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingPacketIdentity => "missing_packet_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::SourceDescriptorIncomplete => "source_descriptor_incomplete",
            Self::SourceTrustClassMismatch => "source_trust_class_mismatch",
            Self::SourceClassDistinguishabilityMissing => "source_class_distinguishability_missing",
            Self::DerivedExplanationMasqueradesAsPrimary => {
                "derived_explanation_masquerades_as_primary"
            }
            Self::LiveExternalDocsHandoffMissing => "live_external_docs_handoff_missing",
            Self::BoundaryDisclosureMissing => "boundary_disclosure_missing",
            Self::ResultObjectIncomplete => "result_object_incomplete",
            Self::ResultSourceRefUnresolved => "result_source_ref_unresolved",
            Self::SourceResultTruthMismatch => "source_result_truth_mismatch",
            Self::SnippetForcesFullContent => "snippet_forces_full_content",
            Self::MissingConsumerSurface => "missing_consumer_surface",
            Self::ProjectionRefUnresolved => "projection_ref_unresolved",
            Self::ConsumerSurfaceProjectionDrift => "consumer_surface_projection_drift",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsObjectValidationFinding {
    /// Finding kind.
    pub finding_kind: DocsObjectFindingKind,
    /// Severity.
    pub severity: DocsObjectFindingSeverity,
    /// Support-safe summary.
    pub summary: String,
}

impl DocsObjectValidationFinding {
    fn blocker(finding_kind: DocsObjectFindingKind, summary: impl Into<String>) -> Self {
        Self {
            finding_kind,
            severity: DocsObjectFindingSeverity::Blocker,
            summary: summary.into(),
        }
    }
}

/// Constructor input for [`DocsObjectReusePacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsObjectReusePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface or workflow label.
    pub surface_label: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Source descriptors.
    pub sources: Vec<DocsSourceDescriptor>,
    /// Result objects.
    pub results: Vec<DocsResult>,
    /// Per-surface projections.
    pub surface_projections: Vec<DocsObjectSurfaceProjection>,
    /// Source contract refs.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
}

/// Export-safe docs-source/result object reuse packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsObjectReusePacket {
    /// Record kind; must equal [`DOCS_SOURCE_RESULT_REUSE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DOCS_SOURCE_RESULT_REUSE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface or workflow label.
    pub surface_label: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Source descriptors.
    pub sources: Vec<DocsSourceDescriptor>,
    /// Result objects.
    pub results: Vec<DocsResult>,
    /// Per-surface projections.
    pub surface_projections: Vec<DocsObjectSurfaceProjection>,
    /// Source contract refs.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Derived promotion state.
    pub promotion_state: DocsObjectPromotionState,
    /// Validation findings.
    #[serde(default)]
    pub validation_findings: Vec<DocsObjectValidationFinding>,
}

impl DocsObjectReusePacket {
    /// Materializes the packet and records its derived findings and promotion state.
    pub fn materialize(input: DocsObjectReusePacketInput) -> Self {
        let mut packet = Self {
            record_kind: DOCS_SOURCE_RESULT_REUSE_RECORD_KIND.to_owned(),
            schema_version: DOCS_SOURCE_RESULT_REUSE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            generated_at: input.generated_at,
            sources: input.sources,
            results: input.results,
            surface_projections: input.surface_projections,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            promotion_state: DocsObjectPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet's invariants, including the stored promotion state.
    pub fn validate(&self) -> Vec<DocsObjectValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when no blocker findings exist.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == DocsObjectFindingSeverity::Blocker)
    }

    /// Wraps the packet in an export-safe support export.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> DocsObjectReuseSupportExport {
        DocsObjectReuseSupportExport {
            record_kind: DOCS_SOURCE_RESULT_REUSE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DOCS_SOURCE_RESULT_REUSE_SCHEMA_VERSION,
            export_id: export_id.into(),
            export_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            export_packet: self.clone(),
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("docs reuse packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Stable Docs Source/Result Object Reuse Across Consumer Surfaces\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Promotion: `{}`\n",
            self.promotion_state.as_str()
        ));
        out.push_str(&format!(
            "- Sources: {} / Results: {} / Projections: {}\n",
            self.sources.len(),
            self.results.len(),
            self.surface_projections.len()
        ));
        out.push_str("\n## Sources\n\n");
        for source in &self.sources {
            out.push_str(&format!(
                "- **{}**: `{}` ({}, {}, {})\n",
                source.source_id,
                source.source_class.as_str(),
                source.trust_class.as_str(),
                source.version_match_state.as_str(),
                source.freshness_state.as_str(),
            ));
        }
        out.push_str("\n## Surfaces\n\n");
        for surface in DocsObjectConsumerSurface::REQUIRED {
            let count = self
                .surface_projections
                .iter()
                .filter(|projection| projection.consumer_surface == surface)
                .count();
            out.push_str(&format!(
                "- `{}`: {} projection(s)\n",
                surface.as_str(),
                count
            ));
        }
        out
    }

    fn derived_findings(&self, check_promotion: bool) -> Vec<DocsObjectValidationFinding> {
        let mut findings = Vec::new();

        if self.record_kind != DOCS_SOURCE_RESULT_REUSE_RECORD_KIND {
            findings.push(DocsObjectValidationFinding::blocker(
                DocsObjectFindingKind::WrongRecordKind,
                "record kind does not match the docs-source/result reuse contract",
            ));
        }
        if self.schema_version != DOCS_SOURCE_RESULT_REUSE_SCHEMA_VERSION {
            findings.push(DocsObjectValidationFinding::blocker(
                DocsObjectFindingKind::WrongSchemaVersion,
                "schema version does not match the docs-source/result reuse contract",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.generated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
        {
            findings.push(DocsObjectValidationFinding::blocker(
                DocsObjectFindingKind::MissingPacketIdentity,
                "packet identity is incomplete",
            ));
        }

        validate_source_contracts(self, &mut findings);
        let source_by_id = self.validate_sources(&mut findings);
        self.validate_distinguishability(&mut findings);
        self.validate_results(&source_by_id, &mut findings);
        self.validate_projections(&source_by_id, &mut findings);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("docs reuse packet serializes"),
        ) {
            findings.push(DocsObjectValidationFinding::blocker(
                DocsObjectFindingKind::RawBoundaryMaterialPresent,
                "export contains forbidden raw boundary material",
            ));
        }

        if check_promotion {
            let derived = promotion_state_for_findings(&findings);
            if self.promotion_state != derived {
                findings.push(DocsObjectValidationFinding::blocker(
                    DocsObjectFindingKind::PromotionStateMismatch,
                    "stored promotion state disagrees with derived findings",
                ));
            }
        }

        findings
    }

    fn validate_sources<'a>(
        &'a self,
        findings: &mut Vec<DocsObjectValidationFinding>,
    ) -> BTreeMap<&'a str, &'a DocsSourceDescriptor> {
        let mut source_by_id: BTreeMap<&str, &DocsSourceDescriptor> = BTreeMap::new();
        for source in &self.sources {
            source_by_id.insert(source.source_id.as_str(), source);

            if !source.is_well_formed() {
                findings.push(DocsObjectValidationFinding::blocker(
                    DocsObjectFindingKind::SourceDescriptorIncomplete,
                    format!("source descriptor {} is incomplete", source.source_id),
                ));
            }
            if !source.raw_boundary_material_excluded {
                findings.push(DocsObjectValidationFinding::blocker(
                    DocsObjectFindingKind::RawBoundaryMaterialPresent,
                    format!(
                        "source descriptor {} retains raw boundary material",
                        source.source_id
                    ),
                ));
            }
            if !source.trust_class.is_admissible_for(source.source_class) {
                findings.push(DocsObjectValidationFinding::blocker(
                    DocsObjectFindingKind::SourceTrustClassMismatch,
                    format!(
                        "source {} labels {} docs with trust class {}",
                        source.source_id,
                        source.source_class.as_str(),
                        source.trust_class.as_str()
                    ),
                ));
            }
            if source.is_derived_explanation() && !source.derived_guardrail_ok() {
                findings.push(DocsObjectValidationFinding::blocker(
                    DocsObjectFindingKind::DerivedExplanationMasqueradesAsPrimary,
                    format!(
                        "derived explanation {} claims primary authority",
                        source.source_id
                    ),
                ));
            }
            if source.is_live_external() && !source.live_external_handoff_ok() {
                findings.push(DocsObjectValidationFinding::blocker(
                    DocsObjectFindingKind::LiveExternalDocsHandoffMissing,
                    format!(
                        "live external source {} is not gated behind an explicit handoff",
                        source.source_id
                    ),
                ));
            }
            if source.requires_disclosure() && !source.has_disclosure() {
                findings.push(DocsObjectValidationFinding::blocker(
                    DocsObjectFindingKind::BoundaryDisclosureMissing,
                    format!(
                        "source {} omits a required disclosure note",
                        source.source_id
                    ),
                ));
            }
        }
        source_by_id
    }

    fn validate_distinguishability(&self, findings: &mut Vec<DocsObjectValidationFinding>) {
        let present: HashSet<CitationSourceClass> = self
            .sources
            .iter()
            .map(|source| source.source_class)
            .collect();
        for required in [
            CitationSourceClass::ProjectDocs,
            CitationSourceClass::MirroredOfficialDocs,
            CitationSourceClass::CuratedKnowledgePack,
            CitationSourceClass::VendorProviderDocs,
            CitationSourceClass::DerivedExplanation,
        ] {
            if !present.contains(&required) {
                findings.push(DocsObjectValidationFinding::blocker(
                    DocsObjectFindingKind::SourceClassDistinguishabilityMissing,
                    format!(
                        "no source represents the {} class so it cannot stay distinguishable",
                        required.as_str()
                    ),
                ));
                return;
            }
        }
    }

    fn validate_results(
        &self,
        source_by_id: &BTreeMap<&str, &DocsSourceDescriptor>,
        findings: &mut Vec<DocsObjectValidationFinding>,
    ) {
        for result in &self.results {
            if !result.is_well_formed() {
                findings.push(DocsObjectValidationFinding::blocker(
                    DocsObjectFindingKind::ResultObjectIncomplete,
                    format!("result object {} is incomplete", result.result_id),
                ));
            }
            if !result.snippet.full_content_excluded {
                findings.push(DocsObjectValidationFinding::blocker(
                    DocsObjectFindingKind::SnippetForcesFullContent,
                    format!(
                        "result {} snippet forces full-content export",
                        result.result_id
                    ),
                ));
            }
            if !result.raw_boundary_material_excluded {
                findings.push(DocsObjectValidationFinding::blocker(
                    DocsObjectFindingKind::RawBoundaryMaterialPresent,
                    format!("result {} retains raw boundary material", result.result_id),
                ));
            }
            match source_by_id.get(result.docs_source_ref.as_str()) {
                None => findings.push(DocsObjectValidationFinding::blocker(
                    DocsObjectFindingKind::ResultSourceRefUnresolved,
                    format!(
                        "result {} references unknown source {}",
                        result.result_id, result.docs_source_ref
                    ),
                )),
                Some(source) => {
                    if !result.agrees_with(source) {
                        findings.push(DocsObjectValidationFinding::blocker(
                            DocsObjectFindingKind::SourceResultTruthMismatch,
                            format!(
                                "result {} disagrees with source {} on class/trust/version/freshness",
                                result.result_id, source.source_id
                            ),
                        ));
                    }
                }
            }
        }
    }

    fn validate_projections(
        &self,
        source_by_id: &BTreeMap<&str, &DocsSourceDescriptor>,
        findings: &mut Vec<DocsObjectValidationFinding>,
    ) {
        let result_ids: BTreeSet<&str> = self
            .results
            .iter()
            .map(|result| result.result_id.as_str())
            .collect();
        let present: BTreeSet<DocsObjectConsumerSurface> = self
            .surface_projections
            .iter()
            .map(|projection| projection.consumer_surface)
            .collect();
        for required in DocsObjectConsumerSurface::REQUIRED {
            if !present.contains(&required) {
                findings.push(DocsObjectValidationFinding::blocker(
                    DocsObjectFindingKind::MissingConsumerSurface,
                    format!(
                        "no projection reuses the objects on the {} surface",
                        required.as_str()
                    ),
                ));
                break;
            }
        }
        for projection in &self.surface_projections {
            if projection.projection_id.trim().is_empty()
                || !result_ids.contains(projection.result_id_ref.as_str())
                || !source_by_id.contains_key(projection.source_id_ref.as_str())
            {
                findings.push(DocsObjectValidationFinding::blocker(
                    DocsObjectFindingKind::ProjectionRefUnresolved,
                    format!(
                        "projection {} references an unknown result or source",
                        projection.projection_id
                    ),
                ));
            }
            if !projection.is_faithful() {
                findings.push(DocsObjectValidationFinding::blocker(
                    DocsObjectFindingKind::ConsumerSurfaceProjectionDrift,
                    format!(
                        "projection {} on {} dropped shared object-model truth",
                        projection.projection_id,
                        projection.consumer_surface.as_str()
                    ),
                ));
            }
        }
    }
}

/// Support-export wrapper preserving the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsObjectReuseSupportExport {
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
    /// Exact packet preserved by the export.
    pub export_packet: DocsObjectReusePacket,
}

impl DocsObjectReuseSupportExport {
    /// Returns true when the export preserves the same packet safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == DOCS_SOURCE_RESULT_REUSE_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == DOCS_SOURCE_RESULT_REUSE_SCHEMA_VERSION
            && self.export_packet_id_ref == self.export_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.export_packet.validate().is_empty()
    }
}

/// Errors emitted while reading the checked-in docs-source/result reuse export.
#[derive(Debug)]
pub enum DocsObjectReuseArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export is not export-safe or its packet failed validation.
    Validation(Vec<DocsObjectValidationFinding>),
    /// Support export wrapper is not export-safe.
    NotExportSafe,
}

impl fmt::Display for DocsObjectReuseArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "docs reuse export parse failed: {error}")
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(formatter, "docs reuse export failed validation: {tokens}")
            }
            Self::NotExportSafe => {
                write!(formatter, "docs reuse export wrapper is not export-safe")
            }
        }
    }
}

impl Error for DocsObjectReuseArtifactError {}

/// Returns the seeded stable docs-source/result reuse packet input.
pub fn seeded_stable_docs_source_result_reuse_input() -> DocsObjectReusePacketInput {
    seed::seeded_input()
}

/// Materializes the checked-in stable docs-source/result reuse packet.
///
/// # Errors
///
/// Returns an error when the seeded packet fails its own stable invariants.
pub fn current_stable_docs_source_result_reuse_packet(
) -> Result<DocsObjectReusePacket, DocsObjectReuseArtifactError> {
    let packet = DocsObjectReusePacket::materialize(seeded_stable_docs_source_result_reuse_input());
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(DocsObjectReuseArtifactError::Validation(findings))
    }
}

/// Reads and validates the checked-in stable support export.
///
/// # Errors
///
/// Returns an error when the checked artifact fails to parse, is not
/// export-safe, or its packet fails validation.
pub fn current_stable_docs_source_result_reuse_export(
) -> Result<DocsObjectReuseSupportExport, DocsObjectReuseArtifactError> {
    let export: DocsObjectReuseSupportExport = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/stable_docs_source_and_result_object_reuse_across_consumer_surfaces/support_export.json"
    )))
    .map_err(DocsObjectReuseArtifactError::SupportExport)?;
    let findings = export.export_packet.validate();
    if !findings.is_empty() {
        return Err(DocsObjectReuseArtifactError::Validation(findings));
    }
    if !export.is_export_safe() {
        return Err(DocsObjectReuseArtifactError::NotExportSafe);
    }
    Ok(export)
}

fn validate_source_contracts(
    packet: &DocsObjectReusePacket,
    findings: &mut Vec<DocsObjectValidationFinding>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    if !refs.contains(DOCS_SOURCE_RESULT_REUSE_SCHEMA_REF)
        || !refs.contains(DOCS_SOURCE_RESULT_REUSE_DOC_REF)
    {
        findings.push(DocsObjectValidationFinding::blocker(
            DocsObjectFindingKind::MissingSourceContracts,
            "source contract refs omit the schema or contract doc",
        ));
    }
}

fn promotion_state_for_findings(
    findings: &[DocsObjectValidationFinding],
) -> DocsObjectPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == DocsObjectFindingSeverity::Blocker)
    {
        DocsObjectPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == DocsObjectFindingSeverity::Warning)
    {
        DocsObjectPromotionState::NarrowedBelowStable
    } else {
        DocsObjectPromotionState::Stable
    }
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

    const PACKET_ID: &str = "packet:stable_docs_source_and_result_object_reuse:001";

    pub(super) fn seeded_input() -> DocsObjectReusePacketInput {
        DocsObjectReusePacketInput {
            packet_id: PACKET_ID.to_owned(),
            surface_label: "workflow:docs_search_cards_hover_ai_glossary_support:stable".to_owned(),
            generated_at: "2026-06-26T00:00:00Z".to_owned(),
            sources: sources(),
            results: results(),
            surface_projections: projections(),
            source_contract_refs: vec![
                DOCS_SOURCE_RESULT_REUSE_SCHEMA_REF.to_owned(),
                DOCS_SOURCE_RESULT_REUSE_DOC_REF.to_owned(),
                DOCS_SOURCE_RESULT_REUSE_ARTIFACT_REF.to_owned(),
                DOCS_SOURCE_RESULT_REUSE_SUMMARY_REF.to_owned(),
            ],
            redaction_class_token: "metadata_safe_default".to_owned(),
        }
    }

    fn sources() -> Vec<DocsSourceDescriptor> {
        vec![
            DocsSourceDescriptor {
                source_id: "docs-source:project-readme".to_owned(),
                source_class: CitationSourceClass::ProjectDocs,
                provider_or_pack_id: "pack:workspace-project-docs".to_owned(),
                provider_or_pack_revision_ref: "rev:project-docs@workspace-head".to_owned(),
                locale: "en-US".to_owned(),
                trust_class: DocsObjectTrustClass::FirstPartyAuthoritative,
                browser_handoff: DocsExternalOpenState::NotRequired,
                browser_handoff_packet_ref: None,
                mirror_offline_posture: DocsMirrorOfflinePosture::LocalProjectPack,
                precedence_class: SourcePrecedenceClass::ProjectAuthoritativeOnly,
                version_match_state: VersionMatchState::ExactBuildMatch,
                freshness_state: DocsFreshnessClass::AuthoritativeLive,
                pack_manifest_ref: Some("manifest:workspace-project-docs".to_owned()),
                disclosure_note: None,
                raw_boundary_material_excluded: true,
            },
            DocsSourceDescriptor {
                source_id: "docs-source:mirror-std".to_owned(),
                source_class: CitationSourceClass::MirroredOfficialDocs,
                provider_or_pack_id: "pack:std-mirror".to_owned(),
                provider_or_pack_revision_ref: "rev:std-mirror@1.84.0".to_owned(),
                locale: "en-US".to_owned(),
                trust_class: DocsObjectTrustClass::SignedMirrorVerified,
                browser_handoff: DocsExternalOpenState::Available,
                browser_handoff_packet_ref: Some("handoff:std-mirror-upstream".to_owned()),
                mirror_offline_posture: DocsMirrorOfflinePosture::MirroredPack,
                precedence_class: SourcePrecedenceClass::ProjectOutranksVendorDefault,
                version_match_state: VersionMatchState::CompatibleMinorDrift,
                freshness_state: DocsFreshnessClass::WarmCached,
                pack_manifest_ref: Some("manifest:std-mirror".to_owned()),
                disclosure_note: Some(
                    "Signed mirror is a compatible minor drift from the active build.".to_owned(),
                ),
                raw_boundary_material_excluded: true,
            },
            DocsSourceDescriptor {
                source_id: "docs-source:ext-pack-cookbook".to_owned(),
                source_class: CitationSourceClass::CuratedKnowledgePack,
                provider_or_pack_id: "pack:extension-cookbook".to_owned(),
                provider_or_pack_revision_ref: "rev:extension-cookbook@2.3.1".to_owned(),
                locale: "en-US".to_owned(),
                trust_class: DocsObjectTrustClass::ExtensionPackSigned,
                browser_handoff: DocsExternalOpenState::NotRequired,
                browser_handoff_packet_ref: None,
                mirror_offline_posture: DocsMirrorOfflinePosture::OfflinePinnedPack,
                precedence_class: SourcePrecedenceClass::ProjectOutranksVendorDefault,
                version_match_state: VersionMatchState::ExactBuildMatch,
                freshness_state: DocsFreshnessClass::WarmCached,
                pack_manifest_ref: Some("manifest:extension-cookbook".to_owned()),
                disclosure_note: Some(
                    "Extension-contributed pack pinned for offline use.".to_owned(),
                ),
                raw_boundary_material_excluded: true,
            },
            DocsSourceDescriptor {
                source_id: "docs-source:vendor-live-api".to_owned(),
                source_class: CitationSourceClass::VendorProviderDocs,
                provider_or_pack_id: "provider:vendor-api-portal".to_owned(),
                provider_or_pack_revision_ref: "rev:vendor-api-portal@live".to_owned(),
                locale: "en-US".to_owned(),
                trust_class: DocsObjectTrustClass::LiveProviderHandoff,
                browser_handoff: DocsExternalOpenState::Available,
                browser_handoff_packet_ref: Some("handoff:vendor-api-portal".to_owned()),
                mirror_offline_posture: DocsMirrorOfflinePosture::LiveOnline,
                precedence_class: SourcePrecedenceClass::VendorOverrideDisclosed,
                version_match_state: VersionMatchState::UnknownTargetBuild,
                freshness_state: DocsFreshnessClass::AuthoritativeLive,
                pack_manifest_ref: None,
                disclosure_note: Some(
                    "Live external docs open through an explicit, isolated browser handoff."
                        .to_owned(),
                ),
                raw_boundary_material_excluded: true,
            },
            DocsSourceDescriptor {
                source_id: "docs-source:derived-explanation".to_owned(),
                source_class: CitationSourceClass::DerivedExplanation,
                provider_or_pack_id: "tool:docs-explainer".to_owned(),
                provider_or_pack_revision_ref: "rev:docs-explainer@1.0.0".to_owned(),
                locale: "en-US".to_owned(),
                trust_class: DocsObjectTrustClass::DerivedInferenceOnly,
                browser_handoff: DocsExternalOpenState::NotRequired,
                browser_handoff_packet_ref: None,
                mirror_offline_posture: DocsMirrorOfflinePosture::GeneratedLocal,
                precedence_class: SourcePrecedenceClass::NotApplicable,
                version_match_state: VersionMatchState::ExactBuildMatch,
                freshness_state: DocsFreshnessClass::AuthoritativeLive,
                pack_manifest_ref: None,
                disclosure_note: Some(
                    "Derived explanation; never primary authority and bound to its citations."
                        .to_owned(),
                ),
                raw_boundary_material_excluded: true,
            },
        ]
    }

    fn results() -> Vec<DocsResult> {
        vec![
            DocsResult {
                result_id: "docs-result:project-overview".to_owned(),
                title: "Workspace project overview".to_owned(),
                docs_source_ref: "docs-source:project-readme".to_owned(),
                source_class: CitationSourceClass::ProjectDocs,
                trust_class: DocsObjectTrustClass::FirstPartyAuthoritative,
                version_match_state: VersionMatchState::ExactBuildMatch,
                freshness_state: DocsFreshnessClass::AuthoritativeLive,
                symbol_refs: vec!["symbol:crate::workspace::overview".to_owned()],
                citation_anchor_refs: vec!["anchor:project-readme#overview".to_owned()],
                snippet: snippet("anchor:project-readme#overview", 1, 18),
                inference_markers: vec![CitationInferenceMarker::RawSource],
                support_export_safe_id: "export:docs-result:project-overview".to_owned(),
                disclosure_note: None,
                raw_boundary_material_excluded: true,
            },
            DocsResult {
                result_id: "docs-result:std-fn".to_owned(),
                title: "Standard library function reference".to_owned(),
                docs_source_ref: "docs-source:mirror-std".to_owned(),
                source_class: CitationSourceClass::MirroredOfficialDocs,
                trust_class: DocsObjectTrustClass::SignedMirrorVerified,
                version_match_state: VersionMatchState::CompatibleMinorDrift,
                freshness_state: DocsFreshnessClass::WarmCached,
                symbol_refs: vec!["symbol:std::vec::Vec::push".to_owned()],
                citation_anchor_refs: vec!["anchor:std-mirror#vec-push".to_owned()],
                snippet: snippet("anchor:std-mirror#vec-push", 40, 64),
                inference_markers: vec![CitationInferenceMarker::RawSource],
                support_export_safe_id: "export:docs-result:std-fn".to_owned(),
                disclosure_note: Some("Mirrored docs are a compatible minor drift.".to_owned()),
                raw_boundary_material_excluded: true,
            },
            DocsResult {
                result_id: "docs-result:cookbook-recipe".to_owned(),
                title: "Extension cookbook recipe".to_owned(),
                docs_source_ref: "docs-source:ext-pack-cookbook".to_owned(),
                source_class: CitationSourceClass::CuratedKnowledgePack,
                trust_class: DocsObjectTrustClass::ExtensionPackSigned,
                version_match_state: VersionMatchState::ExactBuildMatch,
                freshness_state: DocsFreshnessClass::WarmCached,
                symbol_refs: Vec::new(),
                citation_anchor_refs: vec!["anchor:extension-cookbook#async-recipe".to_owned()],
                snippet: snippet("anchor:extension-cookbook#async-recipe", 5, 30),
                inference_markers: vec![CitationInferenceMarker::RawSource],
                support_export_safe_id: "export:docs-result:cookbook-recipe".to_owned(),
                disclosure_note: None,
                raw_boundary_material_excluded: true,
            },
            DocsResult {
                result_id: "docs-result:vendor-endpoint".to_owned(),
                title: "Vendor API endpoint reference".to_owned(),
                docs_source_ref: "docs-source:vendor-live-api".to_owned(),
                source_class: CitationSourceClass::VendorProviderDocs,
                trust_class: DocsObjectTrustClass::LiveProviderHandoff,
                version_match_state: VersionMatchState::UnknownTargetBuild,
                freshness_state: DocsFreshnessClass::AuthoritativeLive,
                symbol_refs: Vec::new(),
                citation_anchor_refs: vec!["anchor:vendor-api-portal#endpoint".to_owned()],
                snippet: snippet("anchor:vendor-api-portal#endpoint", 1, 12),
                inference_markers: vec![CitationInferenceMarker::RawSource],
                support_export_safe_id: "export:docs-result:vendor-endpoint".to_owned(),
                disclosure_note: Some("Opens live external docs through a handoff.".to_owned()),
                raw_boundary_material_excluded: true,
            },
            DocsResult {
                result_id: "docs-result:derived-summary".to_owned(),
                title: "Derived explanation summary".to_owned(),
                docs_source_ref: "docs-source:derived-explanation".to_owned(),
                source_class: CitationSourceClass::DerivedExplanation,
                trust_class: DocsObjectTrustClass::DerivedInferenceOnly,
                version_match_state: VersionMatchState::ExactBuildMatch,
                freshness_state: DocsFreshnessClass::AuthoritativeLive,
                symbol_refs: vec!["symbol:crate::workspace::overview".to_owned()],
                citation_anchor_refs: vec![
                    "anchor:project-readme#overview".to_owned(),
                    "anchor:std-mirror#vec-push".to_owned(),
                ],
                snippet: snippet("anchor:derived-explanation#summary", 1, 8),
                inference_markers: vec![
                    CitationInferenceMarker::Inference,
                    CitationInferenceMarker::GeneratedSummary,
                ],
                support_export_safe_id: "export:docs-result:derived-summary".to_owned(),
                disclosure_note: Some(
                    "Derived summary; cites project and mirrored docs but is not authority."
                        .to_owned(),
                ),
                raw_boundary_material_excluded: true,
            },
        ]
    }

    fn snippet(anchor: &str, start_line: u32, end_line: u32) -> DocsSnippetMeta {
        DocsSnippetMeta {
            snippet_anchor_ref: anchor.to_owned(),
            start_line,
            end_line,
            excerpt_char_budget: 480,
            full_content_excluded: true,
            redaction_note: None,
        }
    }

    fn projections() -> Vec<DocsObjectSurfaceProjection> {
        vec![
            projection(
                DocsObjectConsumerSurface::DocsSearch,
                "projection:docs-search:project-overview",
                "docs-result:project-overview",
                "docs-source:project-readme",
            ),
            projection(
                DocsObjectConsumerSurface::SymbolReferenceCard,
                "projection:symbol-card:std-fn",
                "docs-result:std-fn",
                "docs-source:mirror-std",
            ),
            projection(
                DocsObjectConsumerSurface::HoverPeekDocs,
                "projection:hover-peek:cookbook-recipe",
                "docs-result:cookbook-recipe",
                "docs-source:ext-pack-cookbook",
            ),
            projection(
                DocsObjectConsumerSurface::AiCitation,
                "projection:ai-citation:derived-summary",
                "docs-result:derived-summary",
                "docs-source:derived-explanation",
            ),
            projection(
                DocsObjectConsumerSurface::GlossaryCard,
                "projection:glossary:cookbook-recipe",
                "docs-result:cookbook-recipe",
                "docs-source:ext-pack-cookbook",
            ),
            projection(
                DocsObjectConsumerSurface::SupportExport,
                "projection:support-export:project-overview",
                "docs-result:project-overview",
                "docs-source:project-readme",
            ),
        ]
    }

    fn projection(
        surface: DocsObjectConsumerSurface,
        projection_id: &str,
        result_id_ref: &str,
        source_id_ref: &str,
    ) -> DocsObjectSurfaceProjection {
        DocsObjectSurfaceProjection {
            consumer_surface: surface,
            projection_id: projection_id.to_owned(),
            result_id_ref: result_id_ref.to_owned(),
            source_id_ref: source_id_ref.to_owned(),
            shows_source_class: true,
            shows_version_match: true,
            shows_freshness: true,
            shows_trust_class: true,
            preserves_symbol_or_citation_refs: true,
            preserves_result_identity: true,
            full_content_excluded: true,
            local_badge_vocabulary_used: false,
        }
    }
}
