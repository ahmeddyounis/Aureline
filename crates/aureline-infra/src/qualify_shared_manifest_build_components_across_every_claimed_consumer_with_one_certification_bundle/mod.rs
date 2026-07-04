//! One-bundle qualification of the shared manifest / build components across every
//! claimed infrastructure and execution consumer.
//!
//! This module is the M05-819 qualification capstone that CLOSES the B95 manifest /
//! build component lane by consolidating the whole lane into a single referenceable
//! certification bundle. Where the freeze matrix
//! ([`crate::freeze_the_m5_manifest_editor_schema_validator_resource_link_build_adapter_target_graph_and_fallback_confidence_component_matrix`])
//! froze the reusable manifest-editor / schema-validator / target-context-chip /
//! resource-link / resource-explorer / adapter-source-badge / target-graph /
//! capability-matrix / raw-event / fallback-confidence primitives, the 813-815 lanes
//! resolved their per-target truth, the 816 lane added the execution-confidence
//! primitive, the 817 lane adopted them across handoff consumers, and the 818 lane
//! certified accessibility fallback, this lane produces one qualification packet —
//! keyed on the claim-bearing **consumer** — proving that every claimed
//! infrastructure, live-resource, build / run / test / debug, incident / support, and
//! handoff consumer either passes the shared component parity check on every dimension
//! or narrows automatically, and that release / help / support packets can cite a
//! single certification bundle for all of it.
//!
//! Every [`ManifestBuildQualificationRow`] keys on one
//! [`M5QualifiedManifestBuildConsumer`] and certifies five component parity dimensions
//! ([`M5ManifestBuildQualificationDimension`]):
//!
//! - **Target context.** The consumer keeps the target context visible on every read-
//!   or mutate-capable surface; it never blurs which target it acts on.
//! - **Schema freshness.** The consumer keeps schema freshness explicit and never
//!   presents a stale schema as current.
//! - **Truth-layer labels.** The consumer keeps authored / rendered / planned / live /
//!   cached / provider-overlay truth distinct rather than collapsing them.
//! - **Adapter source kind.** The consumer keeps native-vs-fallback adapter provenance
//!   explicit and never lets lower-confidence discovery overwrite higher-confidence
//!   truth silently.
//! - **Accessibility / export behavior.** The consumer keeps a non-visual fallback and
//!   a text / JSON / Markdown export, never a screenshot alone, and its export
//!   preserves the same target IDs, adapter kinds, and freshness / confidence states.
//!
//! Each dimension carries a [`M5ManifestBuildParityState`]: a `certified` dimension
//! passes, a `disclosed_narrowed` dimension weakened and disclosed the narrowing with a
//! frozen [`crate::M5ManifestBuildDowngradeTrigger`] and a precise reason, and an
//! `undisclosed_drift` dimension hid the drift and is rejected. A consumer that drifts
//! on any dimension without disclosure is blocked (red); a consumer that narrows and
//! discloses is qualified-with-narrowing (yellow); a consumer whose every dimension is
//! certified is qualified (green).
//!
//! The packet consolidates the B95 lane: it lists every underlying certified component
//! packet ([`canonical_component_packet_refs`]) — the frozen matrix, the three
//! primitive resolvers, the execution-confidence primitive, the consumer adoption, and
//! the accessibility fallback — and every row cites the one certification bundle so
//! release, help, and support packets reference a single source of truth.
//!
//! The packet is metadata-only: raw manifest bodies, diff hunks, credentials, and
//! provider payloads never cross this boundary; the packet carries only typed class
//! tokens, opaque summary / evidence refs, booleans, and redacted labels.
//!
//! The boundary schema is
//! [`schemas/ui/m5-manifest-build-component-qualification.schema.json`](../../../../schemas/ui/m5-manifest-build-component-qualification.schema.json).
//! The contract doc is
//! [`docs/infra/m5_manifest_build_component_qualification_contract.md`](../../../../docs/infra/m5_manifest_build_component_qualification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    CopyExportParity, M5ManifestBuildDowngradeTrigger, M5ManifestBuildRequiredLabel,
    MANIFEST_BUILD_A11Y_FALLBACK_ARTIFACT_REF, MANIFEST_BUILD_COMPONENT_MATRIX_ARTIFACT_REF,
    MANIFEST_BUILD_CONSUMER_ARTIFACT_REF, M5_BUILD_CONFIDENCE_ARTIFACT_REF,
    M5_EXECUTION_CONFIDENCE_ARTIFACT_REF, M5_LIVE_RESOURCE_ARTIFACT_REF,
    M5_MANIFEST_AUTHORING_ARTIFACT_REF,
};

/// Schema version stamped on the M05-819 qualification packet.
pub const MANIFEST_BUILD_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ManifestBuildQualificationPacket`].
pub const MANIFEST_BUILD_QUALIFICATION_RECORD_KIND: &str =
    "m5_manifest_build_component_qualification_packet";

/// Stable record-kind tag carried by each [`ManifestBuildQualificationRow`].
pub const MANIFEST_BUILD_QUALIFICATION_ROW_RECORD_KIND: &str =
    "m5_manifest_build_component_qualification_row";

/// Repo-relative path of the boundary schema.
pub const MANIFEST_BUILD_QUALIFICATION_SCHEMA_REF: &str =
    "schemas/ui/m5-manifest-build-component-qualification.schema.json";

/// Repo-relative path of the contract doc.
pub const MANIFEST_BUILD_QUALIFICATION_DOC_REF: &str =
    "docs/infra/m5_manifest_build_component_qualification_contract.md";

/// Repo-relative path of the frozen manifest / build component matrix this lane
/// qualifies against.
pub const MANIFEST_BUILD_QUALIFICATION_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-manifest-build-component-matrix.schema.json";

/// Repo-relative path of the one certification bundle every row cites (AC2); identical
/// to the checked support-export artifact.
pub const MANIFEST_BUILD_QUALIFICATION_BUNDLE_REF: &str =
    "artifacts/release/m5-manifest-build-component-qualification-proof/support_export.json";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical); identical to the bundle ref.
pub const MANIFEST_BUILD_QUALIFICATION_ARTIFACT_REF: &str =
    "artifacts/release/m5-manifest-build-component-qualification-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const MANIFEST_BUILD_QUALIFICATION_CSV_REF: &str =
    "artifacts/release/m5-manifest-build-component-qualification-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const MANIFEST_BUILD_QUALIFICATION_REPORT_REF: &str =
    "artifacts/release/m5-manifest-build-component-qualification-proof/report.md";

/// Repo-relative path of the protected fixture directory.
pub const MANIFEST_BUILD_QUALIFICATION_FIXTURE_DIR: &str =
    "fixtures/ui/m5-manifest-build-component-qualification";

/// The certification state of one parity dimension on one consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManifestBuildParityState {
    /// The dimension passes and still supports the shared component claim.
    Certified,
    /// The dimension weakened and the consumer disclosed the narrowing with a frozen
    /// trigger and a precise reason.
    DisclosedNarrowed,
    /// The dimension hid drift; the consumer is rejected.
    UndisclosedDrift,
}

impl M5ManifestBuildParityState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The claim-bearing consumer of the shared manifest / build components a qualification
/// row keys on. Spans the infrastructure, live-resource, execution launcher, and
/// incident-support consumers, plus the support, help, and release evidence consumers
/// that must reference the same certification bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5QualifiedManifestBuildConsumer {
    /// The infrastructure manifest / target-context surface.
    InfrastructureSurface,
    /// The live-resource link / explorer / compare surface.
    LiveResourceSurface,
    /// The build / run / test / debug execution launcher.
    ExecutionLauncher,
    /// An incident / support consumer.
    IncidentSupport,
    /// A cross-surface handoff consumer.
    HandoffConsumer,
    /// A support export packet.
    SupportPacket,
    /// A help-center consumer.
    HelpCenter,
    /// A release-evidence consumer.
    ReleaseEvidence,
}

impl M5QualifiedManifestBuildConsumer {
    /// Every claimed consumer, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::InfrastructureSurface,
        Self::LiveResourceSurface,
        Self::ExecutionLauncher,
        Self::IncidentSupport,
        Self::HandoffConsumer,
        Self::SupportPacket,
        Self::HelpCenter,
        Self::ReleaseEvidence,
    ];

    /// The support, help, and release evidence consumers that must reference the one
    /// certification bundle (AC2).
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
            Self::InfrastructureSurface => "infrastructure_surface",
            Self::LiveResourceSurface => "live_resource_surface",
            Self::ExecutionLauncher => "execution_launcher",
            Self::IncidentSupport => "incident_support",
            Self::HandoffConsumer => "handoff_consumer",
            Self::SupportPacket => "support_packet",
            Self::HelpCenter => "help_center",
            Self::ReleaseEvidence => "release_evidence",
        }
    }
}

/// A component parity dimension every consumer is qualified against. A consumer that
/// drifts on any dimension without disclosure fails promotion; a consumer that narrows
/// and discloses is qualified with a disclosed narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManifestBuildQualificationDimension {
    /// The target context stays visible on every read- or mutate-capable surface.
    TargetContext,
    /// Schema freshness stays explicit; a stale schema is never presented as current.
    SchemaFreshness,
    /// Authored / rendered / planned / live / cached / provider-overlay truth stays
    /// distinct.
    TruthLayerLabels,
    /// Native-vs-fallback adapter provenance stays explicit; lower-confidence discovery
    /// never overwrites higher-confidence truth silently.
    AdapterSourceKind,
    /// A non-visual fallback and a text / JSON / Markdown export are preserved, and the
    /// export preserves the same target and confidence truth.
    AccessibilityExportBehavior,
}

impl M5ManifestBuildQualificationDimension {
    /// Every parity dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TargetContext,
        Self::SchemaFreshness,
        Self::TruthLayerLabels,
        Self::AdapterSourceKind,
        Self::AccessibilityExportBehavior,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetContext => "target_context",
            Self::SchemaFreshness => "schema_freshness",
            Self::TruthLayerLabels => "truth_layer_labels",
            Self::AdapterSourceKind => "adapter_source_kind",
            Self::AccessibilityExportBehavior => "accessibility_export_behavior",
        }
    }
}

/// The parity result for one dimension on one consumer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestBuildQualificationDimensionParity {
    /// The parity dimension this result covers.
    pub dimension: M5ManifestBuildQualificationDimension,
    /// The certification state of the dimension.
    pub state: M5ManifestBuildParityState,
    /// The frozen downgrade trigger that caused a narrowing; present iff the dimension
    /// narrowed (reused vocabulary).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<M5ManifestBuildDowngradeTrigger>,
    /// A precise, non-generic reason label; required when the dimension narrowed.
    #[serde(default)]
    pub reason_label: String,
}

impl ManifestBuildQualificationDimensionParity {
    /// A certified dimension whose truth still supports the shared component claim.
    pub fn certified(dimension: M5ManifestBuildQualificationDimension) -> Self {
        Self {
            dimension,
            state: M5ManifestBuildParityState::Certified,
            trigger: None,
            reason_label: String::new(),
        }
    }

    /// A disclosed-narrowed dimension carrying a frozen trigger and precise reason.
    pub fn narrowed(
        dimension: M5ManifestBuildQualificationDimension,
        trigger: M5ManifestBuildDowngradeTrigger,
        reason: &str,
    ) -> Self {
        Self {
            dimension,
            state: M5ManifestBuildParityState::DisclosedNarrowed,
            trigger: Some(trigger),
            reason_label: reason.to_owned(),
        }
    }

    /// Whether the dimension parity is honest: a certified dimension carries no spurious
    /// trigger, a narrowed dimension carries a trigger and a precise, non-generic
    /// reason, and an undisclosed-drift dimension is never honest.
    pub fn is_honest(&self) -> bool {
        match self.state {
            M5ManifestBuildParityState::Certified => self.trigger.is_none(),
            M5ManifestBuildParityState::DisclosedNarrowed => {
                self.trigger.is_some() && !label_is_generic(&self.reason_label)
            }
            M5ManifestBuildParityState::UndisclosedDrift => false,
        }
    }
}

/// A named field the export / support packet preserves so a narrowed consumer can never
/// present as fully qualified in an export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManifestBuildQualificationExportField {
    /// The consumer identity.
    ConsumerIdentity,
    /// The target-context parity state.
    TargetContext,
    /// The schema-freshness parity state.
    SchemaFreshness,
    /// The truth-layer-labels parity state.
    TruthLayerLabels,
    /// The adapter-source-kind parity state.
    AdapterSourceKind,
    /// The accessibility / export parity state.
    AccessibilityBehavior,
    /// The derived qualification verdict.
    Verdict,
    /// The narrowed-capability reason.
    NarrowedReason,
}

impl M5ManifestBuildQualificationExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ConsumerIdentity,
        Self::TargetContext,
        Self::SchemaFreshness,
        Self::TruthLayerLabels,
        Self::AdapterSourceKind,
        Self::AccessibilityBehavior,
        Self::Verdict,
        Self::NarrowedReason,
    ];

    /// The export fields every qualified row MUST preserve so support / release exports
    /// carry the same per-dimension parity visible in-product. The narrowed reason is
    /// only meaningful when a dimension narrowed, so it is not mandatory.
    pub const MANDATORY: [Self; 7] = [
        Self::ConsumerIdentity,
        Self::TargetContext,
        Self::SchemaFreshness,
        Self::TruthLayerLabels,
        Self::AdapterSourceKind,
        Self::AccessibilityBehavior,
        Self::Verdict,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerIdentity => "consumer_identity",
            Self::TargetContext => "target_context",
            Self::SchemaFreshness => "schema_freshness",
            Self::TruthLayerLabels => "truth_layer_labels",
            Self::AdapterSourceKind => "adapter_source_kind",
            Self::AccessibilityBehavior => "accessibility_behavior",
            Self::Verdict => "verdict",
            Self::NarrowedReason => "narrowed_reason",
        }
    }
}

/// Derived qualification verdict for a consumer row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManifestBuildQualificationVerdict {
    /// Every parity dimension is certified (green).
    Qualified,
    /// A parity dimension weakened and the consumer narrowed and disclosed (yellow).
    QualifiedWithNarrowing,
    /// A dimension hid drift, the export dropped truth, or the consumer does not use the
    /// shared components (red) — may not promote.
    Blocked,
}

impl M5ManifestBuildQualificationVerdict {
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
pub struct ManifestBuildQualificationRow {
    /// Record kind; must equal [`MANIFEST_BUILD_QUALIFICATION_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`MANIFEST_BUILD_QUALIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The claimed consumer this row qualifies.
    pub consumer: M5QualifiedManifestBuildConsumer,
    /// The consumer adopts the shared reusable components rather than a local fork; must
    /// hold (AC1: every claim-bearing surface uses the same target-context and
    /// confidence truth).
    pub uses_shared_components: bool,
    /// The target context this consumer acts on; must stay visible on every surface.
    pub target_context_ref: String,
    /// The per-dimension parity results; must cover every
    /// [`M5ManifestBuildQualificationDimension`] exactly once.
    #[serde(default)]
    pub dimensions: Vec<ManifestBuildQualificationDimensionParity>,
    /// The copy / export parity of the consumer's support / release export (reused
    /// vocabulary).
    pub copy_export: CopyExportParity,
    /// The named export fields the support / release packet preserves.
    #[serde(default)]
    pub preserved_export_fields: Vec<M5ManifestBuildQualificationExportField>,
    /// The required labels the consumer preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5ManifestBuildRequiredLabel>,
    /// The canonical component packets this consumer's qualification draws from; must be
    /// a non-empty subset of the packet's certified component packets.
    #[serde(default)]
    pub canonical_component_refs: Vec<String>,
    /// The one certification bundle this row cites (AC2); must equal the packet-level
    /// bundle ref.
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

impl ManifestBuildQualificationRow {
    /// The parity result for one dimension, if present.
    pub fn dimension(
        &self,
        dimension: M5ManifestBuildQualificationDimension,
    ) -> Option<&ManifestBuildQualificationDimensionParity> {
        self.dimensions.iter().find(|d| d.dimension == dimension)
    }

    /// Whether the row covers every parity dimension exactly once.
    pub fn covers_all_dimensions(&self) -> bool {
        let seen: BTreeSet<M5ManifestBuildQualificationDimension> =
            self.dimensions.iter().map(|d| d.dimension).collect();
        seen.len() == self.dimensions.len()
            && M5ManifestBuildQualificationDimension::ALL
                .iter()
                .all(|d| seen.contains(d))
    }

    /// Whether every dimension parity is honest (no undisclosed drift, disclosed
    /// narrowings carry a trigger and a precise reason).
    pub fn dimensions_honest(&self) -> bool {
        self.dimensions
            .iter()
            .all(ManifestBuildQualificationDimensionParity::is_honest)
    }

    /// True when any dimension hid drift without disclosure.
    pub fn hides_drift(&self) -> bool {
        self.dimensions
            .iter()
            .any(|d| d.state == M5ManifestBuildParityState::UndisclosedDrift)
    }

    /// True when any dimension disclosed a narrowing (yellow).
    pub fn is_narrowed(&self) -> bool {
        self.dimensions
            .iter()
            .any(|d| d.state == M5ManifestBuildParityState::DisclosedNarrowed)
    }

    /// AC1: the target context stays visible on this consumer's surface.
    pub fn preserves_target_context(&self) -> bool {
        !self.target_context_ref.trim().is_empty()
    }

    /// Whether the export preserves the mandatory per-dimension parity fields and a
    /// text / JSON / Markdown copy (never a screenshot alone).
    pub fn export_preserves_truth(&self) -> bool {
        self.copy_export.is_export_safe()
            && M5ManifestBuildQualificationExportField::MANDATORY
                .iter()
                .all(|field| self.preserved_export_fields.contains(field))
    }

    /// AC3: when a dimension narrowed, the narrowed reason is preserved in the export so
    /// support / release exports can reconstruct why the consumer narrowed.
    pub fn narrowed_reason_exported(&self) -> bool {
        !self.is_narrowed()
            || self
                .preserved_export_fields
                .contains(&M5ManifestBuildQualificationExportField::NarrowedReason)
    }

    /// Derived qualification verdict.
    pub fn verdict(&self) -> M5ManifestBuildQualificationVerdict {
        if !self.uses_shared_components
            || !self.preserves_target_context()
            || self.hides_drift()
            || !self.dimensions_honest()
            || !self.covers_all_dimensions()
            || !self.export_preserves_truth()
            || !self.narrowed_reason_exported()
        {
            return M5ManifestBuildQualificationVerdict::Blocked;
        }
        if self.is_narrowed() {
            M5ManifestBuildQualificationVerdict::QualifiedWithNarrowing
        } else {
            M5ManifestBuildQualificationVerdict::Qualified
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == MANIFEST_BUILD_QUALIFICATION_ROW_RECORD_KIND
            && self.schema_version == MANIFEST_BUILD_QUALIFICATION_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.target_context_ref.trim().is_empty()
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
            "consumer={consumer} target_context={target} schema_freshness={schema} \
truth_layers={truth} adapter_source={adapter} accessibility={access} verdict={verdict}",
            consumer = self.consumer.as_str(),
            target = self.dimension_token(M5ManifestBuildQualificationDimension::TargetContext),
            schema = self.dimension_token(M5ManifestBuildQualificationDimension::SchemaFreshness),
            truth = self.dimension_token(M5ManifestBuildQualificationDimension::TruthLayerLabels),
            adapter =
                self.dimension_token(M5ManifestBuildQualificationDimension::AdapterSourceKind),
            access = self
                .dimension_token(M5ManifestBuildQualificationDimension::AccessibilityExportBehavior),
            verdict = self.verdict().as_str(),
        )
    }

    fn dimension_token(&self, dimension: M5ManifestBuildQualificationDimension) -> &'static str {
        match self.dimension(dimension).map(|d| d.state) {
            Some(state) => state.as_str(),
            None => "absent",
        }
    }
}

/// Rolled-up summary of an M05-819 qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestBuildQualificationSummary {
    pub consumer_count: usize,
    pub all_use_shared_components: bool,
    pub all_preserve_target_context: bool,
    pub all_exports_preserve_truth: bool,
    pub all_narrowing_disclosed: bool,
    pub evidence_consumers_present: bool,
    pub dimensions_covered: usize,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub canonical_packet_count: usize,
}

/// Constructor input for [`ManifestBuildQualificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestBuildQualificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub certification_bundle_ref: String,
    pub certified_component_packets: Vec<String>,
    pub rows: Vec<ManifestBuildQualificationRow>,
}

/// Checked-in M05-819 qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestBuildQualificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub certification_bundle_ref: String,
    #[serde(default)]
    pub certified_component_packets: Vec<String>,
    #[serde(default)]
    pub rows: Vec<ManifestBuildQualificationRow>,
    pub summary: ManifestBuildQualificationSummary,
}

impl ManifestBuildQualificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: ManifestBuildQualificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: MANIFEST_BUILD_QUALIFICATION_SCHEMA_VERSION,
            record_kind: MANIFEST_BUILD_QUALIFICATION_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            certification_bundle_ref: input.certification_bundle_ref,
            certified_component_packets: input.certified_component_packets,
            rows: input.rows,
            summary: ManifestBuildQualificationSummary {
                consumer_count: 0,
                all_use_shared_components: false,
                all_preserve_target_context: false,
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
    pub fn represented_consumers(&self) -> BTreeSet<M5QualifiedManifestBuildConsumer> {
        self.rows.iter().map(|r| r.consumer).collect()
    }

    /// Whether the support, help, and release evidence consumers are all qualified.
    pub fn evidence_consumers_present(&self) -> bool {
        let represented = self.represented_consumers();
        M5QualifiedManifestBuildConsumer::EVIDENCE_CONSUMERS
            .iter()
            .all(|c| represented.contains(c))
    }

    /// The union of parity dimensions certified or narrowed across every row.
    pub fn covered_dimensions(&self) -> BTreeSet<M5ManifestBuildQualificationDimension> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            for parity in &row.dimensions {
                set.insert(parity.dimension);
            }
        }
        set
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ManifestBuildQualificationSummary {
        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.verdict() {
                M5ManifestBuildQualificationVerdict::Qualified => green += 1,
                M5ManifestBuildQualificationVerdict::QualifiedWithNarrowing => yellow += 1,
                M5ManifestBuildQualificationVerdict::Blocked => red += 1,
            }
        }

        ManifestBuildQualificationSummary {
            consumer_count: self.rows.len(),
            all_use_shared_components: self.rows.iter().all(|r| r.uses_shared_components),
            all_preserve_target_context: self
                .rows
                .iter()
                .all(ManifestBuildQualificationRow::preserves_target_context),
            all_exports_preserve_truth: self
                .rows
                .iter()
                .all(ManifestBuildQualificationRow::export_preserves_truth),
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
    pub fn validate(&self) -> Vec<ManifestBuildQualificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != MANIFEST_BUILD_QUALIFICATION_SCHEMA_VERSION {
            violations.push(ManifestBuildQualificationViolation::SchemaVersion {
                expected: MANIFEST_BUILD_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != MANIFEST_BUILD_QUALIFICATION_RECORD_KIND {
            violations.push(ManifestBuildQualificationViolation::RecordKind {
                expected: MANIFEST_BUILD_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
            || self.certification_bundle_ref.trim().is_empty()
            || self.certified_component_packets.is_empty()
        {
            violations.push(ManifestBuildQualificationViolation::MissingIdentity);
        }

        // The one bundle must consolidate every canonical B95 component packet.
        for canonical in canonical_component_packet_refs() {
            if !self.certified_component_packets.contains(&canonical) {
                violations.push(
                    ManifestBuildQualificationViolation::MissingConsolidatedPacket {
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
                violations.push(ManifestBuildQualificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_consumers.insert(row.consumer);
            label_union.extend(row.required_labels.iter().copied());

            if !row.is_complete() {
                violations.push(ManifestBuildQualificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // AC1: the consumer adopts the shared components, not a local fork.
            if !row.uses_shared_components {
                violations.push(ManifestBuildQualificationViolation::SharedComponentsNotUsed {
                    id: row.row_id.clone(),
                });
            }

            // AC1: the target context stays visible.
            if !row.preserves_target_context() {
                violations.push(ManifestBuildQualificationViolation::TargetContextDropped {
                    id: row.row_id.clone(),
                });
            }

            // AC1: every parity dimension is present.
            if !row.covers_all_dimensions() {
                violations.push(
                    ManifestBuildQualificationViolation::MissingDimensionCoverage {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: no dimension hides drift; disclosed narrowings stay honest.
            if row.hides_drift() {
                violations.push(ManifestBuildQualificationViolation::DimensionHidesDrift {
                    id: row.row_id.clone(),
                });
            }
            if !row.dimensions_honest() {
                violations.push(ManifestBuildQualificationViolation::DishonestNarrowing {
                    id: row.row_id.clone(),
                });
            }

            // AC3: the export preserves the same per-dimension parity, including the
            // narrowed reason when the consumer narrowed.
            if !row.export_preserves_truth() {
                violations.push(ManifestBuildQualificationViolation::ExportDropsTruth {
                    id: row.row_id.clone(),
                });
            }
            if !row.narrowed_reason_exported() {
                violations.push(
                    ManifestBuildQualificationViolation::NarrowedReasonNotExported {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC2: each row cites the one certification bundle and draws from the
            // consolidated component packets.
            if row.certification_bundle_ref != self.certification_bundle_ref {
                violations.push(ManifestBuildQualificationViolation::BundleRefMismatch {
                    id: row.row_id.clone(),
                });
            }
            if row.canonical_component_refs.is_empty() {
                violations.push(ManifestBuildQualificationViolation::MissingComponentRefs {
                    id: row.row_id.clone(),
                });
            }
            for component_ref in &row.canonical_component_refs {
                if !certified.contains(component_ref) {
                    violations.push(
                        ManifestBuildQualificationViolation::UncitedComponentPacket {
                            id: row.row_id.clone(),
                            packet_ref: component_ref.clone(),
                        },
                    );
                }
            }

            if row.required_labels.is_empty() {
                violations.push(ManifestBuildQualificationViolation::MissingRequiredLabels {
                    id: row.row_id.clone(),
                });
            }

            if row.verdict() == M5ManifestBuildQualificationVerdict::Blocked {
                violations.push(ManifestBuildQualificationViolation::BlockedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every claimed consumer is qualified at least once.
        for consumer in M5QualifiedManifestBuildConsumer::ALL {
            if !seen_consumers.contains(&consumer) {
                violations
                    .push(ManifestBuildQualificationViolation::MissingConsumerCoverage { consumer });
            }
        }

        // AC2: the support, help, and release evidence consumers are present.
        if !self.evidence_consumers_present() {
            violations.push(ManifestBuildQualificationViolation::MissingEvidenceConsumer);
        }

        // Every parity dimension is covered by the packet as a whole.
        let covered = self.covered_dimensions();
        for dimension in M5ManifestBuildQualificationDimension::ALL {
            if !covered.contains(&dimension) {
                violations
                    .push(ManifestBuildQualificationViolation::MissingDimensionUnion { dimension });
            }
        }

        // The union of preserved required labels covers the frozen set.
        for label in M5ManifestBuildRequiredLabel::ALL {
            if !label_union.contains(&label) {
                violations.push(ManifestBuildQualificationViolation::MissingLabelCoverage { label });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ManifestBuildQualificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("qualification packet serializes"),
        ) {
            violations.push(ManifestBuildQualificationViolation::RawBoundaryMaterialInExport);
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
            "row_id,consumer,target_context,schema_freshness,truth_layer_labels,adapter_source_kind,accessibility_export_behavior,verdict\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{consumer},{target},{schema},{truth},{adapter},{access},{verdict}\n",
                id = row.row_id,
                consumer = row.consumer.as_str(),
                target = row.dimension_token(M5ManifestBuildQualificationDimension::TargetContext),
                schema =
                    row.dimension_token(M5ManifestBuildQualificationDimension::SchemaFreshness),
                truth =
                    row.dimension_token(M5ManifestBuildQualificationDimension::TruthLayerLabels),
                adapter =
                    row.dimension_token(M5ManifestBuildQualificationDimension::AdapterSourceKind),
                access = row.dimension_token(
                    M5ManifestBuildQualificationDimension::AccessibilityExportBehavior,
                ),
                verdict = row.verdict().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Manifest / Build Component Qualification\n\n");
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
            M5QualifiedManifestBuildConsumer::ALL.len(),
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
                if parity.state != M5ManifestBuildParityState::Certified {
                    out.push_str(&format!(
                        "  - Narrowed: dimension={} trigger={} — {}\n",
                        parity.dimension.as_str(),
                        parity
                            .trigger
                            .map(M5ManifestBuildDowngradeTrigger::as_str)
                            .unwrap_or("none"),
                        parity.reason_label,
                    ));
                }
            }
        }
        out
    }
}

/// The canonical B95 component packets the one certification bundle consolidates — the
/// frozen matrix, the three primitive resolvers, the execution-confidence primitive,
/// the consumer adoption, and the accessibility fallback.
pub fn canonical_component_packet_refs() -> Vec<String> {
    vec![
        MANIFEST_BUILD_COMPONENT_MATRIX_ARTIFACT_REF.to_owned(),
        M5_MANIFEST_AUTHORING_ARTIFACT_REF.to_owned(),
        M5_LIVE_RESOURCE_ARTIFACT_REF.to_owned(),
        M5_BUILD_CONFIDENCE_ARTIFACT_REF.to_owned(),
        M5_EXECUTION_CONFIDENCE_ARTIFACT_REF.to_owned(),
        MANIFEST_BUILD_CONSUMER_ARTIFACT_REF.to_owned(),
        MANIFEST_BUILD_A11Y_FALLBACK_ARTIFACT_REF.to_owned(),
    ]
}

/// Reads and validates the checked-in qualification export.
pub fn current_m5_manifest_build_component_qualification_export(
) -> Result<ManifestBuildQualificationPacket, ManifestBuildQualificationArtifactError> {
    let packet: ManifestBuildQualificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-manifest-build-component-qualification-proof/support_export.json"
    )))
    .map_err(ManifestBuildQualificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ManifestBuildQualificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in qualification export.
#[derive(Debug)]
pub enum ManifestBuildQualificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ManifestBuildQualificationViolation>),
}

impl fmt::Display for ManifestBuildQualificationArtifactError {
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

impl Error for ManifestBuildQualificationArtifactError {}

/// Validation failure for M05-819 qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestBuildQualificationViolation {
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
    TargetContextDropped {
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
        consumer: M5QualifiedManifestBuildConsumer,
    },
    MissingEvidenceConsumer,
    MissingDimensionUnion {
        dimension: M5ManifestBuildQualificationDimension,
    },
    MissingLabelCoverage {
        label: M5ManifestBuildRequiredLabel,
    },
    SummaryMismatch,
    RawBoundaryMaterialInExport,
}

impl fmt::Display for ManifestBuildQualificationViolation {
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
            Self::TargetContextDropped { id } => {
                write!(f, "row {id} drops the target context")
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
                write!(f, "row {id} narrowed but does not export the narrowed reason")
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
                write!(f, "parity dimension {dimension:?} is not covered by any row")
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

impl Error for ManifestBuildQualificationViolation {}

/// Whether a narrowed reason label is a generic non-answer rather than a precise label.
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
/// truth shared by the tests, the emit binary, and the on-disk support export so all
/// three stay byte-aligned.
pub fn seeded_m5_manifest_build_component_qualification_packet() -> ManifestBuildQualificationPacket
{
    ManifestBuildQualificationPacket::new(ManifestBuildQualificationPacketInput {
        packet_id: "m5-manifest-build-component-qualification:stable:0001".to_owned(),
        as_of: "2026-07-04T00:00:00Z".to_owned(),
        matrix_ref: MANIFEST_BUILD_QUALIFICATION_COMPONENT_MATRIX_REF.to_owned(),
        certification_bundle_ref: MANIFEST_BUILD_QUALIFICATION_BUNDLE_REF.to_owned(),
        certified_component_packets: canonical_component_packet_refs(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:manifest-build-qualification:{id}")]
}

fn all_required_labels() -> Vec<M5ManifestBuildRequiredLabel> {
    M5ManifestBuildRequiredLabel::ALL.to_vec()
}

fn full_export_fields() -> Vec<M5ManifestBuildQualificationExportField> {
    M5ManifestBuildQualificationExportField::ALL.to_vec()
}

fn all_certified_dimensions() -> Vec<ManifestBuildQualificationDimensionParity> {
    M5ManifestBuildQualificationDimension::ALL
        .iter()
        .map(|d| ManifestBuildQualificationDimensionParity::certified(*d))
        .collect()
}

/// A green (fully qualified) row whose every parity dimension is certified.
fn green_row(
    row_id: &str,
    consumer: M5QualifiedManifestBuildConsumer,
    component_refs: Vec<String>,
    ev_id: &str,
) -> ManifestBuildQualificationRow {
    ManifestBuildQualificationRow {
        record_kind: MANIFEST_BUILD_QUALIFICATION_ROW_RECORD_KIND.to_owned(),
        schema_version: MANIFEST_BUILD_QUALIFICATION_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        consumer,
        uses_shared_components: true,
        target_context_ref: format!("target-context:{ev_id}"),
        dimensions: all_certified_dimensions(),
        copy_export: CopyExportParity::full(),
        preserved_export_fields: full_export_fields(),
        required_labels: all_required_labels(),
        canonical_component_refs: component_refs,
        certification_bundle_ref: MANIFEST_BUILD_QUALIFICATION_BUNDLE_REF.to_owned(),
        source_family_schema_ref: MANIFEST_BUILD_QUALIFICATION_COMPONENT_MATRIX_REF.to_owned(),
        source_refs: vec![MANIFEST_BUILD_QUALIFICATION_COMPONENT_MATRIX_REF.to_owned()],
        observed_at: "2026-07-04T00:00:00Z".to_owned(),
        evidence_refs: ev(ev_id),
    }
}

/// A yellow (qualified-with-narrowing) row whose named dimension weakened and disclosed
/// an honest narrowing.
#[allow(clippy::too_many_arguments)]
fn narrowed_row(
    row_id: &str,
    consumer: M5QualifiedManifestBuildConsumer,
    component_refs: Vec<String>,
    dimension: M5ManifestBuildQualificationDimension,
    trigger: M5ManifestBuildDowngradeTrigger,
    reason: &str,
    ev_id: &str,
) -> ManifestBuildQualificationRow {
    let mut row = green_row(row_id, consumer, component_refs, ev_id);
    for parity in &mut row.dimensions {
        if parity.dimension == dimension {
            *parity =
                ManifestBuildQualificationDimensionParity::narrowed(dimension, trigger, reason);
        }
    }
    row
}

fn seeded_rows() -> Vec<ManifestBuildQualificationRow> {
    use M5ManifestBuildDowngradeTrigger as T;
    use M5ManifestBuildQualificationDimension as D;
    use M5QualifiedManifestBuildConsumer as C;

    let matrix = MANIFEST_BUILD_COMPONENT_MATRIX_ARTIFACT_REF.to_owned();
    let authoring = M5_MANIFEST_AUTHORING_ARTIFACT_REF.to_owned();
    let live_resource = M5_LIVE_RESOURCE_ARTIFACT_REF.to_owned();
    let build = M5_BUILD_CONFIDENCE_ARTIFACT_REF.to_owned();
    let execution = M5_EXECUTION_CONFIDENCE_ARTIFACT_REF.to_owned();
    let consumers = MANIFEST_BUILD_CONSUMER_ARTIFACT_REF.to_owned();
    let a11y = MANIFEST_BUILD_A11Y_FALLBACK_ARTIFACT_REF.to_owned();

    vec![
        // Infrastructure surface — the primary manifest / target-context consumer draws
        // from the matrix, the authoring and live-resource primitives, the consumer
        // adoption, and the accessibility fallback; every parity dimension holds
        // (green).
        green_row(
            "qual:infrastructure-surface",
            C::InfrastructureSurface,
            vec![
                matrix.clone(),
                authoring.clone(),
                live_resource.clone(),
                consumers.clone(),
                a11y.clone(),
            ],
            "infrastructure-surface",
        ),
        // Live-resource surface — a rendered/live divergence narrows the truth-layer
        // dimension, disclosed (yellow).
        narrowed_row(
            "qual:live-resource-surface",
            C::LiveResourceSurface,
            vec![matrix.clone(), live_resource.clone(), consumers.clone()],
            D::TruthLayerLabels,
            T::DriftFromSource,
            "Live-resource surface detected rendered-vs-live divergence; the truth-layer claim narrows to disclosed and shows the divergence before presenting live as current",
            "live-resource-surface",
        ),
        // Execution launcher — a fallback adapter discovery narrows the adapter-source
        // dimension, disclosed (yellow).
        narrowed_row(
            "qual:execution-launcher",
            C::ExecutionLauncher,
            vec![matrix.clone(), build.clone(), execution.clone()],
            D::AdapterSourceKind,
            T::AdapterUnavailable,
            "Build launcher fell back to heuristic adapter discovery; the adapter-source claim narrows to disclosed and keeps native-vs-fallback provenance visible before any run",
            "execution-launcher",
        ),
        // Incident support — a stale manifest schema mirror narrows the schema-freshness
        // dimension, disclosed (yellow).
        narrowed_row(
            "qual:incident-support",
            C::IncidentSupport,
            vec![matrix.clone(), authoring.clone(), consumers.clone()],
            D::SchemaFreshness,
            T::SchemaStale,
            "Incident-support export references a stale manifest schema mirror; the schema-freshness claim narrows to disclosed and marks the mirror pending refresh",
            "incident-support",
        ),
        // Handoff consumer — a read-forward handoff over the shared components; every
        // parity dimension holds (green).
        green_row(
            "qual:handoff-consumer",
            C::HandoffConsumer,
            vec![matrix.clone(), live_resource.clone(), consumers.clone()],
            "handoff-consumer",
        ),
        // Support packet — a target-first support export; every parity dimension holds
        // (green).
        green_row(
            "qual:support-packet",
            C::SupportPacket,
            vec![matrix.clone(), a11y.clone(), consumers.clone()],
            "support-packet",
        ),
        // Help center — a read-only help consumer over the shared components; every
        // parity dimension holds (green).
        green_row(
            "qual:help-center",
            C::HelpCenter,
            vec![matrix.clone(), consumers.clone(), a11y.clone()],
            "help-center",
        ),
        // Release evidence — the release-proof consumer cites the whole bundle; every
        // parity dimension holds (green).
        green_row(
            "qual:release-evidence",
            C::ReleaseEvidence,
            vec![
                matrix,
                authoring,
                live_resource,
                build,
                execution,
                consumers,
                a11y,
            ],
            "release-evidence",
        ),
    ]
}
