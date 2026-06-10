//! Topology maps, ownership surfaces, and codebase-explainer cards with cited
//! evidence and confidence labels.
//!
//! This module implements the M5 codebase-understanding feature that turns the
//! workspace graph into three kinds of inspectable, cited cards the docs and
//! code-understanding surfaces render: a [`UnderstandingCardKind::TopologyMap`]
//! card (a region of the dependency/containment topology), an
//! [`UnderstandingCardKind::OwnershipSurface`] card (who owns a region, and on
//! what basis), and an [`UnderstandingCardKind::CodebaseExplainer`] card (a
//! natural-language explanation of a region or symbol). Each
//! [`UnderstandingCard`] carries the source/version/freshness/locality/confidence
//! chip set, an explicit [`UnderstandingCard::confidence_reason`], a non-empty
//! list of [`CardEvidence`] that backs its claims, its [`CardProvenance`], and the
//! open-raw / open-source escapes that keep derived and inferred cards honest.
//!
//! The [`UnderstandingEvidenceExport`] is the cited projection that support, AI
//! evidence, and review surfaces ingest: one [`EvidenceExportRow`] per card
//! preserving card kind, source class, confidence, derivation, citation state,
//! and the escapes. A topology, ownership, or explainer card that is derived or
//! inferred must stay cited and may not be presented as high-confidence live
//! truth.
//!
//! [`CodebaseUnderstandingCardsPacket::materialize`] computes the validation
//! findings and the promotion state (`stable`, `narrowed_below_stable`, or
//! `blocks_stable`) from the input, so a stale, uncited, over-authoritative, or
//! unattributed set of cards automatically narrows or blocks before it reaches a
//! consumer surface. The packet is an inspectable, serde-serializable truth
//! packet: it carries no raw source files, no raw document bodies, no raw
//! provider payloads, and no credentials — only metadata, chip truth, confidence
//! reasons, cited evidence refs, provenance, finding summaries, and contract refs.
//!
//! The boundary schema is
//! [`schemas/docs/add-topology-maps-ownership-surfaces-and-codebase-explainer-cards-with-cited-evidence-and-confidence-labels.schema.json`](../../../../schemas/docs/add-topology-maps-ownership-surfaces-and-codebase-explainer-cards-with-cited-evidence-and-confidence-labels.schema.json).
//! The contract doc is
//! [`docs/docs/m5/add_topology_maps_ownership_surfaces_and_codebase_explainer_cards_with_cited_evidence_and_confidence_labels.md`](../../../../docs/docs/m5/add_topology_maps_ownership_surfaces_and_codebase_explainer_cards_with_cited_evidence_and_confidence_labels.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/add_topology_maps_ownership_surfaces_and_codebase_explainer_cards_with_cited_evidence_and_confidence_labels/`](../../../../fixtures/docs/m5/add_topology_maps_ownership_surfaces_and_codebase_explainer_cards_with_cited_evidence_and_confidence_labels/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`CodebaseUnderstandingCardsPacket`].
pub const UNDERSTANDING_CARDS_RECORD_KIND: &str = "topology_ownership_and_codebase_explainer_cards";

/// Record-kind tag carried by the support-export wrapper.
pub const UNDERSTANDING_CARDS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "topology_ownership_and_codebase_explainer_cards_support_export";

/// Schema version for understanding-card records.
pub const UNDERSTANDING_CARDS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const UNDERSTANDING_CARDS_SCHEMA_REF: &str =
    "schemas/docs/add-topology-maps-ownership-surfaces-and-codebase-explainer-cards-with-cited-evidence-and-confidence-labels.schema.json";

/// Repo-relative path of the understanding-cards contract doc.
pub const UNDERSTANDING_CARDS_DOC_REF: &str =
    "docs/docs/m5/add_topology_maps_ownership_surfaces_and_codebase_explainer_cards_with_cited_evidence_and_confidence_labels.md";

/// Repo-relative path of the protected fixture directory.
pub const UNDERSTANDING_CARDS_FIXTURE_DIR: &str =
    "fixtures/docs/m5/add_topology_maps_ownership_surfaces_and_codebase_explainer_cards_with_cited_evidence_and_confidence_labels";

/// Repo-relative path of the checked support-export artifact.
pub const UNDERSTANDING_CARDS_ARTIFACT_REF: &str =
    "artifacts/docs/m5/add_topology_maps_ownership_surfaces_and_codebase_explainer_cards_with_cited_evidence_and_confidence_labels/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const UNDERSTANDING_CARDS_SUMMARY_REF: &str =
    "artifacts/docs/m5/add_topology_maps_ownership_surfaces_and_codebase_explainer_cards_with_cited_evidence_and_confidence_labels.md";

/// Kind of codebase-understanding card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnderstandingCardKind {
    /// A region of the dependency / containment topology.
    TopologyMap,
    /// Who owns a region, and on what basis.
    OwnershipSurface,
    /// A natural-language explanation of a region or symbol.
    CodebaseExplainer,
}

impl UnderstandingCardKind {
    /// Stable token recorded in the card.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopologyMap => "topology_map",
            Self::OwnershipSurface => "ownership_surface",
            Self::CodebaseExplainer => "codebase_explainer",
        }
    }
}

/// Source class for a card's underlying evidence, projected as the source chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnderstandingSourceClass {
    /// Symbol or file in the active workspace code.
    WorkspaceCode,
    /// Symbol or file in a resolved dependency / vendored source.
    DependencySource,
    /// The workspace graph index (derived topology / relationships).
    GraphIndex,
    /// A `CODEOWNERS`-style ownership file.
    CodeownersFile,
    /// A declared ownership registry / package inventory.
    OwnershipRegistry,
    /// Workspace-local project docs.
    ProjectDocs,
    /// Generated API/reference docs.
    GeneratedReference,
    /// Pinned, signed mirror of official upstream docs.
    MirroredOfficialDocs,
}

impl UnderstandingSourceClass {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceCode => "workspace_code",
            Self::DependencySource => "dependency_source",
            Self::GraphIndex => "graph_index",
            Self::CodeownersFile => "codeowners_file",
            Self::OwnershipRegistry => "ownership_registry",
            Self::ProjectDocs => "project_docs",
            Self::GeneratedReference => "generated_reference",
            Self::MirroredOfficialDocs => "mirrored_official_docs",
        }
    }
}

/// Version-match state for a card, projected as the version chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnderstandingVersionMatch {
    /// Card matches the active build/workspace revision exactly.
    ExactBuildMatch,
    /// Card is within an accepted compatible drift window.
    CompatibleMinorDrift,
    /// Card drifted incompatibly from the active target.
    IncompatibleDriftDetected,
    /// Pre-release card has not completed verification.
    PreReleaseUnverified,
    /// The target build/workspace revision could not be verified.
    UnknownTargetBuild,
}

impl UnderstandingVersionMatch {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactBuildMatch => "exact_build_match",
            Self::CompatibleMinorDrift => "compatible_minor_drift",
            Self::IncompatibleDriftDetected => "incompatible_drift_detected",
            Self::PreReleaseUnverified => "pre_release_unverified",
            Self::UnknownTargetBuild => "unknown_target_build",
        }
    }

    /// Whether this state may be presented as a confident current-version match.
    pub const fn is_confident_current(self) -> bool {
        matches!(self, Self::ExactBuildMatch)
    }
}

/// Freshness state for a card, projected as the freshness chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnderstandingFreshness {
    /// Card was live and authoritative at materialization time.
    AuthoritativeLive,
    /// Cached card within its freshness window.
    WarmCached,
    /// Cached card usable only with degraded disclosure.
    DegradedCached,
    /// Card is stale and must not claim current authority.
    Stale,
    /// Freshness could not be verified.
    Unverified,
    /// A refresh is pending; the source has not yet re-synced.
    RefreshPending,
}

impl UnderstandingFreshness {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::DegradedCached => "degraded_cached",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
            Self::RefreshPending => "refresh_pending",
        }
    }

    /// Whether this state may claim live authoritative freshness.
    pub const fn is_authoritative_live(self) -> bool {
        matches!(self, Self::AuthoritativeLive)
    }
}

/// Locality / install posture for a card, projected as the locality chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnderstandingLocality {
    /// Resolved from local content or the in-repo index.
    Local,
    /// Resolved through a pinned mirror pack.
    MirroredPack,
    /// Resolved through a remote helper.
    RemoteHelper,
    /// Resolved through a managed (org-hosted) service.
    Managed,
}

impl UnderstandingLocality {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::MirroredPack => "mirrored_pack",
            Self::RemoteHelper => "remote_helper",
            Self::Managed => "managed",
        }
    }
}

/// Confidence label for a card, projected as the confidence chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnderstandingConfidence {
    /// High confidence.
    High,
    /// Medium confidence.
    Medium,
    /// Low confidence.
    Low,
    /// Heuristic only; not a verified claim.
    Heuristic,
}

impl UnderstandingConfidence {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
            Self::Heuristic => "heuristic",
        }
    }
}

/// How a card or evidence item was derived from its underlying source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceDerivation {
    /// The verbatim node/symbol/edge, not a derivation.
    VerbatimNode,
    /// An extracted snippet of the underlying source.
    ExtractedSnippet,
    /// A summary derived over the underlying source.
    DerivedSummary,
    /// An inferred explanation generated over the underlying source.
    InferredExplanation,
}

impl EvidenceDerivation {
    /// Stable token recorded in the provenance.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerbatimNode => "verbatim_node",
            Self::ExtractedSnippet => "extracted_snippet",
            Self::DerivedSummary => "derived_summary",
            Self::InferredExplanation => "inferred_explanation",
        }
    }

    /// Whether this derivation must carry a citation to stay honest.
    pub const fn needs_citation(self) -> bool {
        matches!(self, Self::DerivedSummary | Self::InferredExplanation)
    }
}

/// Kind of subject an evidence item points at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceSubjectKind {
    /// A code symbol (function, type, trait, etc.).
    CodeSymbol,
    /// A code file.
    CodeFile,
    /// A code module / directory region.
    CodeModule,
    /// A crate / package node.
    CrateNode,
    /// A dependency / containment edge in the topology.
    DependencyEdge,
    /// An ownership entry (team / owner).
    OwnerEntry,
    /// A docs node.
    DocsNode,
}

impl EvidenceSubjectKind {
    /// Stable token recorded in the evidence.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeSymbol => "code_symbol",
            Self::CodeFile => "code_file",
            Self::CodeModule => "code_module",
            Self::CrateNode => "crate_node",
            Self::DependencyEdge => "dependency_edge",
            Self::OwnerEntry => "owner_entry",
            Self::DocsNode => "docs_node",
        }
    }
}

/// Kind of edge a topology card surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyEdgeKind {
    /// A depends-on edge between crates / modules.
    DependsOn,
    /// A containment edge (module contains symbol/file).
    Contains,
    /// A trait-implementation edge.
    Implements,
    /// A call edge.
    Calls,
    /// A reference / use edge.
    References,
}

impl TopologyEdgeKind {
    /// Stable token recorded in the edge.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DependsOn => "depends_on",
            Self::Contains => "contains",
            Self::Implements => "implements",
            Self::Calls => "calls",
            Self::References => "references",
        }
    }
}

/// Basis on which an ownership card attributes an owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnershipBasis {
    /// A matched `CODEOWNERS`-style entry.
    CodeownersEntry,
    /// A declared ownership registry / package-inventory entry.
    DeclaredRegistry,
    /// A directory / path convention.
    DirectoryConvention,
    /// A git-history heuristic (not an authoritative declaration).
    GitHistoryHeuristic,
    /// No owner could be attributed.
    Unassigned,
}

impl OwnershipBasis {
    /// Stable token recorded in the owner entry.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeownersEntry => "codeowners_entry",
            Self::DeclaredRegistry => "declared_registry",
            Self::DirectoryConvention => "directory_convention",
            Self::GitHistoryHeuristic => "git_history_heuristic",
            Self::Unassigned => "unassigned",
        }
    }

    /// Whether this basis is an authoritative declaration (not a heuristic or a
    /// gap). Heuristic and unassigned bases may not back a high-confidence
    /// ownership claim.
    pub const fn is_authoritative_declaration(self) -> bool {
        matches!(self, Self::CodeownersEntry | Self::DeclaredRegistry)
    }
}

/// Severity of a degradation or validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnderstandingFindingSeverity {
    /// Blocks a Stable claim; the card set must block.
    Blocking,
    /// Narrows below Stable but the card set stays valid and attributable.
    Narrowing,
    /// Advisory only.
    Advisory,
}

impl UnderstandingFindingSeverity {
    /// Stable token recorded in the finding.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocking => "blocking",
            Self::Narrowing => "narrowing",
            Self::Advisory => "advisory",
        }
    }
}

/// Consumer surface that must project the understanding-cards packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnderstandingConsumerSurface {
    /// Codebase-explorer / understanding panel.
    CodebaseExplorer,
    /// Docs browser / reader.
    DocsBrowser,
    /// Graph / topology panel.
    GraphPanel,
    /// AI context assembly.
    AiContext,
    /// Retrieval-debug inspector.
    RetrievalInspector,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Help / About surface.
    HelpAbout,
}

impl UnderstandingConsumerSurface {
    /// Stable token recorded in the projection.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodebaseExplorer => "codebase_explorer",
            Self::DocsBrowser => "docs_browser",
            Self::GraphPanel => "graph_panel",
            Self::AiContext => "ai_context",
            Self::RetrievalInspector => "retrieval_inspector",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::HelpAbout => "help_about",
        }
    }
}

/// Class of a packet-level understanding degradation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnderstandingDegradationClass {
    /// The workspace graph index is stale relative to the working tree.
    GraphIndexStale,
    /// Ownership could not be resolved for part of the region.
    OwnershipUnresolved,
    /// A mirror is offline; cards served from the last verified snapshot.
    MirrorOfflineSnapshot,
    /// Only part of the topology was indexed at materialization time.
    PartialTopology,
    /// The embedder was unavailable; explainers fell back to lexical signals.
    EmbedderUnavailableLexicalFallback,
    /// The owning pack / source is quarantined.
    QuarantinedPack,
    /// A referenced anchor is broken.
    BrokenAnchor,
}

impl UnderstandingDegradationClass {
    /// Stable token recorded in the degradation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GraphIndexStale => "graph_index_stale",
            Self::OwnershipUnresolved => "ownership_unresolved",
            Self::MirrorOfflineSnapshot => "mirror_offline_snapshot",
            Self::PartialTopology => "partial_topology",
            Self::EmbedderUnavailableLexicalFallback => "embedder_unavailable_lexical_fallback",
            Self::QuarantinedPack => "quarantined_pack",
            Self::BrokenAnchor => "broken_anchor",
        }
    }
}

/// Scope an evidence export covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceExportScope {
    /// Every card in the packet.
    AllCards,
    /// Topology cards only.
    TopologyOnly,
    /// Ownership cards only.
    OwnershipOnly,
    /// Explainer cards only.
    ExplainerOnly,
}

impl EvidenceExportScope {
    /// Stable token recorded in the export.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllCards => "all_cards",
            Self::TopologyOnly => "topology_only",
            Self::OwnershipOnly => "ownership_only",
            Self::ExplainerOnly => "explainer_only",
        }
    }
}

/// Promotion state computed for the understanding-cards packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnderstandingPromotionState {
    /// Card set qualifies for the Stable claim.
    Stable,
    /// Card set narrowed below Stable but stays valid and attributable.
    NarrowedBelowStable,
    /// Card set has a blocking finding and must not present as Stable.
    BlocksStable,
}

impl UnderstandingPromotionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Validation finding kind emitted by [`CodebaseUnderstandingCardsPacket::materialize`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnderstandingFindingKind {
    /// A required identity field is missing.
    MissingIdentity,
    /// The card set is empty.
    CardsEmpty,
    /// A card id is duplicated.
    DuplicateCardId,
    /// A required card kind (topology / ownership / explainer) is missing.
    RequiredCardKindMissing,
    /// A card is missing its title or headline.
    CardTitleOrHeadlineMissing,
    /// A card is missing its explicit confidence reason.
    ConfidenceReasonMissing,
    /// A card carries no backing evidence.
    CardEvidenceMissing,
    /// An evidence id is duplicated within a card.
    DuplicateEvidenceId,
    /// A derived / inferred card is not cited.
    CardNotCited,
    /// A derived / inferred evidence item is not cited.
    EvidenceNotCited,
    /// A card is missing an open-raw / open-source escape ref.
    OpenRawOpenSourceEscapeMissing,
    /// An evidence item is missing an open-raw / open-source escape ref.
    EvidenceEscapeMissing,
    /// An inferred card is presented as a high-confidence claim.
    InferredCardLooksAuthoritative,
    /// A non-current version-match is presented as a confident live match.
    VersionTruthCollapsed,
    /// A topology card surfaces no topology edges.
    TopologyCardMissingEdges,
    /// A topology edge is missing an endpoint.
    TopologyEdgeEndpointMissing,
    /// An ownership card declares no owner.
    OwnershipCardMissingOwner,
    /// A heuristic / unassigned ownership basis backs a high-confidence claim.
    OwnershipBasisUnattributed,
    /// An evidence export row references a card id absent from the cards.
    EvidenceExportRowOrphan,
    /// A card has no matching evidence export row.
    EvidenceExportCoverageMissing,
    /// The evidence export drops a required preservation flag.
    EvidenceExportDropsPreservation,
    /// An export row's card kind disagrees with the card.
    EvidenceExportCardKindMismatch,
    /// An export row's source class disagrees with the card's chip.
    EvidenceExportSourceClassMismatch,
    /// An export row's confidence disagrees with the card's chip.
    EvidenceExportConfidenceMismatch,
    /// A degradation is incomplete (missing summary).
    DegradationIncomplete,
    /// A degradation references a card id absent from the cards.
    DegradationOrphan,
    /// A consumer projection drops a required preservation flag.
    ConsumerProjectionDrift,
    /// A consumer projection references the wrong packet id.
    ConsumerProjectionPacketIdMismatch,
    /// A required consumer surface is missing from the projections.
    RequiredSurfaceCoverageMissing,
    /// Raw bodies, raw source, or secrets crossed the export boundary.
    RawBoundaryMaterialPresent,
}

impl UnderstandingFindingKind {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingIdentity => "missing_identity",
            Self::CardsEmpty => "cards_empty",
            Self::DuplicateCardId => "duplicate_card_id",
            Self::RequiredCardKindMissing => "required_card_kind_missing",
            Self::CardTitleOrHeadlineMissing => "card_title_or_headline_missing",
            Self::ConfidenceReasonMissing => "confidence_reason_missing",
            Self::CardEvidenceMissing => "card_evidence_missing",
            Self::DuplicateEvidenceId => "duplicate_evidence_id",
            Self::CardNotCited => "card_not_cited",
            Self::EvidenceNotCited => "evidence_not_cited",
            Self::OpenRawOpenSourceEscapeMissing => "open_raw_open_source_escape_missing",
            Self::EvidenceEscapeMissing => "evidence_escape_missing",
            Self::InferredCardLooksAuthoritative => "inferred_card_looks_authoritative",
            Self::VersionTruthCollapsed => "version_truth_collapsed",
            Self::TopologyCardMissingEdges => "topology_card_missing_edges",
            Self::TopologyEdgeEndpointMissing => "topology_edge_endpoint_missing",
            Self::OwnershipCardMissingOwner => "ownership_card_missing_owner",
            Self::OwnershipBasisUnattributed => "ownership_basis_unattributed",
            Self::EvidenceExportRowOrphan => "evidence_export_row_orphan",
            Self::EvidenceExportCoverageMissing => "evidence_export_coverage_missing",
            Self::EvidenceExportDropsPreservation => "evidence_export_drops_preservation",
            Self::EvidenceExportCardKindMismatch => "evidence_export_card_kind_mismatch",
            Self::EvidenceExportSourceClassMismatch => "evidence_export_source_class_mismatch",
            Self::EvidenceExportConfidenceMismatch => "evidence_export_confidence_mismatch",
            Self::DegradationIncomplete => "degradation_incomplete",
            Self::DegradationOrphan => "degradation_orphan",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::ConsumerProjectionPacketIdMismatch => "consumer_projection_packet_id_mismatch",
            Self::RequiredSurfaceCoverageMissing => "required_surface_coverage_missing",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
        }
    }

    /// Default severity for this finding kind. Every validation finding blocks
    /// the Stable claim; narrowing comes only from data-carried degradation
    /// severities so a degraded-but-honest card set narrows rather than blocks.
    pub const fn default_severity(self) -> UnderstandingFindingSeverity {
        UnderstandingFindingSeverity::Blocking
    }
}

/// The chip set rendered for one understanding card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnderstandingChipSet {
    /// Source-class chip.
    pub source_class: UnderstandingSourceClass,
    /// Version-match chip.
    pub version_match: UnderstandingVersionMatch,
    /// Freshness chip.
    pub freshness: UnderstandingFreshness,
    /// Locality chip.
    pub locality: UnderstandingLocality,
    /// Confidence chip (the confidence label).
    pub confidence: UnderstandingConfidence,
}

/// One cited evidence item backing a card's claims.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardEvidence {
    /// Stable evidence id within the card.
    pub evidence_id: String,
    /// Kind of subject this evidence points at.
    pub subject_kind: EvidenceSubjectKind,
    /// Node / symbol / file / edge / owner ref (no raw body).
    pub subject_ref: String,
    /// How this evidence was derived from its source.
    pub derivation: EvidenceDerivation,
    /// Whether the evidence is cited back to its source.
    pub cited: bool,
    /// Citation ref when cited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation_ref: Option<String>,
    /// Open-raw escape ref (open the underlying node/symbol/edge).
    pub open_raw_escape_ref: String,
    /// Open-source escape ref (open the upstream/source).
    pub open_source_escape_ref: String,
    /// Human-readable note (no raw bodies).
    pub note: String,
}

/// One topology edge surfaced by a topology card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyEdgeRef {
    /// Source endpoint ref.
    pub from_ref: String,
    /// Target endpoint ref.
    pub to_ref: String,
    /// Edge kind.
    pub edge_kind: TopologyEdgeKind,
    /// Human-readable note (no raw bodies).
    pub note: String,
}

/// One owner attributed by an ownership card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnerRef {
    /// Owner / team ref.
    pub owner_ref: String,
    /// Basis on which the owner is attributed.
    pub ownership_basis: OwnershipBasis,
    /// Coverage note (no raw bodies).
    pub coverage_note: String,
}

/// Provenance carried inline by an understanding card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardProvenance {
    /// Owning pack / crate / repo id.
    pub pack_id_ref: String,
    /// Whether the owning pack/source is pinned.
    pub pack_pinned: bool,
    /// Whether the owning pack's signature is verified.
    pub pack_signed_and_verified: bool,
    /// How the card was derived from its source.
    pub derivation: EvidenceDerivation,
    /// Whether the card is cited back to its source.
    pub cited: bool,
    /// Citation ref when cited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation_ref: Option<String>,
}

/// One codebase-understanding card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnderstandingCard {
    /// Stable card id within this packet.
    pub card_id: String,
    /// Kind of card.
    pub card_kind: UnderstandingCardKind,
    /// Region / module / symbol the card is about (no raw body).
    pub subject_ref: String,
    /// Human-readable title.
    pub title: String,
    /// Human-readable headline / summary (no raw bodies).
    pub headline: String,
    /// Source/version/freshness/locality/confidence chips.
    pub chips: UnderstandingChipSet,
    /// Explicit, human-readable reason for the confidence label.
    pub confidence_reason: String,
    /// Cited evidence backing the card's claims (at least one).
    pub evidence: Vec<CardEvidence>,
    /// Topology edges (topology cards only).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topology_edges: Vec<TopologyEdgeRef>,
    /// Owners (ownership cards only).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub owners: Vec<OwnerRef>,
    /// Inline provenance for the card.
    pub provenance: CardProvenance,
    /// Open-raw escape ref (open the underlying region/symbol).
    pub open_raw_escape_ref: String,
    /// Open-source escape ref (open the upstream/source).
    pub open_source_escape_ref: String,
}

/// One evidence export row, mirroring a card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceExportRow {
    /// The card this export row mirrors.
    pub card_id_ref: String,
    /// Card kind (must match the card).
    pub card_kind: UnderstandingCardKind,
    /// Source class (must match the card's chip).
    pub source_class: UnderstandingSourceClass,
    /// Confidence (must match the card's chip).
    pub confidence: UnderstandingConfidence,
    /// Derivation (mirrors the card's provenance).
    pub derivation: EvidenceDerivation,
    /// Whether the card is cited.
    pub cited: bool,
    /// Number of cited evidence items behind the card.
    pub evidence_count: u32,
    /// Open-raw escape ref.
    pub open_raw_escape_ref: String,
    /// Open-source escape ref.
    pub open_source_escape_ref: String,
}

/// The evidence export projection for the card set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnderstandingEvidenceExport {
    /// Scope this export covers.
    pub scope: EvidenceExportScope,
    /// Whether the export preserves each card's card kind.
    pub preserves_card_kind: bool,
    /// Whether the export preserves each card's source class.
    pub preserves_source_class: bool,
    /// Whether the export preserves each card's confidence label.
    pub preserves_confidence: bool,
    /// Whether the export preserves the open-raw / open-source escapes.
    pub preserves_open_raw_open_source_escape: bool,
    /// Whether the export preserves inference / derivation labels.
    pub preserves_inference_labels: bool,
    /// Per-card export rows.
    pub rows: Vec<EvidenceExportRow>,
}

impl UnderstandingEvidenceExport {
    /// Whether the export preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_card_kind
            && self.preserves_source_class
            && self.preserves_confidence
            && self.preserves_open_raw_open_source_escape
            && self.preserves_inference_labels
    }
}

/// A packet-level understanding degradation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnderstandingDegradation {
    /// Degradation class.
    pub degradation_class: UnderstandingDegradationClass,
    /// Severity.
    pub severity: UnderstandingFindingSeverity,
    /// Human-readable summary (no raw bodies).
    pub summary: String,
    /// The card this degradation annotates, if scoped to one card.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_id_ref: Option<String>,
    /// Optional supporting evidence ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
}

/// How a consumer surface projects the card set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnderstandingConsumerProjection {
    /// Surface that consumes the card set.
    pub surface: UnderstandingConsumerSurface,
    /// Packet id this projection mirrors.
    pub packet_id_ref: String,
    /// Whether the surface preserves the chip set verbatim.
    pub preserves_chips: bool,
    /// Whether the surface preserves all three card kinds.
    pub preserves_cards: bool,
    /// Whether the surface preserves the confidence labels and reasons.
    pub preserves_confidence_labels: bool,
    /// Whether the surface preserves the evidence export.
    pub preserves_evidence_export: bool,
    /// Whether the surface preserves the open-raw / open-source escapes.
    pub preserves_open_raw_open_source_escape: bool,
}

impl UnderstandingConsumerProjection {
    /// Whether the projection preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_chips
            && self.preserves_cards
            && self.preserves_confidence_labels
            && self.preserves_evidence_export
            && self.preserves_open_raw_open_source_escape
    }
}

/// A single validation finding on the card set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnderstandingValidationFinding {
    /// Finding kind.
    pub finding_kind: UnderstandingFindingKind,
    /// Finding severity.
    pub severity: UnderstandingFindingSeverity,
    /// Human-readable summary.
    pub summary: String,
}

/// Constructor input for [`CodebaseUnderstandingCardsPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodebaseUnderstandingCardsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable region label.
    pub region_label: String,
    /// Opaque digest/ref for the region scope.
    pub region_digest_ref: String,
    /// The understanding cards.
    pub cards: Vec<UnderstandingCard>,
    /// The evidence export projection.
    pub evidence_export: UnderstandingEvidenceExport,
    /// Packet-level degradations.
    pub understanding_degradations: Vec<UnderstandingDegradation>,
    /// Consumer projections.
    pub consumer_projections: Vec<UnderstandingConsumerProjection>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp (RFC 3339).
    pub minted_at: String,
}

/// Export-safe topology/ownership/explainer card packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodebaseUnderstandingCardsPacket {
    /// Record kind; must equal [`UNDERSTANDING_CARDS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`UNDERSTANDING_CARDS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable region label.
    pub region_label: String,
    /// Opaque digest/ref for the region scope.
    pub region_digest_ref: String,
    /// The understanding cards.
    pub cards: Vec<UnderstandingCard>,
    /// The evidence export projection.
    pub evidence_export: UnderstandingEvidenceExport,
    /// Packet-level degradations.
    pub understanding_degradations: Vec<UnderstandingDegradation>,
    /// Consumer projections.
    pub consumer_projections: Vec<UnderstandingConsumerProjection>,
    /// Computed promotion state.
    pub promotion_state: UnderstandingPromotionState,
    /// Computed validation findings.
    pub validation_findings: Vec<UnderstandingValidationFinding>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Required consumer surfaces that every card packet must project.
const REQUIRED_SURFACES: [UnderstandingConsumerSurface; 5] = [
    UnderstandingConsumerSurface::CodebaseExplorer,
    UnderstandingConsumerSurface::DocsBrowser,
    UnderstandingConsumerSurface::GraphPanel,
    UnderstandingConsumerSurface::RetrievalInspector,
    UnderstandingConsumerSurface::SupportExport,
];

/// Card kinds that every packet must include so the lane stays the full
/// topology + ownership + explainer surface rather than a partial one.
const REQUIRED_CARD_KINDS: [UnderstandingCardKind; 3] = [
    UnderstandingCardKind::TopologyMap,
    UnderstandingCardKind::OwnershipSurface,
    UnderstandingCardKind::CodebaseExplainer,
];

impl CodebaseUnderstandingCardsPacket {
    /// Materializes a card packet, computing validation findings and the
    /// promotion state from the card input.
    pub fn materialize(input: CodebaseUnderstandingCardsPacketInput) -> Self {
        let mut findings = Vec::new();

        check_identity(&input, &mut findings);
        check_cards(&input, &mut findings);
        check_evidence_export(&input, &mut findings);
        check_degradations(&input, &mut findings);
        check_consumer_projections(&input, &mut findings);
        check_boundary(&input, &mut findings);

        let promotion_state = promotion_state(&findings, &input.understanding_degradations);

        Self {
            record_kind: UNDERSTANDING_CARDS_RECORD_KIND.to_owned(),
            schema_version: UNDERSTANDING_CARDS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            region_label: input.region_label,
            region_digest_ref: input.region_digest_ref,
            cards: input.cards,
            evidence_export: input.evidence_export,
            understanding_degradations: input.understanding_degradations,
            consumer_projections: input.consumer_projections,
            promotion_state,
            validation_findings: findings,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Whether the card set qualifies for the Stable claim with no findings.
    pub fn is_clean_stable(&self) -> bool {
        self.promotion_state == UnderstandingPromotionState::Stable
            && self.validation_findings.is_empty()
    }

    /// Wraps the packet in a support-export envelope.
    pub fn support_export(
        &self,
        export_id: &str,
        exported_at: &str,
    ) -> CodebaseUnderstandingCardsSupportExport {
        CodebaseUnderstandingCardsSupportExport {
            record_kind: UNDERSTANDING_CARDS_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: UNDERSTANDING_CARDS_SCHEMA_VERSION,
            export_id: export_id.to_owned(),
            exported_at: exported_at.to_owned(),
            schema_ref: UNDERSTANDING_CARDS_SCHEMA_REF.to_owned(),
            doc_ref: UNDERSTANDING_CARDS_DOC_REF.to_owned(),
            packet: self.clone(),
        }
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("understanding cards packet serializes")
    }

    /// Deterministic Markdown summary for docs, support, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Topology, Ownership, and Codebase Explainer Cards\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Region: {}\n", self.region_label));
        out.push_str(&format!(
            "- Promotion: `{}` ({} findings)\n",
            self.promotion_state.as_str(),
            self.validation_findings.len()
        ));
        out.push_str(&format!(
            "- Cards: {} | Degradations: {}\n",
            self.cards.len(),
            self.understanding_degradations.len()
        ));
        out.push_str("\n## Cards\n\n");
        for card in &self.cards {
            out.push_str(&format!(
                "- [{}] `{}` ({}) — {} / {} / {} / {} / {}\n",
                card.card_kind.as_str(),
                card.card_id,
                card.title,
                card.chips.source_class.as_str(),
                card.chips.version_match.as_str(),
                card.chips.freshness.as_str(),
                card.chips.locality.as_str(),
                card.chips.confidence.as_str(),
            ));
            out.push_str(&format!(
                "  - Confidence reason: {}\n",
                card.confidence_reason
            ));
            out.push_str(&format!(
                "  - Provenance: {} / cited={} / evidence={}\n",
                card.provenance.derivation.as_str(),
                card.provenance.cited,
                card.evidence.len(),
            ));
        }
        if !self.understanding_degradations.is_empty() {
            out.push_str("\n## Degradations\n\n");
            for degradation in &self.understanding_degradations {
                out.push_str(&format!(
                    "- [{}/{}]: {}\n",
                    degradation.degradation_class.as_str(),
                    degradation.severity.as_str(),
                    degradation.summary,
                ));
            }
        }
        out
    }
}

/// Support-export envelope for the card packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodebaseUnderstandingCardsSupportExport {
    /// Record kind; must equal [`UNDERSTANDING_CARDS_SUPPORT_EXPORT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Contract doc ref.
    pub doc_ref: String,
    /// The wrapped card packet.
    pub packet: CodebaseUnderstandingCardsPacket,
}

/// Errors emitted when reading the checked-in card support export.
#[derive(Debug)]
pub enum CodebaseUnderstandingCardsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Re-materialization disagreed with the checked-in promotion state.
    PromotionDrift {
        /// Promotion state recorded in the export.
        recorded: UnderstandingPromotionState,
        /// Promotion state computed by re-materialization.
        computed: UnderstandingPromotionState,
    },
    /// The checked-in packet should be clean Stable but is not.
    NotCleanStable(Vec<UnderstandingValidationFinding>),
}

impl fmt::Display for CodebaseUnderstandingCardsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "understanding cards export parse failed: {error}"
                )
            }
            Self::PromotionDrift { recorded, computed } => write!(
                formatter,
                "understanding cards promotion drift: recorded {} but computed {}",
                recorded.as_str(),
                computed.as_str()
            ),
            Self::NotCleanStable(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "understanding cards export is not clean stable: {tokens}"
                )
            }
        }
    }
}

impl Error for CodebaseUnderstandingCardsArtifactError {}

/// Reads and re-validates the checked-in stable card support export.
pub fn current_stable_codebase_understanding_cards_export(
) -> Result<CodebaseUnderstandingCardsSupportExport, CodebaseUnderstandingCardsArtifactError> {
    let export: CodebaseUnderstandingCardsSupportExport = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/add_topology_maps_ownership_surfaces_and_codebase_explainer_cards_with_cited_evidence_and_confidence_labels/support_export.json"
    )))
    .map_err(CodebaseUnderstandingCardsArtifactError::SupportExport)?;

    let recomputed = CodebaseUnderstandingCardsPacket::materialize(packet_to_input(&export.packet));
    if recomputed.promotion_state != export.packet.promotion_state {
        return Err(CodebaseUnderstandingCardsArtifactError::PromotionDrift {
            recorded: export.packet.promotion_state,
            computed: recomputed.promotion_state,
        });
    }
    if !export.packet.is_clean_stable() {
        return Err(CodebaseUnderstandingCardsArtifactError::NotCleanStable(
            export.packet.validation_findings.clone(),
        ));
    }
    Ok(export)
}

/// Rebuilds the materialization input from a packet (used for re-validation).
pub fn packet_to_input(
    packet: &CodebaseUnderstandingCardsPacket,
) -> CodebaseUnderstandingCardsPacketInput {
    CodebaseUnderstandingCardsPacketInput {
        packet_id: packet.packet_id.clone(),
        region_label: packet.region_label.clone(),
        region_digest_ref: packet.region_digest_ref.clone(),
        cards: packet.cards.clone(),
        evidence_export: packet.evidence_export.clone(),
        understanding_degradations: packet.understanding_degradations.clone(),
        consumer_projections: packet.consumer_projections.clone(),
        redaction_class_token: packet.redaction_class_token.clone(),
        minted_at: packet.minted_at.clone(),
    }
}

fn push_finding(
    findings: &mut Vec<UnderstandingValidationFinding>,
    kind: UnderstandingFindingKind,
    summary: impl Into<String>,
) {
    findings.push(UnderstandingValidationFinding {
        finding_kind: kind,
        severity: kind.default_severity(),
        summary: summary.into(),
    });
}

fn check_identity(
    input: &CodebaseUnderstandingCardsPacketInput,
    findings: &mut Vec<UnderstandingValidationFinding>,
) {
    if input.packet_id.trim().is_empty()
        || input.region_label.trim().is_empty()
        || input.region_digest_ref.trim().is_empty()
        || input.redaction_class_token.trim().is_empty()
        || input.minted_at.trim().is_empty()
    {
        push_finding(
            findings,
            UnderstandingFindingKind::MissingIdentity,
            "packet identity fields must all be present",
        );
    }
}

fn check_cards(
    input: &CodebaseUnderstandingCardsPacketInput,
    findings: &mut Vec<UnderstandingValidationFinding>,
) {
    if input.cards.is_empty() {
        push_finding(
            findings,
            UnderstandingFindingKind::CardsEmpty,
            "the card set must carry at least one card",
        );
        return;
    }

    let present_kinds: BTreeSet<UnderstandingCardKind> =
        input.cards.iter().map(|card| card.card_kind).collect();
    for required in REQUIRED_CARD_KINDS {
        if !present_kinds.contains(&required) {
            push_finding(
                findings,
                UnderstandingFindingKind::RequiredCardKindMissing,
                format!("required card kind `{}` is missing", required.as_str()),
            );
        }
    }

    let mut seen_card_ids: BTreeSet<&str> = BTreeSet::new();
    for card in &input.cards {
        if !seen_card_ids.insert(card.card_id.as_str()) {
            push_finding(
                findings,
                UnderstandingFindingKind::DuplicateCardId,
                format!("duplicate card id `{}`", card.card_id),
            );
        }
        check_one_card(card, findings);
    }
}

fn check_one_card(card: &UnderstandingCard, findings: &mut Vec<UnderstandingValidationFinding>) {
    if card.title.trim().is_empty() || card.headline.trim().is_empty() {
        push_finding(
            findings,
            UnderstandingFindingKind::CardTitleOrHeadlineMissing,
            format!("card `{}` is missing a title or headline", card.card_id),
        );
    }
    if card.confidence_reason.trim().is_empty() {
        push_finding(
            findings,
            UnderstandingFindingKind::ConfidenceReasonMissing,
            format!("card `{}` is missing a confidence reason", card.card_id),
        );
    }
    if card.open_raw_escape_ref.trim().is_empty() || card.open_source_escape_ref.trim().is_empty() {
        push_finding(
            findings,
            UnderstandingFindingKind::OpenRawOpenSourceEscapeMissing,
            format!(
                "card `{}` must keep open-raw and open-source escapes",
                card.card_id
            ),
        );
    }

    // A derived or inferred card must stay cited; an inferred card may not be
    // presented as high confidence.
    if card.provenance.derivation.needs_citation() && !card.provenance.cited {
        push_finding(
            findings,
            UnderstandingFindingKind::CardNotCited,
            format!(
                "card `{}` is `{}` but is not cited",
                card.card_id,
                card.provenance.derivation.as_str()
            ),
        );
    }
    if matches!(
        card.provenance.derivation,
        EvidenceDerivation::InferredExplanation
    ) && card.chips.confidence == UnderstandingConfidence::High
    {
        push_finding(
            findings,
            UnderstandingFindingKind::InferredCardLooksAuthoritative,
            format!(
                "card `{}` is an inferred explanation presented as high confidence",
                card.card_id
            ),
        );
    }
    if !card.chips.version_match.is_confident_current()
        && card.chips.confidence == UnderstandingConfidence::High
        && card.chips.freshness.is_authoritative_live()
    {
        push_finding(
            findings,
            UnderstandingFindingKind::VersionTruthCollapsed,
            format!(
                "card `{}` presents version `{}` as a confident live match",
                card.card_id,
                card.chips.version_match.as_str()
            ),
        );
    }

    check_card_evidence(card, findings);
    check_card_kind_payload(card, findings);
}

fn check_card_evidence(
    card: &UnderstandingCard,
    findings: &mut Vec<UnderstandingValidationFinding>,
) {
    if card.evidence.is_empty() {
        push_finding(
            findings,
            UnderstandingFindingKind::CardEvidenceMissing,
            format!("card `{}` carries no backing evidence", card.card_id),
        );
        return;
    }

    let mut seen_evidence_ids: BTreeSet<&str> = BTreeSet::new();
    for evidence in &card.evidence {
        if !seen_evidence_ids.insert(evidence.evidence_id.as_str()) {
            push_finding(
                findings,
                UnderstandingFindingKind::DuplicateEvidenceId,
                format!(
                    "card `{}` repeats evidence id `{}`",
                    card.card_id, evidence.evidence_id
                ),
            );
        }
        if evidence.derivation.needs_citation() && !evidence.cited {
            push_finding(
                findings,
                UnderstandingFindingKind::EvidenceNotCited,
                format!(
                    "card `{}` evidence `{}` is `{}` but is not cited",
                    card.card_id,
                    evidence.evidence_id,
                    evidence.derivation.as_str()
                ),
            );
        }
        if evidence.open_raw_escape_ref.trim().is_empty()
            || evidence.open_source_escape_ref.trim().is_empty()
        {
            push_finding(
                findings,
                UnderstandingFindingKind::EvidenceEscapeMissing,
                format!(
                    "card `{}` evidence `{}` must keep open-raw and open-source escapes",
                    card.card_id, evidence.evidence_id
                ),
            );
        }
    }
}

fn check_card_kind_payload(
    card: &UnderstandingCard,
    findings: &mut Vec<UnderstandingValidationFinding>,
) {
    match card.card_kind {
        UnderstandingCardKind::TopologyMap => {
            if card.topology_edges.is_empty() {
                push_finding(
                    findings,
                    UnderstandingFindingKind::TopologyCardMissingEdges,
                    format!(
                        "topology card `{}` surfaces no topology edges",
                        card.card_id
                    ),
                );
            }
            for edge in &card.topology_edges {
                if edge.from_ref.trim().is_empty() || edge.to_ref.trim().is_empty() {
                    push_finding(
                        findings,
                        UnderstandingFindingKind::TopologyEdgeEndpointMissing,
                        format!(
                            "topology card `{}` has an edge missing an endpoint",
                            card.card_id
                        ),
                    );
                }
            }
        }
        UnderstandingCardKind::OwnershipSurface => {
            if card.owners.is_empty() {
                push_finding(
                    findings,
                    UnderstandingFindingKind::OwnershipCardMissingOwner,
                    format!("ownership card `{}` declares no owner", card.card_id),
                );
            }
            // A heuristic or unassigned ownership basis may not back a
            // high-confidence ownership claim.
            if card.chips.confidence == UnderstandingConfidence::High
                && card
                    .owners
                    .iter()
                    .any(|owner| !owner.ownership_basis.is_authoritative_declaration())
            {
                push_finding(
                    findings,
                    UnderstandingFindingKind::OwnershipBasisUnattributed,
                    format!(
                        "ownership card `{}` claims high confidence over a heuristic or unassigned owner",
                        card.card_id
                    ),
                );
            }
        }
        UnderstandingCardKind::CodebaseExplainer => {}
    }
}

fn check_evidence_export(
    input: &CodebaseUnderstandingCardsPacketInput,
    findings: &mut Vec<UnderstandingValidationFinding>,
) {
    let export = &input.evidence_export;
    if !export.preserves_all() {
        push_finding(
            findings,
            UnderstandingFindingKind::EvidenceExportDropsPreservation,
            "the evidence export must preserve card kind, source class, confidence, escapes, and inference labels",
        );
    }

    let mut export_ids: BTreeSet<&str> = BTreeSet::new();
    for row in &export.rows {
        export_ids.insert(row.card_id_ref.as_str());
        let card = input
            .cards
            .iter()
            .find(|card| card.card_id == row.card_id_ref);
        match card {
            None => push_finding(
                findings,
                UnderstandingFindingKind::EvidenceExportRowOrphan,
                format!(
                    "evidence export row references unknown card `{}`",
                    row.card_id_ref
                ),
            ),
            Some(card) => {
                if card.card_kind != row.card_kind {
                    push_finding(
                        findings,
                        UnderstandingFindingKind::EvidenceExportCardKindMismatch,
                        format!(
                            "export for `{}` records kind `{}` but the card is `{}`",
                            row.card_id_ref,
                            row.card_kind.as_str(),
                            card.card_kind.as_str()
                        ),
                    );
                }
                if card.chips.source_class != row.source_class {
                    push_finding(
                        findings,
                        UnderstandingFindingKind::EvidenceExportSourceClassMismatch,
                        format!(
                            "export for `{}` records source `{}` but the card chip is `{}`",
                            row.card_id_ref,
                            row.source_class.as_str(),
                            card.chips.source_class.as_str()
                        ),
                    );
                }
                if card.chips.confidence != row.confidence {
                    push_finding(
                        findings,
                        UnderstandingFindingKind::EvidenceExportConfidenceMismatch,
                        format!(
                            "export for `{}` records confidence `{}` but the card chip is `{}`",
                            row.card_id_ref,
                            row.confidence.as_str(),
                            card.chips.confidence.as_str()
                        ),
                    );
                }
            }
        }
    }

    for card in &input.cards {
        if !export_ids.contains(card.card_id.as_str()) {
            push_finding(
                findings,
                UnderstandingFindingKind::EvidenceExportCoverageMissing,
                format!("card `{}` has no evidence export row", card.card_id),
            );
        }
    }
}

fn check_degradations(
    input: &CodebaseUnderstandingCardsPacketInput,
    findings: &mut Vec<UnderstandingValidationFinding>,
) {
    let card_ids: BTreeSet<&str> = input
        .cards
        .iter()
        .map(|card| card.card_id.as_str())
        .collect();

    for degradation in &input.understanding_degradations {
        if degradation.summary.trim().is_empty() {
            push_finding(
                findings,
                UnderstandingFindingKind::DegradationIncomplete,
                format!(
                    "degradation `{}` is missing a summary",
                    degradation.degradation_class.as_str()
                ),
            );
        }
        if let Some(card_id) = &degradation.card_id_ref {
            if !card_id.trim().is_empty() && !card_ids.contains(card_id.as_str()) {
                push_finding(
                    findings,
                    UnderstandingFindingKind::DegradationOrphan,
                    format!("degradation references unknown card `{}`", card_id),
                );
            }
        }
    }
}

fn check_consumer_projections(
    input: &CodebaseUnderstandingCardsPacketInput,
    findings: &mut Vec<UnderstandingValidationFinding>,
) {
    let present: BTreeSet<UnderstandingConsumerSurface> = input
        .consumer_projections
        .iter()
        .map(|projection| projection.surface)
        .collect();
    for required in REQUIRED_SURFACES {
        if !present.contains(&required) {
            push_finding(
                findings,
                UnderstandingFindingKind::RequiredSurfaceCoverageMissing,
                format!("required surface `{}` is missing", required.as_str()),
            );
        }
    }

    for projection in &input.consumer_projections {
        if projection.packet_id_ref != input.packet_id {
            push_finding(
                findings,
                UnderstandingFindingKind::ConsumerProjectionPacketIdMismatch,
                format!(
                    "surface `{}` references packet `{}`",
                    projection.surface.as_str(),
                    projection.packet_id_ref
                ),
            );
        }
        if !projection.preserves_all() {
            push_finding(
                findings,
                UnderstandingFindingKind::ConsumerProjectionDrift,
                format!(
                    "surface `{}` drops a required preservation flag",
                    projection.surface.as_str()
                ),
            );
        }
    }
}

fn check_boundary(
    input: &CodebaseUnderstandingCardsPacketInput,
    findings: &mut Vec<UnderstandingValidationFinding>,
) {
    let value = serde_json::to_value(input).expect("understanding cards input serializes");
    if json_contains_forbidden_boundary_material(&value) {
        push_finding(
            findings,
            UnderstandingFindingKind::RawBoundaryMaterialPresent,
            "export must not carry raw bodies, raw source, or secrets",
        );
    }
}

/// Computes the promotion state from the worst severity across both the
/// validation findings and the attached degradations.
///
/// A blocking finding (integrity, trust, citation, or boundary violation) blocks
/// the Stable claim; an otherwise-clean card set that carries a narrowing
/// degradation narrows below Stable rather than hiding the cards.
fn promotion_state(
    findings: &[UnderstandingValidationFinding],
    degradations: &[UnderstandingDegradation],
) -> UnderstandingPromotionState {
    let any_blocking = findings
        .iter()
        .any(|finding| finding.severity == UnderstandingFindingSeverity::Blocking)
        || degradations
            .iter()
            .any(|degradation| degradation.severity == UnderstandingFindingSeverity::Blocking);
    if any_blocking {
        return UnderstandingPromotionState::BlocksStable;
    }

    let any_narrowing = findings
        .iter()
        .any(|finding| finding.severity == UnderstandingFindingSeverity::Narrowing)
        || degradations
            .iter()
            .any(|degradation| degradation.severity == UnderstandingFindingSeverity::Narrowing);
    if any_narrowing {
        UnderstandingPromotionState::NarrowedBelowStable
    } else {
        UnderstandingPromotionState::Stable
    }
}

/// Heuristic that rejects obviously forbidden material in the export.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("raw_body:")
                || lower.contains("raw_source:")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Seeded stable card input used by the producer, tests, and fixtures.
pub fn seeded_stable_codebase_understanding_cards_input() -> CodebaseUnderstandingCardsPacketInput {
    let packet_id = "packet:m5:understanding_cards:net_retry_region".to_owned();
    CodebaseUnderstandingCardsPacketInput {
        packet_id: packet_id.clone(),
        region_label: "codebase understanding: the networking retry region".to_owned(),
        region_digest_ref: "regiondigest:sha256:net-retry-region".to_owned(),
        cards: vec![
            topology_card(),
            ownership_card(),
            explainer_card(),
        ],
        evidence_export: seeded_evidence_export(),
        understanding_degradations: vec![UnderstandingDegradation {
            degradation_class: UnderstandingDegradationClass::GraphIndexStale,
            severity: UnderstandingFindingSeverity::Advisory,
            summary: "the workspace graph was indexed before the last two commits; topology edges may lag the working tree".to_owned(),
            card_id_ref: None,
            evidence_ref: Some("evidence:index-freshness:workspace-graph".to_owned()),
        }],
        consumer_projections: required_projections(&packet_id),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-09T00:00:00Z".to_owned(),
    }
}

fn topology_card() -> UnderstandingCard {
    UnderstandingCard {
        card_id: "card:topology:net_retry_region".to_owned(),
        card_kind: UnderstandingCardKind::TopologyMap,
        subject_ref: "region:crates/aureline-net".to_owned(),
        title: "Networking retry region topology".to_owned(),
        headline: "aureline-net depends on aureline-core and contains the retry and http-client modules".to_owned(),
        chips: UnderstandingChipSet {
            source_class: UnderstandingSourceClass::GraphIndex,
            version_match: UnderstandingVersionMatch::ExactBuildMatch,
            freshness: UnderstandingFreshness::WarmCached,
            locality: UnderstandingLocality::Local,
            confidence: UnderstandingConfidence::Medium,
        },
        confidence_reason: "derived from the workspace graph index at the active build revision; warm-cached so labelled medium rather than live".to_owned(),
        evidence: vec![
            CardEvidence {
                evidence_id: "evidence:topology:depends_edge".to_owned(),
                subject_kind: EvidenceSubjectKind::DependencyEdge,
                subject_ref: "edge:aureline-net->aureline-core".to_owned(),
                derivation: EvidenceDerivation::VerbatimNode,
                cited: true,
                citation_ref: Some("cite:graph:edge:aureline-net->aureline-core".to_owned()),
                open_raw_escape_ref: "open-raw:graph:edge:aureline-net->aureline-core".to_owned(),
                open_source_escape_ref: "open-source:repo:crates/aureline-net/Cargo.toml".to_owned(),
                note: "depends-on edge read directly from the resolved dependency graph".to_owned(),
            },
            CardEvidence {
                evidence_id: "evidence:topology:contains_retry".to_owned(),
                subject_kind: EvidenceSubjectKind::CodeModule,
                subject_ref: "module:crates/aureline-net/src/retry.rs".to_owned(),
                derivation: EvidenceDerivation::VerbatimNode,
                cited: true,
                citation_ref: Some("cite:graph:module:aureline-net::retry".to_owned()),
                open_raw_escape_ref: "open-raw:graph:module:aureline-net::retry".to_owned(),
                open_source_escape_ref: "open-source:repo:crates/aureline-net/src/retry.rs".to_owned(),
                note: "containment edge from the crate node to the retry module".to_owned(),
            },
        ],
        topology_edges: vec![
            TopologyEdgeRef {
                from_ref: "crate:aureline-net".to_owned(),
                to_ref: "crate:aureline-core".to_owned(),
                edge_kind: TopologyEdgeKind::DependsOn,
                note: "resolved dependency edge".to_owned(),
            },
            TopologyEdgeRef {
                from_ref: "crate:aureline-net".to_owned(),
                to_ref: "module:aureline-net::retry".to_owned(),
                edge_kind: TopologyEdgeKind::Contains,
                note: "crate contains the retry module".to_owned(),
            },
            TopologyEdgeRef {
                from_ref: "module:aureline-net::http_client".to_owned(),
                to_ref: "symbol:aureline-net::retry::retry_with_backoff".to_owned(),
                edge_kind: TopologyEdgeKind::Calls,
                note: "the http client calls the retry helper".to_owned(),
            },
        ],
        owners: Vec::new(),
        provenance: CardProvenance {
            pack_id_ref: "repo:aureline-workspace".to_owned(),
            pack_pinned: true,
            pack_signed_and_verified: true,
            derivation: EvidenceDerivation::DerivedSummary,
            cited: true,
            citation_ref: Some("cite:graph:region:crates/aureline-net".to_owned()),
        },
        open_raw_escape_ref: "open-raw:graph:region:crates/aureline-net".to_owned(),
        open_source_escape_ref: "open-source:repo:crates/aureline-net".to_owned(),
    }
}

fn ownership_card() -> UnderstandingCard {
    UnderstandingCard {
        card_id: "card:ownership:net_retry_region".to_owned(),
        card_kind: UnderstandingCardKind::OwnershipSurface,
        subject_ref: "region:crates/aureline-net".to_owned(),
        title: "Networking retry region ownership".to_owned(),
        headline: "the networking team owns crates/aureline-net via a matched CODEOWNERS entry".to_owned(),
        chips: UnderstandingChipSet {
            source_class: UnderstandingSourceClass::CodeownersFile,
            version_match: UnderstandingVersionMatch::ExactBuildMatch,
            freshness: UnderstandingFreshness::AuthoritativeLive,
            locality: UnderstandingLocality::Local,
            confidence: UnderstandingConfidence::High,
        },
        confidence_reason: "owner read verbatim from a matched CODEOWNERS entry at the active revision; authoritative declaration so labelled high".to_owned(),
        evidence: vec![CardEvidence {
            evidence_id: "evidence:ownership:codeowners_entry".to_owned(),
            subject_kind: EvidenceSubjectKind::OwnerEntry,
            subject_ref: "owner:team-networking".to_owned(),
            derivation: EvidenceDerivation::VerbatimNode,
            cited: true,
            citation_ref: Some("cite:codeowners:crates/aureline-net".to_owned()),
            open_raw_escape_ref: "open-raw:codeowners:crates/aureline-net".to_owned(),
            open_source_escape_ref: "open-source:repo:.github/CODEOWNERS".to_owned(),
            note: "matched glob `crates/aureline-net/` maps to the networking team".to_owned(),
        }],
        topology_edges: Vec::new(),
        owners: vec![OwnerRef {
            owner_ref: "owner:team-networking".to_owned(),
            ownership_basis: OwnershipBasis::CodeownersEntry,
            coverage_note: "covers the whole crate directory".to_owned(),
        }],
        provenance: CardProvenance {
            pack_id_ref: "repo:aureline-workspace".to_owned(),
            pack_pinned: true,
            pack_signed_and_verified: true,
            derivation: EvidenceDerivation::VerbatimNode,
            cited: true,
            citation_ref: Some("cite:codeowners:crates/aureline-net".to_owned()),
        },
        open_raw_escape_ref: "open-raw:ownership:region:crates/aureline-net".to_owned(),
        open_source_escape_ref: "open-source:repo:.github/CODEOWNERS".to_owned(),
    }
}

fn explainer_card() -> UnderstandingCard {
    UnderstandingCard {
        card_id: "card:explainer:retry_with_backoff".to_owned(),
        card_kind: UnderstandingCardKind::CodebaseExplainer,
        subject_ref: "symbol:aureline-net::retry::retry_with_backoff".to_owned(),
        title: "What retry_with_backoff does".to_owned(),
        headline: "retry_with_backoff retries a fallible send with exponential backoff and jitter, bounded by a max-attempts policy".to_owned(),
        chips: UnderstandingChipSet {
            source_class: UnderstandingSourceClass::WorkspaceCode,
            version_match: UnderstandingVersionMatch::ExactBuildMatch,
            freshness: UnderstandingFreshness::WarmCached,
            locality: UnderstandingLocality::Local,
            confidence: UnderstandingConfidence::Medium,
        },
        confidence_reason: "an inferred explanation generated over the cited symbol and retry-policy guide; labelled medium because it is inferred, not a verbatim doc".to_owned(),
        evidence: vec![
            CardEvidence {
                evidence_id: "evidence:explainer:symbol".to_owned(),
                subject_kind: EvidenceSubjectKind::CodeSymbol,
                subject_ref: "symbol:aureline-net::retry::retry_with_backoff".to_owned(),
                derivation: EvidenceDerivation::ExtractedSnippet,
                cited: true,
                citation_ref: Some("cite:symbol:aureline-net::retry::retry_with_backoff".to_owned()),
                open_raw_escape_ref: "open-raw:symbol:aureline-net::retry::retry_with_backoff".to_owned(),
                open_source_escape_ref: "open-source:repo:crates/aureline-net/src/retry.rs".to_owned(),
                note: "signature and loop body the explanation is grounded in".to_owned(),
            },
            CardEvidence {
                evidence_id: "evidence:explainer:policy_guide".to_owned(),
                subject_kind: EvidenceSubjectKind::DocsNode,
                subject_ref: "docnode:project-docs:net/retry-policy".to_owned(),
                derivation: EvidenceDerivation::VerbatimNode,
                cited: true,
                citation_ref: Some("cite:docnode:project-docs:net/retry-policy".to_owned()),
                open_raw_escape_ref: "open-raw:docnode:project-docs:net/retry-policy".to_owned(),
                open_source_escape_ref: "open-source:repo:docs/net/retry-policy.md".to_owned(),
                note: "the retry-policy guide that documents the backoff window".to_owned(),
            },
        ],
        topology_edges: Vec::new(),
        owners: Vec::new(),
        provenance: CardProvenance {
            pack_id_ref: "repo:aureline-workspace".to_owned(),
            pack_pinned: true,
            pack_signed_and_verified: true,
            derivation: EvidenceDerivation::InferredExplanation,
            cited: true,
            citation_ref: Some("cite:explainer:retry_with_backoff".to_owned()),
        },
        open_raw_escape_ref: "open-raw:symbol:aureline-net::retry::retry_with_backoff".to_owned(),
        open_source_escape_ref: "open-source:repo:crates/aureline-net/src/retry.rs".to_owned(),
    }
}

fn seeded_evidence_export() -> UnderstandingEvidenceExport {
    UnderstandingEvidenceExport {
        scope: EvidenceExportScope::AllCards,
        preserves_card_kind: true,
        preserves_source_class: true,
        preserves_confidence: true,
        preserves_open_raw_open_source_escape: true,
        preserves_inference_labels: true,
        rows: vec![
            EvidenceExportRow {
                card_id_ref: "card:topology:net_retry_region".to_owned(),
                card_kind: UnderstandingCardKind::TopologyMap,
                source_class: UnderstandingSourceClass::GraphIndex,
                confidence: UnderstandingConfidence::Medium,
                derivation: EvidenceDerivation::DerivedSummary,
                cited: true,
                evidence_count: 2,
                open_raw_escape_ref: "open-raw:graph:region:crates/aureline-net".to_owned(),
                open_source_escape_ref: "open-source:repo:crates/aureline-net".to_owned(),
            },
            EvidenceExportRow {
                card_id_ref: "card:ownership:net_retry_region".to_owned(),
                card_kind: UnderstandingCardKind::OwnershipSurface,
                source_class: UnderstandingSourceClass::CodeownersFile,
                confidence: UnderstandingConfidence::High,
                derivation: EvidenceDerivation::VerbatimNode,
                cited: true,
                evidence_count: 1,
                open_raw_escape_ref: "open-raw:ownership:region:crates/aureline-net".to_owned(),
                open_source_escape_ref: "open-source:repo:.github/CODEOWNERS".to_owned(),
            },
            EvidenceExportRow {
                card_id_ref: "card:explainer:retry_with_backoff".to_owned(),
                card_kind: UnderstandingCardKind::CodebaseExplainer,
                source_class: UnderstandingSourceClass::WorkspaceCode,
                confidence: UnderstandingConfidence::Medium,
                derivation: EvidenceDerivation::InferredExplanation,
                cited: true,
                evidence_count: 2,
                open_raw_escape_ref: "open-raw:symbol:aureline-net::retry::retry_with_backoff"
                    .to_owned(),
                open_source_escape_ref: "open-source:repo:crates/aureline-net/src/retry.rs"
                    .to_owned(),
            },
        ],
    }
}

fn required_projections(packet_id: &str) -> Vec<UnderstandingConsumerProjection> {
    [
        UnderstandingConsumerSurface::CodebaseExplorer,
        UnderstandingConsumerSurface::DocsBrowser,
        UnderstandingConsumerSurface::GraphPanel,
        UnderstandingConsumerSurface::AiContext,
        UnderstandingConsumerSurface::RetrievalInspector,
        UnderstandingConsumerSurface::CliHeadless,
        UnderstandingConsumerSurface::SupportExport,
        UnderstandingConsumerSurface::HelpAbout,
    ]
    .into_iter()
    .map(|surface| UnderstandingConsumerProjection {
        surface,
        packet_id_ref: packet_id.to_owned(),
        preserves_chips: true,
        preserves_cards: true,
        preserves_confidence_labels: true,
        preserves_evidence_export: true,
        preserves_open_raw_open_source_escape: true,
    })
    .collect()
}
