//! Stable support-export parity, query-session/search-export, retrieval-debug,
//! and operator-truth inspector truth packet shared by search, graph, and docs
//! support-export surfaces.
//!
//! This module is the search/graph/docs-owned contract for the M4 stable lane
//! that pins how the support-export surfaces of the knowledge plane stay in
//! parity with the live product. Every row carries a closed `lane_class`,
//! `export_packet_class`, `redaction_class`, `live_vs_captured_class`,
//! `downgrade_state`, and `confidence_class` plus a `query_session_id_ref`,
//! `count_summary`, `evidence_refs`, and an explicit `disclosure_ref` so the
//! search shell, graph topology, docs/help, AI context, review workspace,
//! CLI/headless inspector, support export, and release proof index all read
//! the same support-export truth instead of reinventing it locally.
//!
//! The packet is intentionally metadata-only — it never admits raw query
//! text, raw source bodies, secrets, ambient credentials, or provider
//! payloads. Search-export rows MUST default to redaction classes that omit
//! raw query material unless `explicit_literal_consent` is recorded.
//! Operator-truth inspector rows MUST carry an `operator_reconstruction_proof`
//! so a reviewer can explain why a row existed, what was hidden, and how the
//! counts were approximated. Query-session export rows MUST carry a
//! `deep_link_scope_binding` that preserves search intent plus scope metadata
//! (never frozen certainty) and forces recipient re-resolution.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`SupportExportParityTruthPacket`].
pub const SUPPORT_EXPORT_PARITY_TRUTH_PACKET_RECORD_KIND: &str =
    "support_export_parity_truth_stable_packet";

/// Stable record-kind tag for [`SupportExportParityTruthSupportExport`].
pub const SUPPORT_EXPORT_PARITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "support_export_parity_truth_support_export";

/// Integer schema version for stable support-export parity truth packets.
pub const SUPPORT_EXPORT_PARITY_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const SUPPORT_EXPORT_PARITY_TRUTH_SCHEMA_REF: &str =
    "schemas/search/support_export_parity_truth.schema.json";

/// Repo-relative path of the reviewer doc.
pub const SUPPORT_EXPORT_PARITY_TRUTH_DOC_REF: &str =
    "docs/search/m4/ship-search-graph-and-docs-support-export-parity.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const SUPPORT_EXPORT_PARITY_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/search/m4/ship-search-graph-and-docs-support-export-parity.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const SUPPORT_EXPORT_PARITY_TRUTH_FIXTURE_DIR: &str =
    "fixtures/search/m4/support_export_parity_truth_packet";

/// Repo-relative path of the checked-in stable support-export parity packet.
pub const SUPPORT_EXPORT_PARITY_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/search/m4/support_export_parity_truth_packet.json";

/// Closed lane-class vocabulary the packet certifies. Every required lane
/// MUST have at least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneClass {
    /// Search-export snapshot lane (search collection snapshot to support).
    SearchExport,
    /// Graph topology export lane (graph view exported as support evidence).
    GraphTopologyExport,
    /// Docs/help handoff packet lane.
    DocsHandoffExport,
    /// Operator-truth inspector reconstruction lane.
    OperatorTruthInspector,
    /// Retrieval inspector debug packet lane.
    RetrievalDebug,
    /// Query-session export lane (deep link plus scope metadata).
    QuerySessionExport,
}

impl LaneClass {
    /// Every required lane class, in declaration order.
    pub const REQUIRED: [Self; 6] = [
        Self::SearchExport,
        Self::GraphTopologyExport,
        Self::DocsHandoffExport,
        Self::OperatorTruthInspector,
        Self::RetrievalDebug,
        Self::QuerySessionExport,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchExport => "search_export",
            Self::GraphTopologyExport => "graph_topology_export",
            Self::DocsHandoffExport => "docs_handoff_export",
            Self::OperatorTruthInspector => "operator_truth_inspector",
            Self::RetrievalDebug => "retrieval_debug",
            Self::QuerySessionExport => "query_session_export",
        }
    }
}

/// Closed export-packet-class vocabulary attached to a parity row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportPacketClass {
    /// `search_collection_snapshot` family (search-export snapshot).
    SearchCollectionSnapshot,
    /// Graph topology export packet.
    GraphTopologySnapshot,
    /// Docs handoff packet.
    DocsHandoffPacket,
    /// Operator-truth packet (operator-truth inspector reconstruction).
    OperatorTruthPacket,
    /// Retrieval inspector packet.
    RetrievalInspectorPacket,
    /// Query-session packet (intent + scope binding only).
    QuerySessionPacket,
}

impl ExportPacketClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchCollectionSnapshot => "search_collection_snapshot",
            Self::GraphTopologySnapshot => "graph_topology_snapshot",
            Self::DocsHandoffPacket => "docs_handoff_packet",
            Self::OperatorTruthPacket => "operator_truth_packet",
            Self::RetrievalInspectorPacket => "retrieval_inspector_packet",
            Self::QuerySessionPacket => "query_session_packet",
        }
    }

    /// True when this packet class is the natural carrier of the given lane.
    fn matches_lane(self, lane: LaneClass) -> bool {
        matches!(
            (lane, self),
            (LaneClass::SearchExport, Self::SearchCollectionSnapshot)
                | (LaneClass::GraphTopologyExport, Self::GraphTopologySnapshot)
                | (LaneClass::DocsHandoffExport, Self::DocsHandoffPacket)
                | (LaneClass::OperatorTruthInspector, Self::OperatorTruthPacket)
                | (LaneClass::RetrievalDebug, Self::RetrievalInspectorPacket)
                | (LaneClass::QuerySessionExport, Self::QuerySessionPacket)
        )
    }
}

/// Closed redaction-class vocabulary for the row's export posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    /// Raw literal query/result material — local-only, MUST NOT leave the device.
    LiteralLocalOnly,
    /// Default support-export profile: hashes, scope summaries, result refs,
    /// and partiality reasons in place of raw query text.
    HashesScopeAndResultRefs,
    /// Metadata-only profile that excludes even hashed query material.
    MetadataOnlyNoQueryMaterial,
    /// Policy-withheld profile: the row is exported as a withholding stub.
    PolicyWithheld,
    /// Explicit literal consent: the operator opted into literal export.
    ExplicitLiteralConsent,
}

impl RedactionClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiteralLocalOnly => "literal_local_only",
            Self::HashesScopeAndResultRefs => "hashes_scope_and_result_refs",
            Self::MetadataOnlyNoQueryMaterial => "metadata_only_no_query_material",
            Self::PolicyWithheld => "policy_withheld",
            Self::ExplicitLiteralConsent => "explicit_literal_consent",
        }
    }

    /// Redaction classes that are safe to use as the default support-export
    /// posture for a row. `literal_local_only` and `explicit_literal_consent`
    /// MUST NOT be the default; they only apply with an explicit user opt-in.
    pub const fn is_default_export_safe(self) -> bool {
        matches!(
            self,
            Self::HashesScopeAndResultRefs
                | Self::MetadataOnlyNoQueryMaterial
                | Self::PolicyWithheld
        )
    }
}

/// Closed live-vs-captured vocabulary that mirrors the search-export
/// snapshot contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveVsCapturedClass {
    /// Row reflects the live results captured at export time.
    CurrentLiveResults,
    /// Row reflects a captured snapshot that may diverge from live.
    CapturedSnapshot,
    /// Row requires a live rerun to be trusted.
    LiveRerunRequired,
    /// Scope changed since the export was captured.
    ScopeChangedSinceCapture,
    /// Row is empty because the scope changed since capture.
    EmptyBecauseScopeChanged,
}

impl LiveVsCapturedClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentLiveResults => "current_live_results",
            Self::CapturedSnapshot => "captured_snapshot",
            Self::LiveRerunRequired => "live_rerun_required",
            Self::ScopeChangedSinceCapture => "scope_changed_since_capture",
            Self::EmptyBecauseScopeChanged => "empty_because_scope_changed",
        }
    }
}

/// Closed downgrade-state vocabulary for the parity row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeState {
    /// No downgrade; row preserves canonical export truth.
    None,
    /// Query was narrowed/redacted before export.
    NarrowedQueryRedacted,
    /// Result set was truncated; some rows omitted.
    TruncatedResults,
    /// Policy withheld the row's payload from export.
    PolicyWithheld,
    /// Backing provider was unavailable at export time.
    ProviderUnavailable,
    /// Scope changed since the export was captured.
    ScopeChangedSinceCapture,
}

impl DowngradeState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::NarrowedQueryRedacted => "narrowed_query_redacted",
            Self::TruncatedResults => "truncated_results",
            Self::PolicyWithheld => "policy_withheld",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::ScopeChangedSinceCapture => "scope_changed_since_capture",
        }
    }
}

/// Closed confidence-class vocabulary attached to a parity row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceClass {
    /// High confidence — backed by canonical or signed evidence.
    High,
    /// Medium confidence — backed by partial or warming evidence.
    Medium,
    /// Low confidence — thin evidence.
    Low,
    /// Heuristic — labeled explicitly as a guess.
    Heuristic,
}

impl ConfidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
            Self::Heuristic => "heuristic",
        }
    }
}

/// Count summary mirroring the search-export snapshot contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountSummary {
    /// Visible rows in the rendered export.
    pub visible_rows: u32,
    /// Selected rows the operator picked for export.
    pub selected_rows: u32,
    /// Included rows after policy/scope filtering.
    pub included_rows: u32,
    /// Rows omitted from the export (e.g., truncation).
    pub omitted_result_count: u32,
    /// Rows hidden because the current scope no longer covers them.
    pub hidden_by_current_scope_rows: u32,
    /// Rows hidden by policy filters at export time.
    pub hidden_by_policy_rows: u32,
    /// Rows hidden because a remote/managed cache was unavailable.
    pub hidden_by_remote_cache_rows: u32,
    /// True when the count itself is approximate or partial.
    pub count_is_partial: bool,
}

/// Reconstruction proof required on every operator-truth inspector row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorReconstructionProof {
    /// Stable reconstruction id within the inspector.
    pub reconstruction_id: String,
    /// Repo-relative ref explaining why the row existed in the original view.
    pub why_row_existed_ref: String,
    /// Repo-relative ref explaining what was hidden or withheld from the row.
    pub what_was_hidden_ref: String,
    /// Repo-relative ref explaining how counts were approximated.
    pub approximate_count_disclosure_ref: String,
    /// True when the reconstruction excludes raw query text by default.
    pub raw_query_text_excluded: bool,
}

/// Deep-link scope binding required on query-session export rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeepLinkScopeBinding {
    /// Repo-relative ref to the saved deep-link intent.
    pub deep_link_intent_ref: String,
    /// Repo-relative ref to the scope metadata the link reopens against.
    pub scope_metadata_ref: String,
    /// True when the recipient must re-resolve against their own permissions.
    pub requires_recipient_resolution: bool,
    /// True when no frozen certainty about current results is carried.
    pub frozen_certainty_excluded: bool,
}

/// One truth row covered by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportParityRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Lane class this row certifies.
    pub lane_class: LaneClass,
    /// Export packet class carrying the row.
    pub export_packet_class: ExportPacketClass,
    /// Stable packet id within the carrying packet.
    pub packet_id: String,
    /// Workspace identity.
    pub workspace_id: String,
    /// Query-session id ref the row preserves.
    pub query_session_id_ref: String,
    /// Optional planner pass ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub planner_pass_id_ref: Option<String>,
    /// Selected result refs preserved on the row.
    #[serde(default)]
    pub selected_result_refs: Vec<String>,
    /// Included result refs preserved on the row.
    #[serde(default)]
    pub included_result_refs: Vec<String>,
    /// Count summary (visible, selected, included, omitted, hidden).
    pub count_summary: CountSummary,
    /// True when the export omitted at least one row.
    pub omitted_flag: bool,
    /// True when the export truncated the included result set.
    pub truncated_flag: bool,
    /// Default redaction class for this row's export.
    pub redaction_class: RedactionClass,
    /// Evidence refs cited by the row (docs, schemas, packets).
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Live-vs-captured class for the row.
    pub live_vs_captured_class: LiveVsCapturedClass,
    /// Downgrade state for the row.
    pub downgrade_state: DowngradeState,
    /// Confidence class for the row.
    pub confidence_class: ConfidenceClass,
    /// Repo-relative ref to the disclosure shown on the row.
    pub disclosure_ref: String,
    /// Operator-truth reconstruction proof (required for operator-truth rows).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_reconstruction_proof: Option<OperatorReconstructionProof>,
    /// Deep-link scope binding (required for query-session export rows).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deep_link_scope_binding: Option<DeepLinkScopeBinding>,
    /// True when raw query text is excluded from this row.
    pub raw_query_text_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded from this row.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for this row.
    pub captured_at: String,
}

/// Consumer surface that must inherit this packet's truth verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Search shell results pane.
    SearchShell,
    /// Graph topology canvas / table fallback.
    GraphTopology,
    /// Docs/help surface explaining the export.
    DocsHelp,
    /// AI context assembly surface.
    AiContext,
    /// Review workspace surface.
    ReviewWorkspace,
    /// CLI or headless inspection surface.
    CliHeadless,
    /// Support export bundle surface.
    SupportExport,
    /// Release proof index entry.
    ReleaseProofIndex,
}

impl ConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 8] = [
        Self::SearchShell,
        Self::GraphTopology,
        Self::DocsHelp,
        Self::AiContext,
        Self::ReviewWorkspace,
        Self::CliHeadless,
        Self::SupportExport,
        Self::ReleaseProofIndex,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchShell => "search_shell",
            Self::GraphTopology => "graph_topology",
            Self::DocsHelp => "docs_help",
            Self::AiContext => "ai_context",
            Self::ReviewWorkspace => "review_workspace",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::ReleaseProofIndex => "release_proof_index",
        }
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportParityConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Packet id consumed by the projection.
    pub support_export_parity_packet_id_ref: String,
    /// Rendered-at timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the surface preserves the lane vocabulary.
    pub preserves_lane_vocabulary: bool,
    /// True when the surface preserves the export-packet-class vocabulary.
    pub preserves_export_packet_class_vocabulary: bool,
    /// True when the surface preserves the redaction vocabulary.
    pub preserves_redaction_vocabulary: bool,
    /// True when the surface preserves the live-vs-captured vocabulary.
    pub preserves_live_vs_captured_vocabulary: bool,
    /// True when the surface preserves the downgrade-state vocabulary.
    pub preserves_downgrade_vocabulary: bool,
    /// True when the surface preserves query-session/result/count refs.
    pub preserves_query_session_refs: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl SupportExportParityConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.support_export_parity_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_export_packet_class_vocabulary
            && self.preserves_redaction_vocabulary
            && self.preserves_live_vs_captured_vocabulary
            && self.preserves_downgrade_vocabulary
            && self.preserves_query_session_refs
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Closed promotion state for [`SupportExportParityTruthPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionState {
    /// Packet certifies a stable claim for every declared lane row.
    Stable,
    /// Packet must remain narrowed below stable until a recorded gap closes.
    NarrowedBelowStable,
    /// Packet has a blocker finding and cannot publish on stable surfaces.
    BlocksStable,
}

impl PromotionState {
    /// Stable token used in fixtures, schemas, and support exports.
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
pub enum FindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the row below stable.
    Warning,
    /// Blocker that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required lane row is missing from the packet.
    MissingLaneCoverage,
    /// Row is missing its query-session ref.
    MissingQuerySessionRef,
    /// Row's evidence-refs array is empty.
    MissingEvidenceRefs,
    /// Row's disclosure ref is empty.
    MissingDisclosureRef,
    /// Row's count summary is internally inconsistent.
    InvalidCountSummary,
    /// Operator-truth inspector row is missing its reconstruction proof.
    OperatorTruthMissingReconstruction,
    /// Query-session export row is missing its deep-link scope binding.
    QuerySessionMissingDeepLinkBinding,
    /// Deep-link binding drops scope metadata.
    DeepLinkDropsScopeMetadata,
    /// Deep-link binding freezes recipient certainty (must not).
    DeepLinkFreezesRecipientCertainty,
    /// Row admits raw query text past the boundary.
    RawQueryTextPresent,
    /// Row admits secrets past the boundary.
    SecretsPresent,
    /// Row admits ambient authority/credentials past the boundary.
    AmbientAuthorityPresent,
    /// Row's default redaction class is too permissive for support export.
    DefaultExportTooPermissive,
    /// Row's export packet class does not carry the row's lane class.
    ExportPacketClassLaneMismatch,
    /// A required consumer projection is missing.
    MissingConsumerProjection,
    /// A consumer projection drops part of the closed vocabulary.
    ConsumerProjectionDrift,
    /// A consumer projection collapses the lane vocabulary.
    LaneVocabularyCollapsed,
    /// A consumer projection collapses the export-packet-class vocabulary.
    ExportPacketClassVocabularyCollapsed,
    /// A consumer projection collapses the redaction vocabulary.
    RedactionVocabularyCollapsed,
    /// A consumer projection collapses the live-vs-captured vocabulary.
    LiveVsCapturedVocabularyCollapsed,
    /// A consumer projection collapses the downgrade vocabulary.
    DowngradeVocabularyCollapsed,
    /// A consumer projection drops query-session/result/count refs.
    QuerySessionRefsDropped,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl FindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingLaneCoverage => "missing_lane_coverage",
            Self::MissingQuerySessionRef => "missing_query_session_ref",
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::MissingDisclosureRef => "missing_disclosure_ref",
            Self::InvalidCountSummary => "invalid_count_summary",
            Self::OperatorTruthMissingReconstruction => {
                "operator_truth_missing_reconstruction"
            }
            Self::QuerySessionMissingDeepLinkBinding => {
                "query_session_missing_deep_link_binding"
            }
            Self::DeepLinkDropsScopeMetadata => "deep_link_drops_scope_metadata",
            Self::DeepLinkFreezesRecipientCertainty => {
                "deep_link_freezes_recipient_certainty"
            }
            Self::RawQueryTextPresent => "raw_query_text_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::DefaultExportTooPermissive => "default_export_too_permissive",
            Self::ExportPacketClassLaneMismatch => "export_packet_class_lane_mismatch",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::ExportPacketClassVocabularyCollapsed => {
                "export_packet_class_vocabulary_collapsed"
            }
            Self::RedactionVocabularyCollapsed => "redaction_vocabulary_collapsed",
            Self::LiveVsCapturedVocabularyCollapsed => {
                "live_vs_captured_vocabulary_collapsed"
            }
            Self::DowngradeVocabularyCollapsed => "downgrade_vocabulary_collapsed",
            Self::QuerySessionRefsDropped => "query_session_refs_dropped",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    /// Closed finding kind.
    pub finding_kind: FindingKind,
    /// Finding severity.
    pub severity: FindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl ValidationFinding {
    fn new(
        finding_kind: FindingKind,
        severity: FindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// Constructor input for [`SupportExportParityTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportParityTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet as a whole.
    pub generated_at: String,
    /// Lane classes the packet covers.
    #[serde(default)]
    pub covered_lane_classes: Vec<LaneClass>,
    /// Truth rows.
    #[serde(default)]
    pub rows: Vec<SupportExportParityRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<SupportExportParityConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Search/graph/docs-owned packet for support-export parity, query-session/
/// search-export, retrieval-debug, and operator-truth inspector truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportParityTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Packet capture timestamp.
    pub generated_at: String,
    /// Lane classes the packet covers.
    #[serde(default)]
    pub covered_lane_classes: Vec<LaneClass>,
    /// Truth rows.
    #[serde(default)]
    pub rows: Vec<SupportExportParityRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<SupportExportParityConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl SupportExportParityTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: SupportExportParityTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: SUPPORT_EXPORT_PARITY_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: SUPPORT_EXPORT_PARITY_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_lane_classes: input.covered_lane_classes,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: PromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against stable support-export parity invariants.
    pub fn validate(&self) -> Vec<ValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: ConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns the unique lane-class tokens observed across rows.
    pub fn lane_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.lane_class);
        }
        set.into_iter().map(LaneClass::as_str).collect()
    }

    /// Returns the unique export-packet-class tokens observed across rows.
    pub fn export_packet_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.export_packet_class);
        }
        set.into_iter().map(ExportPacketClass::as_str).collect()
    }

    /// Returns the unique redaction-class tokens observed across rows.
    pub fn redaction_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.redaction_class);
        }
        set.into_iter().map(RedactionClass::as_str).collect()
    }

    /// Returns the unique live-vs-captured tokens observed across rows.
    pub fn live_vs_captured_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.live_vs_captured_class);
        }
        set.into_iter().map(LiveVsCapturedClass::as_str).collect()
    }

    /// Returns the unique downgrade-state tokens observed across rows.
    pub fn downgrade_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.downgrade_state);
        }
        set.into_iter().map(DowngradeState::as_str).collect()
    }

    /// Builds a support export wrapping the exact packet shown to product surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> SupportExportParityTruthSupportExport {
        SupportExportParityTruthSupportExport {
            record_kind: SUPPORT_EXPORT_PARITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SUPPORT_EXPORT_PARITY_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            support_export_parity_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            support_export_parity_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != SUPPORT_EXPORT_PARITY_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "support-export parity packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != SUPPORT_EXPORT_PARITY_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "support-export parity packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(ValidationFinding::new(
                FindingKind::MissingIdentity,
                FindingSeverity::Blocker,
                "packet, workflow, and timestamp refs are required",
            ));
        }

        for required in LaneClass::REQUIRED {
            let in_coverage = self.covered_lane_classes.contains(&required);
            let in_rows = self.rows.iter().any(|row| row.lane_class == required);
            if !in_coverage || !in_rows {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers required lane class {}", required.as_str()),
                ));
            }
        }

        for row in &self.rows {
            if row.row_id.trim().is_empty()
                || row.workspace_id.trim().is_empty()
                || row.packet_id.trim().is_empty()
                || row.captured_at.trim().is_empty()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingIdentity,
                    FindingSeverity::Blocker,
                    format!("row {} drops a required identity field", row.row_id),
                ));
            }
            if row.query_session_id_ref.trim().is_empty() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingQuerySessionRef,
                    FindingSeverity::Blocker,
                    format!("row {} drops its query-session ref", row.row_id),
                ));
            }
            if row.disclosure_ref.trim().is_empty() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!("row {} drops its disclosure ref", row.row_id),
                ));
            }
            if row.evidence_refs.is_empty()
                || row.evidence_refs.iter().any(|reference| reference.trim().is_empty())
            {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingEvidenceRefs,
                    FindingSeverity::Blocker,
                    format!("row {} drops evidence refs", row.row_id),
                ));
            }
            if !count_summary_is_internally_consistent(&row.count_summary, row.omitted_flag) {
                findings.push(ValidationFinding::new(
                    FindingKind::InvalidCountSummary,
                    FindingSeverity::Blocker,
                    format!("row {} count summary is internally inconsistent", row.row_id),
                ));
            }
            if !row.export_packet_class.matches_lane(row.lane_class) {
                findings.push(ValidationFinding::new(
                    FindingKind::ExportPacketClassLaneMismatch,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} lane {} disagrees with export packet class {}",
                        row.row_id,
                        row.lane_class.as_str(),
                        row.export_packet_class.as_str()
                    ),
                ));
            }
            if !row.redaction_class.is_default_export_safe() {
                findings.push(ValidationFinding::new(
                    FindingKind::DefaultExportTooPermissive,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} default redaction {} is not export-safe",
                        row.row_id,
                        row.redaction_class.as_str()
                    ),
                ));
            }
            if !row.raw_query_text_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::RawQueryTextPresent,
                    FindingSeverity::Blocker,
                    format!("row {} admits raw query text", row.row_id),
                ));
            }
            if !row.secrets_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::SecretsPresent,
                    FindingSeverity::Blocker,
                    format!("row {} admits secrets", row.row_id),
                ));
            }
            if !row.ambient_authority_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::AmbientAuthorityPresent,
                    FindingSeverity::Blocker,
                    format!("row {} admits ambient authority/credentials", row.row_id),
                ));
            }

            match row.lane_class {
                LaneClass::OperatorTruthInspector => {
                    let proof_ok = row.operator_reconstruction_proof.as_ref().is_some_and(|proof| {
                        !proof.reconstruction_id.trim().is_empty()
                            && !proof.why_row_existed_ref.trim().is_empty()
                            && !proof.what_was_hidden_ref.trim().is_empty()
                            && !proof.approximate_count_disclosure_ref.trim().is_empty()
                            && proof.raw_query_text_excluded
                    });
                    if !proof_ok {
                        findings.push(ValidationFinding::new(
                            FindingKind::OperatorTruthMissingReconstruction,
                            FindingSeverity::Blocker,
                            format!(
                                "row {} is operator_truth_inspector but missing reconstruction proof",
                                row.row_id
                            ),
                        ));
                    }
                }
                LaneClass::QuerySessionExport => match row.deep_link_scope_binding.as_ref() {
                    None => {
                        findings.push(ValidationFinding::new(
                            FindingKind::QuerySessionMissingDeepLinkBinding,
                            FindingSeverity::Blocker,
                            format!(
                                "row {} is query_session_export but missing deep-link binding",
                                row.row_id
                            ),
                        ));
                    }
                    Some(binding) => {
                        if binding.deep_link_intent_ref.trim().is_empty()
                            || binding.scope_metadata_ref.trim().is_empty()
                        {
                            findings.push(ValidationFinding::new(
                                FindingKind::DeepLinkDropsScopeMetadata,
                                FindingSeverity::Blocker,
                                format!(
                                    "row {} deep-link binding drops intent or scope metadata",
                                    row.row_id
                                ),
                            ));
                        }
                        if !binding.requires_recipient_resolution
                            || !binding.frozen_certainty_excluded
                        {
                            findings.push(ValidationFinding::new(
                                FindingKind::DeepLinkFreezesRecipientCertainty,
                                FindingSeverity::Blocker,
                                format!(
                                    "row {} deep-link binding freezes recipient certainty",
                                    row.row_id
                                ),
                            ));
                        }
                    }
                },
                _ => {}
            }
        }

        for required_surface in ConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingConsumerProjection,
                    FindingSeverity::Blocker,
                    format!(
                        "packet {} is missing a preserved {} projection",
                        self.packet_id,
                        required_surface.as_str()
                    ),
                ));
            }
        }
        for projection in &self.consumer_projections {
            if !projection.preserves_truth_for(&self.packet_id) {
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerProjectionDrift,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve support-export parity truth",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_lane_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::LaneVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the lane vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_export_packet_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::ExportPacketClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the export-packet-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_redaction_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::RedactionVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} drops the redaction vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_live_vs_captured_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::LiveVsCapturedVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} drops the live-vs-captured vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_downgrade_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::DowngradeVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} drops the downgrade-state vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_query_session_refs {
                findings.push(ValidationFinding::new(
                    FindingKind::QuerySessionRefsDropped,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} drops query-session/result/count refs",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion
                .retain(|finding| finding.finding_kind != FindingKind::PromotionStateMismatch);
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(ValidationFinding::new(
                    FindingKind::PromotionStateMismatch,
                    FindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }
}

fn count_summary_is_internally_consistent(summary: &CountSummary, omitted_flag: bool) -> bool {
    if summary.selected_rows > summary.visible_rows {
        return false;
    }
    if summary.included_rows > summary.visible_rows + summary.omitted_result_count {
        return false;
    }
    if omitted_flag && summary.omitted_result_count == 0 {
        return false;
    }
    if !omitted_flag && summary.omitted_result_count > 0 {
        return false;
    }
    true
}

fn promotion_state_for_findings(findings: &[ValidationFinding]) -> PromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Blocker)
    {
        PromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Warning)
    {
        PromotionState::NarrowedBelowStable
    } else {
        PromotionState::Stable
    }
}

/// Support-export wrapper that preserves the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportParityTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub support_export_parity_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub support_export_parity_packet: SupportExportParityTruthPacket,
}

impl SupportExportParityTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == SUPPORT_EXPORT_PARITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == SUPPORT_EXPORT_PARITY_TRUTH_SCHEMA_VERSION
            && self.support_export_parity_packet_id_ref
                == self.support_export_parity_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.support_export_parity_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable packet.
#[derive(Debug)]
pub enum SupportExportParityTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for SupportExportParityTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "support-export parity truth packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "support-export parity truth packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for SupportExportParityTruthArtifactError {}

/// Returns the checked-in stable support-export parity truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_support_export_parity_truth_packet(
) -> Result<SupportExportParityTruthPacket, SupportExportParityTruthArtifactError> {
    let packet: SupportExportParityTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/search/m4/support_export_parity_truth_packet.json"
    )))
    .map_err(SupportExportParityTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(SupportExportParityTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_count_summary() -> CountSummary {
        CountSummary {
            visible_rows: 12,
            selected_rows: 4,
            included_rows: 12,
            omitted_result_count: 0,
            hidden_by_current_scope_rows: 0,
            hidden_by_policy_rows: 0,
            hidden_by_remote_cache_rows: 0,
            count_is_partial: false,
        }
    }

    fn sample_row(lane: LaneClass, packet_class: ExportPacketClass) -> SupportExportParityRow {
        SupportExportParityRow {
            row_id: format!("row:{}", lane.as_str()),
            lane_class: lane,
            export_packet_class: packet_class,
            packet_id: format!("packet:{}", packet_class.as_str()),
            workspace_id: "workspace:m4:parity".to_owned(),
            query_session_id_ref: "session:m4:parity:baseline".to_owned(),
            planner_pass_id_ref: Some("planner:m4:parity:baseline".to_owned()),
            selected_result_refs: vec!["result:m4:parity:1".to_owned()],
            included_result_refs: vec![
                "result:m4:parity:1".to_owned(),
                "result:m4:parity:2".to_owned(),
            ],
            count_summary: sample_count_summary(),
            omitted_flag: false,
            truncated_flag: false,
            redaction_class: RedactionClass::HashesScopeAndResultRefs,
            evidence_refs: vec![SUPPORT_EXPORT_PARITY_TRUTH_DOC_REF.to_owned()],
            live_vs_captured_class: LiveVsCapturedClass::CurrentLiveResults,
            downgrade_state: DowngradeState::None,
            confidence_class: ConfidenceClass::High,
            disclosure_ref: SUPPORT_EXPORT_PARITY_TRUTH_DOC_REF.to_owned(),
            operator_reconstruction_proof: if matches!(lane, LaneClass::OperatorTruthInspector) {
                Some(OperatorReconstructionProof {
                    reconstruction_id: "reconstruction:m4:parity:baseline".to_owned(),
                    why_row_existed_ref: "evidence:m4:parity:why".to_owned(),
                    what_was_hidden_ref: "evidence:m4:parity:hidden".to_owned(),
                    approximate_count_disclosure_ref: "evidence:m4:parity:counts".to_owned(),
                    raw_query_text_excluded: true,
                })
            } else {
                None
            },
            deep_link_scope_binding: if matches!(lane, LaneClass::QuerySessionExport) {
                Some(DeepLinkScopeBinding {
                    deep_link_intent_ref: "deep_link:m4:parity:intent".to_owned(),
                    scope_metadata_ref: "scope:m4:parity:metadata".to_owned(),
                    requires_recipient_resolution: true,
                    frozen_certainty_excluded: true,
                })
            } else {
                None
            },
            raw_query_text_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn sample_projection(surface: ConsumerSurface) -> SupportExportParityConsumerProjection {
        SupportExportParityConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            support_export_parity_packet_id_ref: "packet:m4:support_export_parity:baseline"
                .to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_export_packet_class_vocabulary: true,
            preserves_redaction_vocabulary: true,
            preserves_live_vs_captured_vocabulary: true,
            preserves_downgrade_vocabulary: true,
            preserves_query_session_refs: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn baseline_input() -> SupportExportParityTruthPacketInput {
        SupportExportParityTruthPacketInput {
            packet_id: "packet:m4:support_export_parity:baseline".to_owned(),
            workflow_or_surface_id: "workflow.search_graph_docs.support_export_parity"
                .to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_lane_classes: LaneClass::REQUIRED.to_vec(),
            rows: vec![
                sample_row(LaneClass::SearchExport, ExportPacketClass::SearchCollectionSnapshot),
                sample_row(
                    LaneClass::GraphTopologyExport,
                    ExportPacketClass::GraphTopologySnapshot,
                ),
                sample_row(LaneClass::DocsHandoffExport, ExportPacketClass::DocsHandoffPacket),
                sample_row(
                    LaneClass::OperatorTruthInspector,
                    ExportPacketClass::OperatorTruthPacket,
                ),
                sample_row(LaneClass::RetrievalDebug, ExportPacketClass::RetrievalInspectorPacket),
                sample_row(LaneClass::QuerySessionExport, ExportPacketClass::QuerySessionPacket),
            ],
            consumer_projections: ConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(sample_projection)
                .collect(),
            source_contract_refs: vec![SUPPORT_EXPORT_PARITY_TRUTH_DOC_REF.to_owned()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(LaneClass::SearchExport.as_str(), "search_export");
        assert_eq!(
            LaneClass::OperatorTruthInspector.as_str(),
            "operator_truth_inspector"
        );
        assert_eq!(
            ExportPacketClass::OperatorTruthPacket.as_str(),
            "operator_truth_packet"
        );
        assert_eq!(
            RedactionClass::HashesScopeAndResultRefs.as_str(),
            "hashes_scope_and_result_refs"
        );
        assert_eq!(
            LiveVsCapturedClass::ScopeChangedSinceCapture.as_str(),
            "scope_changed_since_capture"
        );
        assert_eq!(DowngradeState::PolicyWithheld.as_str(), "policy_withheld");
        assert_eq!(
            FindingKind::OperatorTruthMissingReconstruction.as_str(),
            "operator_truth_missing_reconstruction"
        );
        assert_eq!(PromotionState::BlocksStable.as_str(), "blocks_stable");
    }

    #[test]
    fn baseline_input_materializes_stable() {
        let packet = SupportExportParityTruthPacket::materialize(baseline_input());
        assert_eq!(packet.promotion_state, PromotionState::Stable);
        assert!(packet.validation_findings.is_empty());
        assert!(packet.is_stable());
    }

    #[test]
    fn raw_query_text_blocks_stable() {
        let mut input = baseline_input();
        input.rows[0].raw_query_text_excluded = false;
        let packet = SupportExportParityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RawQueryTextPresent));
    }

    #[test]
    fn permissive_default_redaction_blocks_stable() {
        let mut input = baseline_input();
        input.rows[0].redaction_class = RedactionClass::LiteralLocalOnly;
        let packet = SupportExportParityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::DefaultExportTooPermissive
        }));
    }

    #[test]
    fn operator_truth_row_without_reconstruction_blocks_stable() {
        let mut input = baseline_input();
        let operator_row = input
            .rows
            .iter_mut()
            .find(|row| row.lane_class == LaneClass::OperatorTruthInspector)
            .expect("operator truth row");
        operator_row.operator_reconstruction_proof = None;
        let packet = SupportExportParityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::OperatorTruthMissingReconstruction
        }));
    }

    #[test]
    fn deep_link_freezing_certainty_blocks_stable() {
        let mut input = baseline_input();
        let session_row = input
            .rows
            .iter_mut()
            .find(|row| row.lane_class == LaneClass::QuerySessionExport)
            .expect("query session row");
        let binding = session_row
            .deep_link_scope_binding
            .as_mut()
            .expect("deep link binding");
        binding.requires_recipient_resolution = false;
        binding.frozen_certainty_excluded = false;
        let packet = SupportExportParityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::DeepLinkFreezesRecipientCertainty
        }));
    }

    #[test]
    fn missing_lane_coverage_blocks_stable() {
        let mut input = baseline_input();
        input.rows.retain(|row| row.lane_class != LaneClass::RetrievalDebug);
        input
            .covered_lane_classes
            .retain(|lane| *lane != LaneClass::RetrievalDebug);
        let packet = SupportExportParityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingLaneCoverage));
    }

    #[test]
    fn missing_consumer_projection_blocks_stable() {
        let mut input = baseline_input();
        input.consumer_projections = vec![sample_projection(ConsumerSurface::SearchShell)];
        let packet = SupportExportParityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn projection_drops_redaction_blocks_stable() {
        let mut input = baseline_input();
        input.consumer_projections = ConsumerSurface::REQUIRED
            .iter()
            .copied()
            .map(|surface| {
                let mut projection = sample_projection(surface);
                if surface == ConsumerSurface::DocsHelp {
                    projection.preserves_redaction_vocabulary = false;
                }
                projection
            })
            .collect();
        let packet = SupportExportParityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::RedactionVocabularyCollapsed
        }));
    }

    #[test]
    fn support_export_is_export_safe_when_packet_is_stable() {
        let packet = SupportExportParityTruthPacket::materialize(baseline_input());
        let export = packet.support_export("export:test", "2026-05-26T12:00:10Z");
        assert!(export.is_export_safe());
    }
}
