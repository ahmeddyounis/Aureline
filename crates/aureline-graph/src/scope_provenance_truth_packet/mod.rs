//! Stable hidden-scope, partial-scope, archived-item, and imported-provider
//! truth packet shared by search and graph surfaces.
//!
//! This module is the search-and-graph-owned contract for the M4 stable
//! lane that pins how rows hidden by scope, narrowed by partial indexing,
//! preserved as archived, or contributed by imported providers are
//! labeled across consumer surfaces. Every row carries a closed
//! `item_class`, `provenance_class`, `freshness_class`, `downgrade_state`,
//! and an explicit `disclosure_ref` so the search shell, graph topology,
//! docs/help, CLI/headless inspector, support export, and release proof
//! index all read the same scope-and-provenance vocabulary instead of
//! reinventing it locally.
//!
//! The packet is intentionally metadata-only — it never admits raw query
//! text, raw source bodies, secrets, ambient credentials, or provider
//! payloads. Imported-provider rows carry an `ImportedMapping` block with
//! a closed outcome label (`exact`, `translated`, `partial`, `shimmed`,
//! `unsupported`), a rollback checkpoint ref, and a mapping diagnostic
//! ref so failed mappings remain diagnosable without leaking provider
//! material. The validator refuses to certify a stable claim when a row
//! drops its disclosure ref, mislabels an imported, archived, partial,
//! or hidden row as canonical, or when any required consumer projection
//! collapses the closed vocabulary.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`ScopeProvenanceTruthPacket`].
pub const SCOPE_PROVENANCE_TRUTH_PACKET_RECORD_KIND: &str =
    "scope_provenance_truth_stable_packet";

/// Stable record-kind tag for [`ScopeProvenanceTruthSupportExport`].
pub const SCOPE_PROVENANCE_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "scope_provenance_truth_support_export";

/// Integer schema version for stable scope/provenance truth packets.
pub const SCOPE_PROVENANCE_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const SCOPE_PROVENANCE_TRUTH_SCHEMA_REF: &str =
    "schemas/search/scope_provenance_truth.schema.json";

/// Repo-relative path of the reviewer doc.
pub const SCOPE_PROVENANCE_TRUTH_DOC_REF: &str =
    "docs/search/m4/certify-hidden-scope-partial-scope-archived-item-and.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const SCOPE_PROVENANCE_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/search/m4/certify-hidden-scope-partial-scope-archived-item-and.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const SCOPE_PROVENANCE_TRUTH_FIXTURE_DIR: &str =
    "fixtures/search/m4/scope_provenance_truth";

/// Repo-relative path of the checked-in stable scope/provenance truth packet.
pub const SCOPE_PROVENANCE_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/search/m4/scope_provenance_truth_packet.json";

/// Closed item-class vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemClass {
    /// Item is hidden by scope rules (policy filter, glob exclusion, workset boundary).
    HiddenScope,
    /// Item lies in a region the index has not yet fully covered.
    PartialScope,
    /// Item is preserved as archived (frozen, historical, read-only).
    ArchivedItem,
    /// Item was contributed by an imported external provider, not derived from canonical workspace truth.
    ImportedProvider,
}

impl ItemClass {
    /// Every required item class, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::HiddenScope,
        Self::PartialScope,
        Self::ArchivedItem,
        Self::ImportedProvider,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HiddenScope => "hidden_scope",
            Self::PartialScope => "partial_scope",
            Self::ArchivedItem => "archived_item",
            Self::ImportedProvider => "imported_provider",
        }
    }
}

/// Closed surface-class vocabulary for the consuming row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// A row in the search results list (file, symbol, command, or text hit).
    SearchRow,
    /// A node in a graph topology view (symbol, file, module, provider artifact).
    GraphNode,
    /// An edge in a graph topology view (import, ownership, call).
    GraphEdge,
}

impl SurfaceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchRow => "search_row",
            Self::GraphNode => "graph_node",
            Self::GraphEdge => "graph_edge",
        }
    }
}

/// Closed provenance-class vocabulary attached to a truth row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceClass {
    /// Truth derived from the canonical workspace graph or index.
    WorkspaceCanonical,
    /// Truth inferred while only a partial index slice is available.
    PartialIndexInferred,
    /// Truth preserved from an archived snapshot (read-only, frozen).
    ArchivePreserved,
    /// Truth derived from an imported external provider mapping.
    ImportedProviderDerived,
    /// Truth derived from a heuristic (best-effort, not authoritative).
    HeuristicDerived,
}

impl ProvenanceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceCanonical => "workspace_canonical",
            Self::PartialIndexInferred => "partial_index_inferred",
            Self::ArchivePreserved => "archive_preserved",
            Self::ImportedProviderDerived => "imported_provider_derived",
            Self::HeuristicDerived => "heuristic_derived",
        }
    }

    /// Provenance classes that an item class is permitted to use.
    fn matches_item_class(self, item: ItemClass) -> bool {
        matches!(
            (item, self),
            (ItemClass::HiddenScope, Self::WorkspaceCanonical | Self::PartialIndexInferred)
                | (
                    ItemClass::PartialScope,
                    Self::PartialIndexInferred | Self::HeuristicDerived,
                )
                | (ItemClass::ArchivedItem, Self::ArchivePreserved)
                | (
                    ItemClass::ImportedProvider,
                    Self::ImportedProviderDerived | Self::HeuristicDerived,
                )
        )
    }
}

/// Closed freshness-class vocabulary attached to a truth row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    /// Row is current as of the last canonical refresh.
    Live,
    /// Row is partially warmed; not the full canonical set yet.
    PartiallyWarmed,
    /// Row is stale but its staleness has been disclosed.
    StaleDisclosed,
    /// Row is frozen as an archived snapshot.
    ArchivedFrozen,
    /// Row carries the snapshot recorded by an imported provider.
    ImportedSnapshot,
    /// Row's freshness is unknown (e.g., heuristic).
    Unknown,
}

impl FreshnessClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::PartiallyWarmed => "partially_warmed",
            Self::StaleDisclosed => "stale_disclosed",
            Self::ArchivedFrozen => "archived_frozen",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::Unknown => "unknown",
        }
    }
}

/// Closed downgrade-state vocabulary. A row is never marked `canonical`
/// when its item class is one of the four non-canonical classes covered
/// by this packet; the validator enforces that mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeState {
    /// Row is canonical workspace truth, no downgrade applied.
    Canonical,
    /// Hidden-by-scope downgrade is disclosed on the row.
    HiddenDisclosed,
    /// Partial-scope downgrade is disclosed on the row.
    PartialDisclosed,
    /// Archived-item downgrade is disclosed on the row.
    ArchivedDisclosed,
    /// Imported-provider downgrade is disclosed on the row.
    ImportedDisclosed,
}

impl DowngradeState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Canonical => "canonical",
            Self::HiddenDisclosed => "hidden_disclosed",
            Self::PartialDisclosed => "partial_disclosed",
            Self::ArchivedDisclosed => "archived_disclosed",
            Self::ImportedDisclosed => "imported_disclosed",
        }
    }

    /// True when this downgrade state matches the item class.
    fn matches_item_class(self, item: ItemClass) -> bool {
        matches!(
            (item, self),
            (ItemClass::HiddenScope, Self::HiddenDisclosed)
                | (ItemClass::PartialScope, Self::PartialDisclosed)
                | (ItemClass::ArchivedItem, Self::ArchivedDisclosed)
                | (ItemClass::ImportedProvider, Self::ImportedDisclosed)
        )
    }
}

/// Closed confidence-class vocabulary attached to a truth row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceClass {
    /// High confidence — backed by canonical or signed evidence.
    High,
    /// Medium confidence — backed by partial or warming evidence.
    Medium,
    /// Low confidence — heuristic or thin evidence.
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

/// Closed imported-mapping outcome labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportedOutcomeLabel {
    /// Imported artifact maps exactly to a canonical workspace concept.
    Exact,
    /// Imported artifact is translated into the closest canonical concept.
    Translated,
    /// Imported artifact maps only partially; gaps are disclosed.
    Partial,
    /// Imported artifact is shimmed behind a compatibility adapter.
    Shimmed,
    /// Imported artifact has no supported mapping; row is labeled as such.
    Unsupported,
}

impl ImportedOutcomeLabel {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Translated => "translated",
            Self::Partial => "partial",
            Self::Shimmed => "shimmed",
            Self::Unsupported => "unsupported",
        }
    }

    /// True when the outcome label requires a diagnostic ref.
    pub const fn requires_diagnostic(self) -> bool {
        matches!(self, Self::Partial | Self::Shimmed | Self::Unsupported)
    }
}

/// Imported-provider mapping block attached to imported-provider rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedMapping {
    /// Stable id for the imported provider that supplied the row.
    pub imported_provider_id: String,
    /// Closed outcome label classifying how the import maps to canonical truth.
    pub outcome_label: ImportedOutcomeLabel,
    /// Repo-relative rollback checkpoint ref so the import can be undone.
    pub rollback_checkpoint_ref: String,
    /// Mapping diagnostic ref (required for partial/shimmed/unsupported outcomes).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mapping_diagnostic_ref: Option<String>,
}

/// Hidden-scope reason metadata on a hidden-scope row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenScopeContext {
    /// Closed token naming why the row was hidden (policy filter, glob, workset).
    pub reason_token: String,
    /// Repo-relative ref to the policy or scope rule that hid the row.
    pub rule_ref: String,
}

/// Partial-scope coverage metadata on a partial-scope row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartialScopeContext {
    /// Stable lane id that is only partially indexed at capture time.
    pub partial_lane_id: String,
    /// Estimated proportion of the lane that is covered (0..=100).
    pub coverage_percent: u8,
}

/// Archived-item metadata on an archived-item row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchivedContext {
    /// Timestamp the item was archived.
    pub archived_at: String,
    /// Repo-relative ref to the archive register that holds the item.
    pub archive_register_ref: String,
}

/// One truth row covered by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeProvenanceRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Item class this row certifies.
    pub item_class: ItemClass,
    /// Surface class this row applies to.
    pub surface_class: SurfaceClass,
    /// Stable workspace identity.
    pub workspace_id: String,
    /// Workset/scope ref the row was captured under.
    pub scope_ref: String,
    /// Provenance class for the row.
    pub provenance_class: ProvenanceClass,
    /// Freshness class for the row.
    pub freshness_class: FreshnessClass,
    /// Downgrade state for the row.
    pub downgrade_state: DowngradeState,
    /// Confidence class for the row.
    pub confidence_class: ConfidenceClass,
    /// Repo-relative ref to the disclosure shown on the row.
    pub disclosure_ref: String,
    /// Hidden-scope reason metadata (required for `hidden_scope` rows).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden_scope_context: Option<HiddenScopeContext>,
    /// Partial-scope context (required for `partial_scope` rows).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partial_scope_context: Option<PartialScopeContext>,
    /// Archived-item context (required for `archived_item` rows).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived_context: Option<ArchivedContext>,
    /// Imported-mapping block (required for `imported_provider` rows).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_mapping: Option<ImportedMapping>,
    /// True when raw private material is excluded from this row.
    pub raw_boundary_material_excluded: bool,
    /// Capture timestamp for this row.
    pub captured_at: String,
}

/// Consumer surface that must inherit this packet's truth verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Search shell results pane (file, symbol, text, command).
    SearchShell,
    /// Graph topology canvas / list / table fallback.
    GraphTopology,
    /// Docs/help surface explaining scope and provenance.
    DocsHelp,
    /// CLI or headless inspection surface.
    CliHeadless,
    /// Support export bundle.
    SupportExport,
    /// Release proof index entry.
    ReleaseProofIndex,
}

impl ConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 6] = [
        Self::SearchShell,
        Self::GraphTopology,
        Self::DocsHelp,
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
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::ReleaseProofIndex => "release_proof_index",
        }
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeProvenanceConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Packet id consumed by the projection.
    pub scope_provenance_packet_id_ref: String,
    /// Rendered-at timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the surface preserves item-class identity per row.
    pub preserves_item_class_vocabulary: bool,
    /// True when the surface preserves provenance-class labels.
    pub preserves_provenance_vocabulary: bool,
    /// True when the surface preserves freshness-class labels.
    pub preserves_freshness_vocabulary: bool,
    /// True when the surface preserves downgrade-state labels.
    pub preserves_downgrade_vocabulary: bool,
    /// True when the surface preserves imported-outcome labels.
    pub preserves_imported_outcome_vocabulary: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl ScopeProvenanceConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.scope_provenance_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_item_class_vocabulary
            && self.preserves_provenance_vocabulary
            && self.preserves_freshness_vocabulary
            && self.preserves_downgrade_vocabulary
            && self.preserves_imported_outcome_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Closed promotion state for [`ScopeProvenanceTruthPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionState {
    /// Packet certifies a stable claim for every declared item-class row.
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

/// Closed validation-finding vocabulary for [`ScopeProvenanceTruthPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required item-class row is missing from the packet.
    MissingItemClassCoverage,
    /// A row drops its disclosure ref.
    MissingDisclosureRef,
    /// A hidden-scope row drops its hidden_scope_context.
    HiddenScopeMissingContext,
    /// A partial-scope row drops its partial_scope_context.
    PartialScopeMissingContext,
    /// An archived row drops its archived_context.
    ArchivedMissingContext,
    /// An imported-provider row drops its imported_mapping.
    ImportedMissingMapping,
    /// An imported row's outcome requires a diagnostic ref but none is present.
    ImportedMissingDiagnostic,
    /// An imported row drops its rollback checkpoint ref.
    ImportedMissingRollback,
    /// A row's provenance class does not match its item class.
    ProvenanceClassMismatch,
    /// A row's downgrade state does not match its item class.
    DowngradeStateMismatch,
    /// A non-canonical row is presented with downgrade_state == canonical.
    NonCanonicalPresentedAsCanonical,
    /// A required consumer projection is missing.
    MissingConsumerProjection,
    /// A consumer projection drops part of the closed vocabulary.
    ConsumerProjectionDrift,
    /// A consumer projection collapses the item-class vocabulary.
    ItemClassVocabularyCollapsed,
    /// A consumer projection collapses the provenance vocabulary.
    ProvenanceVocabularyDropped,
    /// A consumer projection collapses the freshness vocabulary.
    FreshnessVocabularyDropped,
    /// A consumer projection collapses the downgrade vocabulary.
    DowngradeVocabularyDropped,
    /// A consumer projection collapses the imported-outcome vocabulary.
    ImportedOutcomeVocabularyDropped,
    /// Row admits raw query text, source bodies, secrets, or ambient credentials.
    RawBoundaryMaterialPresent,
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
            Self::MissingItemClassCoverage => "missing_item_class_coverage",
            Self::MissingDisclosureRef => "missing_disclosure_ref",
            Self::HiddenScopeMissingContext => "hidden_scope_missing_context",
            Self::PartialScopeMissingContext => "partial_scope_missing_context",
            Self::ArchivedMissingContext => "archived_missing_context",
            Self::ImportedMissingMapping => "imported_missing_mapping",
            Self::ImportedMissingDiagnostic => "imported_missing_diagnostic",
            Self::ImportedMissingRollback => "imported_missing_rollback",
            Self::ProvenanceClassMismatch => "provenance_class_mismatch",
            Self::DowngradeStateMismatch => "downgrade_state_mismatch",
            Self::NonCanonicalPresentedAsCanonical => "non_canonical_presented_as_canonical",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::ItemClassVocabularyCollapsed => "item_class_vocabulary_collapsed",
            Self::ProvenanceVocabularyDropped => "provenance_vocabulary_dropped",
            Self::FreshnessVocabularyDropped => "freshness_vocabulary_dropped",
            Self::DowngradeVocabularyDropped => "downgrade_vocabulary_dropped",
            Self::ImportedOutcomeVocabularyDropped => "imported_outcome_vocabulary_dropped",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
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

/// Constructor input for [`ScopeProvenanceTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeProvenanceTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet as a whole.
    pub generated_at: String,
    /// Item classes the packet covers.
    #[serde(default)]
    pub covered_item_classes: Vec<ItemClass>,
    /// Truth rows.
    #[serde(default)]
    pub rows: Vec<ScopeProvenanceRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<ScopeProvenanceConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Search-and-graph-owned packet for hidden-scope, partial-scope,
/// archived-item, and imported-provider truth on certified surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeProvenanceTruthPacket {
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
    /// Item classes the packet covers.
    #[serde(default)]
    pub covered_item_classes: Vec<ItemClass>,
    /// Truth rows.
    #[serde(default)]
    pub rows: Vec<ScopeProvenanceRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<ScopeProvenanceConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl ScopeProvenanceTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: ScopeProvenanceTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: SCOPE_PROVENANCE_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: SCOPE_PROVENANCE_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_item_classes: input.covered_item_classes,
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

    /// Re-validates the packet against stable scope/provenance invariants.
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

    /// Returns the unique item-class tokens observed across rows.
    pub fn item_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.item_class);
        }
        set.into_iter().map(ItemClass::as_str).collect()
    }

    /// Returns the unique provenance-class tokens observed across rows.
    pub fn provenance_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.provenance_class);
        }
        set.into_iter().map(ProvenanceClass::as_str).collect()
    }

    /// Returns the unique downgrade-state tokens observed across rows.
    pub fn downgrade_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.downgrade_state);
        }
        set.into_iter().map(DowngradeState::as_str).collect()
    }

    /// Returns the unique imported-outcome tokens observed across rows.
    pub fn imported_outcome_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            if let Some(mapping) = &row.imported_mapping {
                set.insert(mapping.outcome_label);
            }
        }
        set.into_iter().map(ImportedOutcomeLabel::as_str).collect()
    }

    /// Builds a support export wrapping the exact packet shown to product surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> ScopeProvenanceTruthSupportExport {
        ScopeProvenanceTruthSupportExport {
            record_kind: SCOPE_PROVENANCE_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SCOPE_PROVENANCE_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            scope_provenance_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            scope_provenance_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != SCOPE_PROVENANCE_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "scope/provenance truth packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != SCOPE_PROVENANCE_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "scope/provenance truth packet has the wrong schema version",
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

        for required in ItemClass::REQUIRED {
            let in_coverage = self.covered_item_classes.contains(&required);
            let in_rows = self.rows.iter().any(|row| row.item_class == required);
            if !in_coverage || !in_rows {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingItemClassCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers required item class {}", required.as_str()),
                ));
            }
        }

        for row in &self.rows {
            if row.row_id.trim().is_empty()
                || row.workspace_id.trim().is_empty()
                || row.scope_ref.trim().is_empty()
                || row.captured_at.trim().is_empty()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingIdentity,
                    FindingSeverity::Blocker,
                    format!("row {} drops a required identity field", row.row_id),
                ));
            }
            if row.disclosure_ref.trim().is_empty() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!("row {} drops its disclosure ref", row.row_id),
                ));
            }
            if !row.provenance_class.matches_item_class(row.item_class) {
                findings.push(ValidationFinding::new(
                    FindingKind::ProvenanceClassMismatch,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} item class {} disagrees with provenance {}",
                        row.row_id,
                        row.item_class.as_str(),
                        row.provenance_class.as_str()
                    ),
                ));
            }
            if !row.downgrade_state.matches_item_class(row.item_class) {
                if row.downgrade_state == DowngradeState::Canonical {
                    findings.push(ValidationFinding::new(
                        FindingKind::NonCanonicalPresentedAsCanonical,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} is {} but presents as canonical",
                            row.row_id,
                            row.item_class.as_str()
                        ),
                    ));
                } else {
                    findings.push(ValidationFinding::new(
                        FindingKind::DowngradeStateMismatch,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} item class {} disagrees with downgrade {}",
                            row.row_id,
                            row.item_class.as_str(),
                            row.downgrade_state.as_str()
                        ),
                    ));
                }
            }
            match row.item_class {
                ItemClass::HiddenScope => {
                    let context_ok = row.hidden_scope_context.as_ref().is_some_and(|context| {
                        !context.reason_token.trim().is_empty()
                            && !context.rule_ref.trim().is_empty()
                    });
                    if !context_ok {
                        findings.push(ValidationFinding::new(
                            FindingKind::HiddenScopeMissingContext,
                            FindingSeverity::Blocker,
                            format!(
                                "row {} is hidden_scope but missing reason/rule context",
                                row.row_id
                            ),
                        ));
                    }
                }
                ItemClass::PartialScope => {
                    let context_ok = row.partial_scope_context.as_ref().is_some_and(|context| {
                        !context.partial_lane_id.trim().is_empty()
                            && context.coverage_percent <= 100
                    });
                    if !context_ok {
                        findings.push(ValidationFinding::new(
                            FindingKind::PartialScopeMissingContext,
                            FindingSeverity::Blocker,
                            format!(
                                "row {} is partial_scope but missing partial-scope context",
                                row.row_id
                            ),
                        ));
                    }
                }
                ItemClass::ArchivedItem => {
                    let context_ok = row.archived_context.as_ref().is_some_and(|context| {
                        !context.archived_at.trim().is_empty()
                            && !context.archive_register_ref.trim().is_empty()
                    });
                    if !context_ok {
                        findings.push(ValidationFinding::new(
                            FindingKind::ArchivedMissingContext,
                            FindingSeverity::Blocker,
                            format!(
                                "row {} is archived_item but missing archived context",
                                row.row_id
                            ),
                        ));
                    }
                }
                ItemClass::ImportedProvider => match row.imported_mapping.as_ref() {
                    None => {
                        findings.push(ValidationFinding::new(
                            FindingKind::ImportedMissingMapping,
                            FindingSeverity::Blocker,
                            format!(
                                "row {} is imported_provider but missing imported_mapping",
                                row.row_id
                            ),
                        ));
                    }
                    Some(mapping) => {
                        if mapping.imported_provider_id.trim().is_empty() {
                            findings.push(ValidationFinding::new(
                                FindingKind::ImportedMissingMapping,
                                FindingSeverity::Blocker,
                                format!(
                                    "row {} imported mapping drops provider id",
                                    row.row_id
                                ),
                            ));
                        }
                        if mapping.rollback_checkpoint_ref.trim().is_empty() {
                            findings.push(ValidationFinding::new(
                                FindingKind::ImportedMissingRollback,
                                FindingSeverity::Blocker,
                                format!(
                                    "row {} imported mapping drops rollback checkpoint ref",
                                    row.row_id
                                ),
                            ));
                        }
                        if mapping.outcome_label.requires_diagnostic()
                            && mapping
                                .mapping_diagnostic_ref
                                .as_deref()
                                .map_or(true, |reference| reference.trim().is_empty())
                        {
                            findings.push(ValidationFinding::new(
                                FindingKind::ImportedMissingDiagnostic,
                                FindingSeverity::Blocker,
                                format!(
                                    "row {} imported outcome {} requires a diagnostic ref",
                                    row.row_id,
                                    mapping.outcome_label.as_str()
                                ),
                            ));
                        }
                    }
                },
            }
            if !row.raw_boundary_material_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::RawBoundaryMaterialPresent,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} admits raw query text, source bodies, secrets, or credentials",
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
                        "projection {} does not preserve scope/provenance truth",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_item_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::ItemClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the item-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_provenance_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::ProvenanceVocabularyDropped,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} drops the provenance vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_freshness_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::FreshnessVocabularyDropped,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} drops the freshness vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_downgrade_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::DowngradeVocabularyDropped,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} drops the downgrade-state vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_imported_outcome_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::ImportedOutcomeVocabularyDropped,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} drops the imported-outcome vocabulary",
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
pub struct ScopeProvenanceTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub scope_provenance_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub scope_provenance_packet: ScopeProvenanceTruthPacket,
}

impl ScopeProvenanceTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == SCOPE_PROVENANCE_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == SCOPE_PROVENANCE_TRUTH_SCHEMA_VERSION
            && self.scope_provenance_packet_id_ref == self.scope_provenance_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.scope_provenance_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable packet.
#[derive(Debug)]
pub enum ScopeProvenanceTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for ScopeProvenanceTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "scope/provenance truth packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "scope/provenance truth packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ScopeProvenanceTruthArtifactError {}

/// Returns the checked-in stable scope/provenance truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_scope_provenance_truth_packet(
) -> Result<ScopeProvenanceTruthPacket, ScopeProvenanceTruthArtifactError> {
    let packet: ScopeProvenanceTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/search/m4/scope_provenance_truth_packet.json"
    )))
    .map_err(ScopeProvenanceTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(ScopeProvenanceTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_hidden_row() -> ScopeProvenanceRow {
        ScopeProvenanceRow {
            row_id: "row:hidden:policy".to_owned(),
            item_class: ItemClass::HiddenScope,
            surface_class: SurfaceClass::SearchRow,
            workspace_id: "workspace:m4:scope_provenance".to_owned(),
            scope_ref: "scope:certified:default".to_owned(),
            provenance_class: ProvenanceClass::WorkspaceCanonical,
            freshness_class: FreshnessClass::Live,
            downgrade_state: DowngradeState::HiddenDisclosed,
            confidence_class: ConfidenceClass::High,
            disclosure_ref: SCOPE_PROVENANCE_TRUTH_DOC_REF.to_owned(),
            hidden_scope_context: Some(HiddenScopeContext {
                reason_token: "policy_filter".to_owned(),
                rule_ref: "policy:redacted_paths".to_owned(),
            }),
            partial_scope_context: None,
            archived_context: None,
            imported_mapping: None,
            raw_boundary_material_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn sample_partial_row() -> ScopeProvenanceRow {
        ScopeProvenanceRow {
            row_id: "row:partial:symbols".to_owned(),
            item_class: ItemClass::PartialScope,
            surface_class: SurfaceClass::SearchRow,
            workspace_id: "workspace:m4:scope_provenance".to_owned(),
            scope_ref: "scope:certified:default".to_owned(),
            provenance_class: ProvenanceClass::PartialIndexInferred,
            freshness_class: FreshnessClass::PartiallyWarmed,
            downgrade_state: DowngradeState::PartialDisclosed,
            confidence_class: ConfidenceClass::Medium,
            disclosure_ref: SCOPE_PROVENANCE_TRUTH_DOC_REF.to_owned(),
            hidden_scope_context: None,
            partial_scope_context: Some(PartialScopeContext {
                partial_lane_id: "symbol_index".to_owned(),
                coverage_percent: 42,
            }),
            archived_context: None,
            imported_mapping: None,
            raw_boundary_material_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn sample_archived_row() -> ScopeProvenanceRow {
        ScopeProvenanceRow {
            row_id: "row:archived:legacy_module".to_owned(),
            item_class: ItemClass::ArchivedItem,
            surface_class: SurfaceClass::GraphNode,
            workspace_id: "workspace:m4:scope_provenance".to_owned(),
            scope_ref: "scope:certified:default".to_owned(),
            provenance_class: ProvenanceClass::ArchivePreserved,
            freshness_class: FreshnessClass::ArchivedFrozen,
            downgrade_state: DowngradeState::ArchivedDisclosed,
            confidence_class: ConfidenceClass::High,
            disclosure_ref: SCOPE_PROVENANCE_TRUTH_DOC_REF.to_owned(),
            hidden_scope_context: None,
            partial_scope_context: None,
            archived_context: Some(ArchivedContext {
                archived_at: "2025-12-01T00:00:00Z".to_owned(),
                archive_register_ref: "archive:legacy/modules".to_owned(),
            }),
            imported_mapping: None,
            raw_boundary_material_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn sample_imported_row() -> ScopeProvenanceRow {
        ScopeProvenanceRow {
            row_id: "row:imported:provider_alpha".to_owned(),
            item_class: ItemClass::ImportedProvider,
            surface_class: SurfaceClass::GraphEdge,
            workspace_id: "workspace:m4:scope_provenance".to_owned(),
            scope_ref: "scope:certified:default".to_owned(),
            provenance_class: ProvenanceClass::ImportedProviderDerived,
            freshness_class: FreshnessClass::ImportedSnapshot,
            downgrade_state: DowngradeState::ImportedDisclosed,
            confidence_class: ConfidenceClass::Medium,
            disclosure_ref: SCOPE_PROVENANCE_TRUTH_DOC_REF.to_owned(),
            hidden_scope_context: None,
            partial_scope_context: None,
            archived_context: None,
            imported_mapping: Some(ImportedMapping {
                imported_provider_id: "provider:imported:alpha".to_owned(),
                outcome_label: ImportedOutcomeLabel::Translated,
                rollback_checkpoint_ref: "rollback:imported_alpha:2026-05".to_owned(),
                mapping_diagnostic_ref: None,
            }),
            raw_boundary_material_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn sample_projection(surface: ConsumerSurface) -> ScopeProvenanceConsumerProjection {
        ScopeProvenanceConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            scope_provenance_packet_id_ref: "packet:m4:scope_provenance".to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_item_class_vocabulary: true,
            preserves_provenance_vocabulary: true,
            preserves_freshness_vocabulary: true,
            preserves_downgrade_vocabulary: true,
            preserves_imported_outcome_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn baseline_input() -> ScopeProvenanceTruthPacketInput {
        ScopeProvenanceTruthPacketInput {
            packet_id: "packet:m4:scope_provenance".to_owned(),
            workflow_or_surface_id: "workflow.search_graph.scope_provenance".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_item_classes: ItemClass::REQUIRED.to_vec(),
            rows: vec![
                sample_hidden_row(),
                sample_partial_row(),
                sample_archived_row(),
                sample_imported_row(),
            ],
            consumer_projections: ConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(sample_projection)
                .collect(),
            source_contract_refs: vec![SCOPE_PROVENANCE_TRUTH_DOC_REF.to_owned()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(ItemClass::HiddenScope.as_str(), "hidden_scope");
        assert_eq!(ItemClass::ImportedProvider.as_str(), "imported_provider");
        assert_eq!(
            ProvenanceClass::ImportedProviderDerived.as_str(),
            "imported_provider_derived"
        );
        assert_eq!(DowngradeState::ArchivedDisclosed.as_str(), "archived_disclosed");
        assert_eq!(ImportedOutcomeLabel::Unsupported.as_str(), "unsupported");
        assert_eq!(
            FindingKind::NonCanonicalPresentedAsCanonical.as_str(),
            "non_canonical_presented_as_canonical"
        );
        assert_eq!(PromotionState::BlocksStable.as_str(), "blocks_stable");
    }

    #[test]
    fn baseline_input_materializes_stable() {
        let packet = ScopeProvenanceTruthPacket::materialize(baseline_input());
        assert_eq!(packet.promotion_state, PromotionState::Stable);
        assert!(packet.validation_findings.is_empty());
        assert!(packet.is_stable());
    }

    #[test]
    fn missing_disclosure_blocks_stable() {
        let mut input = baseline_input();
        input.rows[0].disclosure_ref = String::new();
        let packet = ScopeProvenanceTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingDisclosureRef));
    }

    #[test]
    fn imported_provider_without_mapping_blocks_stable() {
        let mut input = baseline_input();
        input.rows[3].imported_mapping = None;
        let packet = ScopeProvenanceTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::ImportedMissingMapping));
    }

    #[test]
    fn imported_unsupported_requires_diagnostic() {
        let mut input = baseline_input();
        let row = &mut input.rows[3];
        let mapping = row.imported_mapping.as_mut().expect("imported mapping");
        mapping.outcome_label = ImportedOutcomeLabel::Unsupported;
        mapping.mapping_diagnostic_ref = None;
        let packet = ScopeProvenanceTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::ImportedMissingDiagnostic));
    }

    #[test]
    fn non_canonical_presented_as_canonical_blocks_stable() {
        let mut input = baseline_input();
        input.rows[1].downgrade_state = DowngradeState::Canonical;
        let packet = ScopeProvenanceTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::NonCanonicalPresentedAsCanonical
        }));
    }

    #[test]
    fn missing_required_item_class_blocks_stable() {
        let mut input = baseline_input();
        input.rows.retain(|row| row.item_class != ItemClass::ImportedProvider);
        input
            .covered_item_classes
            .retain(|class| *class != ItemClass::ImportedProvider);
        let packet = ScopeProvenanceTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingItemClassCoverage));
    }

    #[test]
    fn missing_consumer_projection_blocks_stable() {
        let mut input = baseline_input();
        input.consumer_projections =
            vec![sample_projection(ConsumerSurface::SearchShell)];
        let packet = ScopeProvenanceTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn projection_drops_imported_outcome_blocks_stable() {
        let mut input = baseline_input();
        input.consumer_projections = ConsumerSurface::REQUIRED
            .iter()
            .copied()
            .map(|surface| {
                let mut projection = sample_projection(surface);
                if surface == ConsumerSurface::DocsHelp {
                    projection.preserves_imported_outcome_vocabulary = false;
                }
                projection
            })
            .collect();
        let packet = ScopeProvenanceTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::ImportedOutcomeVocabularyDropped
        }));
    }

    #[test]
    fn support_export_is_export_safe_when_packet_is_stable() {
        let packet = ScopeProvenanceTruthPacket::materialize(baseline_input());
        let export =
            packet.support_export("export:test", "2026-05-26T12:00:10Z");
        assert!(export.is_export_safe());
    }
}
