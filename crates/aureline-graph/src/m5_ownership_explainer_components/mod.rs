//! Reusable ownership/contract cards and explainer-section cards.
//!
//! This module is the M05-800 component contract that layers the frozen
//! `ownership_card` and `explainer_section_card` families of the M5
//! profiler/topology matrix onto checked-in graph/ownership/explainer fixtures.
//! It is the ownership/explanation analog of
//! [`crate::m5_workset_topology_components`] (M05-799) and reuses that module's
//! shared disclosure vocabulary ([`ComponentConsumerSurface`], [`FreshnessState`],
//! [`Confidence`], [`ProvenanceClass`], and the copy-export / reduced-capability /
//! support-export / auto-narrowing structs) so labels stay identical across every
//! claimed M5 consumer.
//!
//! Ownership cards keep owner, reviewer, maintainer, service-owner, on-call, and
//! approver roles distinct instead of collapsing them into one ambiguous "owner"
//! label, and they carry protected-path / change-control links via
//! `escalation_refs` plus a policy-source `authority_boundary_ref`. Explainer
//! cards cite concrete files/symbols/docs, distinguish generated from curated
//! summaries, and preserve open-detail / ask-follow-up affordances without letting
//! a generated explanation masquerade as uncited primary truth: a generated
//! summary automatically narrows when its citations, freshness, or workset scope
//! truth is incomplete.
//!
//! The topology, onboarding, AI, review, and support consumers reference this
//! packet instead of re-deriving ownership or re-summarizing code inline, so role
//! separation, citations, and provenance survive every copy/export projection.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reuse the shared disclosure vocabulary from the M05-799 sibling module so the
// controlled labels and export/narrowing structs stay byte-identical across the
// ownership, explainer, workset, and topology component lanes.
pub use crate::m5_workset_topology_components::{
    AutoNarrowingContract, ComponentConsumerProjection, ComponentConsumerSurface, Confidence,
    CopyExportProjection, FreshnessState, ProvenanceClass, ReducedCapabilityBanner,
    SupportExportJoin,
};

/// Schema version stamped on the M05-800 component packet.
pub const OWNERSHIP_EXPLAINER_COMPONENT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`OwnershipExplainerComponentPacket`].
pub const OWNERSHIP_EXPLAINER_COMPONENT_RECORD_KIND: &str =
    "m5_ownership_explainer_component_packet";

/// Repo-relative path to the checked-in M05-800 packet.
pub const OWNERSHIP_EXPLAINER_COMPONENT_PACKET_PATH: &str =
    "artifacts/graph/m5/m5-ownership-explainer-components.json";

/// Frozen component matrix this packet consumes by reference.
pub const OWNERSHIP_EXPLAINER_COMPONENT_MATRIX_REF: &str =
    "artifacts/design/m5-profiler-topology-component-matrix.md";

/// Schema for the ownership/contract card family.
pub const OWNERSHIP_CARD_SCHEMA_REF: &str = "schemas/ui/m5-ownership-card.schema.json";

/// Schema for the explainer-section card family.
pub const EXPLAINER_SECTION_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-explainer-section-card.schema.json";

/// Embedded checked-in M05-800 packet JSON.
pub const OWNERSHIP_EXPLAINER_COMPONENT_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/graph/m5/m5-ownership-explainer-components.json"
));

/// Role a principal plays for an owned object, kept distinct so cards never
/// collapse separate responsibilities into one ambiguous owner label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoleType {
    Owner,
    Reviewer,
    Maintainer,
    SubjectMatterExpert,
    ServiceOwner,
    Oncall,
    Approver,
    Observer,
    Unknown,
}

impl RoleType {
    /// Returns true when the role is concrete (not an unresolved placeholder).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::Unknown)
    }
}

/// How an explainer or ownership summary was produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SummaryGenerationMode {
    Curated,
    Generated,
    GeneratedReviewed,
    Imported,
    Unknown,
}

impl SummaryGenerationMode {
    /// Returns true when the summary originates from generation (reviewed or not)
    /// and must therefore narrow when its supporting truth is incomplete.
    pub const fn is_generated_origin(self) -> bool {
        matches!(self, Self::Generated | Self::GeneratedReviewed)
    }
}

/// Kind of artifact a citation points at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationKind {
    File,
    Symbol,
    Doc,
    TopologyNode,
    Ownership,
    Trace,
    Profile,
}

/// Completeness of an explainer card's citation set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationState {
    Complete,
    Partial,
    Missing,
    PolicyLimited,
    Stale,
}

impl CitationState {
    /// Returns true only when every referenced claim is cited.
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// One typed role assignment on an ownership card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleAssignment {
    pub principal_ref: String,
    pub role_type: RoleType,
    pub assignment_source_ref: String,
}

/// One concrete citation attached to an explainer-section card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationRef {
    pub citation_ref: String,
    pub citation_kind: CitationKind,
    pub provenance_class: ProvenanceClass,
}

impl CitationRef {
    /// Returns true when the citation points at a concrete artifact with a
    /// declared provenance (never a blank, uncited reference).
    pub fn is_concrete(&self) -> bool {
        !self.citation_ref.trim().is_empty()
    }
}

/// Reusable ownership/contract card separating distinct roles and preserving
/// protected-path / change-control links.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipCard {
    pub record_kind: String,
    pub schema_version: u32,
    pub card_id: String,
    pub ownership_ref: String,
    #[serde(default)]
    pub owned_object_refs: Vec<String>,
    #[serde(default)]
    pub role_assignments: Vec<RoleAssignment>,
    pub authority_boundary_ref: String,
    pub freshness_state: FreshnessState,
    pub confidence: Confidence,
    pub provenance_class: ProvenanceClass,
    #[serde(default)]
    pub escalation_refs: Vec<String>,
    pub service_oncall_separation: bool,
    pub summary_generation_mode: SummaryGenerationMode,
    #[serde(default)]
    pub source_refs: Vec<String>,
    #[serde(default)]
    pub consumer_surfaces: Vec<ComponentConsumerSurface>,
    pub copy_export: CopyExportProjection,
    pub reduced_capability_banner: ReducedCapabilityBanner,
    pub support_export_join: SupportExportJoin,
    pub auto_narrowing_contract: AutoNarrowingContract,
}

impl OwnershipCard {
    /// Number of distinct role types represented on the card.
    pub fn distinct_role_count(&self) -> usize {
        self.role_assignments
            .iter()
            .map(|r| r.role_type)
            .collect::<BTreeSet<_>>()
            .len()
    }

    /// Returns true when the card keeps distinct responsibilities separate: with
    /// more than one principal it must expose more than one role type instead of
    /// collapsing everyone into one ambiguous owner label (AC1). A single
    /// principal is trivially distinguished.
    pub fn distinguishes_roles(&self) -> bool {
        self.role_assignments.len() <= 1 || self.distinct_role_count() >= 2
    }

    /// Returns true when freshness, confidence, provenance, role assignments, and
    /// protected-path / change-control links survive the export projection.
    pub fn preserves_truth_in_export(&self) -> bool {
        self.copy_export.exports_all(&[
            "role_assignments",
            "escalation_refs",
            "freshness_state",
            "confidence",
            "provenance_class",
            "service_oncall_separation",
        ])
    }

    /// Returns true when the card's reduced-capability banner is narrowed (not the
    /// full-capability state).
    pub fn is_narrowed(&self) -> bool {
        self.reduced_capability_banner.capability_state != "full"
    }

    /// Returns true when a generated ownership summary sits on stale/degraded
    /// freshness yet is still presented at full capability (AC3 violation).
    pub fn generated_but_not_narrowed(&self) -> bool {
        self.summary_generation_mode.is_generated_origin()
            && self.freshness_state.is_degraded()
            && !self.is_narrowed()
    }
}

/// Reusable explainer-section card with cited symbols/files/docs and
/// generated-vs-curated provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplainerSectionCard {
    pub record_kind: String,
    pub schema_version: u32,
    pub card_id: String,
    pub section_ref: String,
    pub topic_ref: String,
    #[serde(default)]
    pub object_refs: Vec<String>,
    pub workset_snapshot_ref: String,
    pub summary_generation_mode: SummaryGenerationMode,
    #[serde(default)]
    pub citation_refs: Vec<CitationRef>,
    pub citation_state: CitationState,
    pub freshness_state: FreshnessState,
    pub confidence: Confidence,
    pub provenance_class: ProvenanceClass,
    #[serde(default)]
    pub topology_refs: Vec<String>,
    #[serde(default)]
    pub ownership_refs: Vec<String>,
    #[serde(default)]
    pub source_refs: Vec<String>,
    #[serde(default)]
    pub consumer_surfaces: Vec<ComponentConsumerSurface>,
    pub copy_export: CopyExportProjection,
    pub reduced_capability_banner: ReducedCapabilityBanner,
    pub support_export_join: SupportExportJoin,
    pub auto_narrowing_contract: AutoNarrowingContract,
}

impl ExplainerSectionCard {
    /// Returns true when the card carries at least one concrete citation (AC2): a
    /// generated summary must never masquerade as uncited primary truth.
    pub fn is_cited(&self) -> bool {
        !self.citation_refs.is_empty() && self.citation_refs.iter().all(CitationRef::is_concrete)
    }

    /// Returns true when the summary is generated in origin.
    pub fn is_generated(&self) -> bool {
        self.summary_generation_mode.is_generated_origin()
    }

    /// Returns true when the card's reduced-capability banner is narrowed.
    pub fn is_narrowed(&self) -> bool {
        self.reduced_capability_banner.capability_state != "full"
    }

    /// Returns true when the card's supporting truth is incomplete: citations are
    /// not complete, or freshness is degraded.
    pub fn truth_incomplete(&self) -> bool {
        !self.citation_state.is_complete() || self.freshness_state.is_degraded()
    }

    /// Returns true when a generated summary with incomplete supporting truth is
    /// still presented at full capability instead of narrowing (AC3 violation).
    pub fn generated_but_not_narrowed(&self) -> bool {
        self.is_generated() && self.truth_incomplete() && !self.is_narrowed()
    }

    /// Returns true when citations, provenance, freshness, and generation mode
    /// survive the export projection.
    pub fn preserves_truth_in_export(&self) -> bool {
        self.copy_export.exports_all(&[
            "summary_generation_mode",
            "citation_refs",
            "citation_state",
            "freshness_state",
            "provenance_class",
        ])
    }
}

/// Rolled-up summary of an M05-800 component packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipExplainerComponentSummary {
    pub ownership_card_count: usize,
    pub explainer_section_card_count: usize,
    pub consumer_projection_count: usize,
    pub ownership_consumer_present: bool,
    pub explainer_consumer_present: bool,
    pub all_ownership_cards_distinguish_roles: bool,
    pub all_explainer_cards_cite_sources: bool,
    pub generated_summaries_narrow_when_incomplete: bool,
    pub all_components_have_copy_export: bool,
}

/// Checked-in M05-800 component packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipExplainerComponentPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub ownership_cards: Vec<OwnershipCard>,
    #[serde(default)]
    pub explainer_section_cards: Vec<ExplainerSectionCard>,
    #[serde(default)]
    pub consumer_projection_rows: Vec<ComponentConsumerProjection>,
    pub summary: OwnershipExplainerComponentSummary,
}

impl OwnershipExplainerComponentPacket {
    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> OwnershipExplainerComponentSummary {
        let mut consumers = BTreeSet::new();
        for row in &self.consumer_projection_rows {
            consumers.insert(row.consumer_surface);
        }
        for card in &self.ownership_cards {
            consumers.extend(card.consumer_surfaces.iter().copied());
        }
        for card in &self.explainer_section_cards {
            consumers.extend(card.consumer_surfaces.iter().copied());
        }

        let all_ownership_cards_distinguish_roles = self
            .ownership_cards
            .iter()
            .all(OwnershipCard::distinguishes_roles);

        let all_explainer_cards_cite_sources = self
            .explainer_section_cards
            .iter()
            .all(ExplainerSectionCard::is_cited);

        let generated_summaries_narrow_when_incomplete = self
            .ownership_cards
            .iter()
            .all(|c| !c.generated_but_not_narrowed())
            && self
                .explainer_section_cards
                .iter()
                .all(|c| !c.generated_but_not_narrowed());

        let all_components_have_copy_export = self
            .ownership_cards
            .iter()
            .all(|c| c.copy_export.is_export_safe())
            && self
                .explainer_section_cards
                .iter()
                .all(|c| c.copy_export.is_export_safe());

        OwnershipExplainerComponentSummary {
            ownership_card_count: self.ownership_cards.len(),
            explainer_section_card_count: self.explainer_section_cards.len(),
            consumer_projection_count: self.consumer_projection_rows.len(),
            ownership_consumer_present: consumers
                .contains(&ComponentConsumerSurface::OwnershipBrowser),
            explainer_consumer_present: consumers
                .contains(&ComponentConsumerSurface::ArchitectureExplainer),
            all_ownership_cards_distinguish_roles,
            all_explainer_cards_cite_sources,
            generated_summaries_narrow_when_incomplete,
            all_components_have_copy_export,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<OwnershipExplainerComponentViolation> {
        let mut violations = Vec::new();

        if self.schema_version != OWNERSHIP_EXPLAINER_COMPONENT_SCHEMA_VERSION {
            violations.push(OwnershipExplainerComponentViolation::SchemaVersion {
                expected: OWNERSHIP_EXPLAINER_COMPONENT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != OWNERSHIP_EXPLAINER_COMPONENT_RECORD_KIND {
            violations.push(OwnershipExplainerComponentViolation::RecordKind {
                expected: OWNERSHIP_EXPLAINER_COMPONENT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let mut ownership_ids = BTreeSet::new();
        for card in &self.ownership_cards {
            if !ownership_ids.insert(card.card_id.clone()) {
                violations.push(OwnershipExplainerComponentViolation::DuplicateId {
                    kind: "ownership_card",
                    id: card.card_id.clone(),
                });
            }
            if card.record_kind != "m5_ownership_card"
                || card.schema_version != 1
                || card.ownership_ref.trim().is_empty()
                || card.owned_object_refs.is_empty()
                || card.role_assignments.is_empty()
                || card.authority_boundary_ref.trim().is_empty()
                || !card.service_oncall_separation
            {
                violations.push(OwnershipExplainerComponentViolation::IncompleteOwnershipCard {
                    id: card.card_id.clone(),
                });
            }
            // AC1: distinct roles are never collapsed into one ambiguous owner.
            if !card.distinguishes_roles() {
                violations.push(OwnershipExplainerComponentViolation::CollapsedRoles {
                    id: card.card_id.clone(),
                });
            }
            // Protected-path / change-control links must be attached.
            if card.escalation_refs.is_empty() {
                violations.push(
                    OwnershipExplainerComponentViolation::MissingChangeControlLinks {
                        id: card.card_id.clone(),
                    },
                );
            }
            // Role separation and provenance must survive the export.
            if !card.preserves_truth_in_export() {
                violations.push(OwnershipExplainerComponentViolation::OwnershipTruthNotExported {
                    id: card.card_id.clone(),
                });
            }
            // AC3: generated ownership summaries narrow on stale/degraded truth.
            if card.generated_but_not_narrowed() {
                violations.push(OwnershipExplainerComponentViolation::GeneratedNotNarrowed {
                    kind: "ownership_card",
                    id: card.card_id.clone(),
                });
            }
            if !card.copy_export.is_export_safe() {
                violations.push(OwnershipExplainerComponentViolation::MissingCopyExport {
                    kind: "ownership_card",
                    id: card.card_id.clone(),
                });
            }
            if card.consumer_surfaces.len() < 2
                || !card
                    .consumer_surfaces
                    .contains(&ComponentConsumerSurface::OwnershipBrowser)
            {
                violations.push(OwnershipExplainerComponentViolation::MissingConsumerParity {
                    kind: "ownership_card",
                    id: card.card_id.clone(),
                });
            }
        }

        let mut explainer_ids = BTreeSet::new();
        for card in &self.explainer_section_cards {
            if !explainer_ids.insert(card.card_id.clone()) {
                violations.push(OwnershipExplainerComponentViolation::DuplicateId {
                    kind: "explainer_section_card",
                    id: card.card_id.clone(),
                });
            }
            if card.record_kind != "m5_explainer_section_card"
                || card.schema_version != 1
                || card.section_ref.trim().is_empty()
                || card.topic_ref.trim().is_empty()
                || card.object_refs.is_empty()
                || card.workset_snapshot_ref.trim().is_empty()
            {
                violations.push(
                    OwnershipExplainerComponentViolation::IncompleteExplainerCard {
                        id: card.card_id.clone(),
                    },
                );
            }
            // AC2: explainer cards carry concrete citations; a generated summary
            // may never masquerade as uncited primary truth.
            if !card.is_cited() {
                violations.push(OwnershipExplainerComponentViolation::UncitedExplainer {
                    id: card.card_id.clone(),
                });
            }
            // A generated summary must be labeled as generated provenance, never
            // presented as indexed/primary truth.
            if card.is_generated() && card.provenance_class != ProvenanceClass::Generated {
                violations.push(
                    OwnershipExplainerComponentViolation::GeneratedMasqueradesAsPrimary {
                        id: card.card_id.clone(),
                    },
                );
            }
            // Citations and provenance must survive the export.
            if !card.preserves_truth_in_export() {
                violations.push(OwnershipExplainerComponentViolation::ExplainerTruthNotExported {
                    id: card.card_id.clone(),
                });
            }
            // AC3: generated summaries narrow when citations/freshness/scope are
            // incomplete.
            if card.generated_but_not_narrowed() {
                violations.push(OwnershipExplainerComponentViolation::GeneratedNotNarrowed {
                    kind: "explainer_section_card",
                    id: card.card_id.clone(),
                });
            }
            if !card.copy_export.is_export_safe() {
                violations.push(OwnershipExplainerComponentViolation::MissingCopyExport {
                    kind: "explainer_section_card",
                    id: card.card_id.clone(),
                });
            }
            // Explainer cards must span the architecture explainer plus at least
            // one onboarding/AI/review consumer (citations survive across them).
            let spans_secondary = card.consumer_surfaces.iter().any(|s| {
                matches!(
                    s,
                    ComponentConsumerSurface::OnboardingTour
                        | ComponentConsumerSurface::AiContextPanel
                        | ComponentConsumerSurface::ReviewWorkspace
                )
            });
            if !card
                .consumer_surfaces
                .contains(&ComponentConsumerSurface::ArchitectureExplainer)
                || !spans_secondary
            {
                violations.push(OwnershipExplainerComponentViolation::MissingConsumerParity {
                    kind: "explainer_section_card",
                    id: card.card_id.clone(),
                });
            }
        }

        // Explainer cards that cite an in-packet ownership card must resolve to a
        // present ownership card (citations are inspectable, not dead links).
        let ownership_refs: BTreeSet<&str> = self
            .ownership_cards
            .iter()
            .map(|c| c.ownership_ref.as_str())
            .collect();
        for card in &self.explainer_section_cards {
            for cite in &card.citation_refs {
                if cite.citation_kind == CitationKind::Ownership
                    && cite.citation_ref.starts_with("ownership:")
                    && !ownership_refs.contains(cite.citation_ref.as_str())
                {
                    violations.push(
                        OwnershipExplainerComponentViolation::DanglingOwnershipCitation {
                            id: card.card_id.clone(),
                            ownership_ref: cite.citation_ref.clone(),
                        },
                    );
                }
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(OwnershipExplainerComponentViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in M05-800 packet.
pub fn current_m5_ownership_explainer_component_packet(
) -> Result<OwnershipExplainerComponentPacket, serde_json::Error> {
    serde_json::from_str(OWNERSHIP_EXPLAINER_COMPONENT_PACKET_JSON)
}

/// Validation failure for M05-800 component packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OwnershipExplainerComponentViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    DuplicateId { kind: &'static str, id: String },
    IncompleteOwnershipCard { id: String },
    CollapsedRoles { id: String },
    MissingChangeControlLinks { id: String },
    OwnershipTruthNotExported { id: String },
    IncompleteExplainerCard { id: String },
    UncitedExplainer { id: String },
    GeneratedMasqueradesAsPrimary { id: String },
    ExplainerTruthNotExported { id: String },
    GeneratedNotNarrowed { kind: &'static str, id: String },
    DanglingOwnershipCitation { id: String, ownership_ref: String },
    MissingCopyExport { kind: &'static str, id: String },
    MissingConsumerParity { kind: &'static str, id: String },
    SummaryMismatch,
}

impl fmt::Display for OwnershipExplainerComponentViolation {
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
            Self::IncompleteOwnershipCard { id } => write!(f, "incomplete ownership card: {id}"),
            Self::CollapsedRoles { id } => {
                write!(
                    f,
                    "ownership card {id} collapses distinct roles into one ambiguous owner label"
                )
            }
            Self::MissingChangeControlLinks { id } => {
                write!(
                    f,
                    "ownership card {id} is missing protected-path / change-control links"
                )
            }
            Self::OwnershipTruthNotExported { id } => {
                write!(
                    f,
                    "ownership card {id} drops role/freshness/confidence/provenance from export"
                )
            }
            Self::IncompleteExplainerCard { id } => {
                write!(f, "incomplete explainer section card: {id}")
            }
            Self::UncitedExplainer { id } => {
                write!(
                    f,
                    "explainer section card {id} carries no concrete citation (would masquerade as uncited primary truth)"
                )
            }
            Self::GeneratedMasqueradesAsPrimary { id } => {
                write!(
                    f,
                    "explainer section card {id} is generated but not labeled with generated provenance"
                )
            }
            Self::ExplainerTruthNotExported { id } => {
                write!(
                    f,
                    "explainer section card {id} drops citations/provenance/freshness from export"
                )
            }
            Self::GeneratedNotNarrowed { kind, id } => {
                write!(
                    f,
                    "{kind} {id} keeps a generated summary at full capability despite incomplete/stale truth"
                )
            }
            Self::DanglingOwnershipCitation { id, ownership_ref } => {
                write!(
                    f,
                    "explainer section card {id} cites ownership {ownership_ref} with no matching ownership card"
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

impl Error for OwnershipExplainerComponentViolation {}
