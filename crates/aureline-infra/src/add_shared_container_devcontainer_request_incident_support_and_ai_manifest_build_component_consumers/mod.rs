//! Shared first consumers for the frozen M5 manifest / build-confidence
//! component families.
//!
//! This module is the **M05-817** first-consumer adoption lane over the frozen
//! M5 manifest / build-confidence component matrix
//! ([`crate::freeze_the_m5_manifest_editor_schema_validator_resource_link_build_adapter_target_graph_and_fallback_confidence_component_matrix`],
//! M05-812) and the three narrowing primitives that implement it (M05-813
//! manifest-authoring, M05-814 live-resource navigation, M05-815
//! build-confidence). Where 812 froze the ten reusable component families and
//! 813-816 turned them into working resolvers, this lane proves the families are
//! reusable *primitives* rather than one infra page and one launcher page by
//! adopting them across the four claimed M5 handoff consumer classes:
//!
//! 1. a container / devcontainer consumer,
//! 2. a request or live-resource handoff consumer,
//! 3. an incident / support consumer, and
//! 4. an AI / explanation consumer.
//!
//! Each adoption row points back to exactly one canonical component family — the
//! schema and support-export packet of the primitive that owns it — instead of
//! cloning surface-local prose, and every consumer (even a read-only,
//! inspect-only, compare-only, export-only, or policy-blocked one) keeps the
//! same target-context identity, freshness, schema source, adapter source,
//! discovery confidence, and degraded-state language the primary infra / build
//! surfaces render. A narrower consumer discloses the reduction with a
//! reduced-capability banner (and, when it punts to another surface, a
//! companion / browser / handoff note) rather than renaming or dropping governed
//! state.
//!
//! Docs / help, support-export, and release-proof surfaces are wired to the same
//! component truth so AI/explainer and incident/support lanes cite the exact
//! target-context and confidence primitives users saw in the original UI rather
//! than a cloned local vocabulary.
//!
//! The controlled vocabulary this lane depends on — the ten
//! [`M5ManifestBuildComponentFamily`], the six [`M5ManifestBuildRequiredLabel`]
//! (with its mandatory four), and the freshness / schema-source / adapter-source
//! / confidence enums — is re-used verbatim from the frozen matrix so exported
//! labels stay byte-identical to the producer packets.
//!
//! The boundary schema is
//! [`schemas/ui/m5-manifest-build-component-consumer.schema.json`](../../../../schemas/ui/m5-manifest-build-component-consumer.schema.json).
//! The contract doc is
//! [`docs/infra/m5_manifest_build_component_consumer_contract.md`](../../../../docs/infra/m5_manifest_build_component_consumer_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    M5AdapterSourceKind, M5DiscoveryConfidence, M5ManifestBuildComponentFamily,
    M5ManifestBuildRequiredLabel, M5ResourceFreshness, M5SchemaFreshness,
    M5_BUILD_CONFIDENCE_ARTIFACT_REF, M5_BUILD_CONFIDENCE_SCHEMA_REF,
    M5_LIVE_RESOURCE_ARTIFACT_REF, M5_LIVE_RESOURCE_SCHEMA_REF, M5_MANIFEST_AUTHORING_ARTIFACT_REF,
    M5_MANIFEST_AUTHORING_SCHEMA_REF, MANIFEST_BUILD_COMPONENT_MATRIX_SUMMARY_REF,
};

/// Schema version stamped on the M05-817 manifest / build consumer packet.
pub const MANIFEST_BUILD_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ManifestBuildConsumerPacket`].
pub const MANIFEST_BUILD_CONSUMER_RECORD_KIND: &str = "m5_manifest_build_component_consumer_packet";

/// Stable record-kind tag for each [`ManifestBuildConsumerRow`].
pub const MANIFEST_BUILD_CONSUMER_ROW_RECORD_KIND: &str =
    "m5_manifest_build_component_consumer_row";

/// Repo-relative path of the boundary schema.
pub const MANIFEST_BUILD_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-manifest-build-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const MANIFEST_BUILD_CONSUMER_DOC_REF: &str =
    "docs/infra/m5_manifest_build_component_consumer_contract.md";

/// Repo-relative path of the frozen component matrix this lane consumes by
/// reference.
pub const MANIFEST_BUILD_CONSUMER_MATRIX_REF: &str = MANIFEST_BUILD_COMPONENT_MATRIX_SUMMARY_REF;

/// Repo-relative path of the protected fixture directory.
pub const MANIFEST_BUILD_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-manifest-build-component-consumers";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const MANIFEST_BUILD_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-manifest-build-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const MANIFEST_BUILD_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-manifest-build-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const MANIFEST_BUILD_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-manifest-build-component-consumer-proof/report.md";

/// Embedded checked-in M05-817 support export JSON.
pub const MANIFEST_BUILD_CONSUMER_ARTIFACT_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5-manifest-build-component-consumer-proof/support_export.json"
));

// --- canonical family bindings ---------------------------------------------

/// The single canonical schema that governs a component family. Consumers must
/// point at this schema instead of inventing a surface-local one.
///
/// The frozen matrix (M05-812) collapsed the ten families into three narrowing
/// primitives; each family is owned by exactly one of them:
/// manifest-authoring (header / schema-validator / target-context chips),
/// live-resource navigation (resource-link / resource-explorer rows), and
/// build-confidence (adapter badge / target-graph / capability matrix /
/// raw-event / fallback-confidence).
pub const fn canonical_schema_ref_for(family: M5ManifestBuildComponentFamily) -> &'static str {
    use M5ManifestBuildComponentFamily as F;
    match family {
        F::ManifestEditorHeader | F::SchemaValidatorRow | F::TargetContextChipGroup => {
            M5_MANIFEST_AUTHORING_SCHEMA_REF
        }
        F::ResourceLinkRow | F::ResourceExplorerRow => M5_LIVE_RESOURCE_SCHEMA_REF,
        F::AdapterSourceBadge
        | F::TargetGraphRow
        | F::CapabilityMatrix
        | F::RawEventDrawer
        | F::FallbackConfidenceDrawer => M5_BUILD_CONFIDENCE_SCHEMA_REF,
    }
}

/// The canonical support-export packet that defines this family's first
/// consumers. A consumer row must reference this packet rather than cloning
/// surface-local prose.
pub const fn canonical_packet_ref_for(family: M5ManifestBuildComponentFamily) -> &'static str {
    use M5ManifestBuildComponentFamily as F;
    match family {
        F::ManifestEditorHeader | F::SchemaValidatorRow | F::TargetContextChipGroup => {
            M5_MANIFEST_AUTHORING_ARTIFACT_REF
        }
        F::ResourceLinkRow | F::ResourceExplorerRow => M5_LIVE_RESOURCE_ARTIFACT_REF,
        F::AdapterSourceBadge
        | F::TargetGraphRow
        | F::CapabilityMatrix
        | F::RawEventDrawer
        | F::FallbackConfidenceDrawer => M5_BUILD_CONFIDENCE_ARTIFACT_REF,
    }
}

// --- minted controlled vocabulary ------------------------------------------

/// The four claimed M5 handoff consumer classes that must each adopt at least
/// one canonical component family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerGroup {
    /// A container / devcontainer consumer (dev environment manifest + target).
    ContainerDevcontainer,
    /// A request or live-resource handoff consumer.
    RequestLiveResourceHandoff,
    /// An incident / support consumer.
    IncidentSupport,
    /// An AI / explanation consumer.
    AiExplanation,
}

impl ConsumerGroup {
    /// Every consumer group that must be present for reusable-primitive proof.
    pub const ALL: [ConsumerGroup; 4] = [
        ConsumerGroup::ContainerDevcontainer,
        ConsumerGroup::RequestLiveResourceHandoff,
        ConsumerGroup::IncidentSupport,
        ConsumerGroup::AiExplanation,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContainerDevcontainer => "container_devcontainer",
            Self::RequestLiveResourceHandoff => "request_live_resource_handoff",
            Self::IncidentSupport => "incident_support",
            Self::AiExplanation => "ai_explanation",
        }
    }
}

/// A concrete surface that adopts a canonical component family. Each surface is
/// pinned to exactly one [`ConsumerGroup`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManifestBuildConsumerSurface {
    /// Devcontainer manifest editor panel (container/devcontainer).
    DevcontainerManifestPanel,
    /// Container target-graph / target-context inspector (container/devcontainer).
    ContainerTargetGraphInspector,
    /// Resource-link handoff from a request / plan surface (request handoff).
    RequestResourceLinkHandoff,
    /// Live-resource explorer handoff surface (request handoff).
    LiveResourceExplorerHandoff,
    /// Incident / support evidence bundle (incident/support).
    IncidentSupportBundle,
    /// Support-export surface (incident/support).
    SupportExport,
    /// Release-proof evidence surface (incident/support).
    ReleaseProof,
    /// AI execution / build explainer (AI/explanation).
    AiExecutionExplainer,
    /// AI confidence narrative (AI/explanation).
    AiConfidenceNarrative,
    /// Docs / help entry path (AI/explanation).
    DocsHelp,
}

impl M5ManifestBuildConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [M5ManifestBuildConsumerSurface; 10] = [
        Self::DevcontainerManifestPanel,
        Self::ContainerTargetGraphInspector,
        Self::RequestResourceLinkHandoff,
        Self::LiveResourceExplorerHandoff,
        Self::IncidentSupportBundle,
        Self::SupportExport,
        Self::ReleaseProof,
        Self::AiExecutionExplainer,
        Self::AiConfidenceNarrative,
        Self::DocsHelp,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DevcontainerManifestPanel => "devcontainer_manifest_panel",
            Self::ContainerTargetGraphInspector => "container_target_graph_inspector",
            Self::RequestResourceLinkHandoff => "request_resource_link_handoff",
            Self::LiveResourceExplorerHandoff => "live_resource_explorer_handoff",
            Self::IncidentSupportBundle => "incident_support_bundle",
            Self::SupportExport => "support_export",
            Self::ReleaseProof => "release_proof",
            Self::AiExecutionExplainer => "ai_execution_explainer",
            Self::AiConfidenceNarrative => "ai_confidence_narrative",
            Self::DocsHelp => "docs_help",
        }
    }

    /// The consumer group this surface belongs to.
    pub const fn consumer_group(self) -> ConsumerGroup {
        match self {
            Self::DevcontainerManifestPanel | Self::ContainerTargetGraphInspector => {
                ConsumerGroup::ContainerDevcontainer
            }
            Self::RequestResourceLinkHandoff | Self::LiveResourceExplorerHandoff => {
                ConsumerGroup::RequestLiveResourceHandoff
            }
            Self::IncidentSupportBundle | Self::SupportExport | Self::ReleaseProof => {
                ConsumerGroup::IncidentSupport
            }
            Self::AiExecutionExplainer | Self::AiConfidenceNarrative | Self::DocsHelp => {
                ConsumerGroup::AiExplanation
            }
        }
    }

    /// True when this is a docs/help, support-export, or release-proof evidence
    /// surface that must reference the canonical component packets.
    pub const fn is_help_support_release(self) -> bool {
        matches!(
            self,
            Self::DocsHelp | Self::SupportExport | Self::ReleaseProof
        )
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

    /// The `capability_state` label this authority maps to.
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
    /// No handoff: the consumer renders the component in place.
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
}

/// Whether the consumer preserves the canonical component's controlled labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelParityState {
    /// Full target-context / freshness / adapter-source / degraded-state parity.
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

/// A reduced-capability banner a narrowed consumer must render.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReducedCapabilityBanner {
    pub banner_id: String,
    pub visible_label: String,
    /// Must equal the row authority's [`AuthorityMode::capability_state`].
    pub capability_state: String,
    #[serde(default)]
    pub missing_capabilities: Vec<String>,
}

/// Copy / export parity a consumer must preserve: text, JSON, and Markdown are
/// all export-safe and screenshots are never the only carrier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyExportParity {
    pub text_copy: bool,
    pub json_copy: bool,
    pub markdown_copy: bool,
    pub screenshot_only_prohibited: bool,
}

impl CopyExportParity {
    /// A fully export-safe projection: all three text carriers are present and
    /// screenshot-only export is prohibited.
    pub const fn full() -> Self {
        Self {
            text_copy: true,
            json_copy: true,
            markdown_copy: true,
            screenshot_only_prohibited: true,
        }
    }

    /// True when the projection preserves text/JSON/Markdown parity and forbids
    /// screenshot-only export.
    pub const fn is_export_safe(&self) -> bool {
        self.text_copy && self.json_copy && self.markdown_copy && self.screenshot_only_prohibited
    }
}

/// One consumer adopting one canonical component family on one surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestBuildConsumerRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub row_id: String,
    pub consumer_group: ConsumerGroup,
    pub consumer_surface: M5ManifestBuildConsumerSurface,
    /// The single canonical component family this consumer reuses.
    pub component_family: M5ManifestBuildComponentFamily,
    /// The canonical schema for the family. Must equal
    /// `canonical_schema_ref_for(component_family)`.
    pub canonical_family_schema_ref: String,
    /// The canonical producer packet(s) this consumer points back to.
    #[serde(default)]
    pub canonical_packet_refs: Vec<String>,
    /// True when the consumer references the canonical family rather than
    /// cloning surface-local prose.
    pub references_canonical_not_local_prose: bool,
    pub authority_mode: AuthorityMode,
    /// The target-context identity the consumer keeps visible on every surface.
    pub target_context_ref: String,
    /// The resource / target freshness, kept visible on every consumer.
    pub freshness: M5ResourceFreshness,
    /// The schema source / freshness kept visible where a schema is involved.
    pub schema_source: M5SchemaFreshness,
    /// The adapter source kind kept visible where build truth is involved.
    pub adapter_source: M5AdapterSourceKind,
    /// The discovery confidence kept visible on every consumer.
    pub discovery_confidence: M5DiscoveryConfidence,
    /// The badges and target-context / freshness labels preserved verbatim.
    #[serde(default)]
    pub preserved_badge_labels: Vec<String>,
    /// The degraded-state vocabulary the consumer keeps visible even when
    /// narrowed (drawn from the frozen downgrade-trigger tokens).
    #[serde(default)]
    pub degraded_state_vocab: Vec<String>,
    /// The required labels the consumer renders; must include the mandatory four.
    #[serde(default)]
    pub required_labels: Vec<M5ManifestBuildRequiredLabel>,
    pub label_parity: LabelParityState,
    /// The surface a narrower consumer hands off to, if any.
    pub handoff_target: HandoffTarget,
    /// The companion / browser / handoff note ref; required when
    /// `handoff_target` is not `None`.
    #[serde(default)]
    pub handoff_note_ref: String,
    #[serde(default)]
    pub source_refs: Vec<String>,
    pub copy_export: CopyExportParity,
    /// Present iff the consumer narrows below full-interactive authority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reduced_capability_banner: Option<ReducedCapabilityBanner>,
}

impl ManifestBuildConsumerRow {
    /// Returns true when the consumer narrows below full authority.
    pub fn is_narrowed(&self) -> bool {
        self.authority_mode.is_narrowed()
    }

    /// AC1: the consumer points back to exactly one canonical family — the
    /// declared schema matches the family, its canonical packet is referenced,
    /// and no surface-local prose is cloned.
    pub fn points_to_canonical_family(&self) -> bool {
        self.canonical_family_schema_ref == canonical_schema_ref_for(self.component_family)
            && self
                .canonical_packet_refs
                .iter()
                .any(|p| p == canonical_packet_ref_for(self.component_family))
            && self.references_canonical_not_local_prose
    }

    /// AC2: the consumer preserves the family's controlled badges, freshness /
    /// adapter-source labels, degraded-state vocabulary, and required labels
    /// rather than renaming or omitting them.
    pub fn preserves_labels(&self) -> bool {
        self.label_parity.keeps_labels()
            && !self.preserved_badge_labels.is_empty()
            && !self.degraded_state_vocab.is_empty()
            && M5ManifestBuildRequiredLabel::MANDATORY
                .iter()
                .all(|l| self.required_labels.contains(l))
    }

    /// AC2: target-context identity stays visible on every read- or
    /// mutate-capable consumer.
    pub fn preserves_target_context(&self) -> bool {
        !self.target_context_ref.trim().is_empty()
    }

    /// A narrower consumer discloses the reduction with a reduced-capability
    /// banner whose state matches the authority mode, and carries a
    /// companion / browser / handoff note whenever it punts to another surface.
    /// A full-interactive consumer must not carry a spurious banner.
    pub fn discloses_narrowing(&self) -> bool {
        match (&self.reduced_capability_banner, self.is_narrowed()) {
            (Some(banner), true) => {
                if banner.banner_id.trim().is_empty()
                    || banner.visible_label.trim().is_empty()
                    || banner.capability_state != self.authority_mode.capability_state()
                    || banner.capability_state == "full"
                    || banner.missing_capabilities.is_empty()
                    || self.label_parity == LabelParityState::Preserved
                {
                    return false;
                }
            }
            (None, true) => return false,
            (Some(_), false) => return false,
            (None, false) => {}
        }
        if self.handoff_target.requires_note() && self.handoff_note_ref.trim().is_empty() {
            return false;
        }
        true
    }

    /// Whether the adapter source and discovery confidence are consistent, so a
    /// heuristic or imported result never claims native / high-confidence truth.
    pub fn confidence_consistent(&self) -> bool {
        self.adapter_source
            .confidence_consistent(self.discovery_confidence)
    }

    /// True when this is a docs/help, support-export, or release-proof evidence
    /// surface that must reference the canonical component packets.
    pub fn is_help_support_release_surface(&self) -> bool {
        self.consumer_surface.is_help_support_release()
    }
}

/// Rolled-up summary of an M05-817 manifest / build consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestBuildConsumerSummary {
    pub row_count: usize,
    pub consumer_group_count: usize,
    pub consumer_surface_count: usize,
    pub component_family_count: usize,
    pub all_rows_point_to_canonical_family: bool,
    pub all_rows_preserve_labels: bool,
    pub all_rows_preserve_target_context: bool,
    pub all_narrowed_rows_disclose: bool,
    pub all_rows_confidence_consistent: bool,
    pub all_rows_have_copy_export: bool,
    pub families_reused_across_groups: bool,
    pub container_devcontainer_consumer_present: bool,
    pub request_live_resource_consumer_present: bool,
    pub incident_support_consumer_present: bool,
    pub ai_explanation_consumer_present: bool,
    pub help_support_release_reference_present: bool,
}

/// Checked-in M05-817 manifest / build consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestBuildConsumerPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    #[serde(default)]
    pub rows: Vec<ManifestBuildConsumerRow>,
    pub summary: ManifestBuildConsumerSummary,
}

impl ManifestBuildConsumerPacket {
    /// True when a component family is adopted by at least two distinct consumer
    /// groups, proving cross-surface reuse rather than a one-page implementation.
    pub fn has_family_reused_across_groups(&self) -> bool {
        for family in M5ManifestBuildComponentFamily::ALL {
            let groups: BTreeSet<ConsumerGroup> = self
                .rows
                .iter()
                .filter(|r| r.component_family == family)
                .map(|r| r.consumer_group)
                .collect();
            if groups.len() >= 2 {
                return true;
            }
        }
        false
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ManifestBuildConsumerSummary {
        let mut groups = BTreeSet::new();
        let mut surfaces = BTreeSet::new();
        let mut families = BTreeSet::new();
        for row in &self.rows {
            groups.insert(row.consumer_group);
            surfaces.insert(row.consumer_surface);
            families.insert(row.component_family);
        }

        let has_group = |g: ConsumerGroup| groups.contains(&g);

        ManifestBuildConsumerSummary {
            row_count: self.rows.len(),
            consumer_group_count: groups.len(),
            consumer_surface_count: surfaces.len(),
            component_family_count: families.len(),
            all_rows_point_to_canonical_family: self
                .rows
                .iter()
                .all(ManifestBuildConsumerRow::points_to_canonical_family),
            all_rows_preserve_labels: self
                .rows
                .iter()
                .all(ManifestBuildConsumerRow::preserves_labels),
            all_rows_preserve_target_context: self
                .rows
                .iter()
                .all(ManifestBuildConsumerRow::preserves_target_context),
            all_narrowed_rows_disclose: self
                .rows
                .iter()
                .all(ManifestBuildConsumerRow::discloses_narrowing),
            all_rows_confidence_consistent: self
                .rows
                .iter()
                .all(ManifestBuildConsumerRow::confidence_consistent),
            all_rows_have_copy_export: self.rows.iter().all(|r| r.copy_export.is_export_safe()),
            families_reused_across_groups: self.has_family_reused_across_groups(),
            container_devcontainer_consumer_present: has_group(
                ConsumerGroup::ContainerDevcontainer,
            ),
            request_live_resource_consumer_present: has_group(
                ConsumerGroup::RequestLiveResourceHandoff,
            ),
            incident_support_consumer_present: has_group(ConsumerGroup::IncidentSupport),
            ai_explanation_consumer_present: has_group(ConsumerGroup::AiExplanation),
            help_support_release_reference_present: self.rows.iter().any(|r| {
                r.is_help_support_release_surface() && r.references_canonical_not_local_prose
            }),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ManifestBuildConsumerViolation> {
        let mut violations = Vec::new();

        if self.schema_version != MANIFEST_BUILD_CONSUMER_SCHEMA_VERSION {
            violations.push(ManifestBuildConsumerViolation::SchemaVersion {
                expected: MANIFEST_BUILD_CONSUMER_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != MANIFEST_BUILD_CONSUMER_RECORD_KIND {
            violations.push(ManifestBuildConsumerViolation::RecordKind {
                expected: MANIFEST_BUILD_CONSUMER_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_groups = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ManifestBuildConsumerViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_groups.insert(row.consumer_group);
            seen_families.insert(row.component_family);

            if row.record_kind != MANIFEST_BUILD_CONSUMER_ROW_RECORD_KIND
                || row.schema_version != MANIFEST_BUILD_CONSUMER_SCHEMA_VERSION
                || row.canonical_family_schema_ref.trim().is_empty()
                || row.canonical_packet_refs.is_empty()
            {
                violations.push(ManifestBuildConsumerViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // The surface must belong to the declared consumer group.
            if row.consumer_surface.consumer_group() != row.consumer_group {
                violations.push(ManifestBuildConsumerViolation::SurfaceGroupMismatch {
                    id: row.row_id.clone(),
                });
            }

            // AC1: exactly one canonical family, no cloned surface-local prose.
            if !row.points_to_canonical_family() {
                violations.push(ManifestBuildConsumerViolation::NotCanonicalFamily {
                    id: row.row_id.clone(),
                });
            }

            // AC2: controlled badges / freshness / adapter / degraded labels kept.
            if !row.preserves_labels() {
                violations.push(ManifestBuildConsumerViolation::LabelParityBroken {
                    id: row.row_id.clone(),
                });
            }

            // AC2: target-context identity stays visible.
            if !row.preserves_target_context() {
                violations.push(ManifestBuildConsumerViolation::TargetContextDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrower consumers disclose reduction with banner + handoff note.
            if !row.discloses_narrowing() {
                violations.push(ManifestBuildConsumerViolation::NarrowedWithoutDisclosure {
                    id: row.row_id.clone(),
                });
            }

            // Adapter source and confidence must not contradict each other.
            if !row.confidence_consistent() {
                violations.push(ManifestBuildConsumerViolation::ConfidenceInconsistent {
                    id: row.row_id.clone(),
                });
            }

            // Copy / export parity: text/JSON/Markdown, screenshot-only prohibited.
            if !row.copy_export.is_export_safe() {
                violations.push(ManifestBuildConsumerViolation::MissingCopyExportParity {
                    id: row.row_id.clone(),
                });
            }
        }

        // AC1: adoption spans all four claimed consumer classes.
        for group in ConsumerGroup::ALL {
            if !seen_groups.contains(&group) {
                violations.push(ManifestBuildConsumerViolation::MissingConsumerGroup { group });
            }
        }

        // AC1: every frozen component family is adopted by at least one consumer.
        for family in M5ManifestBuildComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(ManifestBuildConsumerViolation::MissingComponentFamily { family });
            }
        }

        // AC1: at least one family is reused across two distinct groups.
        if !self.has_family_reused_across_groups() {
            violations.push(ManifestBuildConsumerViolation::NoFamilyReusedAcrossGroups);
        }

        // AC3: docs/help, support-export, and release-proof reference canonical.
        if !self
            .rows
            .iter()
            .any(|r| r.is_help_support_release_surface() && r.references_canonical_not_local_prose)
        {
            violations.push(ManifestBuildConsumerViolation::MissingHelpSupportReleaseReference);
        }

        if self.summary != self.computed_summary() {
            violations.push(ManifestBuildConsumerViolation::SummaryMismatch);
        }

        violations
    }

    /// Deterministic export-safe JSON (the `include_str!` canonical form).
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 manifest/build consumer packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "row_id,consumer_group,consumer_surface,component_family,authority,freshness,adapter_source,confidence,label_parity\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.row_id,
                row.consumer_group.as_str(),
                row.consumer_surface.as_str(),
                row.component_family.as_str(),
                row.authority_mode.capability_state(),
                row.freshness.as_str(),
                row.adapter_source.as_str(),
                row.discovery_confidence.as_str(),
                label_parity_token(row.label_parity),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let s = &self.summary;
        let mut out = String::new();
        out.push_str("# M5 manifest / build component consumers\n\n");
        out.push_str(&format!("- packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- matrix: `{}`\n", self.matrix_ref));
        out.push_str(&format!("- consumer rows: {}\n", s.row_count));
        out.push_str(&format!(
            "- consumer groups: {} / {}\n",
            s.consumer_group_count,
            ConsumerGroup::ALL.len()
        ));
        out.push_str(&format!(
            "- component families adopted: {} / {}\n",
            s.component_family_count,
            M5ManifestBuildComponentFamily::ALL.len()
        ));
        out.push_str(&format!(
            "- families reused across groups: {}\n",
            s.families_reused_across_groups
        ));
        out.push_str(&format!(
            "- all rows point to a canonical family: {}\n",
            s.all_rows_point_to_canonical_family
        ));
        out.push_str(&format!(
            "- all rows preserve target context: {}\n",
            s.all_rows_preserve_target_context
        ));
        out.push_str(&format!(
            "- help/support/release reference canonical: {}\n",
            s.help_support_release_reference_present
        ));
        out
    }
}

/// Reads and validates the checked-in stable M05-817 support export.
pub fn current_stable_m5_manifest_build_consumer_export(
) -> Result<ManifestBuildConsumerPacket, ManifestBuildConsumerArtifactError> {
    let packet: ManifestBuildConsumerPacket =
        serde_json::from_str(MANIFEST_BUILD_CONSUMER_ARTIFACT_JSON)
            .map_err(ManifestBuildConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ManifestBuildConsumerArtifactError::Validation(violations))
    }
}

/// Stable token for a [`LabelParityState`] (independent of serde formatting).
const fn label_parity_token(state: LabelParityState) -> &'static str {
    match state {
        LabelParityState::Preserved => "preserved",
        LabelParityState::DisclosedNarrowed => "disclosed_narrowed",
        LabelParityState::RenamedOrDropped => "renamed_or_dropped",
    }
}

/// Failure reading or validating the checked-in support export.
#[derive(Debug)]
pub enum ManifestBuildConsumerArtifactError {
    SupportExport(serde_json::Error),
    Validation(Vec<ManifestBuildConsumerViolation>),
}

impl fmt::Display for ManifestBuildConsumerArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(e) => write!(f, "support export does not deserialize: {e}"),
            Self::Validation(v) => write!(f, "support export failed validation: {v:?}"),
        }
    }
}

impl Error for ManifestBuildConsumerArtifactError {}

/// Validation failure for M05-817 manifest / build consumer packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestBuildConsumerViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
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
    TargetContextDropped {
        id: String,
    },
    NarrowedWithoutDisclosure {
        id: String,
    },
    ConfidenceInconsistent {
        id: String,
    },
    MissingCopyExportParity {
        id: String,
    },
    MissingConsumerGroup {
        group: ConsumerGroup,
    },
    MissingComponentFamily {
        family: M5ManifestBuildComponentFamily,
    },
    NoFamilyReusedAcrossGroups,
    MissingHelpSupportReleaseReference,
    SummaryMismatch,
}

impl fmt::Display for ManifestBuildConsumerViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema version mismatch: expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete consumer row: {id}"),
            Self::SurfaceGroupMismatch { id } => {
                write!(f, "row {id} surface does not belong to its declared consumer group")
            }
            Self::NotCanonicalFamily { id } => {
                write!(f, "row {id} does not point back to exactly one canonical component family")
            }
            Self::LabelParityBroken { id } => {
                write!(f, "row {id} renames or drops a canonical badge / freshness / degraded label")
            }
            Self::TargetContextDropped { id } => {
                write!(f, "row {id} drops the target-context identity")
            }
            Self::NarrowedWithoutDisclosure { id } => {
                write!(f, "row {id} narrows authority without a reduced-capability banner or handoff note")
            }
            Self::ConfidenceInconsistent { id } => {
                write!(f, "row {id} claims a confidence its adapter source cannot support")
            }
            Self::MissingCopyExportParity { id } => {
                write!(f, "row {id} is missing text/JSON/Markdown copy-export parity")
            }
            Self::MissingConsumerGroup { group } => {
                write!(f, "consumer group {} is not adopted in the packet", group.as_str())
            }
            Self::MissingComponentFamily { family } => {
                write!(f, "component family {} is not adopted by any consumer", family.as_str())
            }
            Self::NoFamilyReusedAcrossGroups => {
                write!(f, "no component family is reused across two distinct consumer groups")
            }
            Self::MissingHelpSupportReleaseReference => write!(
                f,
                "no docs/help, support-export, or release-proof consumer references the canonical families"
            ),
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
        }
    }
}

impl Error for ManifestBuildConsumerViolation {}

// --- seed -------------------------------------------------------------------

/// Builds one consumer row, deriving `label_parity` and the reduced-capability
/// banner from the authority mode: a narrowed consumer is always
/// `DisclosedNarrowed` with a banner; a full-interactive consumer is
/// `Preserved` with no banner.
#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    surface: M5ManifestBuildConsumerSurface,
    family: M5ManifestBuildComponentFamily,
    authority: AuthorityMode,
    target_context_ref: &str,
    freshness: M5ResourceFreshness,
    schema_source: M5SchemaFreshness,
    adapter_source: M5AdapterSourceKind,
    confidence: M5DiscoveryConfidence,
    preserved_badge_labels: &[&str],
    degraded_state_vocab: &[&str],
    handoff_target: HandoffTarget,
    handoff_note_ref: &str,
    missing_capabilities: &[&str],
) -> ManifestBuildConsumerRow {
    let narrowed = authority.is_narrowed();
    let banner = if narrowed {
        Some(ReducedCapabilityBanner {
            banner_id: format!("banner.{row_id}"),
            visible_label: format!(
                "{} — {} on this surface",
                family.as_str(),
                authority.capability_state()
            ),
            capability_state: authority.capability_state().to_owned(),
            missing_capabilities: missing_capabilities
                .iter()
                .map(|s| (*s).to_owned())
                .collect(),
        })
    } else {
        None
    };
    let label_parity = if narrowed {
        LabelParityState::DisclosedNarrowed
    } else {
        LabelParityState::Preserved
    };

    ManifestBuildConsumerRow {
        record_kind: MANIFEST_BUILD_CONSUMER_ROW_RECORD_KIND.to_owned(),
        schema_version: MANIFEST_BUILD_CONSUMER_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        consumer_group: surface.consumer_group(),
        consumer_surface: surface,
        component_family: family,
        canonical_family_schema_ref: canonical_schema_ref_for(family).to_owned(),
        canonical_packet_refs: vec![canonical_packet_ref_for(family).to_owned()],
        references_canonical_not_local_prose: true,
        authority_mode: authority,
        target_context_ref: target_context_ref.to_owned(),
        freshness,
        schema_source,
        adapter_source,
        discovery_confidence: confidence,
        preserved_badge_labels: preserved_badge_labels
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        degraded_state_vocab: degraded_state_vocab
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        required_labels: M5ManifestBuildRequiredLabel::ALL.to_vec(),
        label_parity,
        handoff_target,
        handoff_note_ref: handoff_note_ref.to_owned(),
        source_refs: vec![
            MANIFEST_BUILD_CONSUMER_MATRIX_REF.to_owned(),
            canonical_schema_ref_for(family).to_owned(),
        ],
        copy_export: CopyExportParity::full(),
        reduced_capability_banner: banner,
    }
}

/// Deterministic seeded M05-817 consumer packet, shared by tests, the emit bin,
/// and the checked-in artifact.
pub fn seeded_m5_manifest_build_consumer_packet() -> ManifestBuildConsumerPacket {
    use AuthorityMode as A;
    use HandoffTarget as H;
    use M5AdapterSourceKind as Adapter;
    use M5DiscoveryConfidence as Conf;
    use M5ManifestBuildComponentFamily as Fam;
    use M5ManifestBuildConsumerSurface as Surf;
    use M5ResourceFreshness as Fresh;
    use M5SchemaFreshness as Schema;

    let rows = vec![
        // --- container / devcontainer -----------------------------------
        row(
            "mbc.container.manifest_header",
            Surf::DevcontainerManifestPanel,
            Fam::ManifestEditorHeader,
            A::FullInteractive,
            "target:devcontainer/local-workspace",
            Fresh::LiveFresh,
            Schema::Fresh,
            Adapter::NativeBuildServer,
            Conf::High,
            &[
                "identity:devcontainer.json",
                "target_context:local-workspace",
                "truth_class:authored_desired",
            ],
            &["schema_stale", "target_context_unresolved"],
            H::None,
            "",
            &[],
        ),
        row(
            "mbc.container.schema_validator",
            Surf::DevcontainerManifestPanel,
            Fam::SchemaValidatorRow,
            A::ReadOnly,
            "target:devcontainer/local-workspace",
            Fresh::LiveFresh,
            Schema::Stale,
            Adapter::HeuristicParse,
            Conf::Medium,
            &[
                "identity:schema-validator",
                "target_context:local-workspace",
                "schema_source:stale",
            ],
            &["schema_stale"],
            H::None,
            "",
            &["apply_edit"],
        ),
        row(
            "mbc.container.target_graph",
            Surf::ContainerTargetGraphInspector,
            Fam::TargetGraphRow,
            A::FullInteractive,
            "target:devcontainer/build-graph",
            Fresh::LiveFresh,
            Schema::Fresh,
            Adapter::NativeBuildServer,
            Conf::High,
            &[
                "identity:target-graph-row",
                "target_context:build-graph",
                "adapter_source:native_build_server",
            ],
            &["adapter_unavailable", "drift_from_source"],
            H::None,
            "",
            &[],
        ),
        row(
            "mbc.container.target_context_chips",
            Surf::ContainerTargetGraphInspector,
            Fam::TargetContextChipGroup,
            A::ReadOnly,
            "target:devcontainer/build-graph",
            Fresh::CachedStale,
            Schema::Fresh,
            Adapter::ImportedSnapshot,
            Conf::Low,
            &[
                "identity:target-context-chips",
                "target_context:build-graph",
                "freshness:cached_stale",
            ],
            &["drift_from_source", "low_confidence_discovery"],
            H::None,
            "",
            &["edit_target"],
        ),
        // --- request / live-resource handoff ----------------------------
        row(
            "mbc.request.resource_link",
            Surf::RequestResourceLinkHandoff,
            Fam::ResourceLinkRow,
            A::CompareOnly,
            "target:cluster/ns-payments",
            Fresh::LiveFresh,
            Schema::Fresh,
            Adapter::NativeBuildEvent,
            Conf::High,
            &[
                "identity:resource-link-row",
                "target_context:ns-payments",
                "truth_class:rendered->live",
            ],
            &["drift_from_source", "connector_loss"],
            H::BrowserReadonly,
            "handoff.note.browser.resource-link",
            &["apply_mutation", "edit_link"],
        ),
        row(
            "mbc.request.resource_explorer",
            Surf::LiveResourceExplorerHandoff,
            Fam::ResourceExplorerRow,
            A::ReadOnly,
            "target:cluster/ns-payments",
            Fresh::CachedStale,
            Schema::Unversioned,
            Adapter::ImportedSnapshot,
            Conf::Low,
            &[
                "identity:resource-explorer-row",
                "target_context:ns-payments",
                "freshness:cached_stale",
            ],
            &["connector_loss", "low_confidence_discovery"],
            H::None,
            "",
            &["mutate_resource"],
        ),
        row(
            "mbc.request.adapter_badge",
            Surf::RequestResourceLinkHandoff,
            Fam::AdapterSourceBadge,
            A::InspectOnly,
            "target:cluster/ns-payments",
            Fresh::LiveFresh,
            Schema::Fresh,
            Adapter::HeuristicParse,
            Conf::Medium,
            &[
                "identity:adapter-source-badge",
                "target_context:ns-payments",
                "adapter_source:heuristic_parse",
            ],
            &["adapter_unavailable", "structured_channel_lost"],
            H::None,
            "",
            &["run_target"],
        ),
        // --- incident / support -----------------------------------------
        row(
            "mbc.support.fallback_confidence",
            Surf::IncidentSupportBundle,
            Fam::FallbackConfidenceDrawer,
            A::ExportOnly,
            "target:cluster/ns-payments",
            Fresh::ImportedSnapshot,
            Schema::Unavailable,
            Adapter::HeuristicParse,
            Conf::Low,
            &[
                "identity:fallback-confidence-drawer",
                "target_context:ns-payments",
                "confidence:low",
            ],
            &["structured_channel_lost", "adapter_unavailable"],
            H::HandoffPacket,
            "handoff.note.packet.support-bundle",
            &["run_target", "edit_target"],
        ),
        row(
            "mbc.support.raw_event",
            Surf::IncidentSupportBundle,
            Fam::RawEventDrawer,
            A::ExportOnly,
            "target:cluster/ns-payments",
            Fresh::ImportedSnapshot,
            Schema::Fresh,
            Adapter::NativeBuildEvent,
            Conf::High,
            &[
                "identity:raw-event-drawer",
                "target_context:ns-payments",
                "adapter_source:native_build_event",
            ],
            &["structured_channel_lost"],
            H::None,
            "",
            &["run_target"],
        ),
        row(
            "mbc.support.capability_matrix",
            Surf::SupportExport,
            Fam::CapabilityMatrix,
            A::ExportOnly,
            "target:cluster/ns-payments",
            Fresh::CachedStale,
            Schema::Fresh,
            Adapter::ImportedSnapshot,
            Conf::Medium,
            &[
                "identity:capability-matrix",
                "target_context:ns-payments",
                "truth_class:live",
            ],
            &["policy_block", "adapter_unavailable"],
            H::None,
            "",
            &["run_target", "edit_target"],
        ),
        row(
            "mbc.support.release_target_context",
            Surf::ReleaseProof,
            Fam::TargetContextChipGroup,
            A::ExportOnly,
            "target:devcontainer/build-graph",
            Fresh::PlanOnly,
            Schema::Fresh,
            Adapter::ProviderOverlay,
            Conf::Medium,
            &[
                "identity:target-context-chips",
                "target_context:build-graph",
                "truth_class:planned",
            ],
            &["target_context_unresolved", "drift_from_source"],
            H::None,
            "",
            &["edit_target"],
        ),
        // --- AI / explanation -------------------------------------------
        row(
            "mbc.ai.adapter_badge",
            Surf::AiExecutionExplainer,
            Fam::AdapterSourceBadge,
            A::InspectOnly,
            "target:cluster/ns-payments",
            Fresh::LiveFresh,
            Schema::Fresh,
            Adapter::NativeBuildServer,
            Conf::High,
            &[
                "identity:adapter-source-badge",
                "target_context:ns-payments",
                "adapter_source:native_build_server",
            ],
            &["adapter_unavailable", "structured_channel_lost"],
            H::None,
            "",
            &["run_target"],
        ),
        row(
            "mbc.ai.capability_matrix",
            Surf::AiConfidenceNarrative,
            Fam::CapabilityMatrix,
            A::InspectOnly,
            "target:cluster/ns-payments",
            Fresh::CachedStale,
            Schema::Stale,
            Adapter::HeuristicParse,
            Conf::Low,
            &[
                "identity:capability-matrix",
                "target_context:ns-payments",
                "confidence:low",
            ],
            &["low_confidence_discovery", "policy_block"],
            H::None,
            "",
            &["run_target", "edit_target"],
        ),
        row(
            "mbc.ai.docs_manifest_header",
            Surf::DocsHelp,
            Fam::ManifestEditorHeader,
            A::ReadOnly,
            "target:devcontainer/local-workspace",
            Fresh::PlanOnly,
            Schema::Fresh,
            Adapter::NativeBuildServer,
            Conf::High,
            &[
                "identity:manifest-editor-header",
                "target_context:local-workspace",
                "truth_class:authored_desired",
            ],
            &["schema_stale", "target_context_unresolved"],
            H::None,
            "",
            &["apply_edit"],
        ),
    ];

    let mut packet = ManifestBuildConsumerPacket {
        schema_version: MANIFEST_BUILD_CONSUMER_SCHEMA_VERSION,
        record_kind: MANIFEST_BUILD_CONSUMER_RECORD_KIND.to_owned(),
        packet_id: "m5-manifest-build-component-consumers-001".to_owned(),
        as_of: "2026-07-04".to_owned(),
        matrix_ref: MANIFEST_BUILD_CONSUMER_MATRIX_REF.to_owned(),
        source_contract_refs: vec![
            M5_MANIFEST_AUTHORING_SCHEMA_REF.to_owned(),
            M5_LIVE_RESOURCE_SCHEMA_REF.to_owned(),
            M5_BUILD_CONFIDENCE_SCHEMA_REF.to_owned(),
        ],
        rows,
        summary: ManifestBuildConsumerSummary {
            row_count: 0,
            consumer_group_count: 0,
            consumer_surface_count: 0,
            component_family_count: 0,
            all_rows_point_to_canonical_family: false,
            all_rows_preserve_labels: false,
            all_rows_preserve_target_context: false,
            all_narrowed_rows_disclose: false,
            all_rows_confidence_consistent: false,
            all_rows_have_copy_export: false,
            families_reused_across_groups: false,
            container_devcontainer_consumer_present: false,
            request_live_resource_consumer_present: false,
            incident_support_consumer_present: false,
            ai_explanation_consumer_present: false,
            help_support_release_reference_present: false,
        },
    };
    packet.summary = packet.computed_summary();
    packet
}
