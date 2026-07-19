//! Cross-surface consumers for the frozen M5 profiler/topology component
//! families.
//!
//! This module is the M05-802 first-consumer adoption lane over the frozen M5
//! profiler/topology component matrix (the
//! [`m5-profile-session-hotpath-components`], `m5-trace-heap-compare-components`,
//! `m5-workset-topology-components`, and `m5-ownership-explainer-components`
//! producer packets). Where 797-800 hardened each component *family* and 801
//! certified accessibility fallback, this lane proves the families are reusable
//! *primitives* rather than profile-only or graph-only implementations by
//! adopting them across the five claimed M5 consumer classes:
//!
//! 1. performance tooling (profiler / hotspot / compare),
//! 2. search / graph understanding,
//! 3. one onboarding or explainer entry path,
//! 4. one AI / review consumer, and
//! 5. one incident / support surface.
//!
//! Each adoption row points back to exactly one canonical component family
//! (schema + producer packet) instead of cloning surface-local prose, and every
//! consumer — even a read-only, inspect-only, compare-only, or export-only one —
//! keeps the same capture/source badges, workset scope language, citation
//! vocabulary, and degraded-state labels. Narrower consumers disclose the
//! reduction with a reduced-capability banner (and, when they punt to another
//! surface, a companion/browser/handoff note) rather than renaming or dropping
//! governed state.
//!
//! The disclosure vocabulary (`CopyExportProjection`, `ReducedCapabilityBanner`,
//! `SupportExportJoin`, `AutoNarrowingContract`, `ComponentConsumerSurface`,
//! `FreshnessState`, `Confidence`, `ProvenanceClass`) is re-used verbatim from
//! [`aureline_graph::m5_workset_topology_components`] so the exported labels stay
//! byte-identical to the graph-side component packets and there is no controlled
//! vocabulary drift.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use aureline_graph::{
    AutoNarrowingContract, ComponentConsumerSurface, Confidence, CopyExportProjection,
    FreshnessState, ProvenanceClass, ReducedCapabilityBanner, SupportExportJoin,
};

/// Schema version stamped on the M05-802 cross-surface consumer packet.
pub const CROSS_SURFACE_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`CrossSurfaceConsumerPacket`].
pub const CROSS_SURFACE_CONSUMER_RECORD_KIND: &str = "m5_cross_surface_component_consumer_packet";

/// Stable record-kind tag for each [`CrossSurfaceConsumerRow`].
pub const CROSS_SURFACE_CONSUMER_ROW_RECORD_KIND: &str = "m5_cross_surface_component_consumer_row";

/// Repo-relative path to the checked-in M05-802 packet.
pub const CROSS_SURFACE_CONSUMER_PACKET_PATH: &str =
    "artifacts/support/m5/m5-cross-surface-component-consumers.json";

/// Schema for the M05-802 cross-surface consumer packet.
pub const CROSS_SURFACE_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-cross-surface-component-consumer.schema.json";

/// Frozen component matrix this packet consumes by reference.
pub const CROSS_SURFACE_CONSUMER_MATRIX_REF: &str =
    "artifacts/design/m5-profiler-topology-component-matrix.md";

/// Embedded checked-in M05-802 packet JSON.
pub const CROSS_SURFACE_CONSUMER_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/support/m5/m5-cross-surface-component-consumers.json"
));

/// The ten frozen M5 profiler/topology component families.
///
/// This mirrors the profiler-crate `M5ComponentFamily` enum by name; the support
/// crate references the families by identity rather than depending on
/// `aureline-profiler`, so a support/incident consumer can still point back to
/// exactly one canonical family for a performance or topology evidence object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComponentFamily {
    ProfileSessionCard,
    FlamegraphView,
    IcicleView,
    CallTreeRow,
    HeapProfileCompareCard,
    TraceTimeline,
    WorksetSwitcherRow,
    TopologyNodeCard,
    OwnershipCard,
    ExplainerSectionCard,
}

impl M5ComponentFamily {
    /// Every component family, in matrix order.
    pub const ALL: [M5ComponentFamily; 10] = [
        M5ComponentFamily::ProfileSessionCard,
        M5ComponentFamily::FlamegraphView,
        M5ComponentFamily::IcicleView,
        M5ComponentFamily::CallTreeRow,
        M5ComponentFamily::HeapProfileCompareCard,
        M5ComponentFamily::TraceTimeline,
        M5ComponentFamily::WorksetSwitcherRow,
        M5ComponentFamily::TopologyNodeCard,
        M5ComponentFamily::OwnershipCard,
        M5ComponentFamily::ExplainerSectionCard,
    ];

    /// The single canonical schema that defines this family. Consumers must
    /// point at this schema instead of inventing a surface-local one.
    pub const fn canonical_schema_ref(self) -> &'static str {
        match self {
            M5ComponentFamily::ProfileSessionCard => {
                "schemas/ui/m5-profile-session-card.schema.json"
            }
            // Flamegraph, icicle, and heap/profile-compare share the profile-cost
            // schema per the frozen matrix.
            M5ComponentFamily::FlamegraphView
            | M5ComponentFamily::IcicleView
            | M5ComponentFamily::HeapProfileCompareCard => {
                "schemas/ui/m5-flamegraph-view.schema.json"
            }
            M5ComponentFamily::CallTreeRow => "schemas/ui/m5-call-tree-row.schema.json",
            M5ComponentFamily::TraceTimeline => "schemas/ui/m5-trace-timeline.schema.json",
            M5ComponentFamily::WorksetSwitcherRow => {
                "schemas/ui/m5-workset-switcher-row.schema.json"
            }
            M5ComponentFamily::TopologyNodeCard => "schemas/ui/m5-topology-node-card.schema.json",
            M5ComponentFamily::OwnershipCard => "schemas/ui/m5-ownership-card.schema.json",
            M5ComponentFamily::ExplainerSectionCard => {
                "schemas/ui/m5-explainer-section-card.schema.json"
            }
        }
    }

    /// The canonical producer packet that defines this family's first consumers.
    pub const fn canonical_packet_ref(self) -> &'static str {
        match self {
            M5ComponentFamily::ProfileSessionCard
            | M5ComponentFamily::FlamegraphView
            | M5ComponentFamily::IcicleView
            | M5ComponentFamily::CallTreeRow => {
                "artifacts/perf/m5/m5-profile-session-hotpath-components.json"
            }
            M5ComponentFamily::HeapProfileCompareCard | M5ComponentFamily::TraceTimeline => {
                "artifacts/perf/m5/m5-trace-heap-compare-components.json"
            }
            M5ComponentFamily::WorksetSwitcherRow | M5ComponentFamily::TopologyNodeCard => {
                "artifacts/graph/m5/m5-workset-topology-components.json"
            }
            M5ComponentFamily::OwnershipCard | M5ComponentFamily::ExplainerSectionCard => {
                "artifacts/graph/m5/m5-ownership-explainer-components.json"
            }
        }
    }
}

/// The five claimed M5 consumer classes that must each adopt at least one
/// canonical component family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerGroup {
    /// Performance tooling: profiler / hotspot / trace / heap / compare surfaces.
    PerformanceTooling,
    /// Search and graph/codebase understanding surfaces.
    SearchGraphUnderstanding,
    /// An onboarding or explainer entry path.
    OnboardingExplainer,
    /// An AI or review consumer.
    AiReview,
    /// An incident or support surface.
    IncidentSupport,
}

impl ConsumerGroup {
    /// Every consumer group that must be present for cross-surface reuse.
    pub const ALL: [ConsumerGroup; 5] = [
        ConsumerGroup::PerformanceTooling,
        ConsumerGroup::SearchGraphUnderstanding,
        ConsumerGroup::OnboardingExplainer,
        ConsumerGroup::AiReview,
        ConsumerGroup::IncidentSupport,
    ];
}

/// The rendering authority a consumer exercises over a canonical component.
///
/// A consumer may narrow authority (read-only, inspect-only, compare-only,
/// export-only, policy-blocked) but never rename or drop the governed state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityMode {
    FullInteractive,
    ReadOnly,
    InspectOnly,
    CompareOnly,
    ExportOnly,
    PolicyBlocked,
}

impl AuthorityMode {
    /// Returns true when the consumer narrows below full-interactive authority
    /// and therefore must disclose the reduction with a banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullInteractive)
    }

    /// The matrix `capability_state` label this authority maps to.
    pub const fn capability_state(self) -> &'static str {
        match self {
            Self::FullInteractive => "full",
            Self::ReadOnly => "read_only",
            Self::InspectOnly => "inspect_only",
            Self::CompareOnly => "compare_only",
            Self::ExportOnly => "export_only",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// The surface a narrower consumer hands off to when it cannot render the full
/// component locally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffTarget {
    /// No handoff: the consumer renders the component in-place.
    None,
    CompanionApp,
    BrowserReadonly,
    HandoffPacket,
    CliHeadless,
}

impl HandoffTarget {
    /// Returns true when the consumer punts to another surface and therefore
    /// must carry a companion/browser/handoff note.
    pub const fn requires_note(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Whether the consumer preserves the canonical component's controlled labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelParityState {
    /// Full badge/scope/citation/degraded-state label parity with the family.
    Preserved,
    /// Reduced interactivity, disclosed, but the labels are still preserved.
    DisclosedNarrowed,
    /// A label was renamed, flattened, or dropped (red; blocks review).
    RenamedOrDropped,
}

impl LabelParityState {
    /// Returns true when no controlled label is renamed or dropped.
    pub const fn keeps_labels(self) -> bool {
        !matches!(self, Self::RenamedOrDropped)
    }
}

/// One consumer adopting one canonical component family on one surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossSurfaceConsumerRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub row_id: String,
    pub consumer_group: ConsumerGroup,
    pub consumer_surface: ComponentConsumerSurface,
    /// The single canonical component family this consumer reuses.
    pub component_family: M5ComponentFamily,
    /// The canonical schema for the family. Must equal
    /// `component_family.canonical_schema_ref()`.
    pub canonical_family_schema_ref: String,
    /// The canonical producer packet(s) this consumer points back to.
    #[serde(default)]
    pub canonical_packet_refs: Vec<String>,
    /// True when the consumer references the canonical family rather than
    /// cloning surface-local prose.
    pub references_canonical_not_local_prose: bool,
    pub authority_mode: AuthorityMode,
    /// The evidence object's freshness, kept visible on every consumer.
    pub freshness_state: FreshnessState,
    /// The capture/source badges and workset scope / citation labels the
    /// consumer preserves verbatim.
    #[serde(default)]
    pub preserved_badge_labels: Vec<String>,
    /// The degraded-state vocabulary (fresh/stale/partial/policy-limited, etc.)
    /// the consumer keeps visible even when narrowed.
    #[serde(default)]
    pub degraded_state_vocab: Vec<String>,
    pub label_parity: LabelParityState,
    /// The surface a narrower consumer hands off to, if any.
    pub handoff_target: HandoffTarget,
    /// The companion/browser/handoff note ref; required when `handoff_target`
    /// is not `None`.
    #[serde(default)]
    pub handoff_note_ref: String,
    #[serde(default)]
    pub source_refs: Vec<String>,
    pub copy_export: CopyExportProjection,
    pub reduced_capability_banner: ReducedCapabilityBanner,
    pub support_export_join: SupportExportJoin,
    pub auto_narrowing_contract: AutoNarrowingContract,
}

impl CrossSurfaceConsumerRow {
    /// Returns true when the consumer narrows below full authority.
    pub fn is_narrowed(&self) -> bool {
        self.authority_mode.is_narrowed()
    }

    /// AC2: the consumer points back to exactly one canonical family — the
    /// declared schema matches the family, a producer packet is referenced, and
    /// no surface-local prose is cloned.
    pub fn points_to_canonical_family(&self) -> bool {
        self.canonical_family_schema_ref == self.component_family.canonical_schema_ref()
            && self
                .canonical_packet_refs
                .iter()
                .any(|p| p == self.component_family.canonical_packet_ref())
            && self.references_canonical_not_local_prose
    }

    /// AC1: the consumer preserves the family's controlled badges, scope/citation
    /// language, and degraded-state vocabulary rather than renaming or omitting
    /// them.
    pub fn preserves_labels(&self) -> bool {
        self.label_parity.keeps_labels()
            && !self.preserved_badge_labels.is_empty()
            && !self.degraded_state_vocab.is_empty()
    }

    /// A narrower consumer discloses the reduction with a reduced-capability
    /// banner whose state matches the authority mode, and carries a
    /// companion/browser/handoff note whenever it punts to another surface.
    pub fn discloses_narrowing(&self) -> bool {
        if self.is_narrowed() {
            let banner = &self.reduced_capability_banner;
            if banner.banner_id.trim().is_empty()
                || banner.visible_label.trim().is_empty()
                || banner.capability_state != self.authority_mode.capability_state()
                || banner.capability_state == "full"
                || banner.missing_capabilities.is_empty()
            {
                return false;
            }
        }
        if self.handoff_target.requires_note() && self.handoff_note_ref.trim().is_empty() {
            return false;
        }
        true
    }

    /// Returns true when this consumer is a help/support/release evidence
    /// surface that must reference the canonical component packets.
    pub fn is_help_support_release_surface(&self) -> bool {
        matches!(
            self.consumer_surface,
            ComponentConsumerSurface::DocsHelp
                | ComponentConsumerSurface::SupportExport
                | ComponentConsumerSurface::ReleaseProof
        )
    }
}

/// Rolled-up summary of an M05-802 cross-surface consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossSurfaceConsumerSummary {
    pub row_count: usize,
    pub consumer_group_count: usize,
    pub consumer_surface_count: usize,
    pub component_family_count: usize,
    pub all_rows_point_to_canonical_family: bool,
    pub all_rows_preserve_labels: bool,
    pub all_narrowed_rows_disclose: bool,
    pub all_rows_have_copy_export: bool,
    pub performance_consumer_present: bool,
    pub search_graph_consumer_present: bool,
    pub onboarding_explainer_consumer_present: bool,
    pub ai_review_consumer_present: bool,
    pub incident_support_consumer_present: bool,
    pub help_support_release_reference_present: bool,
}

/// Checked-in M05-802 cross-surface consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossSurfaceConsumerPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<CrossSurfaceConsumerRow>,
    pub summary: CrossSurfaceConsumerSummary,
}

impl CrossSurfaceConsumerPacket {
    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> CrossSurfaceConsumerSummary {
        let mut groups = BTreeSet::new();
        let mut surfaces = BTreeSet::new();
        let mut families = BTreeSet::new();
        for row in &self.rows {
            groups.insert(row.consumer_group);
            surfaces.insert(row.consumer_surface);
            families.insert(row.component_family);
        }

        let has_group = |g: ConsumerGroup| groups.contains(&g);

        CrossSurfaceConsumerSummary {
            row_count: self.rows.len(),
            consumer_group_count: groups.len(),
            consumer_surface_count: surfaces.len(),
            component_family_count: families.len(),
            all_rows_point_to_canonical_family: self
                .rows
                .iter()
                .all(CrossSurfaceConsumerRow::points_to_canonical_family),
            all_rows_preserve_labels: self
                .rows
                .iter()
                .all(CrossSurfaceConsumerRow::preserves_labels),
            all_narrowed_rows_disclose: self
                .rows
                .iter()
                .all(CrossSurfaceConsumerRow::discloses_narrowing),
            all_rows_have_copy_export: self.rows.iter().all(|r| r.copy_export.is_export_safe()),
            performance_consumer_present: has_group(ConsumerGroup::PerformanceTooling),
            search_graph_consumer_present: has_group(ConsumerGroup::SearchGraphUnderstanding),
            onboarding_explainer_consumer_present: has_group(ConsumerGroup::OnboardingExplainer),
            ai_review_consumer_present: has_group(ConsumerGroup::AiReview),
            incident_support_consumer_present: has_group(ConsumerGroup::IncidentSupport),
            help_support_release_reference_present: self.rows.iter().any(|r| {
                r.is_help_support_release_surface() && r.references_canonical_not_local_prose
            }),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<CrossSurfaceConsumerViolation> {
        let mut violations = Vec::new();

        if self.schema_version != CROSS_SURFACE_CONSUMER_SCHEMA_VERSION {
            violations.push(CrossSurfaceConsumerViolation::SchemaVersion {
                expected: CROSS_SURFACE_CONSUMER_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != CROSS_SURFACE_CONSUMER_RECORD_KIND {
            violations.push(CrossSurfaceConsumerViolation::RecordKind {
                expected: CROSS_SURFACE_CONSUMER_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_groups = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(CrossSurfaceConsumerViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_groups.insert(row.consumer_group);

            if row.record_kind != CROSS_SURFACE_CONSUMER_ROW_RECORD_KIND
                || row.schema_version != CROSS_SURFACE_CONSUMER_SCHEMA_VERSION
                || row.canonical_family_schema_ref.trim().is_empty()
                || row.canonical_packet_refs.is_empty()
            {
                violations.push(CrossSurfaceConsumerViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // AC2: exactly one canonical family, no cloned surface-local prose.
            if !row.points_to_canonical_family() {
                violations.push(CrossSurfaceConsumerViolation::NotCanonicalFamily {
                    id: row.row_id.clone(),
                });
            }

            // AC1: controlled badges/scope/citation/degraded labels preserved.
            if !row.preserves_labels() {
                violations.push(CrossSurfaceConsumerViolation::LabelParityBroken {
                    id: row.row_id.clone(),
                });
            }

            // Narrower consumers disclose reduction with banner + handoff note.
            if !row.discloses_narrowing() {
                violations.push(CrossSurfaceConsumerViolation::NarrowedWithoutDisclosure {
                    id: row.row_id.clone(),
                });
            }

            // Copy/export parity: text/JSON/Markdown, screenshot prohibited.
            if !row.copy_export.is_export_safe() {
                violations.push(CrossSurfaceConsumerViolation::MissingCopyExportParity {
                    id: row.row_id.clone(),
                });
            }
        }

        // AC1: cross-surface reuse spans all five claimed consumer classes.
        for group in ConsumerGroup::ALL {
            if !seen_groups.contains(&group) {
                violations.push(CrossSurfaceConsumerViolation::MissingConsumerGroup { group });
            }
        }

        // AC3: help/support/release artifacts reference the canonical components.
        if !self
            .rows
            .iter()
            .any(|r| r.is_help_support_release_surface() && r.references_canonical_not_local_prose)
        {
            violations.push(CrossSurfaceConsumerViolation::MissingHelpSupportReleaseReference);
        }

        if self.summary != self.computed_summary() {
            violations.push(CrossSurfaceConsumerViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in M05-802 packet.
pub fn current_cross_surface_consumer_packet(
) -> Result<CrossSurfaceConsumerPacket, serde_json::Error> {
    serde_json::from_str(CROSS_SURFACE_CONSUMER_PACKET_JSON)
}

/// Validation failure for M05-802 cross-surface consumer packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrossSurfaceConsumerViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    DuplicateId { id: String },
    IncompleteRow { id: String },
    NotCanonicalFamily { id: String },
    LabelParityBroken { id: String },
    NarrowedWithoutDisclosure { id: String },
    MissingCopyExportParity { id: String },
    MissingConsumerGroup { group: ConsumerGroup },
    MissingHelpSupportReleaseReference,
    SummaryMismatch,
}

impl fmt::Display for CrossSurfaceConsumerViolation {
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
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete consumer row: {id}"),
            Self::NotCanonicalFamily { id } => {
                write!(
                    f,
                    "row {id} does not point back to exactly one canonical component family"
                )
            }
            Self::LabelParityBroken { id } => {
                write!(
                    f,
                    "row {id} renames or drops a canonical badge/scope/citation/degraded label"
                )
            }
            Self::NarrowedWithoutDisclosure { id } => {
                write!(
                    f,
                    "row {id} narrows authority without a reduced-capability banner or handoff note"
                )
            }
            Self::MissingCopyExportParity { id } => {
                write!(
                    f,
                    "row {id} is missing text/JSON/Markdown copy-export parity"
                )
            }
            Self::MissingConsumerGroup { group } => {
                write!(f, "consumer group {group:?} is not adopted in the packet")
            }
            Self::MissingHelpSupportReleaseReference => write!(
                f,
                "no help/support/release consumer references the canonical component families"
            ),
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
        }
    }
}

impl Error for CrossSurfaceConsumerViolation {}
