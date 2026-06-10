//! Mirrored docs-pack recall results with source/version/freshness chips and
//! stale-example findings.
//!
//! This module implements the M5 docs-pack recall feature: a ranked recall over
//! the mirror-aware docs packs that returns one [`DocsPackRecallResultRow`] per
//! hit. Every row carries a [`DocsPackRecallChipSet`] — the source-class,
//! version-match, freshness, locality, and confidence chips a reader, the search
//! shell, AI context, and support exports project verbatim — plus an explicit
//! ranking reason and the open-raw / open-source escape refs that keep derived
//! results honest. Stale-example findings ([`DocsPackRecallStaleFinding`]) hang
//! off the recall by `result_id` so nearby-version, stale-example, and
//! quarantined-pack states stay distinct rather than collapsing into one generic
//! "stale" warning.
//!
//! [`DocsPackRecallPacket::materialize`] computes the validation findings and
//! the promotion state (`stable`, `narrowed_below_stable`, or `blocks_stable`)
//! from the recall input, so stale, under-attributed, or over-authoritative
//! recalls automatically narrow before they reach a consumer surface. The packet
//! is an inspectable, serde-serializable truth packet: it carries no raw query
//! text, no raw document bodies, no raw provider payloads, and no credentials —
//! only metadata, chip truth, ranking reasons, and contract references.
//!
//! The boundary schema is
//! [`schemas/docs/implement-mirrored-docs-pack-recall-source-or-version-or-freshness-chips-and-stale-example-findings.schema.json`](../../../../schemas/docs/implement-mirrored-docs-pack-recall-source-or-version-or-freshness-chips-and-stale-example-findings.schema.json).
//! The contract doc is
//! [`docs/docs/m5/implement_mirrored_docs_pack_recall_source_or_version_or_freshness_chips_and_stale_example_findings.md`](../../../../docs/docs/m5/implement_mirrored_docs_pack_recall_source_or_version_or_freshness_chips_and_stale_example_findings.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/implement_mirrored_docs_pack_recall_source_or_version_or_freshness_chips_and_stale_example_findings/`](../../../../fixtures/docs/m5/implement_mirrored_docs_pack_recall_source_or_version_or_freshness_chips_and_stale_example_findings/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`DocsPackRecallPacket`].
pub const DOCS_PACK_RECALL_RECORD_KIND: &str =
    "mirrored_docs_pack_recall_chips_and_stale_example_findings";

/// Record-kind tag carried by the support-export wrapper.
pub const DOCS_PACK_RECALL_SUPPORT_EXPORT_RECORD_KIND: &str =
    "mirrored_docs_pack_recall_chips_and_stale_example_findings_support_export";

/// Schema version for mirrored docs-pack recall records.
pub const DOCS_PACK_RECALL_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DOCS_PACK_RECALL_SCHEMA_REF: &str =
    "schemas/docs/implement-mirrored-docs-pack-recall-source-or-version-or-freshness-chips-and-stale-example-findings.schema.json";

/// Repo-relative path of the recall contract doc.
pub const DOCS_PACK_RECALL_DOC_REF: &str =
    "docs/docs/m5/implement_mirrored_docs_pack_recall_source_or_version_or_freshness_chips_and_stale_example_findings.md";

/// Repo-relative path of the protected fixture directory.
pub const DOCS_PACK_RECALL_FIXTURE_DIR: &str =
    "fixtures/docs/m5/implement_mirrored_docs_pack_recall_source_or_version_or_freshness_chips_and_stale_example_findings";

/// Repo-relative path of the checked support-export artifact.
pub const DOCS_PACK_RECALL_ARTIFACT_REF: &str =
    "artifacts/docs/m5/implement_mirrored_docs_pack_recall_source_or_version_or_freshness_chips_and_stale_example_findings/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const DOCS_PACK_RECALL_SUMMARY_REF: &str =
    "artifacts/docs/m5/implement_mirrored_docs_pack_recall_source_or_version_or_freshness_chips_and_stale_example_findings.md";

/// Source class for a recalled docs node, projected as the source chip.
///
/// Tokens match the canonical docs-pack source vocabulary so downstream
/// consumers keep one set of chip labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackRecallSourceClass {
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
}

impl DocsPackRecallSourceClass {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectDocs => "project_docs",
            Self::GeneratedReference => "generated_reference",
            Self::MirroredOfficialDocs => "mirrored_official_docs",
            Self::CuratedKnowledgePack => "curated_knowledge_pack",
            Self::SupportRunbook => "support_runbook",
            Self::ExtensionDocsPack => "extension_docs_pack",
        }
    }

    /// Whether this class is a mirror of upstream docs rather than local content.
    pub const fn is_mirrored_upstream(self) -> bool {
        matches!(self, Self::MirroredOfficialDocs)
    }
}

/// Version-match state for a recalled source, projected as the version chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackRecallVersionMatch {
    /// Source exactly matches the active build/workspace revision.
    ExactBuildMatch,
    /// Source is within an accepted compatible drift window.
    CompatibleMinorDrift,
    /// Source drifted incompatibly from the active target.
    IncompatibleDriftDetected,
    /// Pre-release source has not completed verification.
    PreReleaseUnverified,
    /// The target build/workspace revision could not be verified.
    UnknownTargetBuild,
}

impl DocsPackRecallVersionMatch {
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

/// Freshness state for a recalled source, projected as the freshness chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackRecallFreshness {
    /// Source was live and authoritative at recall time.
    AuthoritativeLive,
    /// Cached source within its freshness window.
    WarmCached,
    /// Cached source usable only with degraded disclosure.
    DegradedCached,
    /// Source is stale and must not claim current authority.
    Stale,
    /// Freshness could not be verified.
    Unverified,
    /// A refresh is pending; the mirror has not yet re-synced.
    RefreshPending,
}

impl DocsPackRecallFreshness {
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

/// Locality / install posture for a recalled source, projected as the locality chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackRecallLocality {
    /// Resolved from local content.
    Local,
    /// Resolved through a pinned mirror pack.
    MirroredPack,
    /// Resolved through a remote helper.
    RemoteHelper,
    /// Resolved through a managed (org-hosted) service.
    Managed,
}

impl DocsPackRecallLocality {
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
pub enum DocsPackRecallConfidence {
    /// High confidence.
    High,
    /// Medium confidence.
    Medium,
    /// Low confidence.
    Low,
    /// Heuristic only; not a verified match.
    Heuristic,
}

impl DocsPackRecallConfidence {
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

/// Class of a stale-example finding attached to a recall row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackRecallStaleFindingClass {
    /// A nearer-version example exists for the active build.
    NearbyVersion,
    /// The example is stale for the active build.
    StaleExample,
    /// The owning pack is quarantined and must not be presented as publishable.
    QuarantinedPack,
    /// A referenced link/anchor is broken.
    BrokenLink,
    /// The example needs human review before it can be trusted.
    NeedsReview,
    /// Supporting evidence for the example is missing.
    MissingEvidence,
}

impl DocsPackRecallStaleFindingClass {
    /// Stable token recorded in the finding.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NearbyVersion => "nearby_version",
            Self::StaleExample => "stale_example",
            Self::QuarantinedPack => "quarantined_pack",
            Self::BrokenLink => "broken_link",
            Self::NeedsReview => "needs_review",
            Self::MissingEvidence => "missing_evidence",
        }
    }
}

/// Severity of a stale-example or validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackRecallFindingSeverity {
    /// Blocks a Stable claim; the recall must narrow.
    Blocking,
    /// Narrows below Stable but the recall stays valid and attributable.
    Narrowing,
    /// Advisory only.
    Advisory,
}

impl DocsPackRecallFindingSeverity {
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
pub enum DocsPackRecallConsumerSurface {
    /// Docs browser / reader.
    DocsBrowser,
    /// Search shell results.
    SearchShell,
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

impl DocsPackRecallConsumerSurface {
    /// Stable token recorded in the projection.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowser => "docs_browser",
            Self::SearchShell => "search_shell",
            Self::AiContext => "ai_context",
            Self::RetrievalInspector => "retrieval_inspector",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::HelpAbout => "help_about",
        }
    }
}

/// Promotion state computed for the recall packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackRecallPromotionState {
    /// Recall qualifies for the Stable claim.
    Stable,
    /// Recall narrowed below Stable but stays valid and attributable.
    NarrowedBelowStable,
    /// Recall has a blocking finding and must not present as Stable.
    BlocksStable,
}

impl DocsPackRecallPromotionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Validation finding kind emitted by [`DocsPackRecallPacket::materialize`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackRecallFindingKind {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is missing.
    MissingIdentity,
    /// The recall returned no rows.
    ResultRowsEmpty,
    /// Result ranks are not strictly increasing from 1.
    ResultRankNotMonotonic,
    /// A result id is duplicated.
    DuplicateResultId,
    /// A row is missing its explicit ranking reason.
    RankingReasonMissing,
    /// A row is missing an open-raw / open-source escape ref.
    OpenRawOpenSourceEscapeMissing,
    /// Mirror-awareness invariants are not all satisfied.
    MirrorAwarenessIncomplete,
    /// A mirrored-upstream row claims live authority without a pinned, verified pack.
    LiveMirrorLooksMoreAuthoritative,
    /// A non-current version-match is presented as a confident live match.
    VersionTruthCollapsed,
    /// No mirrored-official-docs row is present, so the recall is not mirror-aware.
    MirroredSourceCoverageMissing,
    /// A stale finding is incomplete (missing summary or required ref).
    StaleFindingIncomplete,
    /// A stale finding references a result id absent from the rows.
    StaleFindingOrphan,
    /// A consumer projection drops a required preservation flag.
    ConsumerProjectionDrift,
    /// A consumer projection references the wrong packet id.
    ConsumerProjectionPacketIdMismatch,
    /// A required consumer surface is missing from the projections.
    RequiredSurfaceCoverageMissing,
    /// Raw query text, raw bodies, or secrets crossed the export boundary.
    RawBoundaryMaterialPresent,
}

impl DocsPackRecallFindingKind {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::ResultRowsEmpty => "result_rows_empty",
            Self::ResultRankNotMonotonic => "result_rank_not_monotonic",
            Self::DuplicateResultId => "duplicate_result_id",
            Self::RankingReasonMissing => "ranking_reason_missing",
            Self::OpenRawOpenSourceEscapeMissing => "open_raw_open_source_escape_missing",
            Self::MirrorAwarenessIncomplete => "mirror_awareness_incomplete",
            Self::LiveMirrorLooksMoreAuthoritative => "live_mirror_looks_more_authoritative",
            Self::VersionTruthCollapsed => "version_truth_collapsed",
            Self::MirroredSourceCoverageMissing => "mirrored_source_coverage_missing",
            Self::StaleFindingIncomplete => "stale_finding_incomplete",
            Self::StaleFindingOrphan => "stale_finding_orphan",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::ConsumerProjectionPacketIdMismatch => "consumer_projection_packet_id_mismatch",
            Self::RequiredSurfaceCoverageMissing => "required_surface_coverage_missing",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
        }
    }

    /// Default severity for this finding kind.
    pub const fn default_severity(self) -> DocsPackRecallFindingSeverity {
        match self {
            // Boundary, identity, and over-authoritative findings always block.
            Self::WrongRecordKind
            | Self::WrongSchemaVersion
            | Self::MissingIdentity
            | Self::ResultRowsEmpty
            | Self::ResultRankNotMonotonic
            | Self::DuplicateResultId
            | Self::RankingReasonMissing
            | Self::OpenRawOpenSourceEscapeMissing
            | Self::MirrorAwarenessIncomplete
            | Self::LiveMirrorLooksMoreAuthoritative
            | Self::VersionTruthCollapsed
            | Self::MirroredSourceCoverageMissing
            | Self::StaleFindingIncomplete
            | Self::StaleFindingOrphan
            | Self::ConsumerProjectionDrift
            | Self::ConsumerProjectionPacketIdMismatch
            | Self::RequiredSurfaceCoverageMissing
            | Self::RawBoundaryMaterialPresent => DocsPackRecallFindingSeverity::Blocking,
        }
    }
}

/// The chip set rendered for one recall result row.
///
/// These five chips are the source/version/freshness truth a reader sees; every
/// consumer surface that sets `preserves_chips` must project them verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackRecallChipSet {
    /// Source-class chip.
    pub source_class: DocsPackRecallSourceClass,
    /// Version-match chip.
    pub version_match: DocsPackRecallVersionMatch,
    /// Freshness chip.
    pub freshness: DocsPackRecallFreshness,
    /// Locality chip.
    pub locality: DocsPackRecallLocality,
    /// Confidence chip.
    pub confidence: DocsPackRecallConfidence,
}

/// A stale-example finding attached to a recall row by `result_id`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackRecallStaleFinding {
    /// Finding class; kept distinct so stale states never collapse.
    pub finding_class: DocsPackRecallStaleFindingClass,
    /// Finding severity.
    pub severity: DocsPackRecallFindingSeverity,
    /// The recall row this finding annotates.
    pub result_id_ref: String,
    /// The owning pack id.
    pub pack_id_ref: String,
    /// Human-readable summary (no raw bodies).
    pub summary: String,
    /// For `nearby_version`, the nearer-version anchor ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nearby_version_ref: Option<String>,
    /// Optional supporting evidence ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
}

/// One ranked recall result row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackRecallResultRow {
    /// 1-based rank.
    pub rank: u32,
    /// Stable result id within this recall.
    pub result_id: String,
    /// Docs-node ref (no raw body).
    pub doc_node_ref: String,
    /// Owning pack id.
    pub pack_id_ref: String,
    /// Whether the owning pack is pinned.
    pub pack_pinned: bool,
    /// Whether the owning pack's signature is verified.
    pub pack_signed_and_verified: bool,
    /// Source/version/freshness/locality/confidence chips.
    pub chips: DocsPackRecallChipSet,
    /// Explicit, human-readable ranking reason.
    pub ranking_reason: String,
    /// Open-raw escape ref (open the underlying node).
    pub open_raw_escape_ref: String,
    /// Open-source escape ref (open the upstream/source).
    pub open_source_escape_ref: String,
}

/// Mirror-awareness summary for the recall.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackRecallMirrorAwareness {
    /// A pinned, signed mirror outranks live vendor docs in the recall ordering.
    pub pinned_signed_mirror_outranks_live: bool,
    /// Unpinned live vendor docs are demoted rather than presented as authoritative.
    pub live_vendor_docs_demoted_when_unpinned: bool,
    /// The recall verifies mirror signatures before trusting mirrored rows.
    pub mirror_signature_verified: bool,
}

impl DocsPackRecallMirrorAwareness {
    /// Whether every mirror-awareness invariant holds.
    pub const fn is_complete(self) -> bool {
        self.pinned_signed_mirror_outranks_live
            && self.live_vendor_docs_demoted_when_unpinned
            && self.mirror_signature_verified
    }
}

/// How a consumer surface projects the recall.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackRecallConsumerProjection {
    /// Surface that consumes the recall.
    pub surface: DocsPackRecallConsumerSurface,
    /// Packet id this projection mirrors.
    pub packet_id_ref: String,
    /// Whether the surface preserves the chip set verbatim.
    pub preserves_chips: bool,
    /// Whether the surface preserves the stale-example findings.
    pub preserves_stale_findings: bool,
    /// Whether the surface preserves the per-row ranking reason.
    pub preserves_ranking_reason: bool,
    /// Whether the surface preserves the open-raw / open-source escapes.
    pub preserves_open_raw_open_source_escape: bool,
    /// Whether the surface preserves the mirror-awareness summary.
    pub preserves_mirror_awareness: bool,
}

impl DocsPackRecallConsumerProjection {
    /// Whether the projection preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_chips
            && self.preserves_stale_findings
            && self.preserves_ranking_reason
            && self.preserves_open_raw_open_source_escape
            && self.preserves_mirror_awareness
    }
}

/// Constructor input for [`DocsPackRecallPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackRecallPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable recall label (never raw query text).
    pub recall_label: String,
    /// Opaque digest/ref for the originating query (never raw query text).
    pub query_digest_ref: String,
    /// Mirror-awareness summary.
    pub mirror_awareness: DocsPackRecallMirrorAwareness,
    /// Ranked result rows.
    pub result_rows: Vec<DocsPackRecallResultRow>,
    /// Stale-example findings attached by `result_id`.
    pub stale_example_findings: Vec<DocsPackRecallStaleFinding>,
    /// Consumer projections.
    pub consumer_projections: Vec<DocsPackRecallConsumerProjection>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp (RFC 3339).
    pub minted_at: String,
}

/// A single validation finding on the recall packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackRecallValidationFinding {
    /// Finding kind.
    pub finding_kind: DocsPackRecallFindingKind,
    /// Finding severity.
    pub severity: DocsPackRecallFindingSeverity,
    /// Human-readable summary.
    pub summary: String,
}

/// Export-safe mirrored docs-pack recall packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackRecallPacket {
    /// Record kind; must equal [`DOCS_PACK_RECALL_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DOCS_PACK_RECALL_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable recall label.
    pub recall_label: String,
    /// Opaque digest/ref for the originating query.
    pub query_digest_ref: String,
    /// Mirror-awareness summary.
    pub mirror_awareness: DocsPackRecallMirrorAwareness,
    /// Ranked result rows.
    pub result_rows: Vec<DocsPackRecallResultRow>,
    /// Stale-example findings.
    pub stale_example_findings: Vec<DocsPackRecallStaleFinding>,
    /// Consumer projections.
    pub consumer_projections: Vec<DocsPackRecallConsumerProjection>,
    /// Computed promotion state.
    pub promotion_state: DocsPackRecallPromotionState,
    /// Computed validation findings.
    pub validation_findings: Vec<DocsPackRecallValidationFinding>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Required consumer surfaces that every recall packet must project.
const REQUIRED_SURFACES: [DocsPackRecallConsumerSurface; 4] = [
    DocsPackRecallConsumerSurface::DocsBrowser,
    DocsPackRecallConsumerSurface::SearchShell,
    DocsPackRecallConsumerSurface::RetrievalInspector,
    DocsPackRecallConsumerSurface::SupportExport,
];

impl DocsPackRecallPacket {
    /// Materializes a recall packet, computing validation findings and the
    /// promotion state from the recall input.
    pub fn materialize(input: DocsPackRecallPacketInput) -> Self {
        let mut findings = Vec::new();

        check_identity(&input, &mut findings);
        check_result_rows(&input, &mut findings);
        check_mirror_awareness(&input, &mut findings);
        check_stale_findings(&input, &mut findings);
        check_consumer_projections(&input, &mut findings);
        check_boundary(&input, &mut findings);

        let promotion_state = promotion_state(&findings, &input.stale_example_findings);

        Self {
            record_kind: DOCS_PACK_RECALL_RECORD_KIND.to_owned(),
            schema_version: DOCS_PACK_RECALL_SCHEMA_VERSION,
            packet_id: input.packet_id,
            recall_label: input.recall_label,
            query_digest_ref: input.query_digest_ref,
            mirror_awareness: input.mirror_awareness,
            result_rows: input.result_rows,
            stale_example_findings: input.stale_example_findings,
            consumer_projections: input.consumer_projections,
            promotion_state,
            validation_findings: findings,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Whether the recall qualifies for the Stable claim with no findings.
    pub fn is_clean_stable(&self) -> bool {
        self.promotion_state == DocsPackRecallPromotionState::Stable
            && self.validation_findings.is_empty()
    }

    /// Wraps the packet in a support-export envelope.
    pub fn support_export(
        &self,
        export_id: &str,
        exported_at: &str,
    ) -> DocsPackRecallSupportExport {
        DocsPackRecallSupportExport {
            record_kind: DOCS_PACK_RECALL_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DOCS_PACK_RECALL_SCHEMA_VERSION,
            export_id: export_id.to_owned(),
            exported_at: exported_at.to_owned(),
            schema_ref: DOCS_PACK_RECALL_SCHEMA_REF.to_owned(),
            doc_ref: DOCS_PACK_RECALL_DOC_REF.to_owned(),
            packet: self.clone(),
        }
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("docs pack recall packet serializes")
    }

    /// Deterministic Markdown summary for docs, support, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Mirrored Docs-Pack Recall\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Recall: {}\n", self.recall_label));
        out.push_str(&format!(
            "- Promotion: `{}` ({} findings)\n",
            self.promotion_state.as_str(),
            self.validation_findings.len()
        ));
        out.push_str(&format!(
            "- Rows: {} ({} stale findings)\n",
            self.result_rows.len(),
            self.stale_example_findings.len()
        ));
        out.push_str("\n## Results\n\n");
        for row in &self.result_rows {
            out.push_str(&format!(
                "{}. `{}` — {} / {} / {} / {} / {}\n",
                row.rank,
                row.result_id,
                row.chips.source_class.as_str(),
                row.chips.version_match.as_str(),
                row.chips.freshness.as_str(),
                row.chips.locality.as_str(),
                row.chips.confidence.as_str(),
            ));
            out.push_str(&format!("   - Reason: {}\n", row.ranking_reason));
        }
        if !self.stale_example_findings.is_empty() {
            out.push_str("\n## Stale-example findings\n\n");
            for finding in &self.stale_example_findings {
                out.push_str(&format!(
                    "- `{}` [{}/{}]: {}\n",
                    finding.result_id_ref,
                    finding.finding_class.as_str(),
                    finding.severity.as_str(),
                    finding.summary,
                ));
            }
        }
        out
    }
}

/// Support-export envelope for the recall packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackRecallSupportExport {
    /// Record kind; must equal [`DOCS_PACK_RECALL_SUPPORT_EXPORT_RECORD_KIND`].
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
    pub packet: DocsPackRecallPacket,
}

/// Errors emitted when reading the checked-in recall support export.
#[derive(Debug)]
pub enum DocsPackRecallArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Re-materialization disagreed with the checked-in promotion state.
    PromotionDrift {
        /// Promotion state recorded in the export.
        recorded: DocsPackRecallPromotionState,
        /// Promotion state computed by re-materialization.
        computed: DocsPackRecallPromotionState,
    },
    /// The checked-in packet should be clean Stable but is not.
    NotCleanStable(Vec<DocsPackRecallValidationFinding>),
}

impl fmt::Display for DocsPackRecallArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "docs pack recall export parse failed: {error}")
            }
            Self::PromotionDrift { recorded, computed } => write!(
                formatter,
                "docs pack recall promotion drift: recorded {} but computed {}",
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
                    "docs pack recall export is not clean stable: {tokens}"
                )
            }
        }
    }
}

impl Error for DocsPackRecallArtifactError {}

/// Reads and re-validates the checked-in stable recall support export.
pub fn current_stable_docs_pack_recall_export(
) -> Result<DocsPackRecallSupportExport, DocsPackRecallArtifactError> {
    let export: DocsPackRecallSupportExport = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/implement_mirrored_docs_pack_recall_source_or_version_or_freshness_chips_and_stale_example_findings/support_export.json"
    )))
    .map_err(DocsPackRecallArtifactError::SupportExport)?;

    // Re-materialize from the recorded packet's fields and confirm the recorded
    // promotion state and findings agree with a fresh computation.
    let recomputed = DocsPackRecallPacket::materialize(packet_to_input(&export.packet));
    if recomputed.promotion_state != export.packet.promotion_state {
        return Err(DocsPackRecallArtifactError::PromotionDrift {
            recorded: export.packet.promotion_state,
            computed: recomputed.promotion_state,
        });
    }
    if !export.packet.is_clean_stable() {
        return Err(DocsPackRecallArtifactError::NotCleanStable(
            export.packet.validation_findings.clone(),
        ));
    }
    Ok(export)
}

/// Rebuilds the materialization input from a packet (used for re-validation).
pub fn packet_to_input(packet: &DocsPackRecallPacket) -> DocsPackRecallPacketInput {
    DocsPackRecallPacketInput {
        packet_id: packet.packet_id.clone(),
        recall_label: packet.recall_label.clone(),
        query_digest_ref: packet.query_digest_ref.clone(),
        mirror_awareness: packet.mirror_awareness,
        result_rows: packet.result_rows.clone(),
        stale_example_findings: packet.stale_example_findings.clone(),
        consumer_projections: packet.consumer_projections.clone(),
        redaction_class_token: packet.redaction_class_token.clone(),
        minted_at: packet.minted_at.clone(),
    }
}

fn push_finding(
    findings: &mut Vec<DocsPackRecallValidationFinding>,
    kind: DocsPackRecallFindingKind,
    summary: impl Into<String>,
) {
    findings.push(DocsPackRecallValidationFinding {
        finding_kind: kind,
        severity: kind.default_severity(),
        summary: summary.into(),
    });
}

fn check_identity(
    input: &DocsPackRecallPacketInput,
    findings: &mut Vec<DocsPackRecallValidationFinding>,
) {
    if input.packet_id.trim().is_empty()
        || input.recall_label.trim().is_empty()
        || input.query_digest_ref.trim().is_empty()
        || input.redaction_class_token.trim().is_empty()
        || input.minted_at.trim().is_empty()
    {
        push_finding(
            findings,
            DocsPackRecallFindingKind::MissingIdentity,
            "packet identity fields must all be present",
        );
    }
}

fn check_result_rows(
    input: &DocsPackRecallPacketInput,
    findings: &mut Vec<DocsPackRecallValidationFinding>,
) {
    if input.result_rows.is_empty() {
        push_finding(
            findings,
            DocsPackRecallFindingKind::ResultRowsEmpty,
            "recall returned no rows",
        );
        return;
    }

    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    let mut mirrored_present = false;
    for (index, row) in input.result_rows.iter().enumerate() {
        let expected_rank = (index as u32) + 1;
        if row.rank != expected_rank {
            push_finding(
                findings,
                DocsPackRecallFindingKind::ResultRankNotMonotonic,
                format!(
                    "row `{}` has rank {} but expected {}",
                    row.result_id, row.rank, expected_rank
                ),
            );
        }
        if !seen_ids.insert(row.result_id.as_str()) {
            push_finding(
                findings,
                DocsPackRecallFindingKind::DuplicateResultId,
                format!("duplicate result id `{}`", row.result_id),
            );
        }
        if row.ranking_reason.trim().is_empty() {
            push_finding(
                findings,
                DocsPackRecallFindingKind::RankingReasonMissing,
                format!("row `{}` is missing a ranking reason", row.result_id),
            );
        }
        if row.open_raw_escape_ref.trim().is_empty() || row.open_source_escape_ref.trim().is_empty()
        {
            push_finding(
                findings,
                DocsPackRecallFindingKind::OpenRawOpenSourceEscapeMissing,
                format!(
                    "row `{}` must keep open-raw and open-source escapes",
                    row.result_id
                ),
            );
        }

        if row.chips.source_class.is_mirrored_upstream() {
            mirrored_present = true;
            if row.chips.freshness.is_authoritative_live()
                && (!row.pack_pinned || !row.pack_signed_and_verified)
            {
                push_finding(
                    findings,
                    DocsPackRecallFindingKind::LiveMirrorLooksMoreAuthoritative,
                    format!(
                        "mirrored row `{}` claims live authority without a pinned, verified pack",
                        row.result_id
                    ),
                );
            }
        }

        if !row.chips.version_match.is_confident_current()
            && row.chips.confidence == DocsPackRecallConfidence::High
            && row.chips.freshness.is_authoritative_live()
        {
            push_finding(
                findings,
                DocsPackRecallFindingKind::VersionTruthCollapsed,
                format!(
                    "row `{}` presents version `{}` as a confident live match",
                    row.result_id,
                    row.chips.version_match.as_str()
                ),
            );
        }
    }

    if !mirrored_present {
        push_finding(
            findings,
            DocsPackRecallFindingKind::MirroredSourceCoverageMissing,
            "a mirror-aware recall must surface at least one mirrored_official_docs row",
        );
    }
}

fn check_mirror_awareness(
    input: &DocsPackRecallPacketInput,
    findings: &mut Vec<DocsPackRecallValidationFinding>,
) {
    if !input.mirror_awareness.is_complete() {
        push_finding(
            findings,
            DocsPackRecallFindingKind::MirrorAwarenessIncomplete,
            "every mirror-awareness invariant must hold",
        );
    }
}

fn check_stale_findings(
    input: &DocsPackRecallPacketInput,
    findings: &mut Vec<DocsPackRecallValidationFinding>,
) {
    let row_ids: BTreeSet<&str> = input
        .result_rows
        .iter()
        .map(|row| row.result_id.as_str())
        .collect();

    for finding in &input.stale_example_findings {
        if finding.summary.trim().is_empty()
            || finding.result_id_ref.trim().is_empty()
            || finding.pack_id_ref.trim().is_empty()
            || (finding.finding_class == DocsPackRecallStaleFindingClass::NearbyVersion
                && finding
                    .nearby_version_ref
                    .as_deref()
                    .map(str::trim)
                    .map(str::is_empty)
                    .unwrap_or(true))
        {
            push_finding(
                findings,
                DocsPackRecallFindingKind::StaleFindingIncomplete,
                format!(
                    "stale finding `{}` for `{}` is incomplete",
                    finding.finding_class.as_str(),
                    finding.result_id_ref
                ),
            );
        }
        if !finding.result_id_ref.trim().is_empty()
            && !row_ids.contains(finding.result_id_ref.as_str())
        {
            push_finding(
                findings,
                DocsPackRecallFindingKind::StaleFindingOrphan,
                format!(
                    "stale finding references unknown result `{}`",
                    finding.result_id_ref
                ),
            );
        }
    }
}

fn check_consumer_projections(
    input: &DocsPackRecallPacketInput,
    findings: &mut Vec<DocsPackRecallValidationFinding>,
) {
    let present: BTreeSet<DocsPackRecallConsumerSurface> = input
        .consumer_projections
        .iter()
        .map(|projection| projection.surface)
        .collect();
    for required in REQUIRED_SURFACES {
        if !present.contains(&required) {
            push_finding(
                findings,
                DocsPackRecallFindingKind::RequiredSurfaceCoverageMissing,
                format!("required surface `{}` is missing", required.as_str()),
            );
        }
    }

    for projection in &input.consumer_projections {
        if projection.packet_id_ref != input.packet_id {
            push_finding(
                findings,
                DocsPackRecallFindingKind::ConsumerProjectionPacketIdMismatch,
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
                DocsPackRecallFindingKind::ConsumerProjectionDrift,
                format!(
                    "surface `{}` drops a required preservation flag",
                    projection.surface.as_str()
                ),
            );
        }
    }
}

fn check_boundary(
    input: &DocsPackRecallPacketInput,
    findings: &mut Vec<DocsPackRecallValidationFinding>,
) {
    let value = serde_json::to_value(input).expect("docs pack recall input serializes");
    if json_contains_forbidden_boundary_material(&value) {
        push_finding(
            findings,
            DocsPackRecallFindingKind::RawBoundaryMaterialPresent,
            "export must not carry raw query text, raw bodies, or secrets",
        );
    }
}

/// Computes the promotion state from the worst severity across both the
/// validation findings and the attached stale-example findings.
///
/// A blocking finding (integrity, trust, or boundary violation) blocks the
/// Stable claim; an otherwise-clean recall that carries a narrowing
/// stale-example finding narrows below Stable rather than hiding the result.
fn promotion_state(
    findings: &[DocsPackRecallValidationFinding],
    stale_findings: &[DocsPackRecallStaleFinding],
) -> DocsPackRecallPromotionState {
    let any_blocking = findings
        .iter()
        .any(|finding| finding.severity == DocsPackRecallFindingSeverity::Blocking)
        || stale_findings
            .iter()
            .any(|finding| finding.severity == DocsPackRecallFindingSeverity::Blocking);
    if any_blocking {
        return DocsPackRecallPromotionState::BlocksStable;
    }

    let any_narrowing = findings
        .iter()
        .any(|finding| finding.severity == DocsPackRecallFindingSeverity::Narrowing)
        || stale_findings
            .iter()
            .any(|finding| finding.severity == DocsPackRecallFindingSeverity::Narrowing);
    if any_narrowing {
        DocsPackRecallPromotionState::NarrowedBelowStable
    } else {
        DocsPackRecallPromotionState::Stable
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
pub fn seeded_stable_docs_pack_recall_input() -> DocsPackRecallPacketInput {
    let packet_id = "packet:m5:docs_pack_recall:async_runtime_setup".to_owned();
    DocsPackRecallPacketInput {
        packet_id: packet_id.clone(),
        recall_label: "docs recall: async runtime setup".to_owned(),
        query_digest_ref: "querydigest:sha256:async-runtime-setup".to_owned(),
        mirror_awareness: DocsPackRecallMirrorAwareness {
            pinned_signed_mirror_outranks_live: true,
            live_vendor_docs_demoted_when_unpinned: true,
            mirror_signature_verified: true,
        },
        result_rows: vec![
            DocsPackRecallResultRow {
                rank: 1,
                result_id: "result:project_docs:runtime_overview".to_owned(),
                doc_node_ref: "docnode:project-docs:runtime/overview".to_owned(),
                pack_id_ref: "pack:project-docs:aureline-workspace".to_owned(),
                pack_pinned: true,
                pack_signed_and_verified: true,
                chips: DocsPackRecallChipSet {
                    source_class: DocsPackRecallSourceClass::ProjectDocs,
                    version_match: DocsPackRecallVersionMatch::ExactBuildMatch,
                    freshness: DocsPackRecallFreshness::AuthoritativeLive,
                    locality: DocsPackRecallLocality::Local,
                    confidence: DocsPackRecallConfidence::High,
                },
                ranking_reason: "exact build match on local project docs with strong lexical+semantic overlap".to_owned(),
                open_raw_escape_ref: "open-raw:docnode:project-docs:runtime/overview".to_owned(),
                open_source_escape_ref: "open-source:repo:docs/runtime/overview.md".to_owned(),
            },
            DocsPackRecallResultRow {
                rank: 2,
                result_id: "result:mirrored:tokio_runtime_guide".to_owned(),
                doc_node_ref: "docnode:mirror:tokio/runtime-guide".to_owned(),
                pack_id_ref: "pack:mirrored-official:tokio".to_owned(),
                pack_pinned: true,
                pack_signed_and_verified: true,
                chips: DocsPackRecallChipSet {
                    source_class: DocsPackRecallSourceClass::MirroredOfficialDocs,
                    version_match: DocsPackRecallVersionMatch::CompatibleMinorDrift,
                    freshness: DocsPackRecallFreshness::WarmCached,
                    locality: DocsPackRecallLocality::MirroredPack,
                    confidence: DocsPackRecallConfidence::High,
                },
                ranking_reason: "pinned, signed mirror of official docs within the compatible drift window".to_owned(),
                open_raw_escape_ref: "open-raw:docnode:mirror:tokio/runtime-guide".to_owned(),
                open_source_escape_ref: "open-source:mirror:tokio/runtime-guide".to_owned(),
            },
            DocsPackRecallResultRow {
                rank: 3,
                result_id: "result:curated:async_patterns".to_owned(),
                doc_node_ref: "docnode:knowledge-pack:async-patterns".to_owned(),
                pack_id_ref: "pack:curated:async-patterns".to_owned(),
                pack_pinned: true,
                pack_signed_and_verified: true,
                chips: DocsPackRecallChipSet {
                    source_class: DocsPackRecallSourceClass::CuratedKnowledgePack,
                    version_match: DocsPackRecallVersionMatch::UnknownTargetBuild,
                    freshness: DocsPackRecallFreshness::DegradedCached,
                    locality: DocsPackRecallLocality::MirroredPack,
                    confidence: DocsPackRecallConfidence::Medium,
                },
                ranking_reason: "curated knowledge pack match; target build unknown so version is disclosed, not assumed".to_owned(),
                open_raw_escape_ref: "open-raw:docnode:knowledge-pack:async-patterns".to_owned(),
                open_source_escape_ref: "open-source:pack:curated:async-patterns".to_owned(),
            },
        ],
        stale_example_findings: vec![
            DocsPackRecallStaleFinding {
                finding_class: DocsPackRecallStaleFindingClass::NearbyVersion,
                severity: DocsPackRecallFindingSeverity::Advisory,
                result_id_ref: "result:mirrored:tokio_runtime_guide".to_owned(),
                pack_id_ref: "pack:mirrored-official:tokio".to_owned(),
                summary: "a nearer-version example exists for the active build".to_owned(),
                nearby_version_ref: Some("docnode:mirror:tokio/runtime-guide@active".to_owned()),
                evidence_ref: Some("evidence:snippet-freshness-ledger:tokio".to_owned()),
            },
            DocsPackRecallStaleFinding {
                finding_class: DocsPackRecallStaleFindingClass::StaleExample,
                severity: DocsPackRecallFindingSeverity::Advisory,
                result_id_ref: "result:curated:async_patterns".to_owned(),
                pack_id_ref: "pack:curated:async-patterns".to_owned(),
                summary: "the example predates the current API and is flagged for review".to_owned(),
                nearby_version_ref: None,
                evidence_ref: Some("evidence:stale-example-audit:async-patterns".to_owned()),
            },
        ],
        consumer_projections: required_projections(&packet_id),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-08T00:00:00Z".to_owned(),
    }
}

fn required_projections(packet_id: &str) -> Vec<DocsPackRecallConsumerProjection> {
    [
        DocsPackRecallConsumerSurface::DocsBrowser,
        DocsPackRecallConsumerSurface::SearchShell,
        DocsPackRecallConsumerSurface::AiContext,
        DocsPackRecallConsumerSurface::RetrievalInspector,
        DocsPackRecallConsumerSurface::CliHeadless,
        DocsPackRecallConsumerSurface::SupportExport,
        DocsPackRecallConsumerSurface::HelpAbout,
    ]
    .into_iter()
    .map(|surface| DocsPackRecallConsumerProjection {
        surface,
        packet_id_ref: packet_id.to_owned(),
        preserves_chips: true,
        preserves_stale_findings: true,
        preserves_ranking_reason: true,
        preserves_open_raw_open_source_escape: true,
        preserves_mirror_awareness: true,
    })
    .collect()
}
