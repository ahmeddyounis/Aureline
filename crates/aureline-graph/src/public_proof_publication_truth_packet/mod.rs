//! Stable public-proof publication truth packet for the M4 stable lane.
//!
//! This module is the graph-owned contract that pins how search, graph, and
//! docs public-proof publication stays a single boundary truth across the
//! search shell, graph topology, docs/help, CLI/headless inspector, support
//! export, release proof index, Help/About, and the published-truth
//! dashboard. Every row binds a closed `publication_lane_class`,
//! `publication_row_class`, `publication_state`, `known_limit_class`,
//! `downgrade_automation_class`, `proof_artifact_class`, and
//! `publication_confidence_class` plus an `evidence_refs` array and a
//! `disclosure_ref` whenever the row is narrowed below stable, declares a
//! non-`none_declared` known limit, or binds a non-`none` downgrade
//! automation.
//!
//! The packet is intentionally metadata-only — it never admits raw query
//! text, raw source bodies, secrets, ambient credentials, or provider
//! payloads. A row that claims `published_stable` while leaving its known
//! limit or downgrade automation unbound is refused; the validator narrows
//! below stable instead of inheriting an adjacent published row, and the
//! downgrade-automation vocabulary is preserved verbatim so a stale or
//! partially qualified lane cannot masquerade as published stable.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`PublicProofPublicationTruthPacket`].
pub const PUBLIC_PROOF_PUBLICATION_TRUTH_PACKET_RECORD_KIND: &str =
    "public_proof_publication_truth_stable_packet";

/// Stable record-kind tag for [`PublicProofPublicationTruthSupportExport`].
pub const PUBLIC_PROOF_PUBLICATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "public_proof_publication_truth_support_export";

/// Integer schema version for the public-proof publication packet.
pub const PUBLIC_PROOF_PUBLICATION_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const PUBLIC_PROOF_PUBLICATION_TRUTH_SCHEMA_REF: &str =
    "schemas/search/public_proof_publication_truth.schema.json";

/// Repo-relative path of the reviewer doc.
pub const PUBLIC_PROOF_PUBLICATION_TRUTH_DOC_REF: &str =
    "docs/search/m4/finish-search-graph-and-docs-public-proof-publication.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const PUBLIC_PROOF_PUBLICATION_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/search/m4/finish-search-graph-and-docs-public-proof-publication.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const PUBLIC_PROOF_PUBLICATION_TRUTH_FIXTURE_DIR: &str =
    "fixtures/search/m4/public_proof_publication_truth_packet";

/// Repo-relative path of the checked-in stable packet.
pub const PUBLIC_PROOF_PUBLICATION_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/search/m4/public_proof_publication_truth_packet.json";

/// Closed publication-lane vocabulary. Every required lane MUST have at
/// least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationLaneClass {
    /// Search public-proof publication lane.
    SearchPublicProofLane,
    /// Graph public-proof publication lane.
    GraphPublicProofLane,
    /// Docs public-proof publication lane.
    DocsPublicProofLane,
}

impl PublicationLaneClass {
    /// Every required publication lane, in declaration order.
    pub const REQUIRED: [Self; 3] = [
        Self::SearchPublicProofLane,
        Self::GraphPublicProofLane,
        Self::DocsPublicProofLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchPublicProofLane => "search_public_proof_lane",
            Self::GraphPublicProofLane => "graph_public_proof_lane",
            Self::DocsPublicProofLane => "docs_public_proof_lane",
        }
    }
}

/// Closed publication-row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationRowClass {
    /// A search public-truth row.
    SearchPublicTruth,
    /// A graph public-truth row.
    GraphPublicTruth,
    /// A docs public-truth row.
    DocsPublicTruth,
    /// A known-limit disclosure row attached to a publication lane.
    KnownLimit,
    /// A downgrade-automation rule row attached to a publication lane.
    DowngradeAutomation,
}

impl PublicationRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchPublicTruth => "search_public_truth",
            Self::GraphPublicTruth => "graph_public_truth",
            Self::DocsPublicTruth => "docs_public_truth",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    /// True when this row class is permitted on the given publication lane.
    pub const fn is_permitted_on(self, lane: PublicationLaneClass) -> bool {
        matches!(
            (self, lane),
            (
                Self::SearchPublicTruth,
                PublicationLaneClass::SearchPublicProofLane
            ) | (
                Self::GraphPublicTruth,
                PublicationLaneClass::GraphPublicProofLane
            ) | (
                Self::DocsPublicTruth,
                PublicationLaneClass::DocsPublicProofLane
            ) | (Self::KnownLimit, _)
                | (Self::DowngradeAutomation, _)
        )
    }
}

/// Closed publication-state vocabulary applied to a row. A row is never
/// `published_stable` while its known limit or downgrade automation is
/// unbound; the validator demotes it instead of inheriting an adjacent
/// stable row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationState {
    /// Row carries a published-stable public proof.
    PublishedStable,
    /// Row is intentionally narrowed below stable; the narrowing is disclosed.
    NarrowedBelowStable,
    /// Row is withheld pending a recorded gap; publication holds.
    WithheldPendingGap,
}

impl PublicationState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublishedStable => "published_stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::WithheldPendingGap => "withheld_pending_gap",
        }
    }

    /// True when the publication state must surface a disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::PublishedStable)
    }
}

/// Closed known-limit vocabulary attached to a publication row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The publication is limited to a partially warmed corpus.
    CorpusWarmupPartial,
    /// The publication only certifies a scope subset (e.g., current repo).
    ScopeSubsetOnly,
    /// The publication only certifies imported-provider claims.
    ImportedProviderOnly,
    /// The publication only certifies an archived snapshot.
    ArchivedSnapshotOnly,
    /// The publication is heuristic only (labeled as a guess).
    HeuristicOnly,
    /// The publication is a feature preview only.
    FeaturePreviewOnly,
    /// The publication has no bound known limit class; this never qualifies stable.
    LimitUnbound,
}

impl KnownLimitClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneDeclared => "none_declared",
            Self::CorpusWarmupPartial => "corpus_warmup_partial",
            Self::ScopeSubsetOnly => "scope_subset_only",
            Self::ImportedProviderOnly => "imported_provider_only",
            Self::ArchivedSnapshotOnly => "archived_snapshot_only",
            Self::HeuristicOnly => "heuristic_only",
            Self::FeaturePreviewOnly => "feature_preview_only",
            Self::LimitUnbound => "limit_unbound",
        }
    }

    /// True when this known-limit class satisfies the limit-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::LimitUnbound)
    }

    /// True when this known-limit class must surface an explicit disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::NoneDeclared | Self::LimitUnbound)
    }
}

/// Closed downgrade-automation vocabulary attached to a publication row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeAutomationClass {
    /// No downgrade automation is required for the row.
    None,
    /// Automatically narrow below stable while the index is partially warm.
    AutoNarrowOnPartialWarm,
    /// Automatically withhold publication when the backing provider is unavailable.
    AutoWithholdOnProviderOutage,
    /// Automatically demote to archived snapshot when capture is stale.
    AutoArchiveOnStaleCapture,
    /// Automatically demote below stable when confidence is low.
    AutoDemoteOnLowConfidence,
    /// Automatically block publication when evidence is missing.
    AutoBlockOnMissingEvidence,
    /// Manual-only review required until automation lands.
    ManualOnlyPendingReview,
    /// Automation is unbound; this never qualifies stable.
    AutomationUnbound,
}

impl DowngradeAutomationClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::AutoNarrowOnPartialWarm => "auto_narrow_on_partial_warm",
            Self::AutoWithholdOnProviderOutage => "auto_withhold_on_provider_outage",
            Self::AutoArchiveOnStaleCapture => "auto_archive_on_stale_capture",
            Self::AutoDemoteOnLowConfidence => "auto_demote_on_low_confidence",
            Self::AutoBlockOnMissingEvidence => "auto_block_on_missing_evidence",
            Self::ManualOnlyPendingReview => "manual_only_pending_review",
            Self::AutomationUnbound => "automation_unbound",
        }
    }

    /// True when this automation class satisfies the automation-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::AutomationUnbound)
    }

    /// True when this automation class must surface an explicit disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::None | Self::AutomationUnbound)
    }
}

/// Closed proof-artifact vocabulary describing where a row's public proof
/// is published.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofArtifactClass {
    /// Row publishes to the release proof index entry.
    ReleaseProofIndexEntry,
    /// Row publishes through the support-export packet bundle.
    SupportExportPacket,
    /// Row publishes through the Help/About proof card surface.
    HelpAboutProofCard,
    /// Row publishes through the docs-published truth card surface.
    DocsPublishedTruthCard,
    /// Row publishes through the published-truth dashboard row.
    DashboardPublishedTruthRow,
    /// Row publishes through the CLI/headless inspector row.
    CliHeadlessInspectorRow,
}

impl ProofArtifactClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseProofIndexEntry => "release_proof_index_entry",
            Self::SupportExportPacket => "support_export_packet",
            Self::HelpAboutProofCard => "help_about_proof_card",
            Self::DocsPublishedTruthCard => "docs_published_truth_card",
            Self::DashboardPublishedTruthRow => "dashboard_published_truth_row",
            Self::CliHeadlessInspectorRow => "cli_headless_inspector_row",
        }
    }
}

/// Closed confidence-class vocabulary for a publication row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationConfidenceClass {
    /// High confidence — the publication can certify stable.
    HighConfidence,
    /// Medium confidence — the publication narrows below stable.
    MediumConfidence,
    /// Low confidence — the publication narrows below stable until evidence grows.
    LowConfidence,
}

impl PublicationConfidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HighConfidence => "high_confidence",
            Self::MediumConfidence => "medium_confidence",
            Self::LowConfidence => "low_confidence",
        }
    }
}

/// Stable promotion state derived from packet validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionState {
    /// Packet certifies a stable claim across all required rows.
    Stable,
    /// Packet narrows below stable until a recorded gap closes.
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
    /// Reviewable finding that narrows the packet below stable.
    Warning,
    /// Blocker finding that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for the public-proof publication packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required publication lane has no row.
    MissingPublicationLaneCoverage,
    /// A row's row class is not permitted on its lane.
    RowClassNotPermittedOnLane,
    /// A row has no bound known-limit class.
    MissingKnownLimit,
    /// A row has no bound downgrade-automation class.
    MissingDowngradeAutomation,
    /// A row claims published_stable while one or more bindings is unbound.
    PublishedStableWithUnboundBinding,
    /// A row narrowed below stable or withheld drops its disclosure ref.
    NarrowedRowMissingDisclosureRef,
    /// A row with a non-`none_declared` known limit drops its disclosure ref.
    KnownLimitMissingDisclosureRef,
    /// A row with a non-`none` downgrade automation drops its disclosure ref.
    DowngradeAutomationMissingDisclosureRef,
    /// A row carries no evidence refs.
    MissingEvidenceRefs,
    /// A row admits raw query text, raw source bodies, or other private material.
    RawQueryMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority/credentials past the boundary.
    AmbientAuthorityPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops public-proof truth.
    ConsumerProjectionDrift,
    /// A projection collapses the lane vocabulary.
    LaneVocabularyCollapsed,
    /// A projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A projection collapses the publication-state vocabulary.
    PublicationStateVocabularyCollapsed,
    /// A projection collapses the known-limit vocabulary.
    KnownLimitVocabularyCollapsed,
    /// A projection collapses the downgrade-automation vocabulary.
    DowngradeAutomationVocabularyCollapsed,
    /// A projection collapses the proof-artifact vocabulary.
    ProofArtifactVocabularyCollapsed,
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
            Self::MissingPublicationLaneCoverage => "missing_publication_lane_coverage",
            Self::RowClassNotPermittedOnLane => "row_class_not_permitted_on_lane",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::PublishedStableWithUnboundBinding => "published_stable_with_unbound_binding",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeAutomationMissingDisclosureRef => {
                "downgrade_automation_missing_disclosure_ref"
            }
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::RawQueryMaterialPresent => "raw_query_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::PublicationStateVocabularyCollapsed => "publication_state_vocabulary_collapsed",
            Self::KnownLimitVocabularyCollapsed => "known_limit_vocabulary_collapsed",
            Self::DowngradeAutomationVocabularyCollapsed => {
                "downgrade_automation_vocabulary_collapsed"
            }
            Self::ProofArtifactVocabularyCollapsed => "proof_artifact_vocabulary_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the public-proof publication packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Search shell results pane.
    SearchShell,
    /// Graph topology canvas / table fallback.
    GraphTopology,
    /// Docs/help reviewer surface.
    DocsHelp,
    /// CLI or headless inspection surface.
    CliHeadless,
    /// Support export bundle surface.
    SupportExport,
    /// Release proof index entry.
    ReleaseProofIndex,
    /// Help/About proof card surface.
    HelpAbout,
    /// Published-truth dashboard surface.
    DashboardPublishedTruth,
}

impl ConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 8] = [
        Self::SearchShell,
        Self::GraphTopology,
        Self::DocsHelp,
        Self::CliHeadless,
        Self::SupportExport,
        Self::ReleaseProofIndex,
        Self::HelpAbout,
        Self::DashboardPublishedTruth,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchShell => "search_shell",
            Self::GraphTopology => "graph_topology",
            Self::DocsHelp => "docs_help",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::ReleaseProofIndex => "release_proof_index",
            Self::HelpAbout => "help_about",
            Self::DashboardPublishedTruth => "dashboard_published_truth",
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

/// One public-proof publication row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicProofPublicationRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Publication lane class this row certifies.
    pub lane_class: PublicationLaneClass,
    /// Publication row class.
    pub row_class: PublicationRowClass,
    /// Publication state claimed by the row.
    pub publication_state: PublicationState,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Proof-artifact class through which the row is published.
    pub proof_artifact_class: ProofArtifactClass,
    /// Confidence class for the row.
    pub confidence_class: PublicationConfidenceClass,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional disclosure ref required whenever the row is not `published_stable`,
    /// declares a non-`none_declared` known limit, or binds a non-`none` automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// True when raw query text and source bodies are excluded from this row.
    pub raw_query_material_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded from this row.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl PublicProofPublicationRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.known_limit_class.is_bound() && self.downgrade_automation_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicProofPublicationConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Public-proof publication packet id consumed by the projection.
    pub public_proof_publication_packet_id_ref: String,
    /// Rendered-at timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the lane vocabulary is preserved verbatim.
    pub preserves_lane_vocabulary: bool,
    /// True when the row-class vocabulary is preserved verbatim.
    pub preserves_row_class_vocabulary: bool,
    /// True when the publication-state vocabulary is preserved verbatim.
    pub preserves_publication_state_vocabulary: bool,
    /// True when the known-limit vocabulary is preserved verbatim.
    pub preserves_known_limit_vocabulary: bool,
    /// True when the downgrade-automation vocabulary is preserved verbatim.
    pub preserves_downgrade_automation_vocabulary: bool,
    /// True when the proof-artifact vocabulary is preserved verbatim.
    pub preserves_proof_artifact_vocabulary: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl PublicProofPublicationConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.public_proof_publication_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_publication_state_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_proof_artifact_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`PublicProofPublicationTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicProofPublicationTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Publication lane classes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<PublicationLaneClass>,
    /// Publication rows.
    #[serde(default)]
    pub rows: Vec<PublicProofPublicationRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<PublicProofPublicationConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Graph-owned packet certifying search, graph, and docs public-proof
/// publication for the M4 stable lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicProofPublicationTruthPacket {
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
    /// Publication lane classes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<PublicationLaneClass>,
    /// Publication rows.
    #[serde(default)]
    pub rows: Vec<PublicProofPublicationRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<PublicProofPublicationConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl PublicProofPublicationTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: PublicProofPublicationTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: PUBLIC_PROOF_PUBLICATION_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: PUBLIC_PROOF_PUBLICATION_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_lanes: input.covered_lanes,
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

    /// Re-validates the packet against stable public-proof publication invariants.
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

    /// Returns the unique lane tokens observed across rows.
    pub fn lane_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.lane_class);
        }
        set.into_iter().map(PublicationLaneClass::as_str).collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter().map(PublicationRowClass::as_str).collect()
    }

    /// Returns the unique publication-state tokens observed across rows.
    pub fn publication_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.publication_state);
        }
        set.into_iter().map(PublicationState::as_str).collect()
    }

    /// Returns the unique known-limit tokens observed across rows.
    pub fn known_limit_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.known_limit_class);
        }
        set.into_iter().map(KnownLimitClass::as_str).collect()
    }

    /// Returns the unique downgrade-automation tokens observed across rows.
    pub fn downgrade_automation_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.downgrade_automation_class);
        }
        set.into_iter()
            .map(DowngradeAutomationClass::as_str)
            .collect()
    }

    /// Returns the unique proof-artifact tokens observed across rows.
    pub fn proof_artifact_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.proof_artifact_class);
        }
        set.into_iter().map(ProofArtifactClass::as_str).collect()
    }

    /// Builds a support export wrapping the exact packet shown to product surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> PublicProofPublicationTruthSupportExport {
        PublicProofPublicationTruthSupportExport {
            record_kind: PUBLIC_PROOF_PUBLICATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PUBLIC_PROOF_PUBLICATION_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            public_proof_publication_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            public_proof_publication_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != PUBLIC_PROOF_PUBLICATION_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "public-proof publication packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != PUBLIC_PROOF_PUBLICATION_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "public-proof publication packet has the wrong schema version",
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
        if self.covered_lanes.is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingPublicationLaneCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered publication lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingPublicationLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers publication lane {}", lane.as_str()),
                ));
            }
        }

        for row in &self.rows {
            if row.row_id.trim().is_empty() || row.captured_at.trim().is_empty() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingIdentity,
                    FindingSeverity::Blocker,
                    format!("row {} identity or timestamp is empty", row.row_id),
                ));
            }
            if !row.raw_query_material_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::RawQueryMaterialPresent,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} admits raw query text or source bodies past the boundary",
                        row.row_id
                    ),
                ));
            }
            if !row.secrets_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::SecretsPresent,
                    FindingSeverity::Blocker,
                    format!("row {} admits secrets past the boundary", row.row_id),
                ));
            }
            if !row.ambient_authority_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::AmbientAuthorityPresent,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} admits ambient authority/credentials past the boundary",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.is_permitted_on(row.lane_class) {
                findings.push(ValidationFinding::new(
                    FindingKind::RowClassNotPermittedOnLane,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} not permitted on lane {}",
                        row.row_id,
                        row.row_class.as_str(),
                        row.lane_class.as_str()
                    ),
                ));
            }

            if !row.known_limit_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingKnownLimit,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound known-limit class", row.row_id),
                ));
            }
            if !row.downgrade_automation_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingDowngradeAutomation,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound downgrade-automation class", row.row_id),
                ));
            }

            if matches!(row.publication_state, PublicationState::PublishedStable)
                && !row.all_bindings_satisfied()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::PublishedStableWithUnboundBinding,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} claims published_stable while known limit or downgrade automation is unbound",
                        row.row_id
                    ),
                ));
            }

            if row.publication_state.requires_explicit_disclosure() && row.disclosure_ref.is_none()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::NarrowedRowMissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has publication state {} without a disclosure ref",
                        row.row_id,
                        row.publication_state.as_str()
                    ),
                ));
            }
            if row.known_limit_class.requires_explicit_disclosure() && row.disclosure_ref.is_none()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::KnownLimitMissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} discloses known limit {} without a disclosure ref",
                        row.row_id,
                        row.known_limit_class.as_str()
                    ),
                ));
            }
            if row
                .downgrade_automation_class
                .requires_explicit_disclosure()
                && row.disclosure_ref.is_none()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::DowngradeAutomationMissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} binds downgrade automation {} without a disclosure ref",
                        row.row_id,
                        row.downgrade_automation_class.as_str()
                    ),
                ));
            }

            if row.evidence_refs.is_empty() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingEvidenceRefs,
                    FindingSeverity::Blocker,
                    format!("row {} carries no evidence refs", row.row_id),
                ));
            }

            if matches!(
                row.confidence_class,
                PublicationConfidenceClass::LowConfidence
            ) && matches!(row.publication_state, PublicationState::PublishedStable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::PublishedStableWithUnboundBinding,
                    FindingSeverity::Warning,
                    format!(
                        "row {} claims published_stable at low_confidence; narrowing until evidence grows",
                        row.row_id
                    ),
                ));
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
                        "projection {} does not preserve public-proof publication truth",
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
            if !projection.preserves_row_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::RowClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the row-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_publication_state_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::PublicationStateVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the publication-state vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_known_limit_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::KnownLimitVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the known-limit vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_downgrade_automation_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::DowngradeAutomationVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the downgrade-automation vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_proof_artifact_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::ProofArtifactVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the proof-artifact vocabulary",
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
pub struct PublicProofPublicationTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub public_proof_publication_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub public_proof_publication_packet: PublicProofPublicationTruthPacket,
}

impl PublicProofPublicationTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == PUBLIC_PROOF_PUBLICATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == PUBLIC_PROOF_PUBLICATION_TRUTH_SCHEMA_VERSION
            && self.public_proof_publication_packet_id_ref
                == self.public_proof_publication_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.public_proof_publication_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable public-proof publication packet.
#[derive(Debug)]
pub enum PublicProofPublicationTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for PublicProofPublicationTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "public-proof publication packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "public-proof publication packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for PublicProofPublicationTruthArtifactError {}

/// Returns the checked-in stable public-proof publication truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_public_proof_publication_truth_packet(
) -> Result<PublicProofPublicationTruthPacket, PublicProofPublicationTruthArtifactError> {
    let packet: PublicProofPublicationTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/search/m4/public_proof_publication_truth_packet.json"
    )))
    .map_err(PublicProofPublicationTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(PublicProofPublicationTruthArtifactError::Validation(
            findings,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_truth_row(
        row_id: &str,
        lane: PublicationLaneClass,
        row_class: PublicationRowClass,
    ) -> PublicProofPublicationRow {
        PublicProofPublicationRow {
            row_id: row_id.to_owned(),
            lane_class: lane,
            row_class,
            publication_state: PublicationState::PublishedStable,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnPartialWarm,
            proof_artifact_class: ProofArtifactClass::ReleaseProofIndexEntry,
            confidence_class: PublicationConfidenceClass::HighConfidence,
            evidence_refs: vec![PUBLIC_PROOF_PUBLICATION_TRUTH_DOC_REF.to_owned()],
            disclosure_ref: Some(
                "docs/search/m4/finish-search-graph-and-docs-public-proof-publication.md#auto_narrow_on_partial_warm"
                    .to_owned(),
            ),
            raw_query_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn sample_projection(surface: ConsumerSurface) -> PublicProofPublicationConsumerProjection {
        PublicProofPublicationConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            public_proof_publication_packet_id_ref: "packet:m4:public_proof_publication".to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_publication_state_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_proof_artifact_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn sample_input() -> PublicProofPublicationTruthPacketInput {
        PublicProofPublicationTruthPacketInput {
            packet_id: "packet:m4:public_proof_publication".to_owned(),
            workflow_or_surface_id: "workflow.graph.public_proof_publication".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_lanes: PublicationLaneClass::REQUIRED.to_vec(),
            rows: vec![
                sample_truth_row(
                    "row:search:public_truth",
                    PublicationLaneClass::SearchPublicProofLane,
                    PublicationRowClass::SearchPublicTruth,
                ),
                sample_truth_row(
                    "row:graph:public_truth",
                    PublicationLaneClass::GraphPublicProofLane,
                    PublicationRowClass::GraphPublicTruth,
                ),
                sample_truth_row(
                    "row:docs:public_truth",
                    PublicationLaneClass::DocsPublicProofLane,
                    PublicationRowClass::DocsPublicTruth,
                ),
            ],
            consumer_projections: ConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(sample_projection)
                .collect(),
            source_contract_refs: vec![PUBLIC_PROOF_PUBLICATION_TRUTH_DOC_REF.to_owned()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(
            PublicationLaneClass::DocsPublicProofLane.as_str(),
            "docs_public_proof_lane"
        );
        assert_eq!(
            PublicationRowClass::DowngradeAutomation.as_str(),
            "downgrade_automation"
        );
        assert_eq!(
            PublicationState::WithheldPendingGap.as_str(),
            "withheld_pending_gap"
        );
        assert_eq!(KnownLimitClass::LimitUnbound.as_str(), "limit_unbound");
        assert_eq!(
            DowngradeAutomationClass::AutomationUnbound.as_str(),
            "automation_unbound"
        );
        assert_eq!(
            ProofArtifactClass::DashboardPublishedTruthRow.as_str(),
            "dashboard_published_truth_row"
        );
        assert_eq!(
            ConsumerSurface::DashboardPublishedTruth.as_str(),
            "dashboard_published_truth"
        );
        assert_eq!(PromotionState::BlocksStable.as_str(), "blocks_stable");
        assert_eq!(
            FindingKind::DowngradeAutomationVocabularyCollapsed.as_str(),
            "downgrade_automation_vocabulary_collapsed"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = PublicProofPublicationTruthPacket::materialize(sample_input());
        assert_eq!(packet.promotion_state, PromotionState::Stable);
        assert!(packet.validation_findings.is_empty());
        assert!(packet.is_stable());
        assert!(packet
            .support_export(
                "support:m4:public_proof_publication",
                "2026-05-26T12:00:10Z"
            )
            .is_export_safe());
    }

    #[test]
    fn published_stable_with_unbound_limit_blocks() {
        let mut input = sample_input();
        input.rows[0].known_limit_class = KnownLimitClass::LimitUnbound;
        let packet = PublicProofPublicationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingKnownLimit));
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::PublishedStableWithUnboundBinding));
    }

    #[test]
    fn published_stable_with_unbound_automation_blocks() {
        let mut input = sample_input();
        input.rows[1].downgrade_automation_class = DowngradeAutomationClass::AutomationUnbound;
        let packet = PublicProofPublicationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingDowngradeAutomation));
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::PublishedStableWithUnboundBinding));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        input.rows[0].publication_state = PublicationState::NarrowedBelowStable;
        input.rows[0].disclosure_ref = None;
        let packet = PublicProofPublicationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::NarrowedRowMissingDisclosureRef
        }));
    }

    #[test]
    fn row_class_not_permitted_on_lane_blocks() {
        let mut input = sample_input();
        input.rows[0].row_class = PublicationRowClass::GraphPublicTruth;
        let packet = PublicProofPublicationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RowClassNotPermittedOnLane));
    }

    #[test]
    fn projection_drop_blocks_promotion() {
        let mut input = sample_input();
        input
            .consumer_projections
            .retain(|projection| projection.consumer_surface != ConsumerSurface::HelpAbout);
        let packet = PublicProofPublicationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_downgrade_automation_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == ConsumerSurface::DocsHelp {
                projection.preserves_downgrade_automation_vocabulary = false;
            }
        }
        let packet = PublicProofPublicationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::DowngradeAutomationVocabularyCollapsed
        }));
    }

    #[test]
    fn raw_query_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_query_material_excluded = false;
        let packet = PublicProofPublicationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RawQueryMaterialPresent));
    }

    #[test]
    fn known_limit_with_disclosure_keeps_stable() {
        let mut input = sample_input();
        input.rows[0].known_limit_class = KnownLimitClass::CorpusWarmupPartial;
        input.rows[0].publication_state = PublicationState::NarrowedBelowStable;
        input.rows[0].disclosure_ref = Some(
            "docs/search/m4/finish-search-graph-and-docs-public-proof-publication.md#corpus_warmup_partial"
                .to_owned(),
        );
        let packet = PublicProofPublicationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::Stable);
        assert!(packet.validation_findings.is_empty());
    }
}
