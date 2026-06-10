//! Retrieval-debug surfaces for docs, semantic recall, and AI context with
//! exact / imported / heuristic labelling.
//!
//! This module implements the M5 retrieval-debug inspector: the surface that
//! lets a reader see *why* a result was retrieved and *how trustworthy* it is.
//! Each [`RetrievalDebugEntry`] belongs to one [`RetrievalLane`] (docs search,
//! semantic recall, or AI context assembly), carries the same
//! source/version/freshness/locality/confidence chip set the other docs-recall
//! lanes use, an explicit, non-empty list of [`RankingSignal`] reasons, and one
//! [`RetrievalDerivationLabel`] — the central `exact` / `imported` / `heuristic`
//! label that tells the reader whether the result is a verbatim match from a
//! verified local source (`exact`), came in through an imported / mirrored pack
//! (`imported`), or is an inferred / fuzzy / lexical-fallback match
//! (`heuristic`). A `heuristic` or `imported` entry may not be presented as a
//! high-confidence live truth, and must stay cited and escapable.
//!
//! The [`RetrievalDebugExport`] is the cited projection support, AI evidence,
//! and diagnostics surfaces ingest: one [`RetrievalDebugExportRow`] per entry
//! preserving lane, derivation label, source class, confidence, citation state,
//! ranking-signal count, and the open-raw / open-source escapes.
//!
//! [`RetrievalDebugPacket::materialize`] computes the validation findings and
//! the promotion state (`stable`, `narrowed_below_stable`, or `blocks_stable`)
//! from the input, so a stale, uncited, over-authoritative, reasonless, or
//! unattributed retrieval-debug set automatically narrows or blocks before it
//! reaches a consumer surface. The packet is an inspectable, serde-serializable
//! truth packet: it carries no raw query text, no raw document bodies, no raw
//! source files, no raw provider payloads, and no credentials — only metadata,
//! chip truth, ranking reasons, derivation labels, cited refs, provenance,
//! finding summaries, and contract refs.
//!
//! The boundary schema is
//! [`schemas/docs/ship-retrieval-debug-surfaces-for-docs-recall-and-ai-context-with-exact-or-imported-or-heuristic-labeling.schema.json`](../../../../schemas/docs/ship-retrieval-debug-surfaces-for-docs-recall-and-ai-context-with-exact-or-imported-or-heuristic-labeling.schema.json).
//! The contract doc is
//! [`docs/docs/m5/ship_retrieval_debug_surfaces_for_docs_recall_and_ai_context_with_exact_or_imported_or_heuristic_labeling.md`](../../../../docs/docs/m5/ship_retrieval_debug_surfaces_for_docs_recall_and_ai_context_with_exact_or_imported_or_heuristic_labeling.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/ship_retrieval_debug_surfaces_for_docs_recall_and_ai_context_with_exact_or_imported_or_heuristic_labeling/`](../../../../fixtures/docs/m5/ship_retrieval_debug_surfaces_for_docs_recall_and_ai_context_with_exact_or_imported_or_heuristic_labeling/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`RetrievalDebugPacket`].
pub const RETRIEVAL_DEBUG_RECORD_KIND: &str =
    "retrieval_debug_surfaces_for_docs_recall_and_ai_context";

/// Record-kind tag carried by the support-export wrapper.
pub const RETRIEVAL_DEBUG_SUPPORT_EXPORT_RECORD_KIND: &str =
    "retrieval_debug_surfaces_for_docs_recall_and_ai_context_support_export";

/// Schema version for retrieval-debug records.
pub const RETRIEVAL_DEBUG_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const RETRIEVAL_DEBUG_SCHEMA_REF: &str =
    "schemas/docs/ship-retrieval-debug-surfaces-for-docs-recall-and-ai-context-with-exact-or-imported-or-heuristic-labeling.schema.json";

/// Repo-relative path of the retrieval-debug contract doc.
pub const RETRIEVAL_DEBUG_DOC_REF: &str =
    "docs/docs/m5/ship_retrieval_debug_surfaces_for_docs_recall_and_ai_context_with_exact_or_imported_or_heuristic_labeling.md";

/// Repo-relative path of the protected fixture directory.
pub const RETRIEVAL_DEBUG_FIXTURE_DIR: &str =
    "fixtures/docs/m5/ship_retrieval_debug_surfaces_for_docs_recall_and_ai_context_with_exact_or_imported_or_heuristic_labeling";

/// Repo-relative path of the checked support-export artifact.
pub const RETRIEVAL_DEBUG_ARTIFACT_REF: &str =
    "artifacts/docs/m5/ship_retrieval_debug_surfaces_for_docs_recall_and_ai_context_with_exact_or_imported_or_heuristic_labeling/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const RETRIEVAL_DEBUG_SUMMARY_REF: &str =
    "artifacts/docs/m5/ship_retrieval_debug_surfaces_for_docs_recall_and_ai_context_with_exact_or_imported_or_heuristic_labeling.md";

/// One of the three retrieval lanes the inspector covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalLane {
    /// Lexical / symbol docs search.
    DocsSearch,
    /// Embedding-backed semantic recall.
    SemanticRecall,
    /// AI context assembly (what was injected into a prompt context).
    AiContext,
}

impl RetrievalLane {
    /// Every lane, in declaration order.
    pub const ALL: [Self; 3] = [Self::DocsSearch, Self::SemanticRecall, Self::AiContext];

    /// Stable token recorded in the entry.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsSearch => "docs_search",
            Self::SemanticRecall => "semantic_recall",
            Self::AiContext => "ai_context",
        }
    }
}

/// The central exact / imported / heuristic derivation label for one entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalDerivationLabel {
    /// A verbatim, exact match from a verified local / pinned source.
    Exact,
    /// A match that came in through an imported / mirrored pack or external import.
    Imported,
    /// An inferred, fuzzy, or lexical-fallback match — not an exact result.
    Heuristic,
}

impl RetrievalDerivationLabel {
    /// Every label, in declaration order.
    pub const ALL: [Self; 3] = [Self::Exact, Self::Imported, Self::Heuristic];

    /// Stable token recorded in the entry.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Imported => "imported",
            Self::Heuristic => "heuristic",
        }
    }

    /// Whether this label is a verbatim exact match (the only label that may
    /// back a high-confidence live claim without a downgrade).
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact)
    }

    /// Whether this label must carry a citation to stay honest. Imported and
    /// heuristic results must always be cited back to where they came from.
    pub const fn needs_citation(self) -> bool {
        matches!(self, Self::Imported | Self::Heuristic)
    }

    /// Whether this label may be presented at high confidence. A heuristic
    /// result may never be high confidence; an imported result may only when it
    /// is a verbatim copy, so the inspector keeps imported high-confidence
    /// claims honest through the version/freshness checks rather than the label.
    pub const fn may_be_high_confidence(self) -> bool {
        !matches!(self, Self::Heuristic)
    }
}

/// Source class for an entry's underlying material, projected as the source chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalSourceClass {
    /// Symbol or file in the active workspace code.
    WorkspaceCode,
    /// Symbol or file in a resolved dependency / vendored source.
    DependencySource,
    /// The workspace graph index.
    GraphIndex,
    /// Workspace-local project docs.
    ProjectDocs,
    /// Generated API / reference docs.
    GeneratedReference,
    /// Pinned, signed mirror of official upstream docs.
    MirroredOfficialDocs,
    /// An imported third-party docs pack.
    ImportedPack,
    /// AI-assembled context (a derived context bundle).
    AiAssembledContext,
}

impl RetrievalSourceClass {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceCode => "workspace_code",
            Self::DependencySource => "dependency_source",
            Self::GraphIndex => "graph_index",
            Self::ProjectDocs => "project_docs",
            Self::GeneratedReference => "generated_reference",
            Self::MirroredOfficialDocs => "mirrored_official_docs",
            Self::ImportedPack => "imported_pack",
            Self::AiAssembledContext => "ai_assembled_context",
        }
    }
}

/// Version-match state for an entry, projected as the version chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalVersionMatch {
    /// Entry matches the active build/workspace revision exactly.
    ExactBuildMatch,
    /// Entry is within an accepted compatible drift window.
    CompatibleMinorDrift,
    /// Entry drifted incompatibly from the active target.
    IncompatibleDriftDetected,
    /// Pre-release entry has not completed verification.
    PreReleaseUnverified,
    /// The target build/workspace revision could not be verified.
    UnknownTargetBuild,
}

impl RetrievalVersionMatch {
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

/// Freshness state for an entry, projected as the freshness chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalFreshness {
    /// Entry was live and authoritative at materialization time.
    AuthoritativeLive,
    /// Cached entry within its freshness window.
    WarmCached,
    /// Cached entry usable only with degraded disclosure.
    DegradedCached,
    /// Entry is stale and must not claim current authority.
    Stale,
    /// Freshness could not be verified.
    Unverified,
    /// A refresh is pending; the source has not yet re-synced.
    RefreshPending,
}

impl RetrievalFreshness {
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

/// Locality / install posture for an entry, projected as the locality chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalLocality {
    /// Resolved from local content or the in-repo index.
    Local,
    /// Resolved through a pinned mirror pack.
    MirroredPack,
    /// Resolved through a remote helper.
    RemoteHelper,
    /// Resolved through a managed (org-hosted) service.
    Managed,
}

impl RetrievalLocality {
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

/// Confidence label for an entry, projected as the confidence chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalConfidence {
    /// High confidence.
    High,
    /// Medium confidence.
    Medium,
    /// Low confidence.
    Low,
    /// Heuristic only; not a verified claim.
    Heuristic,
}

impl RetrievalConfidence {
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

/// Kind of ranking signal that contributed to an entry's score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RankingSignalKind {
    /// Lexical / keyword match.
    LexicalMatch,
    /// Semantic / embedding similarity.
    SemanticSimilarity,
    /// Exact symbol-name match.
    SymbolExactMatch,
    /// Path / module proximity to the query scope.
    PathProximity,
    /// Recency boost.
    RecencyBoost,
    /// Boost for a pinned / signed source.
    PinnedSourceBoost,
    /// Penalty for stale freshness.
    FreshnessPenalty,
    /// Penalty for an imported / unverified source.
    ImportedSourcePenalty,
    /// Penalty for a heuristic / fuzzy match.
    HeuristicPenalty,
}

impl RankingSignalKind {
    /// Stable token recorded in the signal.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LexicalMatch => "lexical_match",
            Self::SemanticSimilarity => "semantic_similarity",
            Self::SymbolExactMatch => "symbol_exact_match",
            Self::PathProximity => "path_proximity",
            Self::RecencyBoost => "recency_boost",
            Self::PinnedSourceBoost => "pinned_source_boost",
            Self::FreshnessPenalty => "freshness_penalty",
            Self::ImportedSourcePenalty => "imported_source_penalty",
            Self::HeuristicPenalty => "heuristic_penalty",
        }
    }
}

/// Whether a ranking signal raised, lowered, or did not change the score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalContribution {
    /// The signal raised the score.
    Boost,
    /// The signal lowered the score.
    Penalty,
    /// The signal was recorded but did not move the score.
    Neutral,
}

impl SignalContribution {
    /// Stable token recorded in the signal.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Boost => "boost",
            Self::Penalty => "penalty",
            Self::Neutral => "neutral",
        }
    }
}

/// Kind of subject a retrieval-debug entry points at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalSubjectKind {
    /// A docs node.
    DocsNode,
    /// A code symbol.
    CodeSymbol,
    /// A code file.
    CodeFile,
    /// A code module / directory region.
    CodeModule,
    /// A docs pack node.
    PackNode,
    /// An AI context fragment.
    ContextFragment,
}

impl RetrievalSubjectKind {
    /// Stable token recorded in the entry.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsNode => "docs_node",
            Self::CodeSymbol => "code_symbol",
            Self::CodeFile => "code_file",
            Self::CodeModule => "code_module",
            Self::PackNode => "pack_node",
            Self::ContextFragment => "context_fragment",
        }
    }
}

/// Severity of a degradation or validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalFindingSeverity {
    /// Blocks a Stable claim; the set must block.
    Blocking,
    /// Narrows below Stable but the set stays valid and attributable.
    Narrowing,
    /// Advisory only.
    Advisory,
}

impl RetrievalFindingSeverity {
    /// Stable token recorded in the finding.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocking => "blocking",
            Self::Narrowing => "narrowing",
            Self::Advisory => "advisory",
        }
    }
}

/// Consumer surface that must project the retrieval-debug packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalConsumerSurface {
    /// The retrieval-debug inspector itself.
    RetrievalDebugInspector,
    /// Docs browser / reader.
    DocsBrowser,
    /// Semantic-recall results panel.
    SemanticRecallPanel,
    /// AI context panel.
    AiContextPanel,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
}

impl RetrievalConsumerSurface {
    /// Stable token recorded in the projection.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetrievalDebugInspector => "retrieval_debug_inspector",
            Self::DocsBrowser => "docs_browser",
            Self::SemanticRecallPanel => "semantic_recall_panel",
            Self::AiContextPanel => "ai_context_panel",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// Class of a packet-level retrieval-debug degradation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalDegradationClass {
    /// The embedder was unavailable; recall fell back to lexical signals.
    EmbedderUnavailableLexicalFallback,
    /// A mirror is offline; entries served from the last verified snapshot.
    MirrorOfflineSnapshot,
    /// A retrieval index is stale relative to the working tree.
    IndexStale,
    /// An imported pack could not be verified.
    ImportedPackUnverified,
    /// Only part of the index was searched at materialization time.
    PartialIndex,
    /// The owning pack / source is quarantined.
    QuarantinedPack,
    /// A referenced anchor is broken.
    BrokenAnchor,
}

impl RetrievalDegradationClass {
    /// Stable token recorded in the degradation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmbedderUnavailableLexicalFallback => "embedder_unavailable_lexical_fallback",
            Self::MirrorOfflineSnapshot => "mirror_offline_snapshot",
            Self::IndexStale => "index_stale",
            Self::ImportedPackUnverified => "imported_pack_unverified",
            Self::PartialIndex => "partial_index",
            Self::QuarantinedPack => "quarantined_pack",
            Self::BrokenAnchor => "broken_anchor",
        }
    }
}

/// Scope a retrieval-debug export covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalExportScope {
    /// Every entry in the packet.
    AllEntries,
    /// Docs-search entries only.
    DocsSearchOnly,
    /// Semantic-recall entries only.
    SemanticRecallOnly,
    /// AI-context entries only.
    AiContextOnly,
}

impl RetrievalExportScope {
    /// Stable token recorded in the export.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllEntries => "all_entries",
            Self::DocsSearchOnly => "docs_search_only",
            Self::SemanticRecallOnly => "semantic_recall_only",
            Self::AiContextOnly => "ai_context_only",
        }
    }
}

/// Promotion state computed for the retrieval-debug packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalPromotionState {
    /// Set qualifies for the Stable claim.
    Stable,
    /// Set narrowed below Stable but stays valid and attributable.
    NarrowedBelowStable,
    /// Set has a blocking finding and must not present as Stable.
    BlocksStable,
}

impl RetrievalPromotionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Validation finding kind emitted by [`RetrievalDebugPacket::materialize`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalFindingKind {
    /// A required identity field is missing.
    MissingIdentity,
    /// The entry set is empty.
    EntriesEmpty,
    /// An entry id is duplicated.
    DuplicateEntryId,
    /// A required lane (docs / recall / ai-context) is missing.
    RequiredLaneMissing,
    /// An entry is missing its title or headline.
    EntryTitleOrHeadlineMissing,
    /// An entry is missing its explicit derivation reason.
    DerivationReasonMissing,
    /// An entry surfaces no ranking signals (no ranking reasons).
    RankingSignalsMissing,
    /// A ranking signal is missing its human-readable note.
    RankingSignalNoteMissing,
    /// An imported / heuristic entry is not cited.
    EntryNotCited,
    /// An entry is missing an open-raw / open-source escape ref.
    OpenRawOpenSourceEscapeMissing,
    /// A heuristic-labelled entry is presented as a high-confidence claim.
    HeuristicLabelLooksAuthoritative,
    /// A non-current version-match is presented as a confident live match.
    VersionTruthCollapsed,
    /// An export row references an entry id absent from the entries.
    ExportRowOrphan,
    /// An entry has no matching export row.
    ExportCoverageMissing,
    /// The export drops a required preservation flag.
    ExportDropsPreservation,
    /// An export row's lane disagrees with the entry.
    ExportLaneMismatch,
    /// An export row's derivation label disagrees with the entry.
    ExportDerivationLabelMismatch,
    /// An export row's source class disagrees with the entry's chip.
    ExportSourceClassMismatch,
    /// An export row's confidence disagrees with the entry's chip.
    ExportConfidenceMismatch,
    /// A degradation is incomplete (missing summary).
    DegradationIncomplete,
    /// A degradation references an entry id absent from the entries.
    DegradationOrphan,
    /// A consumer projection drops a required preservation flag.
    ConsumerProjectionDrift,
    /// A consumer projection references the wrong packet id.
    ConsumerProjectionPacketIdMismatch,
    /// A required consumer surface is missing from the projections.
    RequiredSurfaceCoverageMissing,
    /// Raw bodies, raw source, raw query text, or secrets crossed the boundary.
    RawBoundaryMaterialPresent,
}

impl RetrievalFindingKind {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingIdentity => "missing_identity",
            Self::EntriesEmpty => "entries_empty",
            Self::DuplicateEntryId => "duplicate_entry_id",
            Self::RequiredLaneMissing => "required_lane_missing",
            Self::EntryTitleOrHeadlineMissing => "entry_title_or_headline_missing",
            Self::DerivationReasonMissing => "derivation_reason_missing",
            Self::RankingSignalsMissing => "ranking_signals_missing",
            Self::RankingSignalNoteMissing => "ranking_signal_note_missing",
            Self::EntryNotCited => "entry_not_cited",
            Self::OpenRawOpenSourceEscapeMissing => "open_raw_open_source_escape_missing",
            Self::HeuristicLabelLooksAuthoritative => "heuristic_label_looks_authoritative",
            Self::VersionTruthCollapsed => "version_truth_collapsed",
            Self::ExportRowOrphan => "export_row_orphan",
            Self::ExportCoverageMissing => "export_coverage_missing",
            Self::ExportDropsPreservation => "export_drops_preservation",
            Self::ExportLaneMismatch => "export_lane_mismatch",
            Self::ExportDerivationLabelMismatch => "export_derivation_label_mismatch",
            Self::ExportSourceClassMismatch => "export_source_class_mismatch",
            Self::ExportConfidenceMismatch => "export_confidence_mismatch",
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
    /// severities so a degraded-but-honest set narrows rather than blocks.
    pub const fn default_severity(self) -> RetrievalFindingSeverity {
        RetrievalFindingSeverity::Blocking
    }
}

/// The chip set rendered for one retrieval-debug entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalChipSet {
    /// Source-class chip.
    pub source_class: RetrievalSourceClass,
    /// Version-match chip.
    pub version_match: RetrievalVersionMatch,
    /// Freshness chip.
    pub freshness: RetrievalFreshness,
    /// Locality chip.
    pub locality: RetrievalLocality,
    /// Confidence chip (the confidence label).
    pub confidence: RetrievalConfidence,
}

/// One ranking signal that explains why an entry was retrieved / scored.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankingSignal {
    /// Kind of ranking signal.
    pub signal_kind: RankingSignalKind,
    /// Whether the signal raised, lowered, or did not change the score.
    pub contribution: SignalContribution,
    /// Human-readable weight label (e.g. `+0.4`, `strong`) — no raw scores required.
    pub weight_label: String,
    /// Human-readable note (no raw bodies / no raw query text).
    pub note: String,
}

/// One retrieval-debug entry, one retrieved-and-explained result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalDebugEntry {
    /// Stable entry id within this packet.
    pub entry_id: String,
    /// Retrieval lane this entry belongs to.
    pub lane: RetrievalLane,
    /// Kind of subject this entry points at.
    pub subject_kind: RetrievalSubjectKind,
    /// Node / symbol / file / fragment ref (no raw body).
    pub subject_ref: String,
    /// Human-readable title.
    pub title: String,
    /// Human-readable headline / summary (no raw bodies).
    pub headline: String,
    /// Source/version/freshness/locality/confidence chips.
    pub chips: RetrievalChipSet,
    /// The exact / imported / heuristic derivation label.
    pub derivation_label: RetrievalDerivationLabel,
    /// Explicit, human-readable reason for the derivation label.
    pub derivation_reason: String,
    /// Ranking signals that explain the entry's retrieval (at least one).
    pub ranking_signals: Vec<RankingSignal>,
    /// Whether the entry is cited back to its source.
    pub cited: bool,
    /// Citation ref when cited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation_ref: Option<String>,
    /// Open-raw escape ref (open the underlying node/symbol/fragment).
    pub open_raw_escape_ref: String,
    /// Open-source escape ref (open the upstream/source).
    pub open_source_escape_ref: String,
}

/// One export row, mirroring an entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalDebugExportRow {
    /// The entry this export row mirrors.
    pub entry_id_ref: String,
    /// Lane (must match the entry).
    pub lane: RetrievalLane,
    /// Derivation label (must match the entry).
    pub derivation_label: RetrievalDerivationLabel,
    /// Source class (must match the entry's chip).
    pub source_class: RetrievalSourceClass,
    /// Confidence (must match the entry's chip).
    pub confidence: RetrievalConfidence,
    /// Whether the entry is cited.
    pub cited: bool,
    /// Number of ranking signals behind the entry.
    pub ranking_signal_count: u32,
    /// Open-raw escape ref.
    pub open_raw_escape_ref: String,
    /// Open-source escape ref.
    pub open_source_escape_ref: String,
}

/// The retrieval-debug export projection for the entry set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalDebugExport {
    /// Scope this export covers.
    pub scope: RetrievalExportScope,
    /// Whether the export preserves each entry's lane.
    pub preserves_lane: bool,
    /// Whether the export preserves each entry's derivation label.
    pub preserves_derivation_label: bool,
    /// Whether the export preserves each entry's source class.
    pub preserves_source_class: bool,
    /// Whether the export preserves each entry's confidence label.
    pub preserves_confidence: bool,
    /// Whether the export preserves the ranking reasons.
    pub preserves_ranking_reasons: bool,
    /// Whether the export preserves the open-raw / open-source escapes.
    pub preserves_open_raw_open_source_escape: bool,
    /// Per-entry export rows.
    pub rows: Vec<RetrievalDebugExportRow>,
}

impl RetrievalDebugExport {
    /// Whether the export preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_lane
            && self.preserves_derivation_label
            && self.preserves_source_class
            && self.preserves_confidence
            && self.preserves_ranking_reasons
            && self.preserves_open_raw_open_source_escape
    }
}

/// A packet-level retrieval-debug degradation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalDegradation {
    /// Degradation class.
    pub degradation_class: RetrievalDegradationClass,
    /// Severity.
    pub severity: RetrievalFindingSeverity,
    /// Human-readable summary (no raw bodies).
    pub summary: String,
    /// The entry this degradation annotates, if scoped to one entry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_id_ref: Option<String>,
    /// Optional supporting evidence ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
}

/// How a consumer surface projects the retrieval-debug set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalConsumerProjection {
    /// Surface that consumes the set.
    pub surface: RetrievalConsumerSurface,
    /// Packet id this projection mirrors.
    pub packet_id_ref: String,
    /// Whether the surface preserves the chip set verbatim.
    pub preserves_chips: bool,
    /// Whether the surface preserves all three lanes.
    pub preserves_lanes: bool,
    /// Whether the surface preserves the derivation labels.
    pub preserves_derivation_labels: bool,
    /// Whether the surface preserves the ranking reasons.
    pub preserves_ranking_reasons: bool,
    /// Whether the surface preserves the open-raw / open-source escapes.
    pub preserves_open_raw_open_source_escape: bool,
}

impl RetrievalConsumerProjection {
    /// Whether the projection preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_chips
            && self.preserves_lanes
            && self.preserves_derivation_labels
            && self.preserves_ranking_reasons
            && self.preserves_open_raw_open_source_escape
    }
}

/// A single validation finding on the retrieval-debug set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalValidationFinding {
    /// Finding kind.
    pub finding_kind: RetrievalFindingKind,
    /// Finding severity.
    pub severity: RetrievalFindingSeverity,
    /// Human-readable summary.
    pub summary: String,
}

/// Constructor input for [`RetrievalDebugPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalDebugPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable query label (no raw query text).
    pub query_label: String,
    /// Opaque digest/ref for the query.
    pub query_digest_ref: String,
    /// The retrieval-debug entries.
    pub entries: Vec<RetrievalDebugEntry>,
    /// The export projection.
    pub export: RetrievalDebugExport,
    /// Packet-level degradations.
    pub retrieval_degradations: Vec<RetrievalDegradation>,
    /// Consumer projections.
    pub consumer_projections: Vec<RetrievalConsumerProjection>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp (RFC 3339).
    pub minted_at: String,
}

/// Export-safe retrieval-debug packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalDebugPacket {
    /// Record kind; must equal [`RETRIEVAL_DEBUG_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`RETRIEVAL_DEBUG_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable query label.
    pub query_label: String,
    /// Opaque digest/ref for the query.
    pub query_digest_ref: String,
    /// The retrieval-debug entries.
    pub entries: Vec<RetrievalDebugEntry>,
    /// The export projection.
    pub export: RetrievalDebugExport,
    /// Packet-level degradations.
    pub retrieval_degradations: Vec<RetrievalDegradation>,
    /// Consumer projections.
    pub consumer_projections: Vec<RetrievalConsumerProjection>,
    /// Computed promotion state.
    pub promotion_state: RetrievalPromotionState,
    /// Computed validation findings.
    pub validation_findings: Vec<RetrievalValidationFinding>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Required consumer surfaces that every retrieval-debug packet must project.
const REQUIRED_SURFACES: [RetrievalConsumerSurface; 5] = [
    RetrievalConsumerSurface::RetrievalDebugInspector,
    RetrievalConsumerSurface::DocsBrowser,
    RetrievalConsumerSurface::SemanticRecallPanel,
    RetrievalConsumerSurface::AiContextPanel,
    RetrievalConsumerSurface::SupportExport,
];

impl RetrievalDebugPacket {
    /// Materializes a retrieval-debug packet, computing validation findings and
    /// the promotion state from the input.
    pub fn materialize(input: RetrievalDebugPacketInput) -> Self {
        let mut findings = Vec::new();

        check_identity(&input, &mut findings);
        check_entries(&input, &mut findings);
        check_export(&input, &mut findings);
        check_degradations(&input, &mut findings);
        check_consumer_projections(&input, &mut findings);
        check_boundary(&input, &mut findings);

        let promotion_state = promotion_state(&findings, &input.retrieval_degradations);

        Self {
            record_kind: RETRIEVAL_DEBUG_RECORD_KIND.to_owned(),
            schema_version: RETRIEVAL_DEBUG_SCHEMA_VERSION,
            packet_id: input.packet_id,
            query_label: input.query_label,
            query_digest_ref: input.query_digest_ref,
            entries: input.entries,
            export: input.export,
            retrieval_degradations: input.retrieval_degradations,
            consumer_projections: input.consumer_projections,
            promotion_state,
            validation_findings: findings,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Whether the set qualifies for the Stable claim with no findings.
    pub fn is_clean_stable(&self) -> bool {
        self.promotion_state == RetrievalPromotionState::Stable
            && self.validation_findings.is_empty()
    }

    /// Wraps the packet in a support-export envelope.
    pub fn support_export(
        &self,
        export_id: &str,
        exported_at: &str,
    ) -> RetrievalDebugSupportExport {
        RetrievalDebugSupportExport {
            record_kind: RETRIEVAL_DEBUG_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: RETRIEVAL_DEBUG_SCHEMA_VERSION,
            export_id: export_id.to_owned(),
            exported_at: exported_at.to_owned(),
            schema_ref: RETRIEVAL_DEBUG_SCHEMA_REF.to_owned(),
            doc_ref: RETRIEVAL_DEBUG_DOC_REF.to_owned(),
            packet: self.clone(),
        }
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("retrieval-debug packet serializes")
    }

    /// Deterministic Markdown summary for docs, support, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Retrieval-Debug Surfaces (docs, recall, AI context)\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Query: {}\n", self.query_label));
        out.push_str(&format!(
            "- Promotion: `{}` ({} findings)\n",
            self.promotion_state.as_str(),
            self.validation_findings.len()
        ));
        out.push_str(&format!(
            "- Entries: {} | Degradations: {}\n",
            self.entries.len(),
            self.retrieval_degradations.len()
        ));
        out.push_str("\n## Entries\n\n");
        for entry in &self.entries {
            out.push_str(&format!(
                "- [{}] `{}` ({}) — label `{}` — {} / {} / {} / {} / {}\n",
                entry.lane.as_str(),
                entry.entry_id,
                entry.title,
                entry.derivation_label.as_str(),
                entry.chips.source_class.as_str(),
                entry.chips.version_match.as_str(),
                entry.chips.freshness.as_str(),
                entry.chips.locality.as_str(),
                entry.chips.confidence.as_str(),
            ));
            out.push_str(&format!(
                "  - Derivation reason: {}\n",
                entry.derivation_reason
            ));
            out.push_str(&format!(
                "  - Cited: {} | Ranking signals: {}\n",
                entry.cited,
                entry.ranking_signals.len(),
            ));
        }
        if !self.retrieval_degradations.is_empty() {
            out.push_str("\n## Degradations\n\n");
            for degradation in &self.retrieval_degradations {
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

/// Support-export envelope for the retrieval-debug packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalDebugSupportExport {
    /// Record kind; must equal [`RETRIEVAL_DEBUG_SUPPORT_EXPORT_RECORD_KIND`].
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
    /// The wrapped retrieval-debug packet.
    pub packet: RetrievalDebugPacket,
}

/// Errors emitted when reading the checked-in retrieval-debug support export.
#[derive(Debug)]
pub enum RetrievalDebugArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Re-materialization disagreed with the checked-in promotion state.
    PromotionDrift {
        /// Promotion state recorded in the export.
        recorded: RetrievalPromotionState,
        /// Promotion state computed by re-materialization.
        computed: RetrievalPromotionState,
    },
    /// The checked-in packet should be clean Stable but is not.
    NotCleanStable(Vec<RetrievalValidationFinding>),
}

impl fmt::Display for RetrievalDebugArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "retrieval-debug export parse failed: {error}")
            }
            Self::PromotionDrift { recorded, computed } => write!(
                formatter,
                "retrieval-debug promotion drift: recorded {} but computed {}",
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
                    "retrieval-debug export is not clean stable: {tokens}"
                )
            }
        }
    }
}

impl Error for RetrievalDebugArtifactError {}

/// Reads and re-validates the checked-in stable retrieval-debug support export.
pub fn current_stable_retrieval_debug_export(
) -> Result<RetrievalDebugSupportExport, RetrievalDebugArtifactError> {
    let export: RetrievalDebugSupportExport = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/ship_retrieval_debug_surfaces_for_docs_recall_and_ai_context_with_exact_or_imported_or_heuristic_labeling/support_export.json"
    )))
    .map_err(RetrievalDebugArtifactError::SupportExport)?;

    let recomputed = RetrievalDebugPacket::materialize(packet_to_input(&export.packet));
    if recomputed.promotion_state != export.packet.promotion_state {
        return Err(RetrievalDebugArtifactError::PromotionDrift {
            recorded: export.packet.promotion_state,
            computed: recomputed.promotion_state,
        });
    }
    if !export.packet.is_clean_stable() {
        return Err(RetrievalDebugArtifactError::NotCleanStable(
            export.packet.validation_findings.clone(),
        ));
    }
    Ok(export)
}

/// Rebuilds the materialization input from a packet (used for re-validation).
pub fn packet_to_input(packet: &RetrievalDebugPacket) -> RetrievalDebugPacketInput {
    RetrievalDebugPacketInput {
        packet_id: packet.packet_id.clone(),
        query_label: packet.query_label.clone(),
        query_digest_ref: packet.query_digest_ref.clone(),
        entries: packet.entries.clone(),
        export: packet.export.clone(),
        retrieval_degradations: packet.retrieval_degradations.clone(),
        consumer_projections: packet.consumer_projections.clone(),
        redaction_class_token: packet.redaction_class_token.clone(),
        minted_at: packet.minted_at.clone(),
    }
}

fn push_finding(
    findings: &mut Vec<RetrievalValidationFinding>,
    kind: RetrievalFindingKind,
    summary: impl Into<String>,
) {
    findings.push(RetrievalValidationFinding {
        finding_kind: kind,
        severity: kind.default_severity(),
        summary: summary.into(),
    });
}

fn check_identity(
    input: &RetrievalDebugPacketInput,
    findings: &mut Vec<RetrievalValidationFinding>,
) {
    if input.packet_id.trim().is_empty()
        || input.query_label.trim().is_empty()
        || input.query_digest_ref.trim().is_empty()
        || input.redaction_class_token.trim().is_empty()
        || input.minted_at.trim().is_empty()
    {
        push_finding(
            findings,
            RetrievalFindingKind::MissingIdentity,
            "packet identity fields must all be present",
        );
    }
}

fn check_entries(
    input: &RetrievalDebugPacketInput,
    findings: &mut Vec<RetrievalValidationFinding>,
) {
    if input.entries.is_empty() {
        push_finding(
            findings,
            RetrievalFindingKind::EntriesEmpty,
            "the retrieval-debug set must carry at least one entry",
        );
        return;
    }

    let present_lanes: BTreeSet<RetrievalLane> =
        input.entries.iter().map(|entry| entry.lane).collect();
    for required in RetrievalLane::ALL {
        if !present_lanes.contains(&required) {
            push_finding(
                findings,
                RetrievalFindingKind::RequiredLaneMissing,
                format!("required lane `{}` is missing", required.as_str()),
            );
        }
    }

    let mut seen_entry_ids: BTreeSet<&str> = BTreeSet::new();
    for entry in &input.entries {
        if !seen_entry_ids.insert(entry.entry_id.as_str()) {
            push_finding(
                findings,
                RetrievalFindingKind::DuplicateEntryId,
                format!("duplicate entry id `{}`", entry.entry_id),
            );
        }
        check_one_entry(entry, findings);
    }
}

fn check_one_entry(entry: &RetrievalDebugEntry, findings: &mut Vec<RetrievalValidationFinding>) {
    if entry.title.trim().is_empty() || entry.headline.trim().is_empty() {
        push_finding(
            findings,
            RetrievalFindingKind::EntryTitleOrHeadlineMissing,
            format!("entry `{}` is missing a title or headline", entry.entry_id),
        );
    }
    if entry.derivation_reason.trim().is_empty() {
        push_finding(
            findings,
            RetrievalFindingKind::DerivationReasonMissing,
            format!("entry `{}` is missing a derivation reason", entry.entry_id),
        );
    }
    if entry.open_raw_escape_ref.trim().is_empty() || entry.open_source_escape_ref.trim().is_empty()
    {
        push_finding(
            findings,
            RetrievalFindingKind::OpenRawOpenSourceEscapeMissing,
            format!(
                "entry `{}` must keep open-raw and open-source escapes",
                entry.entry_id
            ),
        );
    }

    // An imported or heuristic entry must stay cited.
    if entry.derivation_label.needs_citation() && !entry.cited {
        push_finding(
            findings,
            RetrievalFindingKind::EntryNotCited,
            format!(
                "entry `{}` is `{}` but is not cited",
                entry.entry_id,
                entry.derivation_label.as_str()
            ),
        );
    }
    // A heuristic-labelled entry may never be presented at high confidence.
    if !entry.derivation_label.may_be_high_confidence()
        && entry.chips.confidence == RetrievalConfidence::High
    {
        push_finding(
            findings,
            RetrievalFindingKind::HeuristicLabelLooksAuthoritative,
            format!(
                "entry `{}` is a heuristic match presented as high confidence",
                entry.entry_id
            ),
        );
    }
    // A non-current version may not be presented as a confident live match.
    if !entry.chips.version_match.is_confident_current()
        && entry.chips.confidence == RetrievalConfidence::High
        && entry.chips.freshness.is_authoritative_live()
    {
        push_finding(
            findings,
            RetrievalFindingKind::VersionTruthCollapsed,
            format!(
                "entry `{}` presents version `{}` as a confident live match",
                entry.entry_id,
                entry.chips.version_match.as_str()
            ),
        );
    }

    check_entry_signals(entry, findings);
}

fn check_entry_signals(
    entry: &RetrievalDebugEntry,
    findings: &mut Vec<RetrievalValidationFinding>,
) {
    if entry.ranking_signals.is_empty() {
        push_finding(
            findings,
            RetrievalFindingKind::RankingSignalsMissing,
            format!("entry `{}` surfaces no ranking signals", entry.entry_id),
        );
        return;
    }
    for signal in &entry.ranking_signals {
        if signal.note.trim().is_empty() {
            push_finding(
                findings,
                RetrievalFindingKind::RankingSignalNoteMissing,
                format!(
                    "entry `{}` has a `{}` signal with no note",
                    entry.entry_id,
                    signal.signal_kind.as_str()
                ),
            );
        }
    }
}

fn check_export(input: &RetrievalDebugPacketInput, findings: &mut Vec<RetrievalValidationFinding>) {
    let export = &input.export;
    if !export.preserves_all() {
        push_finding(
            findings,
            RetrievalFindingKind::ExportDropsPreservation,
            "the export must preserve lane, derivation label, source class, confidence, ranking reasons, and escapes",
        );
    }

    let mut export_ids: BTreeSet<&str> = BTreeSet::new();
    for row in &export.rows {
        export_ids.insert(row.entry_id_ref.as_str());
        let entry = input
            .entries
            .iter()
            .find(|entry| entry.entry_id == row.entry_id_ref);
        match entry {
            None => push_finding(
                findings,
                RetrievalFindingKind::ExportRowOrphan,
                format!("export row references unknown entry `{}`", row.entry_id_ref),
            ),
            Some(entry) => {
                if entry.lane != row.lane {
                    push_finding(
                        findings,
                        RetrievalFindingKind::ExportLaneMismatch,
                        format!(
                            "export for `{}` records lane `{}` but the entry is `{}`",
                            row.entry_id_ref,
                            row.lane.as_str(),
                            entry.lane.as_str()
                        ),
                    );
                }
                if entry.derivation_label != row.derivation_label {
                    push_finding(
                        findings,
                        RetrievalFindingKind::ExportDerivationLabelMismatch,
                        format!(
                            "export for `{}` records label `{}` but the entry is `{}`",
                            row.entry_id_ref,
                            row.derivation_label.as_str(),
                            entry.derivation_label.as_str()
                        ),
                    );
                }
                if entry.chips.source_class != row.source_class {
                    push_finding(
                        findings,
                        RetrievalFindingKind::ExportSourceClassMismatch,
                        format!(
                            "export for `{}` records source `{}` but the entry chip is `{}`",
                            row.entry_id_ref,
                            row.source_class.as_str(),
                            entry.chips.source_class.as_str()
                        ),
                    );
                }
                if entry.chips.confidence != row.confidence {
                    push_finding(
                        findings,
                        RetrievalFindingKind::ExportConfidenceMismatch,
                        format!(
                            "export for `{}` records confidence `{}` but the entry chip is `{}`",
                            row.entry_id_ref,
                            row.confidence.as_str(),
                            entry.chips.confidence.as_str()
                        ),
                    );
                }
            }
        }
    }

    for entry in &input.entries {
        if !export_ids.contains(entry.entry_id.as_str()) {
            push_finding(
                findings,
                RetrievalFindingKind::ExportCoverageMissing,
                format!("entry `{}` has no export row", entry.entry_id),
            );
        }
    }
}

fn check_degradations(
    input: &RetrievalDebugPacketInput,
    findings: &mut Vec<RetrievalValidationFinding>,
) {
    let entry_ids: BTreeSet<&str> = input
        .entries
        .iter()
        .map(|entry| entry.entry_id.as_str())
        .collect();

    for degradation in &input.retrieval_degradations {
        if degradation.summary.trim().is_empty() {
            push_finding(
                findings,
                RetrievalFindingKind::DegradationIncomplete,
                format!(
                    "degradation `{}` is missing a summary",
                    degradation.degradation_class.as_str()
                ),
            );
        }
        if let Some(entry_id) = &degradation.entry_id_ref {
            if !entry_id.trim().is_empty() && !entry_ids.contains(entry_id.as_str()) {
                push_finding(
                    findings,
                    RetrievalFindingKind::DegradationOrphan,
                    format!("degradation references unknown entry `{}`", entry_id),
                );
            }
        }
    }
}

fn check_consumer_projections(
    input: &RetrievalDebugPacketInput,
    findings: &mut Vec<RetrievalValidationFinding>,
) {
    let present: BTreeSet<RetrievalConsumerSurface> = input
        .consumer_projections
        .iter()
        .map(|projection| projection.surface)
        .collect();
    for required in REQUIRED_SURFACES {
        if !present.contains(&required) {
            push_finding(
                findings,
                RetrievalFindingKind::RequiredSurfaceCoverageMissing,
                format!("required surface `{}` is missing", required.as_str()),
            );
        }
    }

    for projection in &input.consumer_projections {
        if projection.packet_id_ref != input.packet_id {
            push_finding(
                findings,
                RetrievalFindingKind::ConsumerProjectionPacketIdMismatch,
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
                RetrievalFindingKind::ConsumerProjectionDrift,
                format!(
                    "surface `{}` drops a required preservation flag",
                    projection.surface.as_str()
                ),
            );
        }
    }
}

fn check_boundary(
    input: &RetrievalDebugPacketInput,
    findings: &mut Vec<RetrievalValidationFinding>,
) {
    let value = serde_json::to_value(input).expect("retrieval-debug input serializes");
    if json_contains_forbidden_boundary_material(&value) {
        push_finding(
            findings,
            RetrievalFindingKind::RawBoundaryMaterialPresent,
            "export must not carry raw bodies, raw source, raw query text, or secrets",
        );
    }
}

/// Computes the promotion state from the worst severity across both the
/// validation findings and the attached degradations.
///
/// A blocking finding (integrity, trust, citation, or boundary violation) blocks
/// the Stable claim; an otherwise-clean set that carries a narrowing degradation
/// narrows below Stable rather than hiding the entries.
fn promotion_state(
    findings: &[RetrievalValidationFinding],
    degradations: &[RetrievalDegradation],
) -> RetrievalPromotionState {
    let any_blocking = findings
        .iter()
        .any(|finding| finding.severity == RetrievalFindingSeverity::Blocking)
        || degradations
            .iter()
            .any(|degradation| degradation.severity == RetrievalFindingSeverity::Blocking);
    if any_blocking {
        return RetrievalPromotionState::BlocksStable;
    }

    let any_narrowing = findings
        .iter()
        .any(|finding| finding.severity == RetrievalFindingSeverity::Narrowing)
        || degradations
            .iter()
            .any(|degradation| degradation.severity == RetrievalFindingSeverity::Narrowing);
    if any_narrowing {
        RetrievalPromotionState::NarrowedBelowStable
    } else {
        RetrievalPromotionState::Stable
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
                || lower.contains("raw_query:")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Seeded stable retrieval-debug input used by the producer, tests, and fixtures.
pub fn seeded_stable_retrieval_debug_input() -> RetrievalDebugPacketInput {
    let packet_id = "packet:m5:retrieval_debug:net_retry_query".to_owned();
    RetrievalDebugPacketInput {
        packet_id: packet_id.clone(),
        query_label: "retrieval debug: how does the networking retry backoff work".to_owned(),
        query_digest_ref: "querydigest:sha256:net-retry-backoff".to_owned(),
        entries: vec![docs_search_entry(), semantic_recall_entry(), ai_context_entry()],
        export: seeded_export(),
        retrieval_degradations: vec![RetrievalDegradation {
            degradation_class: RetrievalDegradationClass::IndexStale,
            severity: RetrievalFindingSeverity::Advisory,
            summary: "the recall index was built before the last two commits; recall entries may lag the working tree".to_owned(),
            entry_id_ref: None,
            evidence_ref: Some("evidence:retrieval-debug:index-freshness".to_owned()),
        }],
        consumer_projections: required_projections(&packet_id),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-09T00:00:00Z".to_owned(),
    }
}

fn docs_search_entry() -> RetrievalDebugEntry {
    RetrievalDebugEntry {
        entry_id: "entry:docs:retry_with_backoff_symbol".to_owned(),
        lane: RetrievalLane::DocsSearch,
        subject_kind: RetrievalSubjectKind::CodeSymbol,
        subject_ref: "symbol:aureline-net::retry::retry_with_backoff".to_owned(),
        title: "retry_with_backoff (symbol reference)".to_owned(),
        headline: "exact symbol hit for retry_with_backoff in the workspace docs index".to_owned(),
        chips: RetrievalChipSet {
            source_class: RetrievalSourceClass::WorkspaceCode,
            version_match: RetrievalVersionMatch::ExactBuildMatch,
            freshness: RetrievalFreshness::AuthoritativeLive,
            locality: RetrievalLocality::Local,
            confidence: RetrievalConfidence::High,
        },
        derivation_label: RetrievalDerivationLabel::Exact,
        derivation_reason: "exact symbol-name match against the local workspace index at the active build revision; labelled exact and high".to_owned(),
        ranking_signals: vec![
            RankingSignal {
                signal_kind: RankingSignalKind::SymbolExactMatch,
                contribution: SignalContribution::Boost,
                weight_label: "strong".to_owned(),
                note: "query token matched the symbol name verbatim".to_owned(),
            },
            RankingSignal {
                signal_kind: RankingSignalKind::PathProximity,
                contribution: SignalContribution::Boost,
                weight_label: "+0.2".to_owned(),
                note: "symbol lives in the networking module the query scopes to".to_owned(),
            },
        ],
        cited: true,
        citation_ref: Some("cite:symbol:aureline-net::retry::retry_with_backoff".to_owned()),
        open_raw_escape_ref: "open-raw:symbol:aureline-net::retry::retry_with_backoff".to_owned(),
        open_source_escape_ref: "open-source:repo:crates/aureline-net/src/retry.rs".to_owned(),
    }
}

fn semantic_recall_entry() -> RetrievalDebugEntry {
    RetrievalDebugEntry {
        entry_id: "entry:recall:backoff_policy_guide".to_owned(),
        lane: RetrievalLane::SemanticRecall,
        subject_kind: RetrievalSubjectKind::PackNode,
        subject_ref: "packnode:imported-pack:tokio/retry-guide".to_owned(),
        title: "Exponential backoff guidance (imported pack)".to_owned(),
        headline: "semantically similar guidance pulled from an imported retry-pattern pack".to_owned(),
        chips: RetrievalChipSet {
            source_class: RetrievalSourceClass::ImportedPack,
            version_match: RetrievalVersionMatch::CompatibleMinorDrift,
            freshness: RetrievalFreshness::WarmCached,
            locality: RetrievalLocality::MirroredPack,
            confidence: RetrievalConfidence::Medium,
        },
        derivation_label: RetrievalDerivationLabel::Imported,
        derivation_reason: "came in through a pinned imported pack rather than the workspace; labelled imported and held to medium because it is not workspace-verified".to_owned(),
        ranking_signals: vec![
            RankingSignal {
                signal_kind: RankingSignalKind::SemanticSimilarity,
                contribution: SignalContribution::Boost,
                weight_label: "+0.6".to_owned(),
                note: "embedding similarity to the backoff query was high".to_owned(),
            },
            RankingSignal {
                signal_kind: RankingSignalKind::ImportedSourcePenalty,
                contribution: SignalContribution::Penalty,
                weight_label: "-0.15".to_owned(),
                note: "imported, not workspace-verified; ranked below local hits".to_owned(),
            },
        ],
        cited: true,
        citation_ref: Some("cite:packnode:imported-pack:tokio/retry-guide".to_owned()),
        open_raw_escape_ref: "open-raw:packnode:imported-pack:tokio/retry-guide".to_owned(),
        open_source_escape_ref: "open-source:pack:tokio/retry-guide".to_owned(),
    }
}

fn ai_context_entry() -> RetrievalDebugEntry {
    RetrievalDebugEntry {
        entry_id: "entry:ai_context:retry_explanation_fragment".to_owned(),
        lane: RetrievalLane::AiContext,
        subject_kind: RetrievalSubjectKind::ContextFragment,
        subject_ref: "fragment:ai-context:retry-backoff-explanation".to_owned(),
        title: "Retry/backoff context fragment".to_owned(),
        headline: "a heuristically assembled context fragment summarising the retry path for the prompt".to_owned(),
        chips: RetrievalChipSet {
            source_class: RetrievalSourceClass::AiAssembledContext,
            version_match: RetrievalVersionMatch::ExactBuildMatch,
            freshness: RetrievalFreshness::WarmCached,
            locality: RetrievalLocality::Local,
            confidence: RetrievalConfidence::Low,
        },
        derivation_label: RetrievalDerivationLabel::Heuristic,
        derivation_reason: "assembled by a heuristic chunk-selection pass over the cited symbol and guide; labelled heuristic and held to low confidence".to_owned(),
        ranking_signals: vec![
            RankingSignal {
                signal_kind: RankingSignalKind::SemanticSimilarity,
                contribution: SignalContribution::Boost,
                weight_label: "+0.5".to_owned(),
                note: "fragment chosen for semantic proximity to the query".to_owned(),
            },
            RankingSignal {
                signal_kind: RankingSignalKind::HeuristicPenalty,
                contribution: SignalContribution::Penalty,
                weight_label: "-0.3".to_owned(),
                note: "fuzzy chunk selection; flagged heuristic so it never reads as verified".to_owned(),
            },
        ],
        cited: true,
        citation_ref: Some("cite:fragment:ai-context:retry-backoff-explanation".to_owned()),
        open_raw_escape_ref: "open-raw:fragment:ai-context:retry-backoff-explanation".to_owned(),
        open_source_escape_ref: "open-source:repo:crates/aureline-net/src/retry.rs".to_owned(),
    }
}

fn seeded_export() -> RetrievalDebugExport {
    RetrievalDebugExport {
        scope: RetrievalExportScope::AllEntries,
        preserves_lane: true,
        preserves_derivation_label: true,
        preserves_source_class: true,
        preserves_confidence: true,
        preserves_ranking_reasons: true,
        preserves_open_raw_open_source_escape: true,
        rows: vec![
            RetrievalDebugExportRow {
                entry_id_ref: "entry:docs:retry_with_backoff_symbol".to_owned(),
                lane: RetrievalLane::DocsSearch,
                derivation_label: RetrievalDerivationLabel::Exact,
                source_class: RetrievalSourceClass::WorkspaceCode,
                confidence: RetrievalConfidence::High,
                cited: true,
                ranking_signal_count: 2,
                open_raw_escape_ref: "open-raw:symbol:aureline-net::retry::retry_with_backoff"
                    .to_owned(),
                open_source_escape_ref: "open-source:repo:crates/aureline-net/src/retry.rs"
                    .to_owned(),
            },
            RetrievalDebugExportRow {
                entry_id_ref: "entry:recall:backoff_policy_guide".to_owned(),
                lane: RetrievalLane::SemanticRecall,
                derivation_label: RetrievalDerivationLabel::Imported,
                source_class: RetrievalSourceClass::ImportedPack,
                confidence: RetrievalConfidence::Medium,
                cited: true,
                ranking_signal_count: 2,
                open_raw_escape_ref: "open-raw:packnode:imported-pack:tokio/retry-guide".to_owned(),
                open_source_escape_ref: "open-source:pack:tokio/retry-guide".to_owned(),
            },
            RetrievalDebugExportRow {
                entry_id_ref: "entry:ai_context:retry_explanation_fragment".to_owned(),
                lane: RetrievalLane::AiContext,
                derivation_label: RetrievalDerivationLabel::Heuristic,
                source_class: RetrievalSourceClass::AiAssembledContext,
                confidence: RetrievalConfidence::Low,
                cited: true,
                ranking_signal_count: 2,
                open_raw_escape_ref: "open-raw:fragment:ai-context:retry-backoff-explanation"
                    .to_owned(),
                open_source_escape_ref: "open-source:repo:crates/aureline-net/src/retry.rs"
                    .to_owned(),
            },
        ],
    }
}

fn required_projections(packet_id: &str) -> Vec<RetrievalConsumerProjection> {
    [
        RetrievalConsumerSurface::RetrievalDebugInspector,
        RetrievalConsumerSurface::DocsBrowser,
        RetrievalConsumerSurface::SemanticRecallPanel,
        RetrievalConsumerSurface::AiContextPanel,
        RetrievalConsumerSurface::CliHeadless,
        RetrievalConsumerSurface::SupportExport,
        RetrievalConsumerSurface::Diagnostics,
        RetrievalConsumerSurface::HelpAbout,
    ]
    .into_iter()
    .map(|surface| RetrievalConsumerProjection {
        surface,
        packet_id_ref: packet_id.to_owned(),
        preserves_chips: true,
        preserves_lanes: true,
        preserves_derivation_labels: true,
        preserves_ranking_reasons: true,
        preserves_open_raw_open_source_escape: true,
    })
    .collect()
}
