//! Docs and code semantic recall with a query-session ledger, explicit ranking
//! reasons, and a provenance export.
//!
//! This module implements the M5 unified semantic-recall feature that spans both
//! docs nodes and workspace/dependency code. A recall belongs to a
//! [`SemanticRecallQuerySessionLedger`]: an ordered list of the queries a reader
//! issued in one session, each carrying an opaque query digest, a refinement
//! relation to the prior query (`initial`, `narrowed`, `broadened`,
//! `reformulated`, `pivoted_subject`), the subject scope it searched, and the
//! result ids it surfaced. The session's active ranking is one
//! [`SemanticRecallResultRow`] per hit, and every row carries the
//! source/version/freshness/locality/confidence chip set, an explicit
//! [`SemanticRecallResultRow::ranking_reason`] backed by a
//! [`RankingSignal`] breakdown, its [`ResultProvenance`], and the open-raw /
//! open-source escapes that keep derived and inferred results honest.
//!
//! The [`SemanticRecallProvenanceExport`] is the cited provenance projection that
//! support, AI evidence, and review surfaces ingest: one
//! [`SemanticRecallProvenanceRow`] per result preserving source class,
//! confidence, derivation, citation state, and the escapes. Codebase-explainer
//! rows that are derived or inferred must stay cited and may not be presented as
//! high-confidence live truth.
//!
//! [`SemanticRecallLedgerPacket::materialize`] computes the validation findings
//! and the promotion state (`stable`, `narrowed_below_stable`, or
//! `blocks_stable`) from the input, so a stale, uncited, over-authoritative, or
//! under-attributed recall automatically narrows or blocks before it reaches a
//! consumer surface. The packet is an inspectable, serde-serializable truth
//! packet: it carries no raw query text, no raw document bodies, no raw source
//! files, no raw provider payloads, and no credentials — only metadata, chip
//! truth, ranking reasons, provenance, finding summaries, and contract refs.
//!
//! The boundary schema is
//! [`schemas/docs/implement-docs-and-code-semantic-recall-with-query-session-ledger-ranking-reasons-and-provenance-export.schema.json`](../../../../schemas/docs/implement-docs-and-code-semantic-recall-with-query-session-ledger-ranking-reasons-and-provenance-export.schema.json).
//! The contract doc is
//! [`docs/docs/m5/implement_docs_and_code_semantic_recall_with_query_session_ledger_ranking_reasons_and_provenance_export.md`](../../../../docs/docs/m5/implement_docs_and_code_semantic_recall_with_query_session_ledger_ranking_reasons_and_provenance_export.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/implement_docs_and_code_semantic_recall_with_query_session_ledger_ranking_reasons_and_provenance_export/`](../../../../fixtures/docs/m5/implement_docs_and_code_semantic_recall_with_query_session_ledger_ranking_reasons_and_provenance_export/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`SemanticRecallLedgerPacket`].
pub const SEMANTIC_RECALL_LEDGER_RECORD_KIND: &str =
    "docs_and_code_semantic_recall_query_session_ledger";

/// Record-kind tag carried by the support-export wrapper.
pub const SEMANTIC_RECALL_LEDGER_SUPPORT_EXPORT_RECORD_KIND: &str =
    "docs_and_code_semantic_recall_query_session_ledger_support_export";

/// Schema version for semantic-recall ledger records.
pub const SEMANTIC_RECALL_LEDGER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const SEMANTIC_RECALL_LEDGER_SCHEMA_REF: &str =
    "schemas/docs/implement-docs-and-code-semantic-recall-with-query-session-ledger-ranking-reasons-and-provenance-export.schema.json";

/// Repo-relative path of the recall contract doc.
pub const SEMANTIC_RECALL_LEDGER_DOC_REF: &str =
    "docs/docs/m5/implement_docs_and_code_semantic_recall_with_query_session_ledger_ranking_reasons_and_provenance_export.md";

/// Repo-relative path of the protected fixture directory.
pub const SEMANTIC_RECALL_LEDGER_FIXTURE_DIR: &str =
    "fixtures/docs/m5/implement_docs_and_code_semantic_recall_with_query_session_ledger_ranking_reasons_and_provenance_export";

/// Repo-relative path of the checked support-export artifact.
pub const SEMANTIC_RECALL_LEDGER_ARTIFACT_REF: &str =
    "artifacts/docs/m5/implement_docs_and_code_semantic_recall_with_query_session_ledger_ranking_reasons_and_provenance_export/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const SEMANTIC_RECALL_LEDGER_SUMMARY_REF: &str =
    "artifacts/docs/m5/implement_docs_and_code_semantic_recall_with_query_session_ledger_ranking_reasons_and_provenance_export.md";

/// Source class for a recalled subject, projected as the source chip.
///
/// Tokens extend the canonical docs-pack source vocabulary with the two code
/// classes this lane adds so downstream consumers keep one chip-label set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticRecallSourceClass {
    /// Workspace-local project docs.
    ProjectDocs,
    /// Generated API/reference docs.
    GeneratedReference,
    /// Pinned, signed mirror of official upstream docs.
    MirroredOfficialDocs,
    /// Curated knowledge pack.
    CuratedKnowledgePack,
    /// Support runbook content.
    SupportRunbook,
    /// Third-party extension docs pack.
    ExtensionDocsPack,
    /// Symbol or file in the active workspace code.
    WorkspaceCode,
    /// Symbol or file in a resolved dependency / vendored source.
    DependencySource,
}

impl SemanticRecallSourceClass {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectDocs => "project_docs",
            Self::GeneratedReference => "generated_reference",
            Self::MirroredOfficialDocs => "mirrored_official_docs",
            Self::CuratedKnowledgePack => "curated_knowledge_pack",
            Self::SupportRunbook => "support_runbook",
            Self::ExtensionDocsPack => "extension_docs_pack",
            Self::WorkspaceCode => "workspace_code",
            Self::DependencySource => "dependency_source",
        }
    }
}

/// Version-match state for a recalled subject, projected as the version chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticRecallVersionMatch {
    /// Subject exactly matches the active build/workspace revision.
    ExactBuildMatch,
    /// Subject is within an accepted compatible drift window.
    CompatibleMinorDrift,
    /// Subject drifted incompatibly from the active target.
    IncompatibleDriftDetected,
    /// Pre-release subject has not completed verification.
    PreReleaseUnverified,
    /// The target build/workspace revision could not be verified.
    UnknownTargetBuild,
}

impl SemanticRecallVersionMatch {
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

/// Freshness state for a recalled subject, projected as the freshness chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticRecallFreshness {
    /// Subject was live and authoritative at recall time.
    AuthoritativeLive,
    /// Cached subject within its freshness window.
    WarmCached,
    /// Cached subject usable only with degraded disclosure.
    DegradedCached,
    /// Subject is stale and must not claim current authority.
    Stale,
    /// Freshness could not be verified.
    Unverified,
    /// A refresh is pending; the source has not yet re-synced.
    RefreshPending,
}

impl SemanticRecallFreshness {
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

/// Locality / install posture for a recalled subject, projected as the locality chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticRecallLocality {
    /// Resolved from local content or the in-repo index.
    Local,
    /// Resolved through a pinned mirror pack.
    MirroredPack,
    /// Resolved through a remote helper.
    RemoteHelper,
    /// Resolved through a managed (org-hosted) service.
    Managed,
}

impl SemanticRecallLocality {
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

/// Confidence class for a recall row, projected as the confidence chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticRecallConfidence {
    /// High confidence.
    High,
    /// Medium confidence.
    Medium,
    /// Low confidence.
    Low,
    /// Heuristic only; not a verified match.
    Heuristic,
}

impl SemanticRecallConfidence {
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

/// Kind of subject a recall row points at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticRecallSubjectKind {
    /// A docs node (guide, reference, pack node).
    DocsNode,
    /// A code symbol (function, type, trait, etc.).
    CodeSymbol,
    /// A code file.
    CodeFile,
    /// An extracted code snippet.
    CodeSnippet,
}

impl SemanticRecallSubjectKind {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsNode => "docs_node",
            Self::CodeSymbol => "code_symbol",
            Self::CodeFile => "code_file",
            Self::CodeSnippet => "code_snippet",
        }
    }

    /// Whether the subject is code (rather than a docs node).
    pub const fn is_code(self) -> bool {
        matches!(self, Self::CodeSymbol | Self::CodeFile | Self::CodeSnippet)
    }
}

/// How a recalled result was derived from its underlying source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DerivationClass {
    /// The result is the verbatim node/symbol, not a derivation.
    VerbatimNode,
    /// An extracted snippet of the underlying source.
    ExtractedSnippet,
    /// A summary derived over the underlying source.
    DerivedSummary,
    /// An inferred explanation generated over the underlying source.
    InferredExplanation,
}

impl DerivationClass {
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

/// Refinement relation of a ledger entry to the prior entry in the session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryRefinementRelation {
    /// The first query in the session.
    Initial,
    /// Narrowed from the prior query (more specific).
    Narrowed,
    /// Broadened from the prior query (less specific).
    Broadened,
    /// Reformulated wording of the same intent.
    Reformulated,
    /// Pivoted to a different subject than the prior query.
    PivotedSubject,
}

impl QueryRefinementRelation {
    /// Stable token recorded in the ledger entry.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initial => "initial",
            Self::Narrowed => "narrowed",
            Self::Broadened => "broadened",
            Self::Reformulated => "reformulated",
            Self::PivotedSubject => "pivoted_subject",
        }
    }

    /// Whether this relation marks the first query in a session.
    pub const fn is_initial(self) -> bool {
        matches!(self, Self::Initial)
    }
}

/// Subject scope a ledger entry searched.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticRecallSubjectScope {
    /// Docs nodes only.
    DocsOnly,
    /// Code only.
    CodeOnly,
    /// Both docs and code.
    DocsAndCode,
}

impl SemanticRecallSubjectScope {
    /// Stable token recorded in the ledger entry.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsOnly => "docs_only",
            Self::CodeOnly => "code_only",
            Self::DocsAndCode => "docs_and_code",
        }
    }
}

/// Kind of ranking signal contributing to a row's ranking reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RankingSignalKind {
    /// Lexical / keyword overlap with the query.
    LexicalOverlap,
    /// Embedding / semantic similarity.
    SemanticSimilarity,
    /// Exact symbol-name match.
    SymbolExactMatch,
    /// Proximity in the code/docs graph to a strong hit.
    GraphProximity,
    /// Recency of the subject.
    Recency,
    /// Boost from a pinned pack.
    PinBoost,
    /// Boost from an authoritative source.
    AuthorityBoost,
    /// Path / module affinity to the active context.
    PathAffinity,
}

impl RankingSignalKind {
    /// Stable token recorded in the signal.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LexicalOverlap => "lexical_overlap",
            Self::SemanticSimilarity => "semantic_similarity",
            Self::SymbolExactMatch => "symbol_exact_match",
            Self::GraphProximity => "graph_proximity",
            Self::Recency => "recency",
            Self::PinBoost => "pin_boost",
            Self::AuthorityBoost => "authority_boost",
            Self::PathAffinity => "path_affinity",
        }
    }
}

/// How strongly a ranking signal contributed to a row's placement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalContributionClass {
    /// The dominant signal for this row.
    Primary,
    /// A supporting signal.
    Supporting,
    /// A minor signal.
    Minor,
    /// A penalty that lowered the row.
    Penalty,
}

impl SignalContributionClass {
    /// Stable token recorded in the signal.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Supporting => "supporting",
            Self::Minor => "minor",
            Self::Penalty => "penalty",
        }
    }
}

/// Class of a session-level recall degradation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecallDegradationClass {
    /// The embedder was unavailable; the recall fell back to lexical ranking.
    EmbedderUnavailableLexicalFallback,
    /// The code graph index is stale relative to the working tree.
    CodeGraphStale,
    /// A mirror is offline; results served from the last verified snapshot.
    MirrorOfflineSnapshot,
    /// Only part of the corpus was indexed at recall time.
    PartialIndex,
    /// The session ledger was truncated.
    SessionTruncated,
    /// The owning pack is quarantined.
    QuarantinedPack,
    /// A referenced anchor is broken.
    BrokenAnchor,
}

impl RecallDegradationClass {
    /// Stable token recorded in the degradation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmbedderUnavailableLexicalFallback => "embedder_unavailable_lexical_fallback",
            Self::CodeGraphStale => "code_graph_stale",
            Self::MirrorOfflineSnapshot => "mirror_offline_snapshot",
            Self::PartialIndex => "partial_index",
            Self::SessionTruncated => "session_truncated",
            Self::QuarantinedPack => "quarantined_pack",
            Self::BrokenAnchor => "broken_anchor",
        }
    }
}

/// Severity of a degradation or validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticRecallFindingSeverity {
    /// Blocks a Stable claim; the recall must block.
    Blocking,
    /// Narrows below Stable but the recall stays valid and attributable.
    Narrowing,
    /// Advisory only.
    Advisory,
}

impl SemanticRecallFindingSeverity {
    /// Stable token recorded in the finding.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocking => "blocking",
            Self::Narrowing => "narrowing",
            Self::Advisory => "advisory",
        }
    }
}

/// Consumer surface that must project the recall packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticRecallConsumerSurface {
    /// Docs browser / reader.
    DocsBrowser,
    /// Search shell results.
    SearchShell,
    /// Codebase-explainer panel.
    CodeExplainer,
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

impl SemanticRecallConsumerSurface {
    /// Stable token recorded in the projection.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowser => "docs_browser",
            Self::SearchShell => "search_shell",
            Self::CodeExplainer => "code_explainer",
            Self::AiContext => "ai_context",
            Self::RetrievalInspector => "retrieval_inspector",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::HelpAbout => "help_about",
        }
    }
}

/// Scope a provenance export covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceExportScope {
    /// Every result surfaced across the session.
    FullSession,
    /// Only the active query's ranking.
    ActiveQueryOnly,
    /// Docs results only.
    DocsOnly,
    /// Code results only.
    CodeOnly,
}

impl ProvenanceExportScope {
    /// Stable token recorded in the export.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullSession => "full_session",
            Self::ActiveQueryOnly => "active_query_only",
            Self::DocsOnly => "docs_only",
            Self::CodeOnly => "code_only",
        }
    }
}

/// Promotion state computed for the recall packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticRecallPromotionState {
    /// Recall qualifies for the Stable claim.
    Stable,
    /// Recall narrowed below Stable but stays valid and attributable.
    NarrowedBelowStable,
    /// Recall has a blocking finding and must not present as Stable.
    BlocksStable,
}

impl SemanticRecallPromotionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Validation finding kind emitted by [`SemanticRecallLedgerPacket::materialize`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticRecallFindingKind {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is missing.
    MissingIdentity,
    /// The query-session ledger is empty.
    LedgerEmpty,
    /// Ledger entry sequences are not strictly increasing from 1.
    LedgerSequenceNotMonotonic,
    /// A ledger entry is missing its query digest or label.
    LedgerEntryQueryDigestMissing,
    /// The first entry is not `initial`, or a later entry is marked `initial`.
    LedgerRefinementInconsistent,
    /// A ledger entry surfaces a result id absent from the rows.
    LedgerSurfacedResultOrphan,
    /// The recall returned no rows.
    ResultRowsEmpty,
    /// Result ranks are not strictly increasing from 1.
    ResultRankNotMonotonic,
    /// A result id is duplicated.
    DuplicateResultId,
    /// A row's `origin_query_sequence` is not present in the ledger.
    ResultOriginQueryOrphan,
    /// A row is missing its explicit ranking reason.
    RankingReasonMissing,
    /// A row carries no ranking signals.
    RankingSignalsMissing,
    /// A row is missing an open-raw / open-source escape ref.
    OpenRawOpenSourceEscapeMissing,
    /// A derived / inferred result is not cited.
    CodeResultNotCited,
    /// An inferred result is presented as a high-confidence match.
    InferredResultLooksAuthoritative,
    /// A non-current version-match is presented as a confident live match.
    VersionTruthCollapsed,
    /// A provenance row references a result id absent from the rows.
    ProvenanceExportRowOrphan,
    /// A result row has no matching provenance export row.
    ProvenanceExportCoverageMissing,
    /// The provenance export drops a required preservation flag.
    ProvenanceExportDropsPreservation,
    /// A provenance row's source class disagrees with the result row's chip.
    ProvenanceSourceClassMismatch,
    /// A provenance row's confidence disagrees with the result row's chip.
    ProvenanceConfidenceMismatch,
    /// A degradation is incomplete (missing summary).
    DegradationIncomplete,
    /// A degradation references a result id absent from the rows.
    DegradationOrphan,
    /// A consumer projection drops a required preservation flag.
    ConsumerProjectionDrift,
    /// A consumer projection references the wrong packet id.
    ConsumerProjectionPacketIdMismatch,
    /// A required consumer surface is missing from the projections.
    RequiredSurfaceCoverageMissing,
    /// Raw query text, raw bodies, or secrets crossed the export boundary.
    RawBoundaryMaterialPresent,
}

impl SemanticRecallFindingKind {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::LedgerEmpty => "ledger_empty",
            Self::LedgerSequenceNotMonotonic => "ledger_sequence_not_monotonic",
            Self::LedgerEntryQueryDigestMissing => "ledger_entry_query_digest_missing",
            Self::LedgerRefinementInconsistent => "ledger_refinement_inconsistent",
            Self::LedgerSurfacedResultOrphan => "ledger_surfaced_result_orphan",
            Self::ResultRowsEmpty => "result_rows_empty",
            Self::ResultRankNotMonotonic => "result_rank_not_monotonic",
            Self::DuplicateResultId => "duplicate_result_id",
            Self::ResultOriginQueryOrphan => "result_origin_query_orphan",
            Self::RankingReasonMissing => "ranking_reason_missing",
            Self::RankingSignalsMissing => "ranking_signals_missing",
            Self::OpenRawOpenSourceEscapeMissing => "open_raw_open_source_escape_missing",
            Self::CodeResultNotCited => "code_result_not_cited",
            Self::InferredResultLooksAuthoritative => "inferred_result_looks_authoritative",
            Self::VersionTruthCollapsed => "version_truth_collapsed",
            Self::ProvenanceExportRowOrphan => "provenance_export_row_orphan",
            Self::ProvenanceExportCoverageMissing => "provenance_export_coverage_missing",
            Self::ProvenanceExportDropsPreservation => "provenance_export_drops_preservation",
            Self::ProvenanceSourceClassMismatch => "provenance_source_class_mismatch",
            Self::ProvenanceConfidenceMismatch => "provenance_confidence_mismatch",
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
    /// severities so a degraded-but-honest recall narrows rather than blocks.
    pub const fn default_severity(self) -> SemanticRecallFindingSeverity {
        SemanticRecallFindingSeverity::Blocking
    }
}

/// One ranking signal contributing to a row's ranking reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankingSignal {
    /// Signal kind.
    pub signal: RankingSignalKind,
    /// How strongly the signal contributed.
    pub contribution: SignalContributionClass,
    /// Human-readable note (no raw bodies).
    pub note: String,
}

/// The chip set rendered for one recall result row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecallChipSet {
    /// Source-class chip.
    pub source_class: SemanticRecallSourceClass,
    /// Version-match chip.
    pub version_match: SemanticRecallVersionMatch,
    /// Freshness chip.
    pub freshness: SemanticRecallFreshness,
    /// Locality chip.
    pub locality: SemanticRecallLocality,
    /// Confidence chip.
    pub confidence: SemanticRecallConfidence,
}

/// Provenance carried inline by a recall result row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultProvenance {
    /// Owning pack / crate / repo id.
    pub pack_id_ref: String,
    /// Whether the owning pack/source is pinned.
    pub pack_pinned: bool,
    /// Whether the owning pack's signature is verified.
    pub pack_signed_and_verified: bool,
    /// How the result was derived from its source.
    pub derivation: DerivationClass,
    /// Whether the result is cited back to its source.
    pub cited: bool,
    /// Citation ref when cited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation_ref: Option<String>,
}

/// One ledger entry: a single query within the session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecallLedgerEntry {
    /// 1-based sequence within the session.
    pub sequence: u32,
    /// Opaque digest/ref for the query (never raw query text).
    pub query_digest_ref: String,
    /// Human-readable query label (never raw query text).
    pub query_label: String,
    /// Refinement relation to the prior entry.
    pub refinement: QueryRefinementRelation,
    /// Subject scope this entry searched.
    pub subject_scope: SemanticRecallSubjectScope,
    /// Result ids this entry surfaced.
    pub surfaced_result_ids: Vec<String>,
}

/// The ordered query-session ledger.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecallQuerySessionLedger {
    /// Stable session id.
    pub session_id: String,
    /// Ordered entries.
    pub entries: Vec<SemanticRecallLedgerEntry>,
}

impl SemanticRecallQuerySessionLedger {
    /// Whether the ledger has any entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// One ranked recall result row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecallResultRow {
    /// 1-based rank in the active ranking.
    pub rank: u32,
    /// Stable result id within this recall.
    pub result_id: String,
    /// Kind of subject this row points at.
    pub subject_kind: SemanticRecallSubjectKind,
    /// Docs-node / symbol / file ref (no raw body).
    pub subject_ref: String,
    /// The ledger sequence that first surfaced this result.
    pub origin_query_sequence: u32,
    /// Source/version/freshness/locality/confidence chips.
    pub chips: SemanticRecallChipSet,
    /// Explicit, human-readable ranking reason.
    pub ranking_reason: String,
    /// Ranking-signal breakdown behind the reason.
    pub ranking_signals: Vec<RankingSignal>,
    /// Inline provenance for the result.
    pub provenance: ResultProvenance,
    /// Open-raw escape ref (open the underlying node/symbol).
    pub open_raw_escape_ref: String,
    /// Open-source escape ref (open the upstream/source).
    pub open_source_escape_ref: String,
}

/// One provenance export row, mirroring a result row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecallProvenanceRow {
    /// The recall row this provenance mirrors.
    pub result_id_ref: String,
    /// Subject kind (mirrors the row).
    pub subject_kind: SemanticRecallSubjectKind,
    /// Source class (must match the row's chip).
    pub source_class: SemanticRecallSourceClass,
    /// Confidence (must match the row's chip).
    pub confidence: SemanticRecallConfidence,
    /// Derivation (mirrors the row's provenance).
    pub derivation: DerivationClass,
    /// Whether the result is cited.
    pub cited: bool,
    /// Open-raw escape ref.
    pub open_raw_escape_ref: String,
    /// Open-source escape ref.
    pub open_source_escape_ref: String,
}

/// The provenance export projection for the recall.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecallProvenanceExport {
    /// Scope this export covers.
    pub scope: ProvenanceExportScope,
    /// Whether the export preserves each row's source class.
    pub preserves_source_class: bool,
    /// Whether the export preserves each row's confidence.
    pub preserves_confidence: bool,
    /// Whether the export preserves the open-raw / open-source escapes.
    pub preserves_open_raw_open_source_escape: bool,
    /// Whether the export preserves inference / derivation labels.
    pub preserves_inference_labels: bool,
    /// Per-result provenance rows.
    pub rows: Vec<SemanticRecallProvenanceRow>,
}

impl SemanticRecallProvenanceExport {
    /// Whether the export preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_source_class
            && self.preserves_confidence
            && self.preserves_open_raw_open_source_escape
            && self.preserves_inference_labels
    }
}

/// A session-level recall degradation attached to the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecallDegradation {
    /// Degradation class.
    pub degradation_class: RecallDegradationClass,
    /// Severity.
    pub severity: SemanticRecallFindingSeverity,
    /// Human-readable summary (no raw bodies).
    pub summary: String,
    /// The result this degradation annotates, if scoped to one row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_id_ref: Option<String>,
    /// Optional supporting evidence ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
}

/// How a consumer surface projects the recall.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecallConsumerProjection {
    /// Surface that consumes the recall.
    pub surface: SemanticRecallConsumerSurface,
    /// Packet id this projection mirrors.
    pub packet_id_ref: String,
    /// Whether the surface preserves the chip set verbatim.
    pub preserves_chips: bool,
    /// Whether the surface preserves the query-session ledger.
    pub preserves_query_session_ledger: bool,
    /// Whether the surface preserves the ranking reasons and signals.
    pub preserves_ranking_reasons: bool,
    /// Whether the surface preserves the provenance export.
    pub preserves_provenance_export: bool,
    /// Whether the surface preserves the open-raw / open-source escapes.
    pub preserves_open_raw_open_source_escape: bool,
}

impl SemanticRecallConsumerProjection {
    /// Whether the projection preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_chips
            && self.preserves_query_session_ledger
            && self.preserves_ranking_reasons
            && self.preserves_provenance_export
            && self.preserves_open_raw_open_source_escape
    }
}

/// A single validation finding on the recall packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecallValidationFinding {
    /// Finding kind.
    pub finding_kind: SemanticRecallFindingKind,
    /// Finding severity.
    pub severity: SemanticRecallFindingSeverity,
    /// Human-readable summary.
    pub summary: String,
}

/// Constructor input for [`SemanticRecallLedgerPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecallLedgerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable session label (never raw query text).
    pub session_label: String,
    /// Opaque digest/ref for the session (never raw query text).
    pub session_digest_ref: String,
    /// The ordered query-session ledger.
    pub query_session_ledger: SemanticRecallQuerySessionLedger,
    /// Ranked result rows for the active ranking.
    pub result_rows: Vec<SemanticRecallResultRow>,
    /// The provenance export projection.
    pub provenance_export: SemanticRecallProvenanceExport,
    /// Session-level degradations.
    pub recall_degradations: Vec<RecallDegradation>,
    /// Consumer projections.
    pub consumer_projections: Vec<SemanticRecallConsumerProjection>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp (RFC 3339).
    pub minted_at: String,
}

/// Export-safe docs-and-code semantic-recall packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecallLedgerPacket {
    /// Record kind; must equal [`SEMANTIC_RECALL_LEDGER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SEMANTIC_RECALL_LEDGER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable session label.
    pub session_label: String,
    /// Opaque digest/ref for the session.
    pub session_digest_ref: String,
    /// The ordered query-session ledger.
    pub query_session_ledger: SemanticRecallQuerySessionLedger,
    /// Ranked result rows.
    pub result_rows: Vec<SemanticRecallResultRow>,
    /// The provenance export projection.
    pub provenance_export: SemanticRecallProvenanceExport,
    /// Session-level degradations.
    pub recall_degradations: Vec<RecallDegradation>,
    /// Consumer projections.
    pub consumer_projections: Vec<SemanticRecallConsumerProjection>,
    /// Computed promotion state.
    pub promotion_state: SemanticRecallPromotionState,
    /// Computed validation findings.
    pub validation_findings: Vec<SemanticRecallValidationFinding>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Required consumer surfaces that every recall packet must project.
const REQUIRED_SURFACES: [SemanticRecallConsumerSurface; 5] = [
    SemanticRecallConsumerSurface::DocsBrowser,
    SemanticRecallConsumerSurface::SearchShell,
    SemanticRecallConsumerSurface::CodeExplainer,
    SemanticRecallConsumerSurface::RetrievalInspector,
    SemanticRecallConsumerSurface::SupportExport,
];

impl SemanticRecallLedgerPacket {
    /// Materializes a recall packet, computing validation findings and the
    /// promotion state from the recall input.
    pub fn materialize(input: SemanticRecallLedgerPacketInput) -> Self {
        let mut findings = Vec::new();

        check_identity(&input, &mut findings);
        check_ledger(&input, &mut findings);
        check_result_rows(&input, &mut findings);
        check_provenance_export(&input, &mut findings);
        check_degradations(&input, &mut findings);
        check_consumer_projections(&input, &mut findings);
        check_boundary(&input, &mut findings);

        let promotion_state = promotion_state(&findings, &input.recall_degradations);

        Self {
            record_kind: SEMANTIC_RECALL_LEDGER_RECORD_KIND.to_owned(),
            schema_version: SEMANTIC_RECALL_LEDGER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            session_label: input.session_label,
            session_digest_ref: input.session_digest_ref,
            query_session_ledger: input.query_session_ledger,
            result_rows: input.result_rows,
            provenance_export: input.provenance_export,
            recall_degradations: input.recall_degradations,
            consumer_projections: input.consumer_projections,
            promotion_state,
            validation_findings: findings,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Whether the recall qualifies for the Stable claim with no findings.
    pub fn is_clean_stable(&self) -> bool {
        self.promotion_state == SemanticRecallPromotionState::Stable
            && self.validation_findings.is_empty()
    }

    /// Wraps the packet in a support-export envelope.
    pub fn support_export(
        &self,
        export_id: &str,
        exported_at: &str,
    ) -> SemanticRecallLedgerSupportExport {
        SemanticRecallLedgerSupportExport {
            record_kind: SEMANTIC_RECALL_LEDGER_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SEMANTIC_RECALL_LEDGER_SCHEMA_VERSION,
            export_id: export_id.to_owned(),
            exported_at: exported_at.to_owned(),
            schema_ref: SEMANTIC_RECALL_LEDGER_SCHEMA_REF.to_owned(),
            doc_ref: SEMANTIC_RECALL_LEDGER_DOC_REF.to_owned(),
            packet: self.clone(),
        }
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("semantic recall packet serializes")
    }

    /// Deterministic Markdown summary for docs, support, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Docs and Code Semantic Recall\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Session: {}\n", self.session_label));
        out.push_str(&format!(
            "- Promotion: `{}` ({} findings)\n",
            self.promotion_state.as_str(),
            self.validation_findings.len()
        ));
        out.push_str(&format!(
            "- Queries: {} | Rows: {} | Degradations: {}\n",
            self.query_session_ledger.entries.len(),
            self.result_rows.len(),
            self.recall_degradations.len()
        ));
        out.push_str("\n## Query-session ledger\n\n");
        for entry in &self.query_session_ledger.entries {
            out.push_str(&format!(
                "{}. [{}/{}] {}\n",
                entry.sequence,
                entry.refinement.as_str(),
                entry.subject_scope.as_str(),
                entry.query_label,
            ));
        }
        out.push_str("\n## Results\n\n");
        for row in &self.result_rows {
            out.push_str(&format!(
                "{}. `{}` ({}) — {} / {} / {} / {} / {}\n",
                row.rank,
                row.result_id,
                row.subject_kind.as_str(),
                row.chips.source_class.as_str(),
                row.chips.version_match.as_str(),
                row.chips.freshness.as_str(),
                row.chips.locality.as_str(),
                row.chips.confidence.as_str(),
            ));
            out.push_str(&format!("   - Reason: {}\n", row.ranking_reason));
            out.push_str(&format!(
                "   - Provenance: {} / cited={}\n",
                row.provenance.derivation.as_str(),
                row.provenance.cited,
            ));
        }
        if !self.recall_degradations.is_empty() {
            out.push_str("\n## Degradations\n\n");
            for degradation in &self.recall_degradations {
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

/// Support-export envelope for the recall packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecallLedgerSupportExport {
    /// Record kind; must equal [`SEMANTIC_RECALL_LEDGER_SUPPORT_EXPORT_RECORD_KIND`].
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
    /// The wrapped recall packet.
    pub packet: SemanticRecallLedgerPacket,
}

/// Errors emitted when reading the checked-in recall support export.
#[derive(Debug)]
pub enum SemanticRecallLedgerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Re-materialization disagreed with the checked-in promotion state.
    PromotionDrift {
        /// Promotion state recorded in the export.
        recorded: SemanticRecallPromotionState,
        /// Promotion state computed by re-materialization.
        computed: SemanticRecallPromotionState,
    },
    /// The checked-in packet should be clean Stable but is not.
    NotCleanStable(Vec<SemanticRecallValidationFinding>),
}

impl fmt::Display for SemanticRecallLedgerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "semantic recall export parse failed: {error}")
            }
            Self::PromotionDrift { recorded, computed } => write!(
                formatter,
                "semantic recall promotion drift: recorded {} but computed {}",
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
                    "semantic recall export is not clean stable: {tokens}"
                )
            }
        }
    }
}

impl Error for SemanticRecallLedgerArtifactError {}

/// Reads and re-validates the checked-in stable recall support export.
pub fn current_stable_semantic_recall_ledger_export(
) -> Result<SemanticRecallLedgerSupportExport, SemanticRecallLedgerArtifactError> {
    let export: SemanticRecallLedgerSupportExport = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/implement_docs_and_code_semantic_recall_with_query_session_ledger_ranking_reasons_and_provenance_export/support_export.json"
    )))
    .map_err(SemanticRecallLedgerArtifactError::SupportExport)?;

    let recomputed = SemanticRecallLedgerPacket::materialize(packet_to_input(&export.packet));
    if recomputed.promotion_state != export.packet.promotion_state {
        return Err(SemanticRecallLedgerArtifactError::PromotionDrift {
            recorded: export.packet.promotion_state,
            computed: recomputed.promotion_state,
        });
    }
    if !export.packet.is_clean_stable() {
        return Err(SemanticRecallLedgerArtifactError::NotCleanStable(
            export.packet.validation_findings.clone(),
        ));
    }
    Ok(export)
}

/// Rebuilds the materialization input from a packet (used for re-validation).
pub fn packet_to_input(packet: &SemanticRecallLedgerPacket) -> SemanticRecallLedgerPacketInput {
    SemanticRecallLedgerPacketInput {
        packet_id: packet.packet_id.clone(),
        session_label: packet.session_label.clone(),
        session_digest_ref: packet.session_digest_ref.clone(),
        query_session_ledger: packet.query_session_ledger.clone(),
        result_rows: packet.result_rows.clone(),
        provenance_export: packet.provenance_export.clone(),
        recall_degradations: packet.recall_degradations.clone(),
        consumer_projections: packet.consumer_projections.clone(),
        redaction_class_token: packet.redaction_class_token.clone(),
        minted_at: packet.minted_at.clone(),
    }
}

fn push_finding(
    findings: &mut Vec<SemanticRecallValidationFinding>,
    kind: SemanticRecallFindingKind,
    summary: impl Into<String>,
) {
    findings.push(SemanticRecallValidationFinding {
        finding_kind: kind,
        severity: kind.default_severity(),
        summary: summary.into(),
    });
}

fn check_identity(
    input: &SemanticRecallLedgerPacketInput,
    findings: &mut Vec<SemanticRecallValidationFinding>,
) {
    if input.packet_id.trim().is_empty()
        || input.session_label.trim().is_empty()
        || input.session_digest_ref.trim().is_empty()
        || input.query_session_ledger.session_id.trim().is_empty()
        || input.redaction_class_token.trim().is_empty()
        || input.minted_at.trim().is_empty()
    {
        push_finding(
            findings,
            SemanticRecallFindingKind::MissingIdentity,
            "packet identity fields must all be present",
        );
    }
}

fn check_ledger(
    input: &SemanticRecallLedgerPacketInput,
    findings: &mut Vec<SemanticRecallValidationFinding>,
) {
    let ledger = &input.query_session_ledger;
    if ledger.is_empty() {
        push_finding(
            findings,
            SemanticRecallFindingKind::LedgerEmpty,
            "the query-session ledger must carry at least one query",
        );
        return;
    }

    let row_ids: BTreeSet<&str> = input
        .result_rows
        .iter()
        .map(|row| row.result_id.as_str())
        .collect();

    for (index, entry) in ledger.entries.iter().enumerate() {
        let expected_sequence = (index as u32) + 1;
        if entry.sequence != expected_sequence {
            push_finding(
                findings,
                SemanticRecallFindingKind::LedgerSequenceNotMonotonic,
                format!(
                    "ledger entry {} has sequence {} but expected {}",
                    index, entry.sequence, expected_sequence
                ),
            );
        }
        if entry.query_digest_ref.trim().is_empty() || entry.query_label.trim().is_empty() {
            push_finding(
                findings,
                SemanticRecallFindingKind::LedgerEntryQueryDigestMissing,
                format!(
                    "ledger entry {} is missing its query digest or label",
                    index
                ),
            );
        }
        let is_first = index == 0;
        if is_first != entry.refinement.is_initial() {
            push_finding(
                findings,
                SemanticRecallFindingKind::LedgerRefinementInconsistent,
                format!(
                    "ledger entry {} refinement `{}` is inconsistent with its position",
                    index,
                    entry.refinement.as_str()
                ),
            );
        }
        for surfaced in &entry.surfaced_result_ids {
            if !row_ids.contains(surfaced.as_str()) {
                push_finding(
                    findings,
                    SemanticRecallFindingKind::LedgerSurfacedResultOrphan,
                    format!(
                        "ledger entry {} surfaces unknown result `{}`",
                        index, surfaced
                    ),
                );
            }
        }
    }
}

fn check_result_rows(
    input: &SemanticRecallLedgerPacketInput,
    findings: &mut Vec<SemanticRecallValidationFinding>,
) {
    if input.result_rows.is_empty() {
        push_finding(
            findings,
            SemanticRecallFindingKind::ResultRowsEmpty,
            "recall returned no rows",
        );
        return;
    }

    let ledger_sequences: BTreeSet<u32> = input
        .query_session_ledger
        .entries
        .iter()
        .map(|entry| entry.sequence)
        .collect();

    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    for (index, row) in input.result_rows.iter().enumerate() {
        let expected_rank = (index as u32) + 1;
        if row.rank != expected_rank {
            push_finding(
                findings,
                SemanticRecallFindingKind::ResultRankNotMonotonic,
                format!(
                    "row `{}` has rank {} but expected {}",
                    row.result_id, row.rank, expected_rank
                ),
            );
        }
        if !seen_ids.insert(row.result_id.as_str()) {
            push_finding(
                findings,
                SemanticRecallFindingKind::DuplicateResultId,
                format!("duplicate result id `{}`", row.result_id),
            );
        }
        if !ledger_sequences.contains(&row.origin_query_sequence) {
            push_finding(
                findings,
                SemanticRecallFindingKind::ResultOriginQueryOrphan,
                format!(
                    "row `{}` origin query {} is not in the ledger",
                    row.result_id, row.origin_query_sequence
                ),
            );
        }
        if row.ranking_reason.trim().is_empty() {
            push_finding(
                findings,
                SemanticRecallFindingKind::RankingReasonMissing,
                format!("row `{}` is missing a ranking reason", row.result_id),
            );
        }
        if row.ranking_signals.is_empty() {
            push_finding(
                findings,
                SemanticRecallFindingKind::RankingSignalsMissing,
                format!("row `{}` carries no ranking signals", row.result_id),
            );
        }
        if row.open_raw_escape_ref.trim().is_empty() || row.open_source_escape_ref.trim().is_empty()
        {
            push_finding(
                findings,
                SemanticRecallFindingKind::OpenRawOpenSourceEscapeMissing,
                format!(
                    "row `{}` must keep open-raw and open-source escapes",
                    row.result_id
                ),
            );
        }

        if row.provenance.derivation.needs_citation() && !row.provenance.cited {
            push_finding(
                findings,
                SemanticRecallFindingKind::CodeResultNotCited,
                format!(
                    "row `{}` is `{}` but is not cited",
                    row.result_id,
                    row.provenance.derivation.as_str()
                ),
            );
        }
        if matches!(
            row.provenance.derivation,
            DerivationClass::InferredExplanation
        ) && row.chips.confidence == SemanticRecallConfidence::High
        {
            push_finding(
                findings,
                SemanticRecallFindingKind::InferredResultLooksAuthoritative,
                format!(
                    "row `{}` is an inferred explanation presented as high confidence",
                    row.result_id
                ),
            );
        }
        if !row.chips.version_match.is_confident_current()
            && row.chips.confidence == SemanticRecallConfidence::High
            && row.chips.freshness.is_authoritative_live()
        {
            push_finding(
                findings,
                SemanticRecallFindingKind::VersionTruthCollapsed,
                format!(
                    "row `{}` presents version `{}` as a confident live match",
                    row.result_id,
                    row.chips.version_match.as_str()
                ),
            );
        }
    }
}

fn check_provenance_export(
    input: &SemanticRecallLedgerPacketInput,
    findings: &mut Vec<SemanticRecallValidationFinding>,
) {
    let export = &input.provenance_export;
    if !export.preserves_all() {
        push_finding(
            findings,
            SemanticRecallFindingKind::ProvenanceExportDropsPreservation,
            "the provenance export must preserve source class, confidence, escapes, and inference labels",
        );
    }

    let mut provenance_ids: BTreeSet<&str> = BTreeSet::new();
    for provenance in &export.rows {
        provenance_ids.insert(provenance.result_id_ref.as_str());
        let row = input
            .result_rows
            .iter()
            .find(|row| row.result_id == provenance.result_id_ref);
        match row {
            None => push_finding(
                findings,
                SemanticRecallFindingKind::ProvenanceExportRowOrphan,
                format!(
                    "provenance row references unknown result `{}`",
                    provenance.result_id_ref
                ),
            ),
            Some(row) => {
                if row.chips.source_class != provenance.source_class {
                    push_finding(
                        findings,
                        SemanticRecallFindingKind::ProvenanceSourceClassMismatch,
                        format!(
                            "provenance for `{}` records source `{}` but the row chip is `{}`",
                            provenance.result_id_ref,
                            provenance.source_class.as_str(),
                            row.chips.source_class.as_str()
                        ),
                    );
                }
                if row.chips.confidence != provenance.confidence {
                    push_finding(
                        findings,
                        SemanticRecallFindingKind::ProvenanceConfidenceMismatch,
                        format!(
                            "provenance for `{}` records confidence `{}` but the row chip is `{}`",
                            provenance.result_id_ref,
                            provenance.confidence.as_str(),
                            row.chips.confidence.as_str()
                        ),
                    );
                }
            }
        }
    }

    for row in &input.result_rows {
        if !provenance_ids.contains(row.result_id.as_str()) {
            push_finding(
                findings,
                SemanticRecallFindingKind::ProvenanceExportCoverageMissing,
                format!("result `{}` has no provenance export row", row.result_id),
            );
        }
    }
}

fn check_degradations(
    input: &SemanticRecallLedgerPacketInput,
    findings: &mut Vec<SemanticRecallValidationFinding>,
) {
    let row_ids: BTreeSet<&str> = input
        .result_rows
        .iter()
        .map(|row| row.result_id.as_str())
        .collect();

    for degradation in &input.recall_degradations {
        if degradation.summary.trim().is_empty() {
            push_finding(
                findings,
                SemanticRecallFindingKind::DegradationIncomplete,
                format!(
                    "degradation `{}` is missing a summary",
                    degradation.degradation_class.as_str()
                ),
            );
        }
        if let Some(result_id) = &degradation.result_id_ref {
            if !result_id.trim().is_empty() && !row_ids.contains(result_id.as_str()) {
                push_finding(
                    findings,
                    SemanticRecallFindingKind::DegradationOrphan,
                    format!("degradation references unknown result `{}`", result_id),
                );
            }
        }
    }
}

fn check_consumer_projections(
    input: &SemanticRecallLedgerPacketInput,
    findings: &mut Vec<SemanticRecallValidationFinding>,
) {
    let present: BTreeSet<SemanticRecallConsumerSurface> = input
        .consumer_projections
        .iter()
        .map(|projection| projection.surface)
        .collect();
    for required in REQUIRED_SURFACES {
        if !present.contains(&required) {
            push_finding(
                findings,
                SemanticRecallFindingKind::RequiredSurfaceCoverageMissing,
                format!("required surface `{}` is missing", required.as_str()),
            );
        }
    }

    for projection in &input.consumer_projections {
        if projection.packet_id_ref != input.packet_id {
            push_finding(
                findings,
                SemanticRecallFindingKind::ConsumerProjectionPacketIdMismatch,
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
                SemanticRecallFindingKind::ConsumerProjectionDrift,
                format!(
                    "surface `{}` drops a required preservation flag",
                    projection.surface.as_str()
                ),
            );
        }
    }
}

fn check_boundary(
    input: &SemanticRecallLedgerPacketInput,
    findings: &mut Vec<SemanticRecallValidationFinding>,
) {
    let value = serde_json::to_value(input).expect("semantic recall input serializes");
    if json_contains_forbidden_boundary_material(&value) {
        push_finding(
            findings,
            SemanticRecallFindingKind::RawBoundaryMaterialPresent,
            "export must not carry raw query text, raw bodies, or secrets",
        );
    }
}

/// Computes the promotion state from the worst severity across both the
/// validation findings and the attached degradations.
///
/// A blocking finding (integrity, trust, citation, or boundary violation) blocks
/// the Stable claim; an otherwise-clean recall that carries a narrowing
/// degradation narrows below Stable rather than hiding the result.
fn promotion_state(
    findings: &[SemanticRecallValidationFinding],
    degradations: &[RecallDegradation],
) -> SemanticRecallPromotionState {
    let any_blocking = findings
        .iter()
        .any(|finding| finding.severity == SemanticRecallFindingSeverity::Blocking)
        || degradations
            .iter()
            .any(|degradation| degradation.severity == SemanticRecallFindingSeverity::Blocking);
    if any_blocking {
        return SemanticRecallPromotionState::BlocksStable;
    }

    let any_narrowing = findings
        .iter()
        .any(|finding| finding.severity == SemanticRecallFindingSeverity::Narrowing)
        || degradations
            .iter()
            .any(|degradation| degradation.severity == SemanticRecallFindingSeverity::Narrowing);
    if any_narrowing {
        SemanticRecallPromotionState::NarrowedBelowStable
    } else {
        SemanticRecallPromotionState::Stable
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
                || lower.contains("raw_query:")
                || lower.contains("raw_body:")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Seeded stable recall input used by the producer, tests, and fixtures.
pub fn seeded_stable_semantic_recall_ledger_input() -> SemanticRecallLedgerPacketInput {
    let packet_id = "packet:m5:semantic_recall:retry_backoff_session".to_owned();
    SemanticRecallLedgerPacketInput {
        packet_id: packet_id.clone(),
        session_label: "semantic recall: request retry and backoff handling".to_owned(),
        session_digest_ref: "sessiondigest:sha256:retry-backoff-session".to_owned(),
        query_session_ledger: SemanticRecallQuerySessionLedger {
            session_id: "session:m5:retry_backoff".to_owned(),
            entries: vec![
                SemanticRecallLedgerEntry {
                    sequence: 1,
                    query_digest_ref: "querydigest:sha256:retry-backoff".to_owned(),
                    query_label: "retry and backoff handling".to_owned(),
                    refinement: QueryRefinementRelation::Initial,
                    subject_scope: SemanticRecallSubjectScope::DocsAndCode,
                    surfaced_result_ids: vec!["result:docs:retry_policy_guide".to_owned()],
                },
                SemanticRecallLedgerEntry {
                    sequence: 2,
                    query_digest_ref: "querydigest:sha256:retry-in-http-client".to_owned(),
                    query_label: "retry/backoff in the http client".to_owned(),
                    refinement: QueryRefinementRelation::Narrowed,
                    subject_scope: SemanticRecallSubjectScope::CodeOnly,
                    surfaced_result_ids: vec![
                        "result:code:retry_with_backoff_fn".to_owned(),
                        "result:code:http_client_send".to_owned(),
                    ],
                },
                SemanticRecallLedgerEntry {
                    sequence: 3,
                    query_digest_ref: "querydigest:sha256:exponential-backoff-jitter".to_owned(),
                    query_label: "exponential backoff with jitter".to_owned(),
                    refinement: QueryRefinementRelation::Reformulated,
                    subject_scope: SemanticRecallSubjectScope::DocsAndCode,
                    surfaced_result_ids: vec!["result:docs:mirrored_backoff_reference".to_owned()],
                },
            ],
        },
        result_rows: vec![
            SemanticRecallResultRow {
                rank: 1,
                result_id: "result:code:retry_with_backoff_fn".to_owned(),
                subject_kind: SemanticRecallSubjectKind::CodeSymbol,
                subject_ref: "symbol:aureline-net:retry::retry_with_backoff".to_owned(),
                origin_query_sequence: 2,
                chips: SemanticRecallChipSet {
                    source_class: SemanticRecallSourceClass::WorkspaceCode,
                    version_match: SemanticRecallVersionMatch::ExactBuildMatch,
                    freshness: SemanticRecallFreshness::AuthoritativeLive,
                    locality: SemanticRecallLocality::Local,
                    confidence: SemanticRecallConfidence::High,
                },
                ranking_reason: "exact symbol match in workspace code; called directly by the http client send path".to_owned(),
                ranking_signals: vec![
                    RankingSignal {
                        signal: RankingSignalKind::SymbolExactMatch,
                        contribution: SignalContributionClass::Primary,
                        note: "function name matches the query subject".to_owned(),
                    },
                    RankingSignal {
                        signal: RankingSignalKind::GraphProximity,
                        contribution: SignalContributionClass::Supporting,
                        note: "one call edge from the http client send path".to_owned(),
                    },
                ],
                provenance: ResultProvenance {
                    pack_id_ref: "repo:aureline-workspace".to_owned(),
                    pack_pinned: true,
                    pack_signed_and_verified: true,
                    derivation: DerivationClass::VerbatimNode,
                    cited: true,
                    citation_ref: Some("cite:symbol:aureline-net:retry::retry_with_backoff".to_owned()),
                },
                open_raw_escape_ref: "open-raw:symbol:aureline-net:retry::retry_with_backoff".to_owned(),
                open_source_escape_ref: "open-source:repo:crates/aureline-net/src/retry.rs".to_owned(),
            },
            SemanticRecallResultRow {
                rank: 2,
                result_id: "result:docs:retry_policy_guide".to_owned(),
                subject_kind: SemanticRecallSubjectKind::DocsNode,
                subject_ref: "docnode:project-docs:net/retry-policy".to_owned(),
                origin_query_sequence: 1,
                chips: SemanticRecallChipSet {
                    source_class: SemanticRecallSourceClass::ProjectDocs,
                    version_match: SemanticRecallVersionMatch::ExactBuildMatch,
                    freshness: SemanticRecallFreshness::AuthoritativeLive,
                    locality: SemanticRecallLocality::Local,
                    confidence: SemanticRecallConfidence::High,
                },
                ranking_reason: "exact build match on the local retry-policy guide with strong lexical+semantic overlap".to_owned(),
                ranking_signals: vec![
                    RankingSignal {
                        signal: RankingSignalKind::LexicalOverlap,
                        contribution: SignalContributionClass::Primary,
                        note: "title and headings overlap the query terms".to_owned(),
                    },
                    RankingSignal {
                        signal: RankingSignalKind::SemanticSimilarity,
                        contribution: SignalContributionClass::Supporting,
                        note: "embedding similarity above the strong-match threshold".to_owned(),
                    },
                ],
                provenance: ResultProvenance {
                    pack_id_ref: "pack:project-docs:aureline-workspace".to_owned(),
                    pack_pinned: true,
                    pack_signed_and_verified: true,
                    derivation: DerivationClass::VerbatimNode,
                    cited: true,
                    citation_ref: Some("cite:docnode:project-docs:net/retry-policy".to_owned()),
                },
                open_raw_escape_ref: "open-raw:docnode:project-docs:net/retry-policy".to_owned(),
                open_source_escape_ref: "open-source:repo:docs/net/retry-policy.md".to_owned(),
            },
            SemanticRecallResultRow {
                rank: 3,
                result_id: "result:docs:mirrored_backoff_reference".to_owned(),
                subject_kind: SemanticRecallSubjectKind::DocsNode,
                subject_ref: "docnode:mirror:backoff/reference".to_owned(),
                origin_query_sequence: 3,
                chips: SemanticRecallChipSet {
                    source_class: SemanticRecallSourceClass::MirroredOfficialDocs,
                    version_match: SemanticRecallVersionMatch::CompatibleMinorDrift,
                    freshness: SemanticRecallFreshness::WarmCached,
                    locality: SemanticRecallLocality::MirroredPack,
                    confidence: SemanticRecallConfidence::Medium,
                },
                ranking_reason: "pinned, signed mirror of the upstream backoff reference within the compatible drift window".to_owned(),
                ranking_signals: vec![
                    RankingSignal {
                        signal: RankingSignalKind::SemanticSimilarity,
                        contribution: SignalContributionClass::Primary,
                        note: "concept match on exponential backoff with jitter".to_owned(),
                    },
                    RankingSignal {
                        signal: RankingSignalKind::AuthorityBoost,
                        contribution: SignalContributionClass::Supporting,
                        note: "verified mirror of official upstream docs".to_owned(),
                    },
                ],
                provenance: ResultProvenance {
                    pack_id_ref: "pack:mirrored-official:backoff".to_owned(),
                    pack_pinned: true,
                    pack_signed_and_verified: true,
                    derivation: DerivationClass::VerbatimNode,
                    cited: true,
                    citation_ref: Some("cite:docnode:mirror:backoff/reference".to_owned()),
                },
                open_raw_escape_ref: "open-raw:docnode:mirror:backoff/reference".to_owned(),
                open_source_escape_ref: "open-source:mirror:backoff/reference".to_owned(),
            },
            SemanticRecallResultRow {
                rank: 4,
                result_id: "result:code:http_client_send".to_owned(),
                subject_kind: SemanticRecallSubjectKind::CodeFile,
                subject_ref: "file:crates/aureline-net/src/http_client.rs".to_owned(),
                origin_query_sequence: 2,
                chips: SemanticRecallChipSet {
                    source_class: SemanticRecallSourceClass::WorkspaceCode,
                    version_match: SemanticRecallVersionMatch::ExactBuildMatch,
                    freshness: SemanticRecallFreshness::WarmCached,
                    locality: SemanticRecallLocality::Local,
                    confidence: SemanticRecallConfidence::Medium,
                },
                ranking_reason: "workspace file that invokes the retry helper; explained via a cited summary, not a verbatim node".to_owned(),
                ranking_signals: vec![
                    RankingSignal {
                        signal: RankingSignalKind::GraphProximity,
                        contribution: SignalContributionClass::Primary,
                        note: "calls the top-ranked retry helper".to_owned(),
                    },
                    RankingSignal {
                        signal: RankingSignalKind::PathAffinity,
                        contribution: SignalContributionClass::Minor,
                        note: "same module as the active editor context".to_owned(),
                    },
                ],
                provenance: ResultProvenance {
                    pack_id_ref: "repo:aureline-workspace".to_owned(),
                    pack_pinned: true,
                    pack_signed_and_verified: true,
                    derivation: DerivationClass::DerivedSummary,
                    cited: true,
                    citation_ref: Some("cite:file:crates/aureline-net/src/http_client.rs".to_owned()),
                },
                open_raw_escape_ref: "open-raw:file:crates/aureline-net/src/http_client.rs".to_owned(),
                open_source_escape_ref: "open-source:repo:crates/aureline-net/src/http_client.rs".to_owned(),
            },
        ],
        provenance_export: seeded_provenance_export(),
        recall_degradations: vec![RecallDegradation {
            degradation_class: RecallDegradationClass::CodeGraphStale,
            severity: SemanticRecallFindingSeverity::Advisory,
            summary: "the code graph was indexed before the last two commits; call edges may lag the working tree".to_owned(),
            result_id_ref: None,
            evidence_ref: Some("evidence:index-freshness:code-graph".to_owned()),
        }],
        consumer_projections: required_projections(&packet_id),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-08T00:00:00Z".to_owned(),
    }
}

fn seeded_provenance_export() -> SemanticRecallProvenanceExport {
    SemanticRecallProvenanceExport {
        scope: ProvenanceExportScope::FullSession,
        preserves_source_class: true,
        preserves_confidence: true,
        preserves_open_raw_open_source_escape: true,
        preserves_inference_labels: true,
        rows: vec![
            SemanticRecallProvenanceRow {
                result_id_ref: "result:code:retry_with_backoff_fn".to_owned(),
                subject_kind: SemanticRecallSubjectKind::CodeSymbol,
                source_class: SemanticRecallSourceClass::WorkspaceCode,
                confidence: SemanticRecallConfidence::High,
                derivation: DerivationClass::VerbatimNode,
                cited: true,
                open_raw_escape_ref: "open-raw:symbol:aureline-net:retry::retry_with_backoff"
                    .to_owned(),
                open_source_escape_ref: "open-source:repo:crates/aureline-net/src/retry.rs"
                    .to_owned(),
            },
            SemanticRecallProvenanceRow {
                result_id_ref: "result:docs:retry_policy_guide".to_owned(),
                subject_kind: SemanticRecallSubjectKind::DocsNode,
                source_class: SemanticRecallSourceClass::ProjectDocs,
                confidence: SemanticRecallConfidence::High,
                derivation: DerivationClass::VerbatimNode,
                cited: true,
                open_raw_escape_ref: "open-raw:docnode:project-docs:net/retry-policy".to_owned(),
                open_source_escape_ref: "open-source:repo:docs/net/retry-policy.md".to_owned(),
            },
            SemanticRecallProvenanceRow {
                result_id_ref: "result:docs:mirrored_backoff_reference".to_owned(),
                subject_kind: SemanticRecallSubjectKind::DocsNode,
                source_class: SemanticRecallSourceClass::MirroredOfficialDocs,
                confidence: SemanticRecallConfidence::Medium,
                derivation: DerivationClass::VerbatimNode,
                cited: true,
                open_raw_escape_ref: "open-raw:docnode:mirror:backoff/reference".to_owned(),
                open_source_escape_ref: "open-source:mirror:backoff/reference".to_owned(),
            },
            SemanticRecallProvenanceRow {
                result_id_ref: "result:code:http_client_send".to_owned(),
                subject_kind: SemanticRecallSubjectKind::CodeFile,
                source_class: SemanticRecallSourceClass::WorkspaceCode,
                confidence: SemanticRecallConfidence::Medium,
                derivation: DerivationClass::DerivedSummary,
                cited: true,
                open_raw_escape_ref: "open-raw:file:crates/aureline-net/src/http_client.rs"
                    .to_owned(),
                open_source_escape_ref: "open-source:repo:crates/aureline-net/src/http_client.rs"
                    .to_owned(),
            },
        ],
    }
}

fn required_projections(packet_id: &str) -> Vec<SemanticRecallConsumerProjection> {
    [
        SemanticRecallConsumerSurface::DocsBrowser,
        SemanticRecallConsumerSurface::SearchShell,
        SemanticRecallConsumerSurface::CodeExplainer,
        SemanticRecallConsumerSurface::AiContext,
        SemanticRecallConsumerSurface::RetrievalInspector,
        SemanticRecallConsumerSurface::CliHeadless,
        SemanticRecallConsumerSurface::SupportExport,
        SemanticRecallConsumerSurface::HelpAbout,
    ]
    .into_iter()
    .map(|surface| SemanticRecallConsumerProjection {
        surface,
        packet_id_ref: packet_id.to_owned(),
        preserves_chips: true,
        preserves_query_session_ledger: true,
        preserves_ranking_reasons: true,
        preserves_provenance_export: true,
        preserves_open_raw_open_source_escape: true,
    })
    .collect()
}
