//! One-bundle qualification of the shared visual-designer components across every
//! claimed visual-design and source-round-trip consumer.
//!
//! This module is the M05-811 qualification capstone that CLOSES the B94
//! visual-designer component lane by consolidating the whole lane into a single
//! referenceable certification bundle. Where the freeze matrix
//! ([`crate::freeze_the_m5_design_canvas_structure_tree_property_inspector_source_sync_and_breakpoint_preview_component_matrix`])
//! froze the reusable canvas / tree / inspector / chip / preview-row primitives,
//! the 805-807 lanes resolved their per-target truth, the 808 lane certified
//! accessibility fallback, the 809 lane adopted them across handoff consumers, and
//! the 810 lane certified per-surface auto-narrowing, this lane produces one
//! qualification packet — keyed on the claim-bearing **consumer** — proving that
//! every claimed visual-design, preview, framework-pack, docs / demo, and handoff
//! consumer either passes the shared component parity check on every dimension or
//! narrows automatically, and that release / help / support packets can cite a
//! single certification bundle for all of it.
//!
//! Every [`VisualDesignerQualificationRow`] keys on one
//! [`M5QualifiedComponentConsumer`] and certifies five component parity dimensions
//! ([`M5ComponentQualificationDimension`]):
//!
//! - **Source ownership.** The consumer keeps source canonical and derived state
//!   explicit; it never silently claims ownership it does not hold.
//! - **Mapping quality.** The consumer discloses how well its view maps back to
//!   canonical source.
//! - **Round-trip state.** The consumer discloses whether a visual action writes
//!   back to source, and never collapses an unsupported construct or open conflict
//!   into a silent write-back.
//! - **Token / binding provenance.** The consumer distinguishes token,
//!   bound-expression, inherited, and literal state rather than flattening them.
//! - **Accessibility / export behavior.** The consumer keeps a non-visual fallback
//!   and a text / JSON / Markdown export, never a screenshot alone.
//!
//! Each dimension carries a reused [`crate::AxisCertificationState`]: a `certified`
//! dimension passes, a `disclosed_narrowed` dimension weakened and disclosed the
//! narrowing with a frozen [`crate::M5VisualDesignerDowngradeTrigger`] and a
//! precise reason, and an `undisclosed_drift` dimension hid the drift and is
//! rejected. A consumer that drifts on any dimension without disclosure is blocked
//! (red); a consumer that narrows and discloses is qualified-with-narrowing
//! (yellow); a consumer whose every dimension is certified is qualified (green).
//!
//! The packet consolidates the B94 lane: it lists every underlying certified
//! component packet ([`canonical_component_packet_refs`]) — the frozen matrix, the
//! three primitive resolvers, the accessibility fallback, the consumer adoption,
//! and the surface certification — and every row cites the one certification bundle
//! so release, help, and support packets reference a single source of truth.
//!
//! The packet is metadata-only: raw source bodies, diff hunks, credentials, and
//! provider payloads never cross this boundary; the packet carries only typed class
//! tokens, opaque summary / evidence refs, booleans, and redacted labels.
//!
//! The boundary schema is
//! [`schemas/ui/m5-visual-designer-component-qualification.schema.json`](../../../../schemas/ui/m5-visual-designer-component-qualification.schema.json).
//! The contract doc is
//! [`docs/designer/m5_visual_designer_component_qualification_contract.md`](../../../../docs/designer/m5_visual_designer_component_qualification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    AxisCertificationState, CopyExportParity, M5VisualDesignerDowngradeTrigger,
    M5VisualDesignerRequiredLabel, M5_BREAKPOINT_PREVIEW_ARTIFACT_REF, M5_ROUND_TRIP_ARTIFACT_REF,
    M5_SELECTED_NODE_ARTIFACT_REF, VISUAL_DESIGNER_A11Y_FALLBACK_ARTIFACT_REF,
    VISUAL_DESIGNER_COMPONENT_MATRIX_ARTIFACT_REF, VISUAL_DESIGNER_CONSUMER_ARTIFACT_REF,
    VISUAL_DESIGNER_SURFACE_CERT_ARTIFACT_REF,
};

/// Schema version stamped on the M05-811 qualification packet.
pub const VISUAL_DESIGNER_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`VisualDesignerQualificationPacket`].
pub const VISUAL_DESIGNER_QUALIFICATION_RECORD_KIND: &str =
    "m5_visual_designer_component_qualification_packet";

/// Stable record-kind tag carried by each [`VisualDesignerQualificationRow`].
pub const VISUAL_DESIGNER_QUALIFICATION_ROW_RECORD_KIND: &str =
    "m5_visual_designer_component_qualification_row";

/// Repo-relative path of the boundary schema.
pub const VISUAL_DESIGNER_QUALIFICATION_SCHEMA_REF: &str =
    "schemas/ui/m5-visual-designer-component-qualification.schema.json";

/// Repo-relative path of the contract doc.
pub const VISUAL_DESIGNER_QUALIFICATION_DOC_REF: &str =
    "docs/designer/m5_visual_designer_component_qualification_contract.md";

/// Repo-relative path of the frozen visual-designer component matrix this lane
/// qualifies against.
pub const VISUAL_DESIGNER_QUALIFICATION_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-visual-designer-component-matrix.schema.json";

/// Repo-relative path of the one certification bundle every row cites (AC2).
pub const VISUAL_DESIGNER_QUALIFICATION_BUNDLE_REF: &str =
    "artifacts/release/m5-visual-designer-component-qualification-proof/support_export.json";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical); identical to the bundle ref.
pub const VISUAL_DESIGNER_QUALIFICATION_ARTIFACT_REF: &str =
    "artifacts/release/m5-visual-designer-component-qualification-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const VISUAL_DESIGNER_QUALIFICATION_CSV_REF: &str =
    "artifacts/release/m5-visual-designer-component-qualification-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const VISUAL_DESIGNER_QUALIFICATION_REPORT_REF: &str =
    "artifacts/components/m5-visual-designer-component-qualification.md";

/// Repo-relative path of the protected fixture directory.
pub const VISUAL_DESIGNER_QUALIFICATION_FIXTURE_DIR: &str =
    "fixtures/ui/m5-visual-designer-component-qualification";

/// The claim-bearing consumer of the shared visual-designer components a
/// qualification row keys on. Spans the interactive visual-design, preview,
/// framework-pack, docs / demo, and handoff consumers, plus the support, help, and
/// release evidence consumers that must reference the same certification bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5QualifiedComponentConsumer {
    /// The interactive visual-design surface (canvas / tree / inspector).
    VisualDesignSurface,
    /// The live preview runtime.
    PreviewRuntime,
    /// The framework-pack live preview.
    FrameworkPackPreview,
    /// Docs / demo embeds.
    DocsDemoEmbeds,
    /// A cross-surface handoff consumer.
    HandoffConsumer,
    /// A support export packet.
    SupportPacket,
    /// A help-center consumer.
    HelpCenter,
    /// A release-evidence consumer.
    ReleaseEvidence,
}

impl M5QualifiedComponentConsumer {
    /// Every claimed consumer, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::VisualDesignSurface,
        Self::PreviewRuntime,
        Self::FrameworkPackPreview,
        Self::DocsDemoEmbeds,
        Self::HandoffConsumer,
        Self::SupportPacket,
        Self::HelpCenter,
        Self::ReleaseEvidence,
    ];

    /// The support, help, and release evidence consumers that must reference the
    /// one certification bundle (AC2).
    pub const EVIDENCE_CONSUMERS: [Self; 3] =
        [Self::SupportPacket, Self::HelpCenter, Self::ReleaseEvidence];

    /// True when this consumer is an evidence consumer (support, help, or release).
    pub const fn is_evidence_consumer(self) -> bool {
        matches!(
            self,
            Self::SupportPacket | Self::HelpCenter | Self::ReleaseEvidence
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VisualDesignSurface => "visual_design_surface",
            Self::PreviewRuntime => "preview_runtime",
            Self::FrameworkPackPreview => "framework_pack_preview",
            Self::DocsDemoEmbeds => "docs_demo_embeds",
            Self::HandoffConsumer => "handoff_consumer",
            Self::SupportPacket => "support_packet",
            Self::HelpCenter => "help_center",
            Self::ReleaseEvidence => "release_evidence",
        }
    }
}

/// A component parity dimension every consumer is qualified against. A consumer
/// that drifts on any dimension without disclosure fails promotion; a consumer that
/// narrows and discloses is qualified with a disclosed narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComponentQualificationDimension {
    /// Source stays canonical and derived state stays explicit.
    SourceOwnership,
    /// How well the view maps back to canonical source.
    MappingQuality,
    /// Whether a visual action writes back to source (never a silent write-back).
    RoundTripState,
    /// Token, bound-expression, inherited, and literal state stay distinct.
    TokenBindingProvenance,
    /// A non-visual fallback and a text / JSON / Markdown export are preserved.
    AccessibilityExportBehavior,
}

impl M5ComponentQualificationDimension {
    /// Every parity dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SourceOwnership,
        Self::MappingQuality,
        Self::RoundTripState,
        Self::TokenBindingProvenance,
        Self::AccessibilityExportBehavior,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceOwnership => "source_ownership",
            Self::MappingQuality => "mapping_quality",
            Self::RoundTripState => "round_trip_state",
            Self::TokenBindingProvenance => "token_binding_provenance",
            Self::AccessibilityExportBehavior => "accessibility_export_behavior",
        }
    }
}

/// The parity result for one dimension on one consumer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationDimensionParity {
    /// The parity dimension this result covers.
    pub dimension: M5ComponentQualificationDimension,
    /// The certification state of the dimension (reused vocabulary).
    pub state: AxisCertificationState,
    /// The frozen downgrade trigger that caused a narrowing; present iff the
    /// dimension narrowed (reused vocabulary).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<M5VisualDesignerDowngradeTrigger>,
    /// A precise, non-generic reason label; required when the dimension narrowed.
    #[serde(default)]
    pub reason_label: String,
}

impl QualificationDimensionParity {
    /// A certified dimension whose truth still supports the shared component claim.
    pub fn certified(dimension: M5ComponentQualificationDimension) -> Self {
        Self {
            dimension,
            state: AxisCertificationState::Certified,
            trigger: None,
            reason_label: String::new(),
        }
    }

    /// A disclosed-narrowed dimension carrying a frozen trigger and precise reason.
    pub fn narrowed(
        dimension: M5ComponentQualificationDimension,
        trigger: M5VisualDesignerDowngradeTrigger,
        reason: &str,
    ) -> Self {
        Self {
            dimension,
            state: AxisCertificationState::DisclosedNarrowed,
            trigger: Some(trigger),
            reason_label: reason.to_owned(),
        }
    }

    /// Whether the dimension parity is honest: a certified dimension carries no
    /// spurious trigger, a narrowed dimension carries a trigger and a precise,
    /// non-generic reason, and an undisclosed-drift dimension is never honest.
    pub fn is_honest(&self) -> bool {
        match self.state {
            AxisCertificationState::Certified => self.trigger.is_none(),
            AxisCertificationState::DisclosedNarrowed => {
                self.trigger.is_some() && !label_is_generic(&self.reason_label)
            }
            AxisCertificationState::UndisclosedDrift => false,
        }
    }
}

/// A named field the export / support packet preserves so a narrowed consumer can
/// never present as fully qualified in an export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5QualificationExportField {
    /// The consumer identity.
    ConsumerIdentity,
    /// The source-ownership parity state.
    SourceOwnership,
    /// The mapping-quality parity state.
    MappingQuality,
    /// The round-trip parity state.
    RoundTripState,
    /// The token / binding provenance parity state.
    TokenBindingProvenance,
    /// The accessibility / export parity state.
    AccessibilityBehavior,
    /// The derived qualification verdict.
    Verdict,
    /// The narrowed-capability reason.
    NarrowedReason,
}

impl M5QualificationExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ConsumerIdentity,
        Self::SourceOwnership,
        Self::MappingQuality,
        Self::RoundTripState,
        Self::TokenBindingProvenance,
        Self::AccessibilityBehavior,
        Self::Verdict,
        Self::NarrowedReason,
    ];

    /// The export fields every qualified row MUST preserve so support / release
    /// exports carry the same per-dimension parity visible in-product. The narrowed
    /// reason is only meaningful when a dimension narrowed, so it is not mandatory.
    pub const MANDATORY: [Self; 7] = [
        Self::ConsumerIdentity,
        Self::SourceOwnership,
        Self::MappingQuality,
        Self::RoundTripState,
        Self::TokenBindingProvenance,
        Self::AccessibilityBehavior,
        Self::Verdict,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerIdentity => "consumer_identity",
            Self::SourceOwnership => "source_ownership",
            Self::MappingQuality => "mapping_quality",
            Self::RoundTripState => "round_trip_state",
            Self::TokenBindingProvenance => "token_binding_provenance",
            Self::AccessibilityBehavior => "accessibility_behavior",
            Self::Verdict => "verdict",
            Self::NarrowedReason => "narrowed_reason",
        }
    }
}

/// Derived qualification verdict for a consumer row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComponentQualificationVerdict {
    /// Every parity dimension is certified (green).
    Qualified,
    /// A parity dimension weakened and the consumer narrowed and disclosed (yellow).
    QualifiedWithNarrowing,
    /// A dimension hid drift, the export dropped truth, or the consumer does not use
    /// the shared components (red) — may not promote.
    Blocked,
}

impl M5ComponentQualificationVerdict {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::QualifiedWithNarrowing => "qualified_with_narrowing",
            Self::Blocked => "blocked",
        }
    }
}

/// Qualification row for one claimed component consumer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualDesignerQualificationRow {
    /// Record kind; must equal [`VISUAL_DESIGNER_QUALIFICATION_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`VISUAL_DESIGNER_QUALIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The claimed consumer this row qualifies.
    pub consumer: M5QualifiedComponentConsumer,
    /// The consumer adopts the shared reusable components rather than a local fork;
    /// must hold (AC1: every claim-bearing surface uses the same source-roundtrip
    /// truth).
    pub uses_shared_components: bool,
    /// The per-dimension parity results; must cover every
    /// [`M5ComponentQualificationDimension`] exactly once.
    #[serde(default)]
    pub dimensions: Vec<QualificationDimensionParity>,
    /// The copy / export parity of the consumer's support / release export.
    pub copy_export: CopyExportParity,
    /// The named export fields the support / release packet preserves.
    #[serde(default)]
    pub preserved_export_fields: Vec<M5QualificationExportField>,
    /// The required labels the consumer preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5VisualDesignerRequiredLabel>,
    /// The canonical component packets this consumer's qualification draws from;
    /// must be a non-empty subset of the packet's certified component packets.
    #[serde(default)]
    pub canonical_component_refs: Vec<String>,
    /// The one certification bundle this row cites (AC2); must equal the
    /// packet-level bundle ref.
    pub certification_bundle_ref: String,
    /// Ref to the frozen matrix schema this row qualifies against.
    pub source_family_schema_ref: String,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the qualification posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl VisualDesignerQualificationRow {
    /// The parity result for one dimension, if present.
    pub fn dimension(
        &self,
        dimension: M5ComponentQualificationDimension,
    ) -> Option<&QualificationDimensionParity> {
        self.dimensions.iter().find(|d| d.dimension == dimension)
    }

    /// Whether the row covers every parity dimension exactly once.
    pub fn covers_all_dimensions(&self) -> bool {
        let seen: BTreeSet<M5ComponentQualificationDimension> =
            self.dimensions.iter().map(|d| d.dimension).collect();
        seen.len() == self.dimensions.len()
            && M5ComponentQualificationDimension::ALL
                .iter()
                .all(|d| seen.contains(d))
    }

    /// Whether every dimension parity is honest (no undisclosed drift, disclosed
    /// narrowings carry a trigger and a precise reason).
    pub fn dimensions_honest(&self) -> bool {
        self.dimensions
            .iter()
            .all(QualificationDimensionParity::is_honest)
    }

    /// True when any dimension hid drift without disclosure.
    pub fn hides_drift(&self) -> bool {
        self.dimensions
            .iter()
            .any(|d| d.state == AxisCertificationState::UndisclosedDrift)
    }

    /// True when any dimension disclosed a narrowing (yellow).
    pub fn is_narrowed(&self) -> bool {
        self.dimensions
            .iter()
            .any(|d| d.state == AxisCertificationState::DisclosedNarrowed)
    }

    /// Whether the export preserves the mandatory per-dimension parity fields.
    pub fn export_preserves_truth(&self) -> bool {
        self.copy_export.is_complete()
            && M5QualificationExportField::MANDATORY
                .iter()
                .all(|field| self.preserved_export_fields.contains(field))
    }

    /// AC3: when a dimension narrowed, the narrowed reason is preserved in the
    /// export so support / release exports can reconstruct why the consumer
    /// narrowed.
    pub fn narrowed_reason_exported(&self) -> bool {
        !self.is_narrowed()
            || self
                .preserved_export_fields
                .contains(&M5QualificationExportField::NarrowedReason)
    }

    /// Derived qualification verdict.
    pub fn verdict(&self) -> M5ComponentQualificationVerdict {
        if !self.uses_shared_components
            || self.hides_drift()
            || !self.dimensions_honest()
            || !self.covers_all_dimensions()
            || !self.export_preserves_truth()
            || !self.narrowed_reason_exported()
        {
            return M5ComponentQualificationVerdict::Blocked;
        }
        if self.is_narrowed() {
            M5ComponentQualificationVerdict::QualifiedWithNarrowing
        } else {
            M5ComponentQualificationVerdict::Qualified
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == VISUAL_DESIGNER_QUALIFICATION_ROW_RECORD_KIND
            && self.schema_version == VISUAL_DESIGNER_QUALIFICATION_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.certification_bundle_ref.trim().is_empty()
            && !self.dimensions.is_empty()
            && !self.preserved_export_fields.is_empty()
            && !self.required_labels.is_empty()
            && !self.canonical_component_refs.is_empty()
            && self
                .canonical_component_refs
                .iter()
                .all(|r| !r.trim().is_empty())
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "consumer={consumer} source_ownership={source} mapping={mapping} \
round_trip={round_trip} token_binding={token} accessibility={access} verdict={verdict}",
            consumer = self.consumer.as_str(),
            source = self.dimension_token(M5ComponentQualificationDimension::SourceOwnership),
            mapping = self.dimension_token(M5ComponentQualificationDimension::MappingQuality),
            round_trip = self.dimension_token(M5ComponentQualificationDimension::RoundTripState),
            token = self.dimension_token(M5ComponentQualificationDimension::TokenBindingProvenance),
            access = self
                .dimension_token(M5ComponentQualificationDimension::AccessibilityExportBehavior),
            verdict = self.verdict().as_str(),
        )
    }

    fn dimension_token(&self, dimension: M5ComponentQualificationDimension) -> &'static str {
        match self.dimension(dimension).map(|d| d.state) {
            Some(state) => state.as_str(),
            None => "absent",
        }
    }
}

/// Rolled-up summary of an M05-811 qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualDesignerQualificationSummary {
    pub consumer_count: usize,
    pub all_use_shared_components: bool,
    pub all_exports_preserve_truth: bool,
    pub all_narrowing_disclosed: bool,
    pub evidence_consumers_present: bool,
    pub dimensions_covered: usize,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub canonical_packet_count: usize,
}

/// Constructor input for [`VisualDesignerQualificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualDesignerQualificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub certification_bundle_ref: String,
    pub certified_component_packets: Vec<String>,
    pub rows: Vec<VisualDesignerQualificationRow>,
}

/// Checked-in M05-811 qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualDesignerQualificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub certification_bundle_ref: String,
    #[serde(default)]
    pub certified_component_packets: Vec<String>,
    #[serde(default)]
    pub rows: Vec<VisualDesignerQualificationRow>,
    pub summary: VisualDesignerQualificationSummary,
}

impl VisualDesignerQualificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: VisualDesignerQualificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: VISUAL_DESIGNER_QUALIFICATION_SCHEMA_VERSION,
            record_kind: VISUAL_DESIGNER_QUALIFICATION_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            certification_bundle_ref: input.certification_bundle_ref,
            certified_component_packets: input.certified_component_packets,
            rows: input.rows,
            summary: VisualDesignerQualificationSummary {
                consumer_count: 0,
                all_use_shared_components: false,
                all_exports_preserve_truth: false,
                all_narrowing_disclosed: false,
                evidence_consumers_present: false,
                dimensions_covered: 0,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                canonical_packet_count: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Consumers represented by some row in this packet.
    pub fn represented_consumers(&self) -> BTreeSet<M5QualifiedComponentConsumer> {
        self.rows.iter().map(|r| r.consumer).collect()
    }

    /// Whether the support, help, and release evidence consumers are all qualified.
    pub fn evidence_consumers_present(&self) -> bool {
        let represented = self.represented_consumers();
        M5QualifiedComponentConsumer::EVIDENCE_CONSUMERS
            .iter()
            .all(|c| represented.contains(c))
    }

    /// The union of parity dimensions certified or narrowed across every row.
    pub fn covered_dimensions(&self) -> BTreeSet<M5ComponentQualificationDimension> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            for parity in &row.dimensions {
                set.insert(parity.dimension);
            }
        }
        set
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> VisualDesignerQualificationSummary {
        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.verdict() {
                M5ComponentQualificationVerdict::Qualified => green += 1,
                M5ComponentQualificationVerdict::QualifiedWithNarrowing => yellow += 1,
                M5ComponentQualificationVerdict::Blocked => red += 1,
            }
        }

        VisualDesignerQualificationSummary {
            consumer_count: self.rows.len(),
            all_use_shared_components: self.rows.iter().all(|r| r.uses_shared_components),
            all_exports_preserve_truth: self
                .rows
                .iter()
                .all(VisualDesignerQualificationRow::export_preserves_truth),
            all_narrowing_disclosed: self.rows.iter().all(|r| !r.hides_drift()),
            evidence_consumers_present: self.evidence_consumers_present(),
            dimensions_covered: self.covered_dimensions().len(),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            canonical_packet_count: self.certified_component_packets.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<VisualDesignerQualificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != VISUAL_DESIGNER_QUALIFICATION_SCHEMA_VERSION {
            violations.push(VisualDesignerQualificationViolation::SchemaVersion {
                expected: VISUAL_DESIGNER_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != VISUAL_DESIGNER_QUALIFICATION_RECORD_KIND {
            violations.push(VisualDesignerQualificationViolation::RecordKind {
                expected: VISUAL_DESIGNER_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
            || self.certification_bundle_ref.trim().is_empty()
            || self.certified_component_packets.is_empty()
        {
            violations.push(VisualDesignerQualificationViolation::MissingIdentity);
        }

        // The one bundle must consolidate every canonical B94 component packet.
        for canonical in canonical_component_packet_refs() {
            if !self.certified_component_packets.contains(&canonical) {
                violations.push(
                    VisualDesignerQualificationViolation::MissingConsolidatedPacket {
                        packet_ref: canonical,
                    },
                );
            }
        }

        let certified: BTreeSet<&String> = self.certified_component_packets.iter().collect();
        let mut row_ids = BTreeSet::new();
        let mut seen_consumers = BTreeSet::new();
        let mut label_union = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(VisualDesignerQualificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_consumers.insert(row.consumer);
            label_union.extend(row.required_labels.iter().copied());

            if !row.is_complete() {
                violations.push(VisualDesignerQualificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // AC1: the consumer adopts the shared components, not a local fork.
            if !row.uses_shared_components {
                violations.push(
                    VisualDesignerQualificationViolation::SharedComponentsNotUsed {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: every parity dimension is present.
            if !row.covers_all_dimensions() {
                violations.push(
                    VisualDesignerQualificationViolation::MissingDimensionCoverage {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: no dimension hides drift; disclosed narrowings stay honest.
            if row.hides_drift() {
                violations.push(VisualDesignerQualificationViolation::DimensionHidesDrift {
                    id: row.row_id.clone(),
                });
            }
            if !row.dimensions_honest() {
                violations.push(VisualDesignerQualificationViolation::DishonestNarrowing {
                    id: row.row_id.clone(),
                });
            }

            // AC3: the export preserves the same per-dimension parity, including the
            // narrowed reason when the consumer narrowed.
            if !row.export_preserves_truth() {
                violations.push(VisualDesignerQualificationViolation::ExportDropsTruth {
                    id: row.row_id.clone(),
                });
            }
            if !row.narrowed_reason_exported() {
                violations.push(
                    VisualDesignerQualificationViolation::NarrowedReasonNotExported {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC2: each row cites the one certification bundle and draws from the
            // consolidated component packets.
            if row.certification_bundle_ref != self.certification_bundle_ref {
                violations.push(VisualDesignerQualificationViolation::BundleRefMismatch {
                    id: row.row_id.clone(),
                });
            }
            if row.canonical_component_refs.is_empty() {
                violations.push(VisualDesignerQualificationViolation::MissingComponentRefs {
                    id: row.row_id.clone(),
                });
            }
            for component_ref in &row.canonical_component_refs {
                if !certified.contains(component_ref) {
                    violations.push(
                        VisualDesignerQualificationViolation::UncitedComponentPacket {
                            id: row.row_id.clone(),
                            packet_ref: component_ref.clone(),
                        },
                    );
                }
            }

            if row.required_labels.is_empty() {
                violations.push(
                    VisualDesignerQualificationViolation::MissingRequiredLabels {
                        id: row.row_id.clone(),
                    },
                );
            }

            if row.verdict() == M5ComponentQualificationVerdict::Blocked {
                violations.push(VisualDesignerQualificationViolation::BlockedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every claimed consumer is qualified at least once.
        for consumer in M5QualifiedComponentConsumer::ALL {
            if !seen_consumers.contains(&consumer) {
                violations.push(
                    VisualDesignerQualificationViolation::MissingConsumerCoverage { consumer },
                );
            }
        }

        // AC2: the support, help, and release evidence consumers are present.
        if !self.evidence_consumers_present() {
            violations.push(VisualDesignerQualificationViolation::MissingEvidenceConsumer);
        }

        // Every parity dimension is covered by the packet as a whole.
        let covered = self.covered_dimensions();
        for dimension in M5ComponentQualificationDimension::ALL {
            if !covered.contains(&dimension) {
                violations.push(
                    VisualDesignerQualificationViolation::MissingDimensionUnion { dimension },
                );
            }
        }

        // The union of preserved required labels covers the frozen set.
        for label in M5VisualDesignerRequiredLabel::ALL {
            if !label_union.contains(&label) {
                violations
                    .push(VisualDesignerQualificationViolation::MissingLabelCoverage { label });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(VisualDesignerQualificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("qualification packet serializes"),
        ) {
            violations.push(VisualDesignerQualificationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("qualification packet serializes")
    }

    /// Deterministic CSV of the qualified rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,consumer,source_ownership,mapping_quality,round_trip_state,token_binding_provenance,accessibility_export_behavior,verdict\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{consumer},{source},{mapping},{round_trip},{token},{access},{verdict}\n",
                id = row.row_id,
                consumer = row.consumer.as_str(),
                source = row.dimension_token(M5ComponentQualificationDimension::SourceOwnership),
                mapping = row.dimension_token(M5ComponentQualificationDimension::MappingQuality),
                round_trip = row.dimension_token(M5ComponentQualificationDimension::RoundTripState),
                token =
                    row.dimension_token(M5ComponentQualificationDimension::TokenBindingProvenance),
                access = row.dimension_token(
                    M5ComponentQualificationDimension::AccessibilityExportBehavior,
                ),
                verdict = row.verdict().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Visual-Designer Component Qualification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!("- Bundle: `{}`\n", self.certification_bundle_ref));
        out.push_str(&format!(
            "- Consolidates {} certified component packets\n",
            self.certified_component_packets.len(),
        ));
        out.push_str(&format!(
            "- Consumers: {} qualified across {} / {} claimed consumers\n",
            self.summary.consumer_count,
            self.represented_consumers().len(),
            M5QualifiedComponentConsumer::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.consumer.as_str(),
                row.chip_tokens(),
            ));
            for parity in &row.dimensions {
                if parity.state != AxisCertificationState::Certified {
                    out.push_str(&format!(
                        "  - Narrowed: dimension={} trigger={} — {}\n",
                        parity.dimension.as_str(),
                        parity
                            .trigger
                            .map(M5VisualDesignerDowngradeTrigger::as_str)
                            .unwrap_or("none"),
                        parity.reason_label,
                    ));
                }
            }
        }
        out
    }
}

/// The canonical B94 component packets the one certification bundle consolidates —
/// the frozen matrix, the three primitive resolvers, the accessibility fallback,
/// the consumer adoption, and the surface certification.
pub fn canonical_component_packet_refs() -> Vec<String> {
    vec![
        VISUAL_DESIGNER_COMPONENT_MATRIX_ARTIFACT_REF.to_owned(),
        M5_SELECTED_NODE_ARTIFACT_REF.to_owned(),
        M5_ROUND_TRIP_ARTIFACT_REF.to_owned(),
        M5_BREAKPOINT_PREVIEW_ARTIFACT_REF.to_owned(),
        VISUAL_DESIGNER_A11Y_FALLBACK_ARTIFACT_REF.to_owned(),
        VISUAL_DESIGNER_CONSUMER_ARTIFACT_REF.to_owned(),
        VISUAL_DESIGNER_SURFACE_CERT_ARTIFACT_REF.to_owned(),
    ]
}

/// Reads and validates the checked-in qualification export.
pub fn current_m5_visual_designer_component_qualification_export(
) -> Result<VisualDesignerQualificationPacket, VisualDesignerQualificationArtifactError> {
    let packet: VisualDesignerQualificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-visual-designer-component-qualification-proof/support_export.json"
    )))
    .map_err(VisualDesignerQualificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(VisualDesignerQualificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in qualification export.
#[derive(Debug)]
pub enum VisualDesignerQualificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<VisualDesignerQualificationViolation>),
}

impl fmt::Display for VisualDesignerQualificationArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "qualification export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "qualification export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for VisualDesignerQualificationArtifactError {}

/// Validation failure for M05-811 qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VisualDesignerQualificationViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    MissingConsolidatedPacket {
        packet_ref: String,
    },
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    SharedComponentsNotUsed {
        id: String,
    },
    MissingDimensionCoverage {
        id: String,
    },
    DimensionHidesDrift {
        id: String,
    },
    DishonestNarrowing {
        id: String,
    },
    ExportDropsTruth {
        id: String,
    },
    NarrowedReasonNotExported {
        id: String,
    },
    BundleRefMismatch {
        id: String,
    },
    MissingComponentRefs {
        id: String,
    },
    UncitedComponentPacket {
        id: String,
        packet_ref: String,
    },
    MissingRequiredLabels {
        id: String,
    },
    BlockedRow {
        id: String,
    },
    MissingConsumerCoverage {
        consumer: M5QualifiedComponentConsumer,
    },
    MissingEvidenceConsumer,
    MissingDimensionUnion {
        dimension: M5ComponentQualificationDimension,
    },
    MissingLabelCoverage {
        label: M5VisualDesignerRequiredLabel,
    },
    SummaryMismatch,
    RawBoundaryMaterialInExport,
}

impl fmt::Display for VisualDesignerQualificationViolation {
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
            Self::MissingConsolidatedPacket { packet_ref } => {
                write!(
                    f,
                    "the certification bundle does not consolidate component packet {packet_ref}"
                )
            }
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete qualification row: {id}"),
            Self::SharedComponentsNotUsed { id } => {
                write!(f, "row {id} does not adopt the shared reusable components")
            }
            Self::MissingDimensionCoverage { id } => {
                write!(f, "row {id} does not cover every parity dimension")
            }
            Self::DimensionHidesDrift { id } => {
                write!(
                    f,
                    "row {id} has a parity dimension that hid drift without disclosure"
                )
            }
            Self::DishonestNarrowing { id } => {
                write!(
                    f,
                    "row {id} narrows a dimension without a frozen trigger and a precise reason"
                )
            }
            Self::ExportDropsTruth { id } => {
                write!(
                    f,
                    "row {id} export does not preserve the mandatory per-dimension parity fields"
                )
            }
            Self::NarrowedReasonNotExported { id } => {
                write!(
                    f,
                    "row {id} narrowed but does not export the narrowed reason"
                )
            }
            Self::BundleRefMismatch { id } => {
                write!(
                    f,
                    "row {id} cites a certification bundle other than the packet bundle"
                )
            }
            Self::MissingComponentRefs { id } => {
                write!(f, "row {id} cites no canonical component packets")
            }
            Self::UncitedComponentPacket { id, packet_ref } => {
                write!(
                    f,
                    "row {id} cites component packet {packet_ref} that is not in the bundle"
                )
            }
            Self::MissingRequiredLabels { id } => {
                write!(f, "row {id} preserves no required labels")
            }
            Self::BlockedRow { id } => write!(f, "row {id} is blocked (red) and may not promote"),
            Self::MissingConsumerCoverage { consumer } => {
                write!(
                    f,
                    "claimed consumer {consumer:?} is not qualified in the packet"
                )
            }
            Self::MissingEvidenceConsumer => {
                write!(
                    f,
                    "support, help, or release evidence consumer is not qualified"
                )
            }
            Self::MissingDimensionUnion { dimension } => {
                write!(
                    f,
                    "parity dimension {dimension:?} is not covered by any row"
                )
            }
            Self::MissingLabelCoverage { label } => {
                write!(f, "required label {label:?} is not preserved by any row")
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawBoundaryMaterialInExport => {
                write!(f, "export contains raw boundary material")
            }
        }
    }
}

impl Error for VisualDesignerQualificationViolation {}

/// Whether a narrowed reason label is a generic non-answer rather than a precise
/// label.
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
            | "stale"
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

/// Builds the canonical, checked-in qualification packet. This is the one source of
/// truth shared by the tests, the example dump, and the on-disk support export so
/// all three stay byte-aligned.
pub fn seeded_m5_visual_designer_component_qualification_packet(
) -> VisualDesignerQualificationPacket {
    VisualDesignerQualificationPacket::new(VisualDesignerQualificationPacketInput {
        packet_id: "m5-visual-designer-component-qualification:stable:0001".to_owned(),
        as_of: "2026-07-04T00:00:00Z".to_owned(),
        matrix_ref: VISUAL_DESIGNER_QUALIFICATION_COMPONENT_MATRIX_REF.to_owned(),
        certification_bundle_ref: VISUAL_DESIGNER_QUALIFICATION_BUNDLE_REF.to_owned(),
        certified_component_packets: canonical_component_packet_refs(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:visual-designer-qualification:{id}")]
}

fn all_required_labels() -> Vec<M5VisualDesignerRequiredLabel> {
    M5VisualDesignerRequiredLabel::ALL.to_vec()
}

fn full_export_fields() -> Vec<M5QualificationExportField> {
    M5QualificationExportField::ALL.to_vec()
}

fn full_copy_export() -> CopyExportParity {
    CopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: M5QualificationExportField::ALL
            .iter()
            .map(|f| f.as_str().to_owned())
            .collect(),
        screenshot_only_prohibited: true,
    }
}

fn all_certified_dimensions() -> Vec<QualificationDimensionParity> {
    M5ComponentQualificationDimension::ALL
        .iter()
        .map(|d| QualificationDimensionParity::certified(*d))
        .collect()
}

/// A green (fully qualified) row whose every parity dimension is certified.
fn green_row(
    row_id: &str,
    consumer: M5QualifiedComponentConsumer,
    component_refs: Vec<String>,
    ev_id: &str,
) -> VisualDesignerQualificationRow {
    VisualDesignerQualificationRow {
        record_kind: VISUAL_DESIGNER_QUALIFICATION_ROW_RECORD_KIND.to_owned(),
        schema_version: VISUAL_DESIGNER_QUALIFICATION_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        consumer,
        uses_shared_components: true,
        dimensions: all_certified_dimensions(),
        copy_export: full_copy_export(),
        preserved_export_fields: full_export_fields(),
        required_labels: all_required_labels(),
        canonical_component_refs: component_refs,
        certification_bundle_ref: VISUAL_DESIGNER_QUALIFICATION_BUNDLE_REF.to_owned(),
        source_family_schema_ref: VISUAL_DESIGNER_QUALIFICATION_COMPONENT_MATRIX_REF.to_owned(),
        source_refs: vec![VISUAL_DESIGNER_QUALIFICATION_COMPONENT_MATRIX_REF.to_owned()],
        observed_at: "2026-07-04T00:00:00Z".to_owned(),
        evidence_refs: ev(ev_id),
    }
}

/// A yellow (qualified-with-narrowing) row whose named dimension weakened and
/// disclosed an honest narrowing.
fn narrowed_row(
    row_id: &str,
    consumer: M5QualifiedComponentConsumer,
    component_refs: Vec<String>,
    dimension: M5ComponentQualificationDimension,
    trigger: M5VisualDesignerDowngradeTrigger,
    reason: &str,
    ev_id: &str,
) -> VisualDesignerQualificationRow {
    let mut row = green_row(row_id, consumer, component_refs, ev_id);
    for parity in &mut row.dimensions {
        if parity.dimension == dimension {
            *parity = QualificationDimensionParity::narrowed(dimension, trigger, reason);
        }
    }
    row
}

fn seeded_rows() -> Vec<VisualDesignerQualificationRow> {
    use M5ComponentQualificationDimension as D;
    use M5QualifiedComponentConsumer as C;
    use M5VisualDesignerDowngradeTrigger as T;

    let matrix = VISUAL_DESIGNER_COMPONENT_MATRIX_ARTIFACT_REF.to_owned();
    let selected = M5_SELECTED_NODE_ARTIFACT_REF.to_owned();
    let round_trip = M5_ROUND_TRIP_ARTIFACT_REF.to_owned();
    let breakpoint = M5_BREAKPOINT_PREVIEW_ARTIFACT_REF.to_owned();
    let a11y = VISUAL_DESIGNER_A11Y_FALLBACK_ARTIFACT_REF.to_owned();
    let consumers = VISUAL_DESIGNER_CONSUMER_ARTIFACT_REF.to_owned();
    let surface = VISUAL_DESIGNER_SURFACE_CERT_ARTIFACT_REF.to_owned();

    vec![
        // Visual-design surface — the primary designer consumer draws from the
        // matrix, all three primitives, and the accessibility fallback; every parity
        // dimension holds (green).
        green_row(
            "qual:visual-design-surface",
            C::VisualDesignSurface,
            vec![
                matrix.clone(),
                selected.clone(),
                round_trip.clone(),
                breakpoint.clone(),
                a11y.clone(),
            ],
            "visual-design-surface",
        ),
        // Preview runtime — an approximate source mapping narrows the mapping-quality
        // dimension, disclosed (yellow).
        narrowed_row(
            "qual:preview-runtime",
            C::PreviewRuntime,
            vec![matrix.clone(), breakpoint.clone(), surface.clone()],
            D::MappingQuality,
            T::UnmappedSource,
            "Preview runtime resolves its source mapping only approximately; the mapping-quality claim narrows to disclosed and keeps the source-first anchor visible",
            "preview-runtime",
        ),
        // Framework-pack preview — an open round-trip conflict narrows the round-trip
        // dimension to inspect-only, disclosed (yellow).
        narrowed_row(
            "qual:framework-pack-preview",
            C::FrameworkPackPreview,
            vec![matrix.clone(), round_trip.clone(), consumers.clone()],
            D::RoundTripState,
            T::RoundTripConflictOpen,
            "Framework-pack preview has an open round-trip conflict; the round-trip claim narrows to inspect-only until the conflict resolves, never a silent write-back",
            "framework-pack-preview",
        ),
        // Docs / demo embeds — a read-only embed over the shared components; every
        // parity dimension holds (green).
        green_row(
            "qual:docs-demo-embeds",
            C::DocsDemoEmbeds,
            vec![matrix.clone(), consumers.clone(), a11y.clone()],
            "docs-demo-embeds",
        ),
        // Handoff consumer — a bound expression drifted from its source binding,
        // narrowing the token / binding provenance dimension, disclosed (yellow).
        narrowed_row(
            "qual:handoff-consumer",
            C::HandoffConsumer,
            vec![matrix.clone(), selected.clone(), consumers.clone()],
            D::TokenBindingProvenance,
            T::DriftedFromSource,
            "Handoff consumer detected a bound expression drifted from its source binding; the provenance claim narrows to disclosed and keeps the binding distinct from a literal",
            "handoff-consumer",
        ),
        // Support packet — a source-first support export; every parity dimension holds
        // (green).
        green_row(
            "qual:support-packet",
            C::SupportPacket,
            vec![matrix.clone(), surface.clone(), a11y.clone()],
            "support-packet",
        ),
        // Help center — a read-only help consumer over the shared components; every
        // parity dimension holds (green).
        green_row(
            "qual:help-center",
            C::HelpCenter,
            vec![matrix.clone(), consumers.clone(), surface.clone()],
            "help-center",
        ),
        // Release evidence — the release-proof consumer cites the whole bundle; every
        // parity dimension holds (green).
        green_row(
            "qual:release-evidence",
            C::ReleaseEvidence,
            vec![
                matrix,
                selected,
                round_trip,
                breakpoint,
                a11y,
                consumers,
                surface,
            ],
            "release-evidence",
        ),
    ]
}
