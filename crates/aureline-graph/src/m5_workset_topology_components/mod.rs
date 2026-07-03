//! Reusable workset switcher rows, repo-lens scope banners, topology node cards,
//! and relationship chips.
//!
//! This module is the M05-799 component contract that layers the frozen
//! `workset_switcher_row` and `topology_node_card` families of the M5
//! profiler/topology matrix onto checked-in graph/scope fixtures. It keeps
//! workset scope, included/excluded roots, index coverage, and no-silent-widening
//! state explicit on workset switcher rows (the repo-lens scope banner is the
//! row's scope disclosure), and keeps freshness, confidence, provenance, impact
//! direction, and partial/blocked language explicit on topology node cards and
//! their relationship chips.
//!
//! The graph, search, review, onboarding, and AI consumers reference this packet
//! instead of inferring hidden workset widening or full-graph certainty: a result
//! or explainer can always state whether it is limited by the active workset, and
//! the fresh/warming/stale/cached/partial/policy-limited vocabulary survives every
//! copy/export projection.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped on the M05-799 component packet.
pub const WORKSET_TOPOLOGY_COMPONENT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`WorksetTopologyComponentPacket`].
pub const WORKSET_TOPOLOGY_COMPONENT_RECORD_KIND: &str = "m5_workset_topology_component_packet";

/// Repo-relative path to the checked-in M05-799 packet.
pub const WORKSET_TOPOLOGY_COMPONENT_PACKET_PATH: &str =
    "artifacts/graph/m5/m5-workset-topology-components.json";

/// Frozen component matrix this packet consumes by reference.
pub const WORKSET_TOPOLOGY_COMPONENT_MATRIX_REF: &str =
    "artifacts/design/m5-profiler-topology-component-matrix.md";

/// Schema for the workset switcher row family.
pub const WORKSET_SWITCHER_ROW_SCHEMA_REF: &str = "schemas/ui/m5-workset-switcher-row.schema.json";

/// Schema for the topology node card family.
pub const TOPOLOGY_NODE_CARD_SCHEMA_REF: &str = "schemas/ui/m5-topology-node-card.schema.json";

/// Embedded checked-in M05-799 packet JSON.
pub const WORKSET_TOPOLOGY_COMPONENT_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/graph/m5/m5-workset-topology-components.json"
));

/// Claimed consumer surface, aligned to the matrix `consumer_surface` vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentConsumerSurface {
    DesktopProfilerWorkspace,
    HotspotWorkspace,
    TraceViewer,
    HeapAnalysis,
    ProfileCompare,
    TopologyMap,
    OwnershipBrowser,
    ArchitectureExplainer,
    SearchResults,
    ReviewWorkspace,
    OnboardingTour,
    AiContextPanel,
    IncidentWorkspace,
    DocsHelp,
    CliHeadless,
    SupportExport,
    ReleaseProof,
}

/// Workset scope mode from the matrix `workset_scope` vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorksetScope {
    FullWorkspace,
    NamedWorkset,
    SparseSlice,
    ImportedSnapshot,
    SupportBundleScope,
    Unknown,
}

impl WorksetScope {
    /// Returns true when the scope covers the whole workspace (org-wide truth).
    pub const fn is_org_wide(self) -> bool {
        matches!(self, Self::FullWorkspace)
    }

    /// Returns true when the scope is a limited slice and consumers must not
    /// imply org-wide truth.
    pub const fn is_scope_limited(self) -> bool {
        !self.is_org_wide()
    }
}

/// Index coverage state disclosed by a workset switcher row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageState {
    FullLoaded,
    SparseSliceLoaded,
    PartialLoaded,
    ImportedSnapshot,
    Unknown,
}

/// Source of the workset scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeSource {
    Local,
    Managed,
    Imported,
    SupportBundle,
    Unknown,
}

/// Scope-change affordance state from the matrix `scope_change_state` vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeChangeState {
    Unchanged,
    ExplicitWidenAvailable,
    ExplicitNarrowAvailable,
    SuggestedWidenRequiresReview,
    PolicyBlocked,
    Unknown,
}

/// Direction of a scope-change action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeChangeDirection {
    Widen,
    Narrow,
    Keep,
}

/// Freshness vocabulary shared with the matrix `freshness_state` labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessState {
    Live,
    Current,
    WarmCached,
    Cached,
    Imported,
    Stale,
    Superseded,
    Partial,
    Expired,
    PolicyLimited,
    Unknown,
}

impl FreshnessState {
    /// Returns true when the surface must disclose degraded/limited freshness.
    pub const fn is_degraded(self) -> bool {
        matches!(
            self,
            Self::Stale
                | Self::Superseded
                | Self::Partial
                | Self::Expired
                | Self::PolicyLimited
                | Self::Unknown
        )
    }
}

/// Confidence vocabulary shared with the matrix `confidence` labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    Confirmed,
    High,
    Medium,
    Low,
    Unknown,
}

/// Provenance vocabulary shared with the matrix `provenance_class` labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceClass {
    Indexed,
    Imported,
    Inferred,
    Provider,
    Annotation,
    RuntimeCapture,
    Curated,
    Generated,
}

/// Relation fidelity vocabulary shown by topology node cards and chips.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationFidelity {
    Exact,
    Approximate,
    Imported,
    Partial,
    Stale,
    Blocked,
}

impl RelationFidelity {
    /// Returns true when the edge must carry a visible partial/blocked note.
    pub const fn requires_partiality_note(self) -> bool {
        matches!(self, Self::Partial | Self::Stale | Self::Blocked)
    }
}

/// Node kind vocabulary from the topology node card schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeKind {
    File,
    Directory,
    Symbol,
    Module,
    Doc,
    Ownership,
    ProviderResource,
    WorksetScope,
}

/// Direction of a relationship chip relative to the anchor node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeDirection {
    Incoming,
    Outgoing,
}

/// Index coverage disclosure carried by a workset switcher row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexCoverage {
    pub covered_node_count: u64,
    pub covered_edge_count: u64,
    pub not_loaded_count: u64,
    pub hidden_result_count: u64,
    pub coverage_state: CoverageState,
}

/// A scope-change action offered by a workset switcher row / repo-lens banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeAction {
    pub action_id: String,
    pub direction: ScopeChangeDirection,
    pub requires_review: bool,
}

/// Copy/export projection preserving controlled labels across text/JSON/Markdown.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyExportProjection {
    #[serde(default)]
    pub formats: Vec<String>,
    #[serde(default)]
    pub export_fields: Vec<String>,
    pub text: String,
    pub json: String,
    pub markdown: String,
    pub screenshot_only_prohibited: bool,
}

impl CopyExportProjection {
    /// Returns true when the projection is copy/export safe across all formats.
    pub fn is_export_safe(&self) -> bool {
        self.screenshot_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }

    /// Returns true when every named field survives into the export projection.
    pub fn exports_all(&self, fields: &[&str]) -> bool {
        fields
            .iter()
            .all(|needle| self.export_fields.iter().any(|f| f == needle))
    }
}

/// Reduced-capability banner shown when a component is narrowed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReducedCapabilityBanner {
    pub banner_id: String,
    pub severity: String,
    pub capability_state: String,
    pub visible_label: String,
    #[serde(default)]
    pub missing_capabilities: Vec<String>,
    #[serde(default)]
    pub preserved_fields: Vec<String>,
}

/// Support-export join with raw-material exclusion flags.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportJoin {
    pub join_id: String,
    pub schema_ref: String,
    #[serde(default)]
    pub joined_object_kinds: Vec<String>,
    pub raw_profile_samples_exported: bool,
    pub raw_trace_events_exported: bool,
    pub gui_cli_support_label_parity: bool,
}

/// Auto-narrowing contract capping claims on missing/stale/policy-blocked truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutoNarrowingContract {
    #[serde(default)]
    pub narrow_on_missing_or_stale: Vec<String>,
    pub stale_or_missing_effect: String,
    pub policy_blocked_effect: String,
    pub degraded_state_reason_field: String,
    pub release_help_claim_ceiling: String,
}

/// Reusable workset switcher row; its scope disclosure is the repo-lens banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetSwitcherRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub row_id: String,
    pub workset_ref: String,
    pub snapshot_ref: String,
    pub workset_scope: WorksetScope,
    #[serde(default)]
    pub included_root_refs: Vec<String>,
    #[serde(default)]
    pub excluded_root_refs: Vec<String>,
    pub index_coverage: IndexCoverage,
    pub scope_source: ScopeSource,
    pub surface_binding_ref: String,
    pub no_silent_widening: bool,
    pub scope_change_state: ScopeChangeState,
    #[serde(default)]
    pub actions: Vec<ScopeAction>,
    pub freshness_state: FreshnessState,
    #[serde(default)]
    pub source_refs: Vec<String>,
    #[serde(default)]
    pub consumer_surfaces: Vec<ComponentConsumerSurface>,
    pub copy_export: CopyExportProjection,
    pub reduced_capability_banner: ReducedCapabilityBanner,
    pub support_export_join: SupportExportJoin,
    pub auto_narrowing_contract: AutoNarrowingContract,
}

impl WorksetSwitcherRow {
    /// Number of included repos/folder roots (repo-lens count).
    pub fn included_root_count(&self) -> usize {
        self.included_root_refs.len()
    }

    /// Returns true when this row represents a scope-limited slice; consumers may
    /// state the workset limitation instead of implying org-wide truth.
    pub fn is_scope_limited(&self) -> bool {
        self.workset_scope.is_scope_limited()
    }

    /// Returns true when this row would allow the scope to widen implicitly: the
    /// no-silent-widening flag is off, or a widen action is offered without
    /// review. This must never be true for a governed row (AC2).
    pub fn permits_silent_widening(&self) -> bool {
        !self.no_silent_widening
            || self
                .actions
                .iter()
                .any(|a| a.direction == ScopeChangeDirection::Widen && !a.requires_review)
    }
}

/// Edge summary carried by a topology node card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdgeSummary {
    pub incoming_count: u64,
    pub outgoing_count: u64,
    pub hidden_edge_count: u64,
    #[serde(default)]
    pub edge_refs: Vec<String>,
}

/// Reusable topology node card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyNodeCard {
    pub record_kind: String,
    pub schema_version: u32,
    pub card_id: String,
    pub node_ref: String,
    pub node_kind: NodeKind,
    pub namespace_ref: String,
    pub workspace_ref: String,
    pub active_workset_snapshot_ref: String,
    pub freshness_state: FreshnessState,
    pub confidence: Confidence,
    pub provenance_class: ProvenanceClass,
    pub edge_summary: EdgeSummary,
    pub relation_fidelity: RelationFidelity,
    #[serde(default)]
    pub ownership_refs: Vec<String>,
    #[serde(default)]
    pub explainer_refs: Vec<String>,
    pub export_permalink_ref: String,
    #[serde(default)]
    pub source_refs: Vec<String>,
    #[serde(default)]
    pub consumer_surfaces: Vec<ComponentConsumerSurface>,
    pub copy_export: CopyExportProjection,
    pub reduced_capability_banner: ReducedCapabilityBanner,
    pub support_export_join: SupportExportJoin,
    pub auto_narrowing_contract: AutoNarrowingContract,
}

impl TopologyNodeCard {
    /// Returns true when freshness, confidence, and provenance survive the export
    /// projection (fresh/warming/stale/cached/partial/policy-limited language must
    /// stay visible across consumers).
    pub fn preserves_truth_in_export(&self) -> bool {
        self.copy_export
            .exports_all(&["freshness_state", "confidence", "provenance_class"])
    }
}

/// Reusable relationship chip attached to a topology node card.
///
/// A chip is one governed edge rendered next to the anchor node: it preserves the
/// related node's kind/name/ID, impact direction and count, relation fidelity,
/// confidence, provenance source, and any partial/blocked note.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationshipChip {
    pub chip_id: String,
    pub node_ref: String,
    pub related_node_ref: String,
    pub related_node_kind: NodeKind,
    pub related_node_label: String,
    pub direction: EdgeDirection,
    pub impact_count: u64,
    pub relation_fidelity: RelationFidelity,
    pub confidence: Confidence,
    pub provenance_class: ProvenanceClass,
    #[serde(default)]
    pub partial_or_blocked_note_ref: String,
}

impl RelationshipChip {
    /// Returns true when the chip must disclose a partial/blocked note but does
    /// not carry one.
    pub fn missing_required_note(&self) -> bool {
        self.relation_fidelity.requires_partiality_note()
            && self.partial_or_blocked_note_ref.trim().is_empty()
    }
}

/// A consumer that references one or more component instances.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentConsumerProjection {
    pub consumer_surface: ComponentConsumerSurface,
    #[serde(default)]
    pub component_refs: Vec<String>,
}

/// Rolled-up summary of an M05-799 component packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetTopologyComponentSummary {
    pub workset_switcher_row_count: usize,
    pub topology_node_card_count: usize,
    pub relationship_chip_count: usize,
    pub consumer_projection_count: usize,
    pub search_consumer_present: bool,
    pub topology_consumer_present: bool,
    pub scope_limited_and_full_both_present: bool,
    pub no_row_permits_silent_widening: bool,
    pub all_nodes_preserve_freshness_confidence_provenance: bool,
    pub all_components_have_copy_export: bool,
}

/// Checked-in M05-799 component packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetTopologyComponentPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub workset_switcher_rows: Vec<WorksetSwitcherRow>,
    #[serde(default)]
    pub topology_node_cards: Vec<TopologyNodeCard>,
    #[serde(default)]
    pub relationship_chips: Vec<RelationshipChip>,
    #[serde(default)]
    pub consumer_projection_rows: Vec<ComponentConsumerProjection>,
    pub summary: WorksetTopologyComponentSummary,
}

impl WorksetTopologyComponentPacket {
    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> WorksetTopologyComponentSummary {
        let mut consumers = BTreeSet::new();
        for row in &self.consumer_projection_rows {
            consumers.insert(row.consumer_surface);
        }
        for row in &self.workset_switcher_rows {
            consumers.extend(row.consumer_surfaces.iter().copied());
        }
        for card in &self.topology_node_cards {
            consumers.extend(card.consumer_surfaces.iter().copied());
        }

        let scope_limited_and_full_both_present = self
            .workset_switcher_rows
            .iter()
            .any(|r| r.is_scope_limited())
            && self
                .workset_switcher_rows
                .iter()
                .any(|r| r.workset_scope.is_org_wide());

        let no_row_permits_silent_widening = self
            .workset_switcher_rows
            .iter()
            .all(|r| !r.permits_silent_widening());

        let all_nodes_preserve_freshness_confidence_provenance = self
            .topology_node_cards
            .iter()
            .all(TopologyNodeCard::preserves_truth_in_export);

        let all_components_have_copy_export = self
            .workset_switcher_rows
            .iter()
            .all(|r| r.copy_export.is_export_safe())
            && self
                .topology_node_cards
                .iter()
                .all(|c| c.copy_export.is_export_safe());

        WorksetTopologyComponentSummary {
            workset_switcher_row_count: self.workset_switcher_rows.len(),
            topology_node_card_count: self.topology_node_cards.len(),
            relationship_chip_count: self.relationship_chips.len(),
            consumer_projection_count: self.consumer_projection_rows.len(),
            search_consumer_present: consumers.contains(&ComponentConsumerSurface::SearchResults),
            topology_consumer_present: consumers.contains(&ComponentConsumerSurface::TopologyMap),
            scope_limited_and_full_both_present,
            no_row_permits_silent_widening,
            all_nodes_preserve_freshness_confidence_provenance,
            all_components_have_copy_export,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<WorksetTopologyComponentViolation> {
        let mut violations = Vec::new();

        if self.schema_version != WORKSET_TOPOLOGY_COMPONENT_SCHEMA_VERSION {
            violations.push(WorksetTopologyComponentViolation::SchemaVersion {
                expected: WORKSET_TOPOLOGY_COMPONENT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != WORKSET_TOPOLOGY_COMPONENT_RECORD_KIND {
            violations.push(WorksetTopologyComponentViolation::RecordKind {
                expected: WORKSET_TOPOLOGY_COMPONENT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.workset_switcher_rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(WorksetTopologyComponentViolation::DuplicateId {
                    kind: "workset_switcher_row",
                    id: row.row_id.clone(),
                });
            }
            if row.record_kind != "m5_workset_switcher_row"
                || row.schema_version != 1
                || row.workset_ref.trim().is_empty()
                || row.snapshot_ref.trim().is_empty()
                || row.included_root_refs.is_empty()
                || row.surface_binding_ref.trim().is_empty()
                || row.actions.is_empty()
            {
                violations.push(WorksetTopologyComponentViolation::IncompleteWorksetRow {
                    id: row.row_id.clone(),
                });
            }
            // AC2: workset controls never widen implicitly.
            if row.permits_silent_widening() {
                violations.push(WorksetTopologyComponentViolation::SilentWideningPermitted {
                    id: row.row_id.clone(),
                });
            }
            // AC1: scope + no-silent-widening truth must survive the export so a
            // consumer can state the workset limitation.
            if !row
                .copy_export
                .exports_all(&["workset_scope", "no_silent_widening"])
            {
                violations.push(WorksetTopologyComponentViolation::WorksetScopeNotExported {
                    id: row.row_id.clone(),
                });
            }
            if !row.copy_export.is_export_safe() {
                violations.push(WorksetTopologyComponentViolation::MissingCopyExport {
                    kind: "workset_switcher_row",
                    id: row.row_id.clone(),
                });
            }
            if row.consumer_surfaces.len() < 2
                || !(row
                    .consumer_surfaces
                    .contains(&ComponentConsumerSurface::SearchResults)
                    || row
                        .consumer_surfaces
                        .contains(&ComponentConsumerSurface::TopologyMap))
            {
                violations.push(WorksetTopologyComponentViolation::MissingConsumerParity {
                    kind: "workset_switcher_row",
                    id: row.row_id.clone(),
                });
            }
        }

        let mut node_refs = BTreeSet::new();
        let mut card_ids = BTreeSet::new();
        for card in &self.topology_node_cards {
            if !card_ids.insert(card.card_id.clone()) {
                violations.push(WorksetTopologyComponentViolation::DuplicateId {
                    kind: "topology_node_card",
                    id: card.card_id.clone(),
                });
            }
            node_refs.insert(card.node_ref.clone());
            if card.record_kind != "m5_topology_node_card"
                || card.schema_version != 1
                || card.node_ref.trim().is_empty()
                || card.namespace_ref.trim().is_empty()
                || card.workspace_ref.trim().is_empty()
                || card.active_workset_snapshot_ref.trim().is_empty()
                || card.export_permalink_ref.trim().is_empty()
            {
                violations.push(WorksetTopologyComponentViolation::IncompleteTopologyCard {
                    id: card.card_id.clone(),
                });
            }
            // AC3: fresh/warming/stale/cached/partial/policy-limited language must
            // survive across consumers, alongside confidence and provenance.
            if !card.preserves_truth_in_export() {
                violations.push(
                    WorksetTopologyComponentViolation::TopologyTruthNotExported {
                        id: card.card_id.clone(),
                    },
                );
            }
            if !card.copy_export.is_export_safe() {
                violations.push(WorksetTopologyComponentViolation::MissingCopyExport {
                    kind: "topology_node_card",
                    id: card.card_id.clone(),
                });
            }
            if card.consumer_surfaces.len() < 2
                || !card
                    .consumer_surfaces
                    .contains(&ComponentConsumerSurface::TopologyMap)
            {
                violations.push(WorksetTopologyComponentViolation::MissingConsumerParity {
                    kind: "topology_node_card",
                    id: card.card_id.clone(),
                });
            }
        }

        let mut chip_ids = BTreeSet::new();
        for chip in &self.relationship_chips {
            if !chip_ids.insert(chip.chip_id.clone()) {
                violations.push(WorksetTopologyComponentViolation::DuplicateId {
                    kind: "relationship_chip",
                    id: chip.chip_id.clone(),
                });
            }
            if chip.related_node_ref.trim().is_empty() || chip.related_node_label.trim().is_empty()
            {
                violations.push(
                    WorksetTopologyComponentViolation::IncompleteRelationshipChip {
                        id: chip.chip_id.clone(),
                    },
                );
            }
            // Chips must attach to a topology node card present in the packet.
            if !node_refs.contains(&chip.node_ref) {
                violations.push(
                    WorksetTopologyComponentViolation::DanglingRelationshipChip {
                        id: chip.chip_id.clone(),
                    },
                );
            }
            // Partial/stale/blocked relations must carry a visible note.
            if chip.missing_required_note() {
                violations.push(WorksetTopologyComponentViolation::RelationshipNoteMissing {
                    id: chip.chip_id.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(WorksetTopologyComponentViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in M05-799 packet.
pub fn current_m5_workset_topology_component_packet(
) -> Result<WorksetTopologyComponentPacket, serde_json::Error> {
    serde_json::from_str(WORKSET_TOPOLOGY_COMPONENT_PACKET_JSON)
}

/// Validation failure for M05-799 component packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorksetTopologyComponentViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    DuplicateId { kind: &'static str, id: String },
    IncompleteWorksetRow { id: String },
    SilentWideningPermitted { id: String },
    WorksetScopeNotExported { id: String },
    IncompleteTopologyCard { id: String },
    TopologyTruthNotExported { id: String },
    IncompleteRelationshipChip { id: String },
    DanglingRelationshipChip { id: String },
    RelationshipNoteMissing { id: String },
    MissingCopyExport { kind: &'static str, id: String },
    MissingConsumerParity { kind: &'static str, id: String },
    SummaryMismatch,
}

impl fmt::Display for WorksetTopologyComponentViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::DuplicateId { kind, id } => write!(f, "duplicate {kind} id: {id}"),
            Self::IncompleteWorksetRow { id } => write!(f, "incomplete workset switcher row: {id}"),
            Self::SilentWideningPermitted { id } => {
                write!(f, "workset switcher row {id} permits silent scope widening")
            }
            Self::WorksetScopeNotExported { id } => {
                write!(
                    f,
                    "workset switcher row {id} drops scope / no-silent-widening truth from export"
                )
            }
            Self::IncompleteTopologyCard { id } => write!(f, "incomplete topology node card: {id}"),
            Self::TopologyTruthNotExported { id } => {
                write!(
                    f,
                    "topology node card {id} drops freshness/confidence/provenance from export"
                )
            }
            Self::IncompleteRelationshipChip { id } => {
                write!(f, "incomplete relationship chip: {id}")
            }
            Self::DanglingRelationshipChip { id } => {
                write!(f, "relationship chip {id} references an absent node card")
            }
            Self::RelationshipNoteMissing { id } => {
                write!(
                    f,
                    "relationship chip {id} hides a partial/blocked relation without a note"
                )
            }
            Self::MissingCopyExport { kind, id } => {
                write!(f, "{kind} {id} is missing a copy/export-safe projection")
            }
            Self::MissingConsumerParity { kind, id } => {
                write!(
                    f,
                    "{kind} {id} is missing required plus secondary consumer parity"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
        }
    }
}

impl Error for WorksetTopologyComponentViolation {}
