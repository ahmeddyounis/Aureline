//! Shared framework-pack, preview-runtime, browser-runtime, docs-demo, and
//! onboarding consumers for the frozen M5 visual-designer components.
//!
//! This module is the M05-809 first-consumer adoption lane over the frozen M5
//! visual-designer component matrix
//! ([`crate::freeze_the_m5_design_canvas_structure_tree_property_inspector_source_sync_and_breakpoint_preview_component_matrix`])
//! and the 805-807 primitive resolvers (selected-node, source round-trip
//! honesty, breakpoint / device preview). Where the freeze matrix defines the
//! reusable canvas / tree / inspector / chip / preview-row primitives, 805-807
//! resolve their per-target truth, and 808 certifies accessibility fallback,
//! this lane proves the seven families are reusable *primitives* rather than a
//! single designer-page implementation by adopting them across the four claimed
//! M5 handoff consumer classes:
//!
//! 1. a framework-pack preview consumer,
//! 2. a preview-runtime inspection consumer,
//! 3. a browser-runtime inspection or demo handoff, and
//! 4. a docs / help / onboarding surface.
//!
//! Each [`VisualDesignerConsumerRow`] points back to exactly one canonical
//! component family (the primitive schema + release-proof packet) instead of
//! cloning surface-local prose, and every consumer — even a read-only,
//! inspect-only, compare-only, or export-only one — keeps the identical label
//! families for support class, runtime origin, unsupported constructs,
//! round-trip conflicts, and open-source fallbacks, plus the same token,
//! density, and motion behavior mandated by the design-system contract. A
//! narrower consumer discloses the reduction with a reduced-capability banner
//! (and, when it punts to another surface, a companion / browser / handoff note)
//! rather than renaming or dropping governed state.
//!
//! The controlled vocabulary ([`crate::M5VisualDesignerComponentFamily`],
//! [`crate::M5VisualDesignerRequiredLabel`], and [`crate::CopyExportParity`]) is
//! reused verbatim from the frozen matrix and the sibling primitive packets so
//! the adopted labels stay byte-identical and there is no controlled-vocabulary
//! drift.
//!
//! The packet is metadata-only: raw source bodies, diff hunks, credentials, and
//! provider payloads never cross this boundary; the packet carries only typed
//! class tokens, opaque summary / evidence refs, booleans, and redacted labels.
//!
//! The boundary schema is
//! [`schemas/ui/m5-visual-designer-component-consumer.schema.json`](../../../../schemas/ui/m5-visual-designer-component-consumer.schema.json).
//! The contract doc is
//! [`docs/designer/m5_visual_designer_component_consumer_contract.md`](../../../../docs/designer/m5_visual_designer_component_consumer_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    CopyExportParity, M5VisualDesignerComponentFamily, M5VisualDesignerRequiredLabel,
    M5_BREAKPOINT_PREVIEW_ARTIFACT_REF, M5_BREAKPOINT_PREVIEW_SCHEMA_REF,
    M5_ROUND_TRIP_ARTIFACT_REF, M5_ROUND_TRIP_SCHEMA_REF, M5_SELECTED_NODE_ARTIFACT_REF,
    M5_SELECTED_NODE_SCHEMA_REF,
};

/// Schema version stamped on the M05-809 consumer packet.
pub const VISUAL_DESIGNER_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`VisualDesignerConsumerPacket`].
pub const VISUAL_DESIGNER_CONSUMER_RECORD_KIND: &str =
    "m5_visual_designer_component_consumer_packet";

/// Stable record-kind tag carried by each [`VisualDesignerConsumerRow`].
pub const VISUAL_DESIGNER_CONSUMER_ROW_RECORD_KIND: &str =
    "m5_visual_designer_component_consumer_row";

/// Repo-relative path of the boundary schema.
pub const VISUAL_DESIGNER_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-visual-designer-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const VISUAL_DESIGNER_CONSUMER_DOC_REF: &str =
    "docs/designer/m5_visual_designer_component_consumer_contract.md";

/// Repo-relative path of the frozen visual-designer component matrix these
/// consumers adopt.
pub const VISUAL_DESIGNER_CONSUMER_MATRIX_REF: &str =
    "schemas/ui/m5-visual-designer-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const VISUAL_DESIGNER_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-visual-designer-component-consumers";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const VISUAL_DESIGNER_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-visual-designer-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const VISUAL_DESIGNER_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-visual-designer-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const VISUAL_DESIGNER_CONSUMER_REPORT_REF: &str =
    "artifacts/components/m5-visual-designer-component-consumers.md";

/// The controlled label families a consumer must preserve identically across
/// every surface (support class, runtime origin, unsupported constructs,
/// round-trip conflicts, and open-source fallbacks). The union of every row's
/// `preserved_label_families` must cover this set.
pub const REQUIRED_LABEL_FAMILIES: [&str; 5] = [
    "support_class",
    "runtime_origin",
    "unsupported_construct",
    "round_trip_conflict",
    "open_source_fallback",
];

/// The canonical primitive schema that defines a component family's contract.
/// Consumers must point at this schema instead of inventing a surface-local one.
pub const fn canonical_schema_ref_for(family: M5VisualDesignerComponentFamily) -> &'static str {
    match family {
        // The design canvas, structure tree, and property inspector are resolved
        // by the M05-805 selected-node primitive.
        M5VisualDesignerComponentFamily::DesignCanvas
        | M5VisualDesignerComponentFamily::StructureTreeRow
        | M5VisualDesignerComponentFamily::PropertyInspectorRow => M5_SELECTED_NODE_SCHEMA_REF,
        // The source-sync chip, unsupported-construct card, and round-trip
        // conflict banner are resolved by the M05-806 round-trip honesty
        // primitive.
        M5VisualDesignerComponentFamily::SourceSyncChip
        | M5VisualDesignerComponentFamily::UnsupportedConstructCard
        | M5VisualDesignerComponentFamily::RoundTripConflictBanner => M5_ROUND_TRIP_SCHEMA_REF,
        // The breakpoint / device-preview row is resolved by the M05-807
        // breakpoint primitive.
        M5VisualDesignerComponentFamily::BreakpointPreviewRow => M5_BREAKPOINT_PREVIEW_SCHEMA_REF,
    }
}

/// The canonical release-proof packet that defines a component family's first
/// resolved truth. Consumers point back to this packet rather than cloning it.
pub const fn canonical_packet_ref_for(family: M5VisualDesignerComponentFamily) -> &'static str {
    match family {
        M5VisualDesignerComponentFamily::DesignCanvas
        | M5VisualDesignerComponentFamily::StructureTreeRow
        | M5VisualDesignerComponentFamily::PropertyInspectorRow => M5_SELECTED_NODE_ARTIFACT_REF,
        M5VisualDesignerComponentFamily::SourceSyncChip
        | M5VisualDesignerComponentFamily::UnsupportedConstructCard
        | M5VisualDesignerComponentFamily::RoundTripConflictBanner => M5_ROUND_TRIP_ARTIFACT_REF,
        M5VisualDesignerComponentFamily::BreakpointPreviewRow => M5_BREAKPOINT_PREVIEW_ARTIFACT_REF,
    }
}

/// The four claimed M5 handoff consumer classes that must each adopt at least
/// one canonical component family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerGroup {
    /// A framework-pack preview consumer.
    FrameworkPack,
    /// A preview-runtime inspection consumer.
    PreviewRuntime,
    /// A browser-runtime inspection or demo handoff consumer.
    BrowserRuntimeDemo,
    /// A docs / help / onboarding surface.
    DocsOnboarding,
}

impl ConsumerGroup {
    /// Every consumer group that must be present for cross-surface reuse.
    pub const ALL: [ConsumerGroup; 4] = [
        ConsumerGroup::FrameworkPack,
        ConsumerGroup::PreviewRuntime,
        ConsumerGroup::BrowserRuntimeDemo,
        ConsumerGroup::DocsOnboarding,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrameworkPack => "framework_pack",
            Self::PreviewRuntime => "preview_runtime",
            Self::BrowserRuntimeDemo => "browser_runtime_demo",
            Self::DocsOnboarding => "docs_onboarding",
        }
    }
}

/// The concrete M5 handoff surface a visual-designer component is embedded in.
/// Each surface belongs to exactly one [`ConsumerGroup`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandoffConsumerSurface {
    /// A framework-pack preview lane.
    FrameworkPackPreviewLane,
    /// The preview-runtime inspector.
    PreviewRuntimeInspector,
    /// The browser-runtime inspector.
    BrowserRuntimeInspector,
    /// A demo / share handoff.
    DemoHandoff,
    /// A docs / onboarding walkthrough.
    DocsOnboardingWalkthrough,
    /// The help center.
    HelpCenter,
}

impl M5HandoffConsumerSurface {
    /// The consumer group this surface belongs to.
    pub const fn consumer_group(self) -> ConsumerGroup {
        match self {
            Self::FrameworkPackPreviewLane => ConsumerGroup::FrameworkPack,
            Self::PreviewRuntimeInspector => ConsumerGroup::PreviewRuntime,
            Self::BrowserRuntimeInspector | Self::DemoHandoff => ConsumerGroup::BrowserRuntimeDemo,
            Self::DocsOnboardingWalkthrough | Self::HelpCenter => ConsumerGroup::DocsOnboarding,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrameworkPackPreviewLane => "framework_pack_preview_lane",
            Self::PreviewRuntimeInspector => "preview_runtime_inspector",
            Self::BrowserRuntimeInspector => "browser_runtime_inspector",
            Self::DemoHandoff => "demo_handoff",
            Self::DocsOnboardingWalkthrough => "docs_onboarding_walkthrough",
            Self::HelpCenter => "help_center",
        }
    }
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

    /// The banner `capability_state` label this authority maps to.
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
    /// must carry a companion / browser / handoff note.
    pub const fn requires_note(self) -> bool {
        !matches!(self, Self::None)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::HandoffPacket => "handoff_packet",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// Whether the consumer preserves the canonical component's controlled labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelParityState {
    /// Full badge / scope / citation / degraded-state label parity.
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

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::RenamedOrDropped => "renamed_or_dropped",
        }
    }
}

/// The token / density / motion fidelity a consumer keeps against the
/// design-system contract, even when it is inspect-only or compare-only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesignSystemFidelity {
    /// Design tokens (color / type / spacing) match the design-system contract.
    pub token_consistent: bool,
    /// Density behavior matches the design-system contract.
    pub density_consistent: bool,
    /// Motion behavior matches the design-system contract.
    pub motion_consistent: bool,
    /// Ref to the design-system contract the consumer conforms to.
    pub design_system_contract_ref: String,
}

impl DesignSystemFidelity {
    /// Whether token, density, and motion behavior are all consistent with the
    /// design-system contract and a contract ref is named.
    pub fn is_consistent(&self) -> bool {
        self.token_consistent
            && self.density_consistent
            && self.motion_consistent
            && !self.design_system_contract_ref.trim().is_empty()
    }
}

/// The reduced-capability banner a narrower consumer shows to disclose the
/// interactivity it drops relative to the full designer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReducedCapabilityBanner {
    /// Stable banner id.
    pub banner_id: String,
    /// The visible, non-generic banner label.
    pub visible_label: String,
    /// The capability state; must match the row's `authority_mode`.
    pub capability_state: String,
    /// The capabilities the narrowed surface is missing relative to full.
    #[serde(default)]
    pub missing_capabilities: Vec<String>,
}

/// One consumer adopting one canonical visual-designer component family on one
/// M5 handoff surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualDesignerConsumerRow {
    /// Record kind; must equal [`VISUAL_DESIGNER_CONSUMER_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`VISUAL_DESIGNER_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The claimed consumer class.
    pub consumer_group: ConsumerGroup,
    /// The concrete handoff surface; must belong to `consumer_group`.
    pub consumer_surface: M5HandoffConsumerSurface,
    /// The single canonical component family this consumer reuses.
    pub component_family: M5VisualDesignerComponentFamily,
    /// The canonical primitive schema for the family. Must equal
    /// [`canonical_schema_ref_for`] of `component_family`.
    pub canonical_family_schema_ref: String,
    /// The canonical release-proof packet(s) this consumer points back to. Must
    /// contain [`canonical_packet_ref_for`] of `component_family`.
    #[serde(default)]
    pub canonical_packet_refs: Vec<String>,
    /// True when the consumer references the canonical family rather than
    /// cloning surface-local prose.
    pub references_canonical_not_local_prose: bool,
    /// The rendering authority the consumer exercises.
    pub authority_mode: AuthorityMode,
    /// Token / density / motion fidelity against the design-system contract.
    pub design_system: DesignSystemFidelity,
    /// The controlled label families the consumer preserves verbatim (subset of
    /// [`REQUIRED_LABEL_FAMILIES`]).
    #[serde(default)]
    pub preserved_label_families: Vec<String>,
    /// The degraded-state vocabulary the consumer keeps visible even when
    /// narrowed.
    #[serde(default)]
    pub degraded_state_vocab: Vec<String>,
    /// The required labels the consumer preserves (reused matrix vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5VisualDesignerRequiredLabel>,
    /// Whether the consumer keeps the controlled labels.
    pub label_parity: LabelParityState,
    /// The surface a narrower consumer hands off to, if any.
    pub handoff_target: HandoffTarget,
    /// The companion / browser / handoff note ref; required when
    /// `handoff_target` is not `None`.
    #[serde(default)]
    pub handoff_note_ref: String,
    /// The reduced-capability banner, present only when the consumer narrows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reduced_capability_banner: Option<ReducedCapabilityBanner>,
    /// The copy / export parity of the adopted component (reused vocabulary).
    pub copy_export: CopyExportParity,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the adoption was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl VisualDesignerConsumerRow {
    /// Returns true when the consumer narrows below full authority.
    pub fn is_narrowed(&self) -> bool {
        self.authority_mode.is_narrowed()
    }

    /// The surface's declared group matches the row's declared group.
    pub fn surface_group_consistent(&self) -> bool {
        self.consumer_surface.consumer_group() == self.consumer_group
    }

    /// AC1 (canonical): the consumer points back to exactly one canonical family
    /// — the declared schema matches the family, a release-proof packet is
    /// referenced, and no surface-local prose is cloned.
    pub fn points_to_canonical_family(&self) -> bool {
        self.canonical_family_schema_ref == canonical_schema_ref_for(self.component_family)
            && self
                .canonical_packet_refs
                .iter()
                .any(|p| p == canonical_packet_ref_for(self.component_family))
            && self.references_canonical_not_local_prose
    }

    /// The consumer preserves the family's controlled label families and
    /// degraded-state vocabulary rather than renaming or omitting them.
    pub fn preserves_labels(&self) -> bool {
        self.label_parity.keeps_labels()
            && !self.preserved_label_families.is_empty()
            && self
                .preserved_label_families
                .iter()
                .all(|f| REQUIRED_LABEL_FAMILIES.contains(&f.as_str()))
            && !self.degraded_state_vocab.is_empty()
            && !self.required_labels.is_empty()
    }

    /// A narrower consumer discloses the reduction with a reduced-capability
    /// banner whose state matches the authority mode, and carries a
    /// companion / browser / handoff note whenever it punts to another surface.
    pub fn discloses_narrowing(&self) -> bool {
        if self.is_narrowed() {
            match &self.reduced_capability_banner {
                None => return false,
                Some(banner) => {
                    if banner.banner_id.trim().is_empty()
                        || banner.visible_label.trim().is_empty()
                        || label_is_generic(&banner.visible_label)
                        || banner.capability_state != self.authority_mode.capability_state()
                        || banner.capability_state == "full"
                        || banner.missing_capabilities.is_empty()
                    {
                        return false;
                    }
                }
            }
            // A narrowed consumer that keeps every label is disclosed-narrowed,
            // never plain preserved.
            if self.label_parity == LabelParityState::Preserved {
                return false;
            }
        } else if self.reduced_capability_banner.is_some() {
            // A full-interactive consumer must not carry a spurious banner.
            return false;
        }
        if self.handoff_target.requires_note() && self.handoff_note_ref.trim().is_empty() {
            return false;
        }
        true
    }

    /// AC1 (parity for degraded / inspect-only consumers): the consumer keeps
    /// design-system token / density / motion behavior even when it is narrowed.
    pub fn keeps_design_system_fidelity(&self) -> bool {
        self.design_system.is_consistent()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == VISUAL_DESIGNER_CONSUMER_ROW_RECORD_KIND
            && self.schema_version == VISUAL_DESIGNER_CONSUMER_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.canonical_family_schema_ref.trim().is_empty()
            && !self.canonical_packet_refs.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "surface={surface} group={group} family={family} authority={authority} \
label_parity={label_parity} handoff={handoff}",
            surface = self.consumer_surface.as_str(),
            group = self.consumer_group.as_str(),
            family = self.component_family.as_str(),
            authority = self.authority_mode.capability_state(),
            label_parity = self.label_parity.as_str(),
            handoff = self.handoff_target.as_str(),
        )
    }
}

/// Rolled-up summary of an M05-809 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualDesignerConsumerSummary {
    pub row_count: usize,
    pub consumer_group_count: usize,
    pub consumer_surface_count: usize,
    pub component_family_count: usize,
    pub all_rows_point_to_canonical_family: bool,
    pub all_rows_preserve_labels: bool,
    pub all_rows_keep_design_system_fidelity: bool,
    pub all_narrowed_rows_disclose: bool,
    pub all_rows_have_copy_export: bool,
    pub framework_pack_consumer_present: bool,
    pub preview_runtime_consumer_present: bool,
    pub browser_runtime_demo_consumer_present: bool,
    pub docs_onboarding_consumer_present: bool,
    pub label_family_coverage_complete: bool,
    pub families_reused_across_groups: usize,
}

/// Constructor input for [`VisualDesignerConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualDesignerConsumerPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<VisualDesignerConsumerRow>,
}

/// Checked-in M05-809 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualDesignerConsumerPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<VisualDesignerConsumerRow>,
    pub summary: VisualDesignerConsumerSummary,
}

impl VisualDesignerConsumerPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: VisualDesignerConsumerPacketInput) -> Self {
        let mut packet = Self {
            schema_version: VISUAL_DESIGNER_CONSUMER_SCHEMA_VERSION,
            record_kind: VISUAL_DESIGNER_CONSUMER_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: VisualDesignerConsumerSummary {
                row_count: 0,
                consumer_group_count: 0,
                consumer_surface_count: 0,
                component_family_count: 0,
                all_rows_point_to_canonical_family: false,
                all_rows_preserve_labels: false,
                all_rows_keep_design_system_fidelity: false,
                all_narrowed_rows_disclose: false,
                all_rows_have_copy_export: false,
                framework_pack_consumer_present: false,
                preview_runtime_consumer_present: false,
                browser_runtime_demo_consumer_present: false,
                docs_onboarding_consumer_present: false,
                label_family_coverage_complete: false,
                families_reused_across_groups: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5VisualDesignerComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// The union of every row's preserved label families.
    pub fn covered_label_families(&self) -> BTreeSet<String> {
        self.rows
            .iter()
            .flat_map(|r| r.preserved_label_families.iter().cloned())
            .collect()
    }

    /// The count of component families adopted by two or more distinct consumer
    /// groups — the strongest evidence that a family is a reusable primitive.
    pub fn families_reused_across_groups(&self) -> usize {
        M5VisualDesignerComponentFamily::ALL
            .iter()
            .filter(|family| {
                let groups: BTreeSet<ConsumerGroup> = self
                    .rows
                    .iter()
                    .filter(|r| r.component_family == **family)
                    .map(|r| r.consumer_group)
                    .collect();
                groups.len() >= 2
            })
            .count()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> VisualDesignerConsumerSummary {
        let mut groups = BTreeSet::new();
        let mut surfaces = BTreeSet::new();
        let mut families = BTreeSet::new();
        for row in &self.rows {
            groups.insert(row.consumer_group);
            surfaces.insert(row.consumer_surface);
            families.insert(row.component_family);
        }

        let has_group = |g: ConsumerGroup| groups.contains(&g);
        let covered = self.covered_label_families();

        VisualDesignerConsumerSummary {
            row_count: self.rows.len(),
            consumer_group_count: groups.len(),
            consumer_surface_count: surfaces.len(),
            component_family_count: families.len(),
            all_rows_point_to_canonical_family: self
                .rows
                .iter()
                .all(VisualDesignerConsumerRow::points_to_canonical_family),
            all_rows_preserve_labels: self
                .rows
                .iter()
                .all(VisualDesignerConsumerRow::preserves_labels),
            all_rows_keep_design_system_fidelity: self
                .rows
                .iter()
                .all(VisualDesignerConsumerRow::keeps_design_system_fidelity),
            all_narrowed_rows_disclose: self
                .rows
                .iter()
                .all(VisualDesignerConsumerRow::discloses_narrowing),
            all_rows_have_copy_export: self.rows.iter().all(|r| r.copy_export.is_complete()),
            framework_pack_consumer_present: has_group(ConsumerGroup::FrameworkPack),
            preview_runtime_consumer_present: has_group(ConsumerGroup::PreviewRuntime),
            browser_runtime_demo_consumer_present: has_group(ConsumerGroup::BrowserRuntimeDemo),
            docs_onboarding_consumer_present: has_group(ConsumerGroup::DocsOnboarding),
            label_family_coverage_complete: REQUIRED_LABEL_FAMILIES
                .iter()
                .all(|f| covered.contains(*f)),
            families_reused_across_groups: self.families_reused_across_groups(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<VisualDesignerConsumerViolation> {
        let mut violations = Vec::new();

        if self.schema_version != VISUAL_DESIGNER_CONSUMER_SCHEMA_VERSION {
            violations.push(VisualDesignerConsumerViolation::SchemaVersion {
                expected: VISUAL_DESIGNER_CONSUMER_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != VISUAL_DESIGNER_CONSUMER_RECORD_KIND {
            violations.push(VisualDesignerConsumerViolation::RecordKind {
                expected: VISUAL_DESIGNER_CONSUMER_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(VisualDesignerConsumerViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_groups = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(VisualDesignerConsumerViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_groups.insert(row.consumer_group);

            if !row.is_complete() {
                violations.push(VisualDesignerConsumerViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // The concrete surface must belong to the declared consumer group.
            if !row.surface_group_consistent() {
                violations.push(VisualDesignerConsumerViolation::SurfaceGroupMismatch {
                    id: row.row_id.clone(),
                });
            }

            // AC1: exactly one canonical family, no cloned surface-local prose.
            if !row.points_to_canonical_family() {
                violations.push(VisualDesignerConsumerViolation::NotCanonicalFamily {
                    id: row.row_id.clone(),
                });
            }

            // Label parity: controlled label families / degraded vocab preserved.
            if !row.preserves_labels() {
                violations.push(VisualDesignerConsumerViolation::LabelParityBroken {
                    id: row.row_id.clone(),
                });
            }

            // Design-system parity: token / density / motion consistent even when
            // inspect-only or compare-only.
            if !row.keeps_design_system_fidelity() {
                violations.push(VisualDesignerConsumerViolation::DesignSystemDrift {
                    id: row.row_id.clone(),
                });
            }

            // AC2: narrower consumers disclose reduction with banner + note.
            if !row.discloses_narrowing() {
                violations.push(VisualDesignerConsumerViolation::NarrowedWithoutDisclosure {
                    id: row.row_id.clone(),
                });
            }

            // Copy / export parity: text / JSON / Markdown, screenshot prohibited.
            if !row.copy_export.is_complete() {
                violations.push(VisualDesignerConsumerViolation::MissingCopyExportParity {
                    id: row.row_id.clone(),
                });
            }
        }

        // Cross-surface reuse spans all four claimed consumer classes.
        for group in ConsumerGroup::ALL {
            if !seen_groups.contains(&group) {
                violations.push(VisualDesignerConsumerViolation::MissingConsumerGroup { group });
            }
        }

        // Every frozen family is adopted by at least one consumer.
        let families = self.represented_families();
        for family in M5VisualDesignerComponentFamily::ALL {
            if !families.contains(&family) {
                violations.push(VisualDesignerConsumerViolation::MissingFamilyCoverage { family });
            }
        }

        // AC1: at least one family is reused across two or more consumer groups
        // so multiple M5 surfaces point back to one canonical family.
        if self.families_reused_across_groups() == 0 {
            violations.push(VisualDesignerConsumerViolation::NoFamilyReusedAcrossGroups);
        }

        // The controlled label families are collectively preserved.
        let covered = self.covered_label_families();
        for family in REQUIRED_LABEL_FAMILIES {
            if !covered.contains(family) {
                violations.push(VisualDesignerConsumerViolation::MissingLabelFamily {
                    family: family.to_owned(),
                });
            }
        }

        // AC3: a docs / help / onboarding consumer references the canonical
        // components rather than cloning local visual-designer semantics.
        if !self.rows.iter().any(|r| {
            r.consumer_group == ConsumerGroup::DocsOnboarding
                && r.references_canonical_not_local_prose
        }) {
            violations.push(VisualDesignerConsumerViolation::MissingDocsOnboardingReference);
        }

        if self.summary != self.computed_summary() {
            violations.push(VisualDesignerConsumerViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("consumer packet serializes"),
        ) {
            violations.push(VisualDesignerConsumerViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("consumer packet serializes")
    }

    /// Deterministic CSV of the adoption rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,consumer_group,consumer_surface,component_family,authority,label_parity,handoff\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{group},{surface},{family},{authority},{label_parity},{handoff}\n",
                id = row.row_id,
                group = row.consumer_group.as_str(),
                surface = row.consumer_surface.as_str(),
                family = row.component_family.as_str(),
                authority = row.authority_mode.capability_state(),
                label_parity = row.label_parity.as_str(),
                handoff = row.handoff_target.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Visual-Designer Component Consumers\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Rows: {} across {} consumer groups and {} / {} frozen families\n",
            self.summary.row_count,
            self.summary.consumer_group_count,
            self.represented_families().len(),
            M5VisualDesignerComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Families reused across groups: {}\n",
            self.summary.families_reused_across_groups,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!("- **{}** — {}\n", row.row_id, row.chip_tokens()));
        }
        out
    }
}

/// Reads and validates the checked-in consumer export.
pub fn current_m5_visual_designer_component_consumers_export(
) -> Result<VisualDesignerConsumerPacket, VisualDesignerConsumerArtifactError> {
    let packet: VisualDesignerConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-visual-designer-component-consumer-proof/support_export.json"
    )))
    .map_err(VisualDesignerConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(VisualDesignerConsumerArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in consumer export.
#[derive(Debug)]
pub enum VisualDesignerConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<VisualDesignerConsumerViolation>),
}

impl fmt::Display for VisualDesignerConsumerArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "consumer export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "consumer export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for VisualDesignerConsumerArtifactError {}

/// Validation failure for M05-809 consumer packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VisualDesignerConsumerViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    SurfaceGroupMismatch {
        id: String,
    },
    NotCanonicalFamily {
        id: String,
    },
    LabelParityBroken {
        id: String,
    },
    DesignSystemDrift {
        id: String,
    },
    NarrowedWithoutDisclosure {
        id: String,
    },
    MissingCopyExportParity {
        id: String,
    },
    MissingConsumerGroup {
        group: ConsumerGroup,
    },
    MissingFamilyCoverage {
        family: M5VisualDesignerComponentFamily,
    },
    NoFamilyReusedAcrossGroups,
    MissingLabelFamily {
        family: String,
    },
    MissingDocsOnboardingReference,
    SummaryMismatch,
    RawBoundaryMaterialInExport,
}

impl fmt::Display for VisualDesignerConsumerViolation {
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
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete consumer row: {id}"),
            Self::SurfaceGroupMismatch { id } => {
                write!(
                    f,
                    "row {id} declares a surface that does not belong to its consumer group"
                )
            }
            Self::NotCanonicalFamily { id } => {
                write!(
                    f,
                    "row {id} does not point back to exactly one canonical component family"
                )
            }
            Self::LabelParityBroken { id } => {
                write!(
                    f,
                    "row {id} renames or drops a canonical support-class / runtime-origin / \
unsupported-construct / round-trip / open-source-fallback label"
                )
            }
            Self::DesignSystemDrift { id } => {
                write!(
                    f,
                    "row {id} drifts from the design-system token / density / motion contract"
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
                    "row {id} is missing text / JSON / Markdown copy-export parity"
                )
            }
            Self::MissingConsumerGroup { group } => {
                write!(f, "consumer group {group:?} is not adopted in the packet")
            }
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "component family {family:?} is not adopted in the packet"
                )
            }
            Self::NoFamilyReusedAcrossGroups => write!(
                f,
                "no component family is adopted across two or more consumer groups"
            ),
            Self::MissingLabelFamily { family } => {
                write!(
                    f,
                    "controlled label family {family} is not preserved anywhere"
                )
            }
            Self::MissingDocsOnboardingReference => write!(
                f,
                "no docs / onboarding consumer references the canonical component families"
            ),
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawBoundaryMaterialInExport => {
                write!(f, "export contains raw boundary material")
            }
        }
    }
}

impl Error for VisualDesignerConsumerViolation {}

/// Whether a banner label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in consumer packet. This is the one source of
/// truth shared by the tests, the example dump, and the on-disk support export
/// so all three stay byte-aligned.
pub fn seeded_m5_visual_designer_component_consumers_packet() -> VisualDesignerConsumerPacket {
    VisualDesignerConsumerPacket::new(VisualDesignerConsumerPacketInput {
        packet_id: "m5-visual-designer-component-consumers:stable:0001".to_owned(),
        as_of: "2026-07-03T00:00:00Z".to_owned(),
        matrix_ref: VISUAL_DESIGNER_CONSUMER_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:visual-designer-consumer:{id}")]
}

fn all_required_labels() -> Vec<M5VisualDesignerRequiredLabel> {
    M5VisualDesignerRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> CopyExportParity {
    CopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn design_system() -> DesignSystemFidelity {
    DesignSystemFidelity {
        token_consistent: true,
        density_consistent: true,
        motion_consistent: true,
        design_system_contract_ref: "docs/designer/m5_visual_designer_component_matrix.md"
            .to_owned(),
    }
}

fn labels(families: &[&str]) -> Vec<String> {
    families.iter().map(|f| (*f).to_owned()).collect()
}

fn degraded_vocab() -> Vec<String> {
    vec![
        "fresh".to_owned(),
        "stale".to_owned(),
        "partial".to_owned(),
        "policy_limited".to_owned(),
    ]
}

fn banner(
    id: &str,
    label: &str,
    authority: AuthorityMode,
    missing: &[&str],
) -> ReducedCapabilityBanner {
    ReducedCapabilityBanner {
        banner_id: id.to_owned(),
        visible_label: label.to_owned(),
        capability_state: authority.capability_state().to_owned(),
        missing_capabilities: missing.iter().map(|m| (*m).to_owned()).collect(),
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    consumer_surface: M5HandoffConsumerSurface,
    component_family: M5VisualDesignerComponentFamily,
    authority_mode: AuthorityMode,
    label_families: &[&str],
    export_fields: &[&str],
    handoff_target: HandoffTarget,
    handoff_note_ref: &str,
    reduced_capability_banner: Option<ReducedCapabilityBanner>,
) -> VisualDesignerConsumerRow {
    let label_parity = if authority_mode.is_narrowed() {
        LabelParityState::DisclosedNarrowed
    } else {
        LabelParityState::Preserved
    };
    VisualDesignerConsumerRow {
        record_kind: VISUAL_DESIGNER_CONSUMER_ROW_RECORD_KIND.to_owned(),
        schema_version: VISUAL_DESIGNER_CONSUMER_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        consumer_group: consumer_surface.consumer_group(),
        consumer_surface,
        component_family,
        canonical_family_schema_ref: canonical_schema_ref_for(component_family).to_owned(),
        canonical_packet_refs: vec![canonical_packet_ref_for(component_family).to_owned()],
        references_canonical_not_local_prose: true,
        authority_mode,
        design_system: design_system(),
        preserved_label_families: labels(label_families),
        degraded_state_vocab: degraded_vocab(),
        required_labels: all_required_labels(),
        label_parity,
        handoff_target,
        handoff_note_ref: handoff_note_ref.to_owned(),
        reduced_capability_banner,
        copy_export: copy_export(export_fields),
        source_refs: vec![VISUAL_DESIGNER_CONSUMER_MATRIX_REF.to_owned()],
        observed_at: "2026-07-03T00:00:00Z".to_owned(),
        evidence_refs: ev(row_id),
    }
}

fn seeded_rows() -> Vec<VisualDesignerConsumerRow> {
    use AuthorityMode::*;
    use M5HandoffConsumerSurface::*;
    use M5VisualDesignerComponentFamily::*;

    vec![
        // --- Framework-pack preview consumer -------------------------------
        // Full-interactive framework-pack designer editing on the shared canvas.
        row(
            "consumer:framework-pack:design-canvas",
            FrameworkPackPreviewLane,
            DesignCanvas,
            FullInteractive,
            &["support_class", "runtime_origin"],
            &["selection_id", "canvas_state", "source_revision_ref"],
            HandoffTarget::None,
            "",
            None,
        ),
        // Framework-pack breakpoint deck comparing device previews (compare-only).
        row(
            "consumer:framework-pack:breakpoint-preview-row",
            FrameworkPackPreviewLane,
            BreakpointPreviewRow,
            CompareOnly,
            &["runtime_origin", "open_source_fallback"],
            &["device_class", "runtime_origin", "mapping_quality"],
            HandoffTarget::None,
            "",
            Some(banner(
                "banner:framework-pack:breakpoint",
                "Compare-only device deck: switch breakpoints and inspect runtime origin; live editing stays on the desktop designer",
                CompareOnly,
                &["live_viewport_resize", "interactive_edit"],
            )),
        ),
        // Framework-pack structure outline for the packaged component (read-only).
        row(
            "consumer:framework-pack:structure-tree-row",
            FrameworkPackPreviewLane,
            StructureTreeRow,
            ReadOnly,
            &["support_class"],
            &["node_kind", "source_span_ref", "selection_id"],
            HandoffTarget::None,
            "",
            Some(banner(
                "banner:framework-pack:structure",
                "Read-only structure outline: navigate the packaged component tree; reordering stays on the desktop designer",
                ReadOnly,
                &["reorder", "interactive_edit"],
            )),
        ),
        // --- Preview-runtime inspection consumer ---------------------------
        // Preview-runtime property inspector (inspect-only).
        row(
            "consumer:preview-runtime:property-inspector-row",
            PreviewRuntimeInspector,
            PropertyInspectorRow,
            InspectOnly,
            &["support_class", "runtime_origin"],
            &["value_state", "write_scope", "preview_diff"],
            HandoffTarget::None,
            "",
            Some(banner(
                "banner:preview-runtime:inspector",
                "Inspect-only property rows: read token / bound / inherited / literal state; edits round-trip through the source-first designer",
                InspectOnly,
                &["write_value", "commit_edit"],
            )),
        ),
        // Preview-runtime source-sync chip strip (inspect-only).
        row(
            "consumer:preview-runtime:source-sync-chip",
            PreviewRuntimeInspector,
            SourceSyncChip,
            InspectOnly,
            &["support_class", "round_trip_conflict"],
            &["sync_class", "recovery_route"],
            HandoffTarget::None,
            "",
            Some(banner(
                "banner:preview-runtime:source-sync",
                "Inspect-only sync chips: read how the runtime preview relates to canonical source; recovery routes open the source-first designer",
                InspectOnly,
                &["apply_recovery", "writeback"],
            )),
        ),
        // Preview-runtime breakpoint compare — reuses the breakpoint family in a
        // second group (compare-only) to prove one canonical family is shared.
        row(
            "consumer:preview-runtime:breakpoint-preview-row",
            PreviewRuntimeInspector,
            BreakpointPreviewRow,
            CompareOnly,
            &["runtime_origin"],
            &["device_class", "runtime_origin", "mapping_quality"],
            HandoffTarget::None,
            "",
            Some(banner(
                "banner:preview-runtime:breakpoint",
                "Compare-only runtime breakpoints: read live-versus-mock posture and mapping quality; viewport editing stays on the designer",
                CompareOnly,
                &["live_viewport_resize"],
            )),
        ),
        // --- Browser-runtime / demo handoff consumer -----------------------
        // Browser-runtime structure inspector projected read-only, handing off to
        // a read-only browser surface.
        row(
            "consumer:browser-runtime:structure-tree-row",
            BrowserRuntimeInspector,
            StructureTreeRow,
            ReadOnly,
            &["runtime_origin", "support_class"],
            &["node_kind", "source_span_ref", "selection_id"],
            HandoffTarget::BrowserReadonly,
            "handoff:browser-runtime:structure-open-in-designer",
            Some(banner(
                "banner:browser-runtime:structure",
                "Read-only browser structure map: inspect the live DOM-to-source tree; open the desktop designer to edit",
                ReadOnly,
                &["interactive_edit", "reorder"],
            )),
        ),
        // Browser-runtime round-trip conflict banner projected read-only.
        row(
            "consumer:browser-runtime:round-trip-conflict-banner",
            BrowserRuntimeInspector,
            RoundTripConflictBanner,
            ReadOnly,
            &["round_trip_conflict", "support_class"],
            &["conflict_class", "resolution_route"],
            HandoffTarget::None,
            "",
            Some(banner(
                "banner:browser-runtime:round-trip",
                "Read-only conflict banner: see that source changed under a live edit; resolution routes open the source-first designer",
                ReadOnly,
                &["resolve_conflict", "writeback"],
            )),
        ),
        // Demo / share handoff of the unsupported-construct card (export-only),
        // punting to a handoff packet.
        row(
            "consumer:demo-handoff:unsupported-construct-card",
            DemoHandoff,
            UnsupportedConstructCard,
            ExportOnly,
            &["unsupported_construct", "open_source_fallback"],
            &["reason", "card_label"],
            HandoffTarget::HandoffPacket,
            "handoff:demo:unsupported-construct-packet",
            Some(banner(
                "banner:demo:unsupported-construct",
                "Export-only demo card: shows why a construct is unsupported and the source-first fallback; open the packet in the designer to act",
                ExportOnly,
                &["interactive_edit", "apply_fallback"],
            )),
        ),
        // --- Docs / help / onboarding consumer -----------------------------
        // Onboarding walkthrough embedding the design canvas (inspect-only),
        // reusing the canvas family in a second group.
        row(
            "consumer:docs-onboarding:design-canvas",
            DocsOnboardingWalkthrough,
            DesignCanvas,
            InspectOnly,
            &["support_class", "runtime_origin"],
            &["selection_id", "canvas_state", "source_revision_ref"],
            HandoffTarget::None,
            "",
            Some(banner(
                "banner:docs-onboarding:design-canvas",
                "Inspect-only onboarding canvas: walk the source-backed selection story; editing opens the desktop designer",
                InspectOnly,
                &["interactive_edit"],
            )),
        ),
        // Onboarding walkthrough embedding the source-sync chip (read-only),
        // reusing the source-sync family in a second group.
        row(
            "consumer:docs-onboarding:source-sync-chip",
            DocsOnboardingWalkthrough,
            SourceSyncChip,
            ReadOnly,
            &["support_class", "open_source_fallback"],
            &["sync_class", "recovery_route"],
            HandoffTarget::None,
            "",
            Some(banner(
                "banner:docs-onboarding:source-sync",
                "Read-only onboarding sync chip: learn how a surface relates to canonical source and its open-source fallback",
                ReadOnly,
                &["apply_recovery"],
            )),
        ),
        // Help center reference for the unsupported-construct card (read-only),
        // reusing the unsupported-construct family in a second group.
        row(
            "consumer:help-center:unsupported-construct-card",
            HelpCenter,
            UnsupportedConstructCard,
            ReadOnly,
            &["unsupported_construct"],
            &["reason", "card_label"],
            HandoffTarget::None,
            "",
            Some(banner(
                "banner:help-center:unsupported-construct",
                "Read-only help reference: explains each unsupported-construct reason and its source-first fallback",
                ReadOnly,
                &["interactive_edit"],
            )),
        ),
    ]
}
