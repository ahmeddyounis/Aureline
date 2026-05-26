//! Stable topology, explainer, and companion-adjacent surface audit truth
//! packet for the M4 stable lane.
//!
//! This module is the graph-owned contract that audits every row presented
//! on the topology canvas, the impact explainer, and the companion-adjacent
//! navigator/filter/export/history surfaces. Each row pins a closed
//! `audit_surface_class`, `audit_row_class`, `qualification_state`,
//! `scope_disclosure_class`, `freshness_disclosure_class`,
//! `provenance_disclosure_class`, `downgrade_state_disclosure_class`, and
//! `confidence_class` plus per-pillar evidence refs and, when narrowed,
//! an explicit `disclosure_ref` so the topology canvas, explainer panel,
//! docs/help, CLI/headless inspector, support export, and release proof
//! index all read one boundary truth instead of reinventing audit posture
//! locally.
//!
//! The packet is intentionally metadata-only — it never admits raw query
//! text, raw source bodies, secrets, ambient credentials, or provider
//! payloads. A row that claims `qualified_stable` while leaving any of the
//! four audit pillars (scope, freshness, provenance, downgrade state)
//! unbound is refused; the validator narrows below stable instead of
//! inheriting adjacent qualified rows.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`AuditTopologyExplainerCompanionTruthPacket`].
pub const AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_PACKET_RECORD_KIND: &str =
    "audit_topology_explainer_companion_truth_stable_packet";

/// Stable record-kind tag for [`AuditTopologyExplainerCompanionTruthSupportExport`].
pub const AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "audit_topology_explainer_companion_truth_support_export";

/// Integer schema version for the audit topology/explainer/companion packet.
pub const AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_SCHEMA_REF: &str =
    "schemas/search/audit_topology_explainer_companion_truth.schema.json";

/// Repo-relative path of the reviewer doc.
pub const AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_DOC_REF: &str =
    "docs/search/m4/audit-topology-explainer-and-companion-adjacent-surfaces-and.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_FIXTURE_DIR: &str =
    "fixtures/search/m4/audit_topology_explainer_companion_truth_packet";

/// Repo-relative path of the checked-in stable packet.
pub const AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/search/m4/audit_topology_explainer_companion_truth_packet.json";

/// Repo-relative path of the human-readable reviewer artifact.
pub const AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/search/m4/audit-topology-explainer-and-companion-adjacent-surfaces-and.md";

/// Closed audit surface-class vocabulary the packet certifies. Every row
/// binds to exactly one surface so consumers cannot quietly mix
/// companion-history evidence into the explainer panel's audit posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditSurfaceClass {
    /// Graph topology canvas.
    TopologyCanvas,
    /// Non-canvas table fallback for the topology view.
    TopologyTable,
    /// Impact explainer panel.
    ImpactExplainer,
    /// Compact evidence card.
    EvidenceCard,
    /// Companion-adjacent navigator (related/adjacent panes).
    CompanionNavigator,
    /// Companion-adjacent filter chip rail.
    CompanionFilter,
    /// Companion-adjacent export sheet.
    CompanionExport,
    /// Companion-adjacent recently-viewed history rail.
    CompanionHistory,
}

impl AuditSurfaceClass {
    /// Every required audit surface, in declaration order.
    pub const REQUIRED: [Self; 8] = [
        Self::TopologyCanvas,
        Self::TopologyTable,
        Self::ImpactExplainer,
        Self::EvidenceCard,
        Self::CompanionNavigator,
        Self::CompanionFilter,
        Self::CompanionExport,
        Self::CompanionHistory,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopologyCanvas => "topology_canvas",
            Self::TopologyTable => "topology_table",
            Self::ImpactExplainer => "impact_explainer",
            Self::EvidenceCard => "evidence_card",
            Self::CompanionNavigator => "companion_navigator",
            Self::CompanionFilter => "companion_filter",
            Self::CompanionExport => "companion_export",
            Self::CompanionHistory => "companion_history",
        }
    }

    /// True when this surface is one of the companion-adjacent surfaces.
    pub const fn is_companion_adjacent(self) -> bool {
        matches!(
            self,
            Self::CompanionNavigator
                | Self::CompanionFilter
                | Self::CompanionExport
                | Self::CompanionHistory
        )
    }
}

/// Closed audit row-class vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditRowClass {
    /// A node on the topology canvas or table.
    TopologyNode,
    /// An edge on the topology canvas or table.
    TopologyEdge,
    /// An impact edge in the explainer panel.
    ImpactEdge,
    /// A row inside an evidence card.
    EvidenceCardRow,
    /// A companion-adjacent action chip (e.g., open related, pivot scope).
    CompanionAction,
    /// A companion-adjacent filter chip.
    CompanionFilterRow,
    /// A companion-adjacent export sheet row.
    CompanionExportRow,
    /// A companion-adjacent history pin/row.
    CompanionHistoryRow,
}

impl AuditRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopologyNode => "topology_node",
            Self::TopologyEdge => "topology_edge",
            Self::ImpactEdge => "impact_edge",
            Self::EvidenceCardRow => "evidence_card_row",
            Self::CompanionAction => "companion_action",
            Self::CompanionFilterRow => "companion_filter_row",
            Self::CompanionExportRow => "companion_export_row",
            Self::CompanionHistoryRow => "companion_history_row",
        }
    }

    /// True when the row class is permitted on the given surface.
    pub const fn is_permitted_on(self, surface: AuditSurfaceClass) -> bool {
        matches!(
            (self, surface),
            (Self::TopologyNode, AuditSurfaceClass::TopologyCanvas)
                | (Self::TopologyNode, AuditSurfaceClass::TopologyTable)
                | (Self::TopologyEdge, AuditSurfaceClass::TopologyCanvas)
                | (Self::TopologyEdge, AuditSurfaceClass::TopologyTable)
                | (Self::ImpactEdge, AuditSurfaceClass::ImpactExplainer)
                | (Self::EvidenceCardRow, AuditSurfaceClass::EvidenceCard)
                | (Self::EvidenceCardRow, AuditSurfaceClass::ImpactExplainer)
                | (Self::CompanionAction, AuditSurfaceClass::CompanionNavigator)
                | (Self::CompanionFilterRow, AuditSurfaceClass::CompanionFilter)
                | (Self::CompanionExportRow, AuditSurfaceClass::CompanionExport)
                | (Self::CompanionHistoryRow, AuditSurfaceClass::CompanionHistory)
        )
    }
}

/// Closed qualification-state vocabulary applied to a row. A row is
/// never `qualified_stable` while any audit pillar is unbound; the
/// validator demotes it to `not_qualified_stable` and emits a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationState {
    /// Row meets every audit pillar and is qualified for the stable lane.
    QualifiedStable,
    /// Row is intentionally narrowed below stable; the narrowing is disclosed.
    NarrowedBelowStable,
    /// Row is missing one or more audit pillars and is not yet qualified for stable.
    NotQualifiedStable,
}

impl QualificationState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QualifiedStable => "qualified_stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::NotQualifiedStable => "not_qualified_stable",
        }
    }

    /// True when the qualification state must remain visibly disclosed.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::QualifiedStable)
    }
}

/// Closed audit-pillar vocabulary. The packet binds per-pillar evidence
/// for each row; any row claiming `qualified_stable` MUST satisfy every
/// pillar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditPillar {
    /// Scope vocabulary (current repo / selected workset / full workspace / remote cache / outside scope).
    Scope,
    /// Freshness vocabulary (live / partially warmed / stale disclosed / archived frozen / imported snapshot).
    Freshness,
    /// Provenance vocabulary (workspace canonical / partial inferred / archived / imported / heuristic).
    Provenance,
    /// Downgrade-state vocabulary (none / narrowed below stable / blocked stable / imported / archived).
    DowngradeState,
}

impl AuditPillar {
    /// Every required audit pillar, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::Scope,
        Self::Freshness,
        Self::Provenance,
        Self::DowngradeState,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Scope => "scope",
            Self::Freshness => "freshness",
            Self::Provenance => "provenance",
            Self::DowngradeState => "downgrade_state",
        }
    }
}

/// Closed scope-disclosure vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeDisclosureClass {
    /// Row is bound to the current repo scope.
    CurrentRepo,
    /// Row is bound to the active selected workset.
    SelectedWorkset,
    /// Row is bound to the full workspace scope.
    FullWorkspace,
    /// Row is bound to a remote-cache scope.
    RemoteCache,
    /// Row is bound to a slice that is outside the active scope and disclosed as such.
    OutsideCurrentScope,
    /// Row has no scope disclosure; this never qualifies stable.
    ScopeUnbound,
}

impl ScopeDisclosureClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentRepo => "current_repo",
            Self::SelectedWorkset => "selected_workset",
            Self::FullWorkspace => "full_workspace",
            Self::RemoteCache => "remote_cache",
            Self::OutsideCurrentScope => "outside_current_scope",
            Self::ScopeUnbound => "scope_unbound",
        }
    }

    /// True when this scope disclosure satisfies the scope audit pillar.
    pub const fn satisfies_pillar(self) -> bool {
        !matches!(self, Self::ScopeUnbound)
    }
}

/// Closed freshness-disclosure vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessDisclosureClass {
    /// Row is current as of the last canonical refresh.
    Live,
    /// Row is partially warmed; not yet at the full canonical set.
    PartiallyWarmed,
    /// Row is stale but its staleness is disclosed.
    StaleDisclosed,
    /// Row is frozen as an archived snapshot.
    ArchivedFrozen,
    /// Row carries the snapshot recorded by an imported provider.
    ImportedSnapshot,
    /// Row has no freshness disclosure; this never qualifies stable.
    FreshnessUnbound,
}

impl FreshnessDisclosureClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::PartiallyWarmed => "partially_warmed",
            Self::StaleDisclosed => "stale_disclosed",
            Self::ArchivedFrozen => "archived_frozen",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::FreshnessUnbound => "freshness_unbound",
        }
    }

    /// True when this freshness disclosure satisfies the freshness audit pillar.
    pub const fn satisfies_pillar(self) -> bool {
        !matches!(self, Self::FreshnessUnbound)
    }
}

/// Closed provenance-disclosure vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceDisclosureClass {
    /// Row is derived from the canonical workspace graph.
    WorkspaceCanonical,
    /// Row is inferred while only a partial index slice is available.
    PartialIndexInferred,
    /// Row is preserved from an archived snapshot.
    ArchivePreserved,
    /// Row is derived from an imported external provider mapping.
    ImportedProviderDerived,
    /// Row is derived from a heuristic (best-effort, not authoritative).
    HeuristicDerived,
    /// Row has no provenance disclosure; this never qualifies stable.
    ProvenanceUnbound,
}

impl ProvenanceDisclosureClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceCanonical => "workspace_canonical",
            Self::PartialIndexInferred => "partial_index_inferred",
            Self::ArchivePreserved => "archive_preserved",
            Self::ImportedProviderDerived => "imported_provider_derived",
            Self::HeuristicDerived => "heuristic_derived",
            Self::ProvenanceUnbound => "provenance_unbound",
        }
    }

    /// True when this provenance disclosure satisfies the provenance audit pillar.
    pub const fn satisfies_pillar(self) -> bool {
        !matches!(self, Self::ProvenanceUnbound)
    }

    /// True when this provenance disclosure must remain visibly disclosed.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::WorkspaceCanonical | Self::ProvenanceUnbound)
    }
}

/// Closed downgrade-state-disclosure vocabulary attached to a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeStateDisclosureClass {
    /// Row has no active downgrade.
    None,
    /// Row is intentionally narrowed below stable.
    NarrowedBelowStable,
    /// Row is blocked from the stable lane until a gap closes.
    BlocksStable,
    /// Row inherits an imported snapshot rather than canonical truth.
    ImportedSnapshot,
    /// Row is frozen as an archived snapshot.
    ArchivedFrozen,
    /// Row has no downgrade disclosure; this never qualifies stable.
    DowngradeStateUnbound,
}

impl DowngradeStateDisclosureClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::ArchivedFrozen => "archived_frozen",
            Self::DowngradeStateUnbound => "downgrade_state_unbound",
        }
    }

    /// True when this downgrade-state disclosure satisfies the downgrade audit pillar.
    pub const fn satisfies_pillar(self) -> bool {
        !matches!(self, Self::DowngradeStateUnbound)
    }

    /// True when this downgrade-state disclosure must surface a disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::None | Self::DowngradeStateUnbound)
    }
}

/// Closed confidence-class vocabulary for a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditConfidenceClass {
    /// High confidence: the audit can certify stable.
    HighConfidence,
    /// Medium confidence: the audit narrows below stable.
    MediumConfidence,
    /// Low confidence: the audit narrows below stable until evidence grows.
    LowConfidence,
}

impl AuditConfidenceClass {
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
pub enum AuditPromotionState {
    /// Packet certifies a stable claim across all required rows.
    Stable,
    /// Packet narrows below stable until a recorded gap closes.
    NarrowedBelowStable,
    /// Packet has a blocker finding and cannot publish on stable surfaces.
    BlocksStable,
}

impl AuditPromotionState {
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
pub enum AuditFindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the packet below stable.
    Warning,
    /// Blocker finding that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for the audit packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditFindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required audit surface has no row.
    MissingAuditSurfaceRow,
    /// A row's row class is not permitted on its surface.
    RowClassNotPermittedOnSurface,
    /// A row has no scope disclosure.
    MissingScopeDisclosure,
    /// A row has no freshness disclosure.
    MissingFreshnessDisclosure,
    /// A row has no provenance disclosure.
    MissingProvenanceDisclosure,
    /// A row has no downgrade-state disclosure.
    MissingDowngradeStateDisclosure,
    /// A row claims qualified_stable while one of the four audit pillars is unbound.
    NonQualifiedRowMasqueradingStable,
    /// A row is narrowed below stable but drops its disclosure ref.
    NarrowedRowMissingDisclosureRef,
    /// A provenance disclosure that requires an explicit ref drops it.
    ProvenanceMissingDisclosureRef,
    /// A downgrade-state disclosure that requires an explicit ref drops it.
    DowngradeMissingDisclosureRef,
    /// Packet admits raw query text, raw source bodies, secrets, or ambient credentials.
    RawQueryMaterialPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops audit truth.
    ConsumerProjectionDrift,
    /// A projection collapses the surface vocabulary.
    SurfaceVocabularyCollapsed,
    /// A projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A projection collapses the qualification-state vocabulary.
    QualificationVocabularyCollapsed,
    /// A projection collapses one of the audit-pillar vocabularies.
    AuditPillarCollapsed,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl AuditFindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingAuditSurfaceRow => "missing_audit_surface_row",
            Self::RowClassNotPermittedOnSurface => "row_class_not_permitted_on_surface",
            Self::MissingScopeDisclosure => "missing_scope_disclosure",
            Self::MissingFreshnessDisclosure => "missing_freshness_disclosure",
            Self::MissingProvenanceDisclosure => "missing_provenance_disclosure",
            Self::MissingDowngradeStateDisclosure => "missing_downgrade_state_disclosure",
            Self::NonQualifiedRowMasqueradingStable => "non_qualified_row_masquerading_stable",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::ProvenanceMissingDisclosureRef => "provenance_missing_disclosure_ref",
            Self::DowngradeMissingDisclosureRef => "downgrade_missing_disclosure_ref",
            Self::RawQueryMaterialPresent => "raw_query_material_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::SurfaceVocabularyCollapsed => "surface_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::QualificationVocabularyCollapsed => "qualification_vocabulary_collapsed",
            Self::AuditPillarCollapsed => "audit_pillar_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the audit packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditConsumerSurface {
    /// Topology canvas and table fallback.
    TopologyCanvas,
    /// Impact explainer panel and evidence cards.
    ExplainerPanel,
    /// Docs/help reviewer surface.
    DocsHelp,
    /// CLI or headless inspection surface.
    CliHeadless,
    /// Support export bundle.
    SupportExport,
    /// Release proof index entry.
    ReleaseProofIndex,
}

impl AuditConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 6] = [
        Self::TopologyCanvas,
        Self::ExplainerPanel,
        Self::DocsHelp,
        Self::CliHeadless,
        Self::SupportExport,
        Self::ReleaseProofIndex,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopologyCanvas => "topology_canvas",
            Self::ExplainerPanel => "explainer_panel",
            Self::DocsHelp => "docs_help",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::ReleaseProofIndex => "release_proof_index",
        }
    }
}

/// One validation finding emitted by the audit packet validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditValidationFinding {
    /// Closed finding kind.
    pub finding_kind: AuditFindingKind,
    /// Finding severity.
    pub severity: AuditFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl AuditValidationFinding {
    fn new(
        finding_kind: AuditFindingKind,
        severity: AuditFindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// One audited row on a topology, explainer, or companion-adjacent surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Audited surface class for this row.
    pub surface_class: AuditSurfaceClass,
    /// Audited row class.
    pub row_class: AuditRowClass,
    /// Qualification state claimed by the row.
    pub qualification_state: QualificationState,
    /// Scope disclosure (audit pillar 1).
    pub scope_disclosure: ScopeDisclosureClass,
    /// Freshness disclosure (audit pillar 2).
    pub freshness_disclosure: FreshnessDisclosureClass,
    /// Provenance disclosure (audit pillar 3).
    pub provenance_disclosure: ProvenanceDisclosureClass,
    /// Downgrade-state disclosure (audit pillar 4).
    pub downgrade_state_disclosure: DowngradeStateDisclosureClass,
    /// Audit confidence class for this row.
    pub confidence_class: AuditConfidenceClass,
    /// Optional disclosure ref required whenever the row is not `qualified_stable`,
    /// or whenever any disclosure class requires an explicit ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// Optional provenance ref required whenever provenance is not `workspace_canonical`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance_disclosure_ref: Option<String>,
    /// Optional downgrade-state ref required whenever the downgrade state is not `none`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_disclosure_ref: Option<String>,
    /// True when raw query text, source bodies, secrets, and ambient credentials are excluded.
    pub raw_query_material_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl AuditRow {
    fn satisfies_all_pillars(&self) -> bool {
        self.scope_disclosure.satisfies_pillar()
            && self.freshness_disclosure.satisfies_pillar()
            && self.provenance_disclosure.satisfies_pillar()
            && self.downgrade_state_disclosure.satisfies_pillar()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: AuditConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Audit packet id consumed by the projection.
    pub audit_packet_id_ref: String,
    /// Rendered-at timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the surface vocabulary is preserved verbatim.
    pub preserves_surface_vocabulary: bool,
    /// True when the row-class vocabulary is preserved verbatim.
    pub preserves_row_class_vocabulary: bool,
    /// True when the qualification-state vocabulary is preserved verbatim.
    pub preserves_qualification_vocabulary: bool,
    /// True when all four audit-pillar vocabularies are preserved verbatim.
    pub preserves_audit_pillars: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl AuditConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.audit_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_surface_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_qualification_vocabulary
            && self.preserves_audit_pillars
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`AuditTopologyExplainerCompanionTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditTopologyExplainerCompanionTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Audit surface classes the packet covers.
    #[serde(default)]
    pub covered_surfaces: Vec<AuditSurfaceClass>,
    /// Audited rows.
    #[serde(default)]
    pub rows: Vec<AuditRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<AuditConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Graph-owned packet auditing topology, explainer, and companion-adjacent
/// surface rows on the M4 stable lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditTopologyExplainerCompanionTruthPacket {
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
    /// Audit surface classes the packet covers.
    #[serde(default)]
    pub covered_surfaces: Vec<AuditSurfaceClass>,
    /// Audited rows.
    #[serde(default)]
    pub rows: Vec<AuditRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<AuditConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: AuditPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<AuditValidationFinding>,
}

impl AuditTopologyExplainerCompanionTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: AuditTopologyExplainerCompanionTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_surfaces: input.covered_surfaces,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: AuditPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against stable audit invariants.
    pub fn validate(&self) -> Vec<AuditValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == AuditFindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: AuditConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns the unique surface-class tokens observed across rows.
    pub fn surface_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.surface_class);
        }
        set.into_iter().map(AuditSurfaceClass::as_str).collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter().map(AuditRowClass::as_str).collect()
    }

    /// Returns the unique qualification-state tokens observed across rows.
    pub fn qualification_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.qualification_state);
        }
        set.into_iter().map(QualificationState::as_str).collect()
    }

    /// Returns the unique scope-disclosure tokens observed across rows.
    pub fn scope_disclosure_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.scope_disclosure);
        }
        set.into_iter().map(ScopeDisclosureClass::as_str).collect()
    }

    /// Returns the unique freshness-disclosure tokens observed across rows.
    pub fn freshness_disclosure_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.freshness_disclosure);
        }
        set.into_iter()
            .map(FreshnessDisclosureClass::as_str)
            .collect()
    }

    /// Returns the unique provenance-disclosure tokens observed across rows.
    pub fn provenance_disclosure_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.provenance_disclosure);
        }
        set.into_iter()
            .map(ProvenanceDisclosureClass::as_str)
            .collect()
    }

    /// Returns the unique downgrade-state-disclosure tokens observed across rows.
    pub fn downgrade_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.downgrade_state_disclosure);
        }
        set.into_iter()
            .map(DowngradeStateDisclosureClass::as_str)
            .collect()
    }

    /// Builds a support export wrapping the exact packet shown to product surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> AuditTopologyExplainerCompanionTruthSupportExport {
        AuditTopologyExplainerCompanionTruthSupportExport {
            record_kind:
                AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            audit_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            audit_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<AuditValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(AuditValidationFinding::new(
                AuditFindingKind::WrongRecordKind,
                AuditFindingSeverity::Blocker,
                "audit packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_SCHEMA_VERSION
        {
            findings.push(AuditValidationFinding::new(
                AuditFindingKind::WrongSchemaVersion,
                AuditFindingSeverity::Blocker,
                "audit packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(AuditValidationFinding::new(
                AuditFindingKind::MissingIdentity,
                AuditFindingSeverity::Blocker,
                "packet, workflow, and timestamp refs are required",
            ));
        }
        if self.covered_surfaces.is_empty() {
            findings.push(AuditValidationFinding::new(
                AuditFindingKind::MissingAuditSurfaceRow,
                AuditFindingSeverity::Blocker,
                "packet must declare at least one covered audit surface",
            ));
        }

        for surface in &self.covered_surfaces {
            let present = self.rows.iter().any(|row| row.surface_class == *surface);
            if !present {
                findings.push(AuditValidationFinding::new(
                    AuditFindingKind::MissingAuditSurfaceRow,
                    AuditFindingSeverity::Blocker,
                    format!("no row covers audit surface {}", surface.as_str()),
                ));
            }
        }

        for row in &self.rows {
            if row.row_id.trim().is_empty() || row.captured_at.trim().is_empty() {
                findings.push(AuditValidationFinding::new(
                    AuditFindingKind::MissingIdentity,
                    AuditFindingSeverity::Blocker,
                    format!("row {} identity or timestamp is empty", row.row_id),
                ));
            }
            if !row.raw_query_material_excluded {
                findings.push(AuditValidationFinding::new(
                    AuditFindingKind::RawQueryMaterialPresent,
                    AuditFindingSeverity::Blocker,
                    format!(
                        "row {} admits raw query text, source bodies, secrets, or ambient credentials",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.is_permitted_on(row.surface_class) {
                findings.push(AuditValidationFinding::new(
                    AuditFindingKind::RowClassNotPermittedOnSurface,
                    AuditFindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} not permitted on surface {}",
                        row.row_id,
                        row.row_class.as_str(),
                        row.surface_class.as_str()
                    ),
                ));
            }

            if !row.scope_disclosure.satisfies_pillar() {
                findings.push(AuditValidationFinding::new(
                    AuditFindingKind::MissingScopeDisclosure,
                    AuditFindingSeverity::Blocker,
                    format!("row {} has no scope disclosure", row.row_id),
                ));
            }
            if !row.freshness_disclosure.satisfies_pillar() {
                findings.push(AuditValidationFinding::new(
                    AuditFindingKind::MissingFreshnessDisclosure,
                    AuditFindingSeverity::Blocker,
                    format!("row {} has no freshness disclosure", row.row_id),
                ));
            }
            if !row.provenance_disclosure.satisfies_pillar() {
                findings.push(AuditValidationFinding::new(
                    AuditFindingKind::MissingProvenanceDisclosure,
                    AuditFindingSeverity::Blocker,
                    format!("row {} has no provenance disclosure", row.row_id),
                ));
            }
            if !row.downgrade_state_disclosure.satisfies_pillar() {
                findings.push(AuditValidationFinding::new(
                    AuditFindingKind::MissingDowngradeStateDisclosure,
                    AuditFindingSeverity::Blocker,
                    format!("row {} has no downgrade-state disclosure", row.row_id),
                ));
            }

            if matches!(row.qualification_state, QualificationState::QualifiedStable)
                && !row.satisfies_all_pillars()
            {
                findings.push(AuditValidationFinding::new(
                    AuditFindingKind::NonQualifiedRowMasqueradingStable,
                    AuditFindingSeverity::Blocker,
                    format!(
                        "row {} claims qualified_stable while one or more audit pillars are unbound",
                        row.row_id
                    ),
                ));
            }

            if row.qualification_state.requires_explicit_disclosure()
                && row.disclosure_ref.is_none()
            {
                findings.push(AuditValidationFinding::new(
                    AuditFindingKind::NarrowedRowMissingDisclosureRef,
                    AuditFindingSeverity::Blocker,
                    format!(
                        "row {} has qualification state {} without a disclosure ref",
                        row.row_id,
                        row.qualification_state.as_str()
                    ),
                ));
            }

            if row
                .provenance_disclosure
                .requires_explicit_disclosure()
                && row.provenance_disclosure_ref.is_none()
            {
                findings.push(AuditValidationFinding::new(
                    AuditFindingKind::ProvenanceMissingDisclosureRef,
                    AuditFindingSeverity::Blocker,
                    format!(
                        "row {} has provenance disclosure {} without a provenance disclosure ref",
                        row.row_id,
                        row.provenance_disclosure.as_str()
                    ),
                ));
            }

            if row
                .downgrade_state_disclosure
                .requires_explicit_disclosure()
                && row.downgrade_disclosure_ref.is_none()
            {
                findings.push(AuditValidationFinding::new(
                    AuditFindingKind::DowngradeMissingDisclosureRef,
                    AuditFindingSeverity::Blocker,
                    format!(
                        "row {} has downgrade disclosure {} without a downgrade disclosure ref",
                        row.row_id,
                        row.downgrade_state_disclosure.as_str()
                    ),
                ));
            }

            if matches!(row.confidence_class, AuditConfidenceClass::LowConfidence)
                && matches!(row.qualification_state, QualificationState::QualifiedStable)
            {
                findings.push(AuditValidationFinding::new(
                    AuditFindingKind::NonQualifiedRowMasqueradingStable,
                    AuditFindingSeverity::Warning,
                    format!(
                        "row {} claims qualified_stable at low_confidence; narrowing until evidence grows",
                        row.row_id
                    ),
                ));
            }
        }

        for required_surface in AuditConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(AuditValidationFinding::new(
                    AuditFindingKind::MissingConsumerProjection,
                    AuditFindingSeverity::Blocker,
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
                findings.push(AuditValidationFinding::new(
                    AuditFindingKind::ConsumerProjectionDrift,
                    AuditFindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve audit truth",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_surface_vocabulary {
                findings.push(AuditValidationFinding::new(
                    AuditFindingKind::SurfaceVocabularyCollapsed,
                    AuditFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the surface vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_row_class_vocabulary {
                findings.push(AuditValidationFinding::new(
                    AuditFindingKind::RowClassVocabularyCollapsed,
                    AuditFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the row-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_qualification_vocabulary {
                findings.push(AuditValidationFinding::new(
                    AuditFindingKind::QualificationVocabularyCollapsed,
                    AuditFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the qualification-state vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_audit_pillars {
                findings.push(AuditValidationFinding::new(
                    AuditFindingKind::AuditPillarCollapsed,
                    AuditFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses one of the audit-pillar vocabularies",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion
                .retain(|finding| finding.finding_kind != AuditFindingKind::PromotionStateMismatch);
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(AuditValidationFinding::new(
                    AuditFindingKind::PromotionStateMismatch,
                    AuditFindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }
}

fn promotion_state_for_findings(findings: &[AuditValidationFinding]) -> AuditPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == AuditFindingSeverity::Blocker)
    {
        AuditPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == AuditFindingSeverity::Warning)
    {
        AuditPromotionState::NarrowedBelowStable
    } else {
        AuditPromotionState::Stable
    }
}

/// Support-export wrapper that preserves the product audit packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditTopologyExplainerCompanionTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub audit_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub audit_packet: AuditTopologyExplainerCompanionTruthPacket,
}

impl AuditTopologyExplainerCompanionTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_SCHEMA_VERSION
            && self.audit_packet_id_ref == self.audit_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.audit_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable audit packet.
#[derive(Debug)]
pub enum AuditTopologyExplainerCompanionTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<AuditValidationFinding>),
}

impl fmt::Display for AuditTopologyExplainerCompanionTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "audit topology/explainer/companion packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "audit topology/explainer/companion packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for AuditTopologyExplainerCompanionTruthArtifactError {}

/// Returns the checked-in stable audit topology/explainer/companion truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_audit_topology_explainer_companion_truth_packet(
) -> Result<
    AuditTopologyExplainerCompanionTruthPacket,
    AuditTopologyExplainerCompanionTruthArtifactError,
> {
    let packet: AuditTopologyExplainerCompanionTruthPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/search/m4/audit_topology_explainer_companion_truth_packet.json"
        )))
        .map_err(AuditTopologyExplainerCompanionTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(AuditTopologyExplainerCompanionTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_row(
        row_id: &str,
        surface: AuditSurfaceClass,
        row_class: AuditRowClass,
    ) -> AuditRow {
        AuditRow {
            row_id: row_id.to_owned(),
            surface_class: surface,
            row_class,
            qualification_state: QualificationState::QualifiedStable,
            scope_disclosure: ScopeDisclosureClass::SelectedWorkset,
            freshness_disclosure: FreshnessDisclosureClass::Live,
            provenance_disclosure: ProvenanceDisclosureClass::WorkspaceCanonical,
            downgrade_state_disclosure: DowngradeStateDisclosureClass::None,
            confidence_class: AuditConfidenceClass::HighConfidence,
            disclosure_ref: None,
            provenance_disclosure_ref: None,
            downgrade_disclosure_ref: None,
            raw_query_material_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn sample_projection(surface: AuditConsumerSurface) -> AuditConsumerProjection {
        AuditConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            audit_packet_id_ref: "packet:m4:audit_topology_explainer_companion".to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_surface_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_qualification_vocabulary: true,
            preserves_audit_pillars: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn sample_input() -> AuditTopologyExplainerCompanionTruthPacketInput {
        AuditTopologyExplainerCompanionTruthPacketInput {
            packet_id: "packet:m4:audit_topology_explainer_companion".to_owned(),
            workflow_or_surface_id: "workflow.graph.audit_topology_explainer_companion"
                .to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_surfaces: AuditSurfaceClass::REQUIRED.to_vec(),
            rows: vec![
                sample_row(
                    "row:topology_canvas:node:auth_module",
                    AuditSurfaceClass::TopologyCanvas,
                    AuditRowClass::TopologyNode,
                ),
                sample_row(
                    "row:topology_table:edge:imports_auth_module",
                    AuditSurfaceClass::TopologyTable,
                    AuditRowClass::TopologyEdge,
                ),
                sample_row(
                    "row:impact_explainer:impact_edge:checkout",
                    AuditSurfaceClass::ImpactExplainer,
                    AuditRowClass::ImpactEdge,
                ),
                sample_row(
                    "row:evidence_card:row:ownership_rule",
                    AuditSurfaceClass::EvidenceCard,
                    AuditRowClass::EvidenceCardRow,
                ),
                sample_row(
                    "row:companion_navigator:action:open_related",
                    AuditSurfaceClass::CompanionNavigator,
                    AuditRowClass::CompanionAction,
                ),
                sample_row(
                    "row:companion_filter:filter:owned_by_payments",
                    AuditSurfaceClass::CompanionFilter,
                    AuditRowClass::CompanionFilterRow,
                ),
                sample_row(
                    "row:companion_export:row:export_evidence_card",
                    AuditSurfaceClass::CompanionExport,
                    AuditRowClass::CompanionExportRow,
                ),
                sample_row(
                    "row:companion_history:row:recently_pinned",
                    AuditSurfaceClass::CompanionHistory,
                    AuditRowClass::CompanionHistoryRow,
                ),
            ],
            consumer_projections: AuditConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(sample_projection)
                .collect(),
            source_contract_refs: vec![
                AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_DOC_REF.to_owned(),
            ],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(
            AuditSurfaceClass::CompanionHistory.as_str(),
            "companion_history"
        );
        assert_eq!(AuditRowClass::ImpactEdge.as_str(), "impact_edge");
        assert_eq!(
            QualificationState::NarrowedBelowStable.as_str(),
            "narrowed_below_stable"
        );
        assert_eq!(
            ScopeDisclosureClass::OutsideCurrentScope.as_str(),
            "outside_current_scope"
        );
        assert_eq!(
            FreshnessDisclosureClass::ImportedSnapshot.as_str(),
            "imported_snapshot"
        );
        assert_eq!(
            ProvenanceDisclosureClass::HeuristicDerived.as_str(),
            "heuristic_derived"
        );
        assert_eq!(
            DowngradeStateDisclosureClass::ArchivedFrozen.as_str(),
            "archived_frozen"
        );
        assert_eq!(
            AuditFindingKind::NonQualifiedRowMasqueradingStable.as_str(),
            "non_qualified_row_masquerading_stable"
        );
        assert_eq!(AuditPromotionState::BlocksStable.as_str(), "blocks_stable");
        assert_eq!(
            AuditConsumerSurface::ReleaseProofIndex.as_str(),
            "release_proof_index"
        );
        assert_eq!(AuditPillar::DowngradeState.as_str(), "downgrade_state");
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = AuditTopologyExplainerCompanionTruthPacket::materialize(sample_input());
        assert_eq!(packet.promotion_state, AuditPromotionState::Stable);
        assert!(packet.validation_findings.is_empty());
        assert!(packet.is_stable());
        assert!(packet
            .support_export("support:m4:audit", "2026-05-26T12:00:10Z")
            .is_export_safe());
    }

    #[test]
    fn unbound_scope_blocks_qualified_stable() {
        let mut input = sample_input();
        input.rows[0].scope_disclosure = ScopeDisclosureClass::ScopeUnbound;
        let packet = AuditTopologyExplainerCompanionTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, AuditPromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == AuditFindingKind::MissingScopeDisclosure));
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == AuditFindingKind::NonQualifiedRowMasqueradingStable
        }));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        input.rows[0].qualification_state = QualificationState::NarrowedBelowStable;
        input.rows[0].disclosure_ref = None;
        let packet = AuditTopologyExplainerCompanionTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, AuditPromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == AuditFindingKind::NarrowedRowMissingDisclosureRef
        }));
    }

    #[test]
    fn row_class_not_permitted_on_surface_blocks() {
        let mut input = sample_input();
        input.rows[0].row_class = AuditRowClass::CompanionFilterRow;
        let packet = AuditTopologyExplainerCompanionTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, AuditPromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == AuditFindingKind::RowClassNotPermittedOnSurface
        }));
    }

    #[test]
    fn projection_drop_blocks_promotion() {
        let mut input = sample_input();
        input
            .consumer_projections
            .retain(|projection| projection.consumer_surface != AuditConsumerSurface::SupportExport);
        let packet = AuditTopologyExplainerCompanionTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, AuditPromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == AuditFindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_audit_pillar_blocks_promotion() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == AuditConsumerSurface::DocsHelp {
                projection.preserves_audit_pillars = false;
            }
        }
        let packet = AuditTopologyExplainerCompanionTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, AuditPromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == AuditFindingKind::AuditPillarCollapsed));
    }

    #[test]
    fn raw_query_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_query_material_excluded = false;
        let packet = AuditTopologyExplainerCompanionTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, AuditPromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == AuditFindingKind::RawQueryMaterialPresent));
    }

    #[test]
    fn provenance_requiring_disclosure_ref_must_carry_it() {
        let mut input = sample_input();
        input.rows[0].provenance_disclosure = ProvenanceDisclosureClass::ImportedProviderDerived;
        input.rows[0].provenance_disclosure_ref = None;
        // imported-provider derived must keep its disclosure ref
        let packet = AuditTopologyExplainerCompanionTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, AuditPromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == AuditFindingKind::ProvenanceMissingDisclosureRef
        }));
    }
}
