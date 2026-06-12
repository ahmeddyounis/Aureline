//! Canonical infrastructure surface qualification and evidence index.
//!
//! This module certifies the claimed infrastructure, DevOps/SRE, and
//! incident-adjacent surface families that Aureline currently evidences through
//! checked-in infrastructure packets. It does not mint a new truth model.
//! Instead, it binds the existing source-intelligence, target-context,
//! live-resource, plan-viewer, provider-overlay, and relation-graph packets
//! into one auto-narrowing qualification record and one canonical evidence
//! index that docs/help, support, and public-truth consumers can cite
//! directly.
//!
//! The packet keeps three promises aligned with the infrastructure architecture
//! contract:
//!
//! - claimed surfaces may remain `stable_qualified` only when their required
//!   proof classes are current and present;
//! - missing relationship, target-context, plan/live, or handoff-boundary proof
//!   narrows the displayed posture automatically rather than allowing adjacent
//!   evidence to imply maturity;
//! - docs/help, support playbooks, and public-truth consumers all bind to the
//!   same evidence index entry set instead of restating infrastructure posture
//!   in parallel text.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    cluster_context_and_live_resource::CLUSTER_LIVE_RESOURCE_SCHEMA_REF,
    plan_and_validation_viewers::PLAN_AND_VALIDATION_VIEWER_SCHEMA_REF,
    provider_overlay_and_vendor_console_handoff_continuity::PROVIDER_OVERLAY_HANDOFF_SCHEMA_REF,
    relation_graph_incident_support_parity::RELATION_GRAPH_PARITY_SCHEMA_REF,
    source_intelligence_and_resource_relationships::SOURCE_INTELLIGENCE_RELATIONSHIP_SCHEMA_REF,
    target_context_and_control_plane_boundary::{
        QualificationPosture, CONTROL_PLANE_BOUNDARY_SCHEMA_REF,
    },
};

/// Stable record kind carried by [`InfrastructureSurfaceQualificationPacket`].
pub const INFRASTRUCTURE_SURFACE_QUALIFICATION_RECORD_KIND: &str =
    "infrastructure_surface_qualification";

/// Schema version for infrastructure surface qualification packets.
pub const INFRASTRUCTURE_SURFACE_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const INFRASTRUCTURE_SURFACE_QUALIFICATION_SCHEMA_REF: &str =
    "schemas/infra/infrastructure-surface-qualification.schema.json";

/// Repo-relative path of the certification contract doc.
pub const INFRASTRUCTURE_SURFACE_QUALIFICATION_DOC_REF: &str =
    "docs/infra/infrastructure-surface-qualification.md";

/// Repo-relative path of the help-facing index summary.
pub const INFRASTRUCTURE_SURFACE_QUALIFICATION_HELP_REF: &str =
    "docs/help/infrastructure-surface-qualification.md";

/// Repo-relative path of the protected fixture directory.
pub const INFRASTRUCTURE_SURFACE_QUALIFICATION_FIXTURE_DIR: &str =
    "fixtures/infra/infrastructure-surface-qualification";

/// Repo-relative path of the checked support-export artifact.
pub const INFRASTRUCTURE_SURFACE_QUALIFICATION_ARTIFACT_REF: &str =
    "artifacts/infra/infrastructure-surface-qualification/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const INFRASTRUCTURE_SURFACE_QUALIFICATION_SUMMARY_REF: &str =
    "artifacts/infra/infrastructure-surface-qualification.md";

/// Claimed infrastructure or incident-adjacent surface family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureSurface {
    /// Source-intelligence matrix and object packet vocabulary.
    SourceIntelligence,
    /// Live-counterpart, ownership, and impact graph flows.
    LiveCounterpartGraph,
    /// Planned, dry-run, admission, and policy viewers.
    PlanAndValidation,
    /// Exact target-context and live-resource views.
    LiveResourceContext,
    /// Provider-overlay disclosure and vendor-console handoff continuity.
    ProviderOverlayHandoff,
    /// Incident workspace and support-export reopen parity.
    IncidentSupportParity,
    /// Shared docs/help/support/public-truth evidence index.
    PublicEvidenceIndex,
}

impl InfrastructureSurface {
    /// Every claimed surface family in declaration order.
    pub const ALL: [Self; 7] = [
        Self::SourceIntelligence,
        Self::LiveCounterpartGraph,
        Self::PlanAndValidation,
        Self::LiveResourceContext,
        Self::ProviderOverlayHandoff,
        Self::IncidentSupportParity,
        Self::PublicEvidenceIndex,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceIntelligence => "source_intelligence",
            Self::LiveCounterpartGraph => "live_counterpart_graph",
            Self::PlanAndValidation => "plan_and_validation",
            Self::LiveResourceContext => "live_resource_context",
            Self::ProviderOverlayHandoff => "provider_overlay_handoff",
            Self::IncidentSupportParity => "incident_support_parity",
            Self::PublicEvidenceIndex => "public_evidence_index",
        }
    }
}

/// Proof class a claimed surface requires before it can stay fully qualified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureProofClass {
    /// Shared relation-edge and object-model proof.
    RelationshipGraph,
    /// Exact target-context and authority-boundary proof.
    TargetContext,
    /// Live-counterpart or observed-object join proof.
    LiveCounterpart,
    /// Plan, diff, dry-run, admission, or policy-check proof.
    PlanValidation,
    /// Explicit provider-overlay or vendor-console boundary proof.
    HandoffBoundary,
    /// Wrong-target blocking or narrowing proof.
    WrongTargetDrill,
    /// Stale-live overlay narrowing proof.
    StaleLiveOverlayDrill,
    /// Support/export or reopen parity proof.
    ExportParity,
}

impl InfrastructureProofClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RelationshipGraph => "relationship_graph",
            Self::TargetContext => "target_context",
            Self::LiveCounterpart => "live_counterpart",
            Self::PlanValidation => "plan_validation",
            Self::HandoffBoundary => "handoff_boundary",
            Self::WrongTargetDrill => "wrong_target_drill",
            Self::StaleLiveOverlayDrill => "stale_live_overlay_drill",
            Self::ExportParity => "export_parity",
        }
    }
}

/// Currency of the checked proof packet for one claimed surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceCurrency {
    /// Proof is current for publication.
    Current,
    /// Proof is stale and must narrow.
    Stale,
}

impl EvidenceCurrency {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
        }
    }
}

/// Qualification verdict derived for one claimed surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureSurfaceVerdict {
    /// The surface remains fully qualified at the claimed posture.
    Certified,
    /// The surface is published only after narrowing automatically.
    Narrowed,
    /// The surface has insufficient evidence and may not claim maturity.
    Blocked,
}

impl InfrastructureSurfaceVerdict {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Narrowed => "narrowed",
            Self::Blocked => "blocked",
        }
    }
}

/// Auto-narrow reason recorded when a surface cannot keep its claimed posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureNarrowReason {
    /// Relationship or graph proof is absent.
    MissingRelationshipProof,
    /// Target-context proof is absent.
    MissingTargetContextProof,
    /// Live-counterpart proof is absent.
    MissingLiveCounterpartProof,
    /// Planned or validated viewer proof is absent.
    MissingPlanValidationProof,
    /// Handoff-boundary or overlay proof is absent.
    MissingHandoffBoundaryProof,
    /// Wrong-target drill proof is absent.
    MissingWrongTargetDrill,
    /// Stale-live overlay drill proof is absent.
    MissingStaleLiveOverlayDrill,
    /// Support/export parity proof is absent.
    MissingExportParityProof,
    /// The surface proof packet is stale.
    EvidenceStale,
}

impl InfrastructureNarrowReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingRelationshipProof => "missing_relationship_proof",
            Self::MissingTargetContextProof => "missing_target_context_proof",
            Self::MissingLiveCounterpartProof => "missing_live_counterpart_proof",
            Self::MissingPlanValidationProof => "missing_plan_validation_proof",
            Self::MissingHandoffBoundaryProof => "missing_handoff_boundary_proof",
            Self::MissingWrongTargetDrill => "missing_wrong_target_drill",
            Self::MissingStaleLiveOverlayDrill => "missing_stale_live_overlay_drill",
            Self::MissingExportParityProof => "missing_export_parity_proof",
            Self::EvidenceStale => "evidence_stale",
        }
    }
}

/// Consumer that must cite the shared evidence index directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureEvidenceConsumer {
    /// Help-facing docs summary.
    DocsHelp,
    /// In-product Help / About proof card or equivalent truth surface.
    HelpAbout,
    /// Support playbook or support-export lane.
    SupportPlaybook,
    /// Release/public-truth publication lane.
    PublicTruth,
}

impl InfrastructureEvidenceConsumer {
    /// Every required consumer in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DocsHelp,
        Self::HelpAbout,
        Self::SupportPlaybook,
        Self::PublicTruth,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsHelp => "docs_help",
            Self::HelpAbout => "help_about",
            Self::SupportPlaybook => "support_playbook",
            Self::PublicTruth => "public_truth",
        }
    }
}

/// One claimed infrastructure surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureSurfaceRow {
    /// Claimed surface family.
    pub surface: InfrastructureSurface,
    /// Posture the surface wants to publish.
    pub claimed_posture: QualificationPosture,
    /// Posture actually displayed after auto-narrowing.
    pub displayed_posture: QualificationPosture,
    /// Certification verdict for the row.
    pub verdict: InfrastructureSurfaceVerdict,
    /// Currency of the backing proof packet.
    pub evidence_currency: EvidenceCurrency,
    /// Canonical upstream packet refs that back this row.
    pub packet_refs: Vec<String>,
    /// Proof classes the row requires to keep its claim.
    pub required_proofs: Vec<InfrastructureProofClass>,
    /// Proof classes currently satisfied by the row.
    pub satisfied_proofs: Vec<InfrastructureProofClass>,
    /// Auto-narrow reasons currently active for the row.
    pub narrow_reasons: Vec<InfrastructureNarrowReason>,
    /// Short scope summary shown to docs/help/support consumers.
    pub scope_summary: String,
}

/// One evidence-index entry reused across docs/help/support/public truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureEvidenceIndexEntry {
    /// Stable evidence entry id.
    pub entry_id: String,
    /// Surface family covered by the entry.
    pub surface: InfrastructureSurface,
    /// Primary schema ref the consumer cites.
    pub schema_ref: String,
    /// Primary contract doc ref the consumer cites.
    pub doc_ref: String,
    /// Primary artifact or support-export ref the consumer cites.
    pub artifact_ref: String,
    /// Drill and fixture refs that keep narrowing behavior reviewable.
    pub drill_refs: Vec<String>,
    /// Export-safe entry summary.
    pub support_summary: String,
}

/// Consumer binding that proves a downstream surface uses the shared index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureEvidenceConsumerBinding {
    /// Consumer that cites the index.
    pub consumer: InfrastructureEvidenceConsumer,
    /// Consumer-facing ref bound to the shared index.
    pub consumer_ref: String,
    /// Surfaces this consumer renders from the shared index.
    pub surface_refs: Vec<InfrastructureSurface>,
    /// True when the consumer reads the shared index directly.
    pub uses_shared_index: bool,
    /// True when the consumer shows the displayed posture.
    pub shows_displayed_posture: bool,
    /// True when the consumer shows active narrow reasons.
    pub shows_narrow_reasons: bool,
    /// True when handoff-boundary posture remains visible.
    pub shows_handoff_boundary: bool,
    /// Export-safe consumer summary.
    pub support_summary: String,
}

/// Input builder for [`InfrastructureSurfaceQualificationPacket`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfrastructureSurfaceQualificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-facing certification label.
    pub certification_label: String,
    /// Claimed surface rows.
    pub surface_rows: Vec<InfrastructureSurfaceRow>,
    /// Canonical evidence index entries.
    pub evidence_index: Vec<InfrastructureEvidenceIndexEntry>,
    /// Downstream consumer bindings.
    pub consumer_bindings: Vec<InfrastructureEvidenceConsumerBinding>,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Mint timestamp.
    pub minted_at: String,
    /// Export-safe packet summary.
    pub support_summary: String,
}

/// Export-safe infrastructure surface qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureSurfaceQualificationPacket {
    /// Record kind; must equal [`INFRASTRUCTURE_SURFACE_QUALIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`INFRASTRUCTURE_SURFACE_QUALIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-facing certification label.
    pub certification_label: String,
    /// Claimed infrastructure surface rows.
    pub surface_rows: Vec<InfrastructureSurfaceRow>,
    /// Canonical evidence index.
    pub evidence_index: Vec<InfrastructureEvidenceIndexEntry>,
    /// Downstream consumer bindings.
    pub consumer_bindings: Vec<InfrastructureEvidenceConsumerBinding>,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Mint timestamp.
    pub minted_at: String,
    /// Export-safe packet summary.
    pub support_summary: String,
}

impl InfrastructureSurfaceQualificationPacket {
    /// Builds a packet from checked input.
    pub fn new(input: InfrastructureSurfaceQualificationPacketInput) -> Self {
        Self {
            record_kind: INFRASTRUCTURE_SURFACE_QUALIFICATION_RECORD_KIND.to_owned(),
            schema_version: INFRASTRUCTURE_SURFACE_QUALIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            certification_label: input.certification_label,
            surface_rows: input.surface_rows,
            evidence_index: input.evidence_index,
            consumer_bindings: input.consumer_bindings,
            source_contract_refs: input.source_contract_refs,
            minted_at: input.minted_at,
            support_summary: input.support_summary,
        }
    }

    /// Returns every narrowed or blocked surface.
    pub fn narrowed_surfaces(&self) -> Vec<InfrastructureSurface> {
        self.surface_rows
            .iter()
            .filter(|row| row.displayed_posture != QualificationPosture::StableQualified)
            .map(|row| row.surface)
            .collect()
    }

    /// Returns every blocked surface.
    pub fn blocked_surfaces(&self) -> Vec<InfrastructureSurface> {
        self.surface_rows
            .iter()
            .filter(|row| matches!(row.verdict, InfrastructureSurfaceVerdict::Blocked))
            .map(|row| row.surface)
            .collect()
    }

    /// Validates the packet invariants.
    pub fn validate(&self) -> Vec<InfrastructureSurfaceQualificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != INFRASTRUCTURE_SURFACE_QUALIFICATION_RECORD_KIND {
            violations.push(InfrastructureSurfaceQualificationViolation::WrongRecordKind);
        }
        if self.schema_version != INFRASTRUCTURE_SURFACE_QUALIFICATION_SCHEMA_VERSION {
            violations.push(InfrastructureSurfaceQualificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.certification_label.trim().is_empty()
            || self.minted_at.trim().is_empty()
            || self.support_summary.trim().is_empty()
        {
            violations.push(InfrastructureSurfaceQualificationViolation::MissingIdentity);
        }

        validate_source_contract_refs(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_evidence_index(self, &mut violations);
        validate_consumer_bindings(self, &mut violations);

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("infrastructure surface qualification packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable = self
            .surface_rows
            .iter()
            .filter(|row| row.displayed_posture == crate::QualificationPosture::StableQualified)
            .count();
        let mut out = String::new();
        out.push_str("# Infrastructure surface qualification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.certification_label));
        out.push_str(&format!(
            "- Surface families: {} ({} stable-qualified, {} narrowed/blocked)\n",
            self.surface_rows.len(),
            stable,
            self.narrowed_surfaces().len()
        ));
        out.push_str(&format!(
            "- Evidence index entries: {}\n",
            self.evidence_index.len()
        ));
        out.push_str(&format!(
            "- Shared consumers: {}\n",
            self.consumer_bindings.len()
        ));
        out.push_str("\n## Surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "- **{}**: `{}` / `{}`\n",
                row.surface.as_str(),
                posture_token(row.displayed_posture),
                row.verdict.as_str()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!("  - Evidence: {}\n", row.packet_refs.join(", ")));
        }
        out
    }
}

/// Errors emitted when reading the checked support-export artifact.
#[derive(Debug)]
pub enum InfrastructureSurfaceQualificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<InfrastructureSurfaceQualificationViolation>),
}

impl fmt::Display for InfrastructureSurfaceQualificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "infrastructure surface qualification export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "infrastructure surface qualification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for InfrastructureSurfaceQualificationArtifactError {}

/// Validation failures emitted by [`InfrastructureSurfaceQualificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InfrastructureSurfaceQualificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required surface row is missing or duplicated.
    SurfaceCoverageIncomplete,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row does not cite any upstream packet.
    SurfaceMissingPacketRefs,
    /// A surface row carries an inconsistent proof set.
    ProofCoverageMismatch,
    /// A surface row did not narrow correctly for its missing or stale proof.
    DisplayedPostureMismatch,
    /// A surface verdict does not agree with the displayed posture.
    VerdictPostureMismatch,
    /// An evidence-index entry is missing or duplicated.
    EvidenceIndexCoverageIncomplete,
    /// An evidence-index entry references the wrong canonical schema or doc.
    EvidenceIndexRefMismatch,
    /// A required consumer binding is missing or incomplete.
    ConsumerBindingIncomplete,
    /// A consumer binding does not cover every surfaced row.
    ConsumerBindingCoverageMismatch,
}

impl InfrastructureSurfaceQualificationViolation {
    /// Stable token exported in validation failures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::SurfaceCoverageIncomplete => "surface_coverage_incomplete",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::SurfaceMissingPacketRefs => "surface_missing_packet_refs",
            Self::ProofCoverageMismatch => "proof_coverage_mismatch",
            Self::DisplayedPostureMismatch => "displayed_posture_mismatch",
            Self::VerdictPostureMismatch => "verdict_posture_mismatch",
            Self::EvidenceIndexCoverageIncomplete => "evidence_index_coverage_incomplete",
            Self::EvidenceIndexRefMismatch => "evidence_index_ref_mismatch",
            Self::ConsumerBindingIncomplete => "consumer_binding_incomplete",
            Self::ConsumerBindingCoverageMismatch => "consumer_binding_coverage_mismatch",
        }
    }
}

/// Reads and validates the checked support-export artifact.
pub fn current_infrastructure_surface_qualification_export(
) -> Result<InfrastructureSurfaceQualificationPacket, InfrastructureSurfaceQualificationArtifactError>
{
    let payload = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/infra/infrastructure-surface-qualification/support_export.json"
    ));
    let packet: InfrastructureSurfaceQualificationPacket = serde_json::from_str(payload)
        .map_err(InfrastructureSurfaceQualificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(InfrastructureSurfaceQualificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Returns the canonical seeded infrastructure surface qualification packet.
pub fn seeded_infrastructure_surface_qualification_packet(
) -> InfrastructureSurfaceQualificationPacket {
    InfrastructureSurfaceQualificationPacket::new(
        InfrastructureSurfaceQualificationPacketInput {
            packet_id: "infra-surface-qualification:stable:0001".to_owned(),
            certification_label: "Infrastructure Surface Qualification".to_owned(),
            surface_rows: vec![
                surface_row(
                    InfrastructureSurface::SourceIntelligence,
                    vec![
                        "fixtures/infra/source-intelligence-and-resource-relationships/qualified_matrix_packet.json"
                            .to_owned(),
                        "fixtures/infra/source-intelligence-and-resource-relationships/qualified_object_packet.json"
                            .to_owned(),
                    ],
                    vec![
                        InfrastructureProofClass::RelationshipGraph,
                        InfrastructureProofClass::ExportParity,
                    ],
                    vec![
                        InfrastructureProofClass::RelationshipGraph,
                        InfrastructureProofClass::ExportParity,
                    ],
                    EvidenceCurrency::Current,
                    "Source-intelligence keeps Terraform, Kubernetes, devcontainer, CI, and policy truth layers explicit and reuses one object and relation vocabulary across later consumers.",
                ),
                surface_row(
                    InfrastructureSurface::LiveCounterpartGraph,
                    vec![
                        "fixtures/infra/source-intelligence-and-resource-relationships/qualified_object_packet.json"
                            .to_owned(),
                        "artifacts/infra/relation-graph-incident-support-parity-support-export.json"
                            .to_owned(),
                    ],
                    vec![
                        InfrastructureProofClass::RelationshipGraph,
                        InfrastructureProofClass::TargetContext,
                        InfrastructureProofClass::LiveCounterpart,
                        InfrastructureProofClass::ExportParity,
                    ],
                    vec![
                        InfrastructureProofClass::RelationshipGraph,
                        InfrastructureProofClass::TargetContext,
                        InfrastructureProofClass::LiveCounterpart,
                        InfrastructureProofClass::ExportParity,
                    ],
                    EvidenceCurrency::Current,
                    "Live-counterpart and impact flows reopen the same object ids, relation ids, and target slice instead of reconstructing a private graph per surface.",
                ),
                surface_row(
                    InfrastructureSurface::PlanAndValidation,
                    vec![
                        "fixtures/infra/plan-and-validation-viewers/qualified_viewer_packet.json"
                            .to_owned(),
                    ],
                    vec![
                        InfrastructureProofClass::PlanValidation,
                        InfrastructureProofClass::TargetContext,
                        InfrastructureProofClass::ExportParity,
                    ],
                    vec![
                        InfrastructureProofClass::PlanValidation,
                        InfrastructureProofClass::TargetContext,
                        InfrastructureProofClass::ExportParity,
                    ],
                    EvidenceCurrency::Current,
                    "Plan, diff, dry-run, admission, and policy viewers keep planned truth explicit, preserve tool identity, and block hidden live authority.",
                ),
                surface_row(
                    InfrastructureSurface::LiveResourceContext,
                    vec![
                        "fixtures/infra/target-context-and-control-plane-boundary/qualified_context_parity_packet.json"
                            .to_owned(),
                        "fixtures/infra/cluster-context-and-live-resource/qualified_cluster_context_packet.json"
                            .to_owned(),
                    ],
                    vec![
                        InfrastructureProofClass::TargetContext,
                        InfrastructureProofClass::WrongTargetDrill,
                        InfrastructureProofClass::StaleLiveOverlayDrill,
                        InfrastructureProofClass::ExportParity,
                    ],
                    vec![
                        InfrastructureProofClass::TargetContext,
                        InfrastructureProofClass::WrongTargetDrill,
                        InfrastructureProofClass::StaleLiveOverlayDrill,
                        InfrastructureProofClass::ExportParity,
                    ],
                    EvidenceCurrency::Current,
                    "Live-resource and context-strip surfaces preserve exact target identity, keep desired or rendered or plan or live or overlay modes separate, and fail closed on wrong-target or stale-live drills.",
                ),
                surface_row(
                    InfrastructureSurface::ProviderOverlayHandoff,
                    vec![
                        "artifacts/infra/provider-overlay-and-vendor-console-handoff-continuity/support_export.json"
                            .to_owned(),
                    ],
                    vec![
                        InfrastructureProofClass::TargetContext,
                        InfrastructureProofClass::HandoffBoundary,
                        InfrastructureProofClass::StaleLiveOverlayDrill,
                        InfrastructureProofClass::ExportParity,
                    ],
                    vec![
                        InfrastructureProofClass::TargetContext,
                        InfrastructureProofClass::HandoffBoundary,
                        InfrastructureProofClass::StaleLiveOverlayDrill,
                        InfrastructureProofClass::ExportParity,
                    ],
                    EvidenceCurrency::Current,
                    "Provider overlays stay explicitly provider-owned, preserve canonical truth beside the overlay, and keep vendor-console handoffs attributable and return-safe.",
                ),
                surface_row(
                    InfrastructureSurface::IncidentSupportParity,
                    vec![
                        "artifacts/infra/relation-graph-incident-support-parity-support-export.json"
                            .to_owned(),
                        "artifacts/infra/provider-overlay-and-vendor-console-handoff-continuity/support_export.json"
                            .to_owned(),
                        "fixtures/infra/cluster-context-and-live-resource/qualified_cluster_context_packet.json"
                            .to_owned(),
                        "fixtures/infra/plan-and-validation-viewers/qualified_viewer_packet.json"
                            .to_owned(),
                    ],
                    vec![
                        InfrastructureProofClass::RelationshipGraph,
                        InfrastructureProofClass::TargetContext,
                        InfrastructureProofClass::LiveCounterpart,
                        InfrastructureProofClass::PlanValidation,
                        InfrastructureProofClass::HandoffBoundary,
                        InfrastructureProofClass::WrongTargetDrill,
                        InfrastructureProofClass::StaleLiveOverlayDrill,
                        InfrastructureProofClass::ExportParity,
                    ],
                    vec![
                        InfrastructureProofClass::RelationshipGraph,
                        InfrastructureProofClass::TargetContext,
                        InfrastructureProofClass::LiveCounterpart,
                        InfrastructureProofClass::PlanValidation,
                        InfrastructureProofClass::HandoffBoundary,
                        InfrastructureProofClass::WrongTargetDrill,
                        InfrastructureProofClass::StaleLiveOverlayDrill,
                        InfrastructureProofClass::ExportParity,
                    ],
                    EvidenceCurrency::Current,
                    "Incident and support consumers reopen the same graph slice, preserve handoff lineage, and keep wrong-target and stale-live labels visible instead of flattening operational evidence.",
                ),
                surface_row(
                    InfrastructureSurface::PublicEvidenceIndex,
                    vec![
                        INFRASTRUCTURE_SURFACE_QUALIFICATION_ARTIFACT_REF.to_owned(),
                        INFRASTRUCTURE_SURFACE_QUALIFICATION_HELP_REF.to_owned(),
                    ],
                    vec![
                        InfrastructureProofClass::RelationshipGraph,
                        InfrastructureProofClass::TargetContext,
                        InfrastructureProofClass::HandoffBoundary,
                        InfrastructureProofClass::ExportParity,
                    ],
                    vec![
                        InfrastructureProofClass::RelationshipGraph,
                        InfrastructureProofClass::TargetContext,
                        InfrastructureProofClass::HandoffBoundary,
                        InfrastructureProofClass::ExportParity,
                    ],
                    EvidenceCurrency::Current,
                    "Docs/help, support playbooks, Help / About, and public-truth publication all cite the same infrastructure evidence index rather than restating depth posture in isolated text.",
                ),
            ],
            evidence_index: vec![
                evidence_entry(
                    "evidence:infra:source-intelligence",
                    InfrastructureSurface::SourceIntelligence,
                    SOURCE_INTELLIGENCE_RELATIONSHIP_SCHEMA_REF,
                    "docs/infra/source-intelligence-and-resource-relationships.md",
                    "fixtures/infra/source-intelligence-and-resource-relationships/qualified_matrix_packet.json",
                    vec![
                        "fixtures/infra/source-intelligence-and-resource-relationships/missing_truth_layer_and_profile_packet.json"
                            .to_owned(),
                        "fixtures/infra/source-intelligence-and-resource-relationships/missing_rendered_lineage_object_packet.json"
                            .to_owned(),
                    ],
                    "Canonical vocabulary and object-packet proof for authored, rendered, planned, observed, and overlay infrastructure truth.",
                ),
                evidence_entry(
                    "evidence:infra:live-counterpart",
                    InfrastructureSurface::LiveCounterpartGraph,
                    RELATION_GRAPH_PARITY_SCHEMA_REF,
                    "docs/infra/relation-graph-incident-support-parity.md",
                    "artifacts/infra/relation-graph-incident-support-parity-support-export.json",
                    vec![
                        "fixtures/infra/relation-graph-incident-support-parity/missing_connector_skew_drill_packet.json"
                            .to_owned(),
                        "fixtures/infra/relation-graph-incident-support-parity/permission_limited_binding_dropped_packet.json"
                            .to_owned(),
                    ],
                    "Exact graph selections, relation-set signatures, and reopen parity for live-counterpart and impact flows.",
                ),
                evidence_entry(
                    "evidence:infra:plan-validation",
                    InfrastructureSurface::PlanAndValidation,
                    PLAN_AND_VALIDATION_VIEWER_SCHEMA_REF,
                    "docs/infra/plan-and-validation-viewers.md",
                    "fixtures/infra/plan-and-validation-viewers/qualified_viewer_packet.json",
                    vec![
                        "fixtures/infra/plan-and-validation-viewers/hidden_live_authority_packet.json"
                            .to_owned(),
                        "fixtures/infra/plan-and-validation-viewers/missing_tool_identity_and_review_gate_packet.json"
                            .to_owned(),
                    ],
                    "Plan and validation viewer proof for tool identity, target context, and explicit review-before-apply posture.",
                ),
                evidence_entry(
                    "evidence:infra:live-resource-context",
                    InfrastructureSurface::LiveResourceContext,
                    CLUSTER_LIVE_RESOURCE_SCHEMA_REF,
                    "docs/infra/cluster-context-and-live-resource.md",
                    "fixtures/infra/cluster-context-and-live-resource/qualified_cluster_context_packet.json",
                    vec![
                        "fixtures/infra/cluster-context-and-live-resource/stale_live_downgraded_packet.json"
                            .to_owned(),
                        "fixtures/infra/cluster-context-and-live-resource/wrong_target_blended_view_packet.json"
                            .to_owned(),
                    ],
                    "Cluster-context and live-resource proof for explicit target strips, separate truth modes, and wrong-target blocking.",
                ),
                evidence_entry(
                    "evidence:infra:overlay-handoff",
                    InfrastructureSurface::ProviderOverlayHandoff,
                    PROVIDER_OVERLAY_HANDOFF_SCHEMA_REF,
                    "docs/infra/provider-overlay-and-vendor-console-handoff-continuity.md",
                    "artifacts/infra/provider-overlay-and-vendor-console-handoff-continuity/support_export.json",
                    vec![
                        "fixtures/infra/provider-overlay-and-vendor-console-handoff-continuity/blurred_overlay_truth_packet.json"
                            .to_owned(),
                        "fixtures/infra/provider-overlay-and-vendor-console-handoff-continuity/generic_shell_return_packet.json"
                            .to_owned(),
                    ],
                    "Provider-overlay disclosure and vendor-console handoff continuity proof for code, incident, preview, route, and infrastructure surfaces.",
                ),
                evidence_entry(
                    "evidence:infra:incident-support",
                    InfrastructureSurface::IncidentSupportParity,
                    RELATION_GRAPH_PARITY_SCHEMA_REF,
                    "docs/infra/relation-graph-incident-support-parity.md",
                    "artifacts/infra/relation-graph-incident-support-parity-support-export.json",
                    vec![
                        "fixtures/infra/relation-graph-incident-support-parity/missing_connector_skew_drill_packet.json"
                            .to_owned(),
                        "fixtures/infra/relation-graph-incident-support-parity/permission_limited_binding_dropped_packet.json"
                            .to_owned(),
                    ],
                    "Incident and support parity proof for reopen-safe graph state, export posture, and control-plane lineage.",
                ),
                evidence_entry(
                    "evidence:infra:public-index",
                    InfrastructureSurface::PublicEvidenceIndex,
                    INFRASTRUCTURE_SURFACE_QUALIFICATION_SCHEMA_REF,
                    INFRASTRUCTURE_SURFACE_QUALIFICATION_DOC_REF,
                    INFRASTRUCTURE_SURFACE_QUALIFICATION_ARTIFACT_REF,
                    vec![
                        "fixtures/infra/infrastructure-surface-qualification/missing_relationship_proof_packet.json"
                            .to_owned(),
                        "fixtures/infra/infrastructure-surface-qualification/stale_public_index_packet.json"
                            .to_owned(),
                    ],
                    "Shared evidence index that downstream docs/help, support, and public-truth consumers cite directly.",
                ),
            ],
            consumer_bindings: vec![
                consumer_binding(
                    InfrastructureEvidenceConsumer::DocsHelp,
                    INFRASTRUCTURE_SURFACE_QUALIFICATION_HELP_REF,
                ),
                consumer_binding(
                    InfrastructureEvidenceConsumer::HelpAbout,
                    "docs/help/help_about_truth_source.md",
                ),
                consumer_binding(
                    InfrastructureEvidenceConsumer::SupportPlaybook,
                    "artifacts/infra/infrastructure-surface-qualification/support_export.json",
                ),
                consumer_binding(
                    InfrastructureEvidenceConsumer::PublicTruth,
                    "artifacts/infra/infrastructure-surface-qualification/support_export.json",
                ),
            ],
            source_contract_refs: vec![
                CONTROL_PLANE_BOUNDARY_SCHEMA_REF.to_owned(),
                CLUSTER_LIVE_RESOURCE_SCHEMA_REF.to_owned(),
                PLAN_AND_VALIDATION_VIEWER_SCHEMA_REF.to_owned(),
                PROVIDER_OVERLAY_HANDOFF_SCHEMA_REF.to_owned(),
                RELATION_GRAPH_PARITY_SCHEMA_REF.to_owned(),
                SOURCE_INTELLIGENCE_RELATIONSHIP_SCHEMA_REF.to_owned(),
                INFRASTRUCTURE_SURFACE_QUALIFICATION_SCHEMA_REF.to_owned(),
            ],
            minted_at: "2026-06-12T17:40:00Z".to_owned(),
            support_summary:
                "Canonical infrastructure qualification packet and evidence index for source intelligence, live counterpart, plan viewer, target-context, handoff-boundary, and incident-support parity surfaces."
                    .to_owned(),
        },
    )
}

fn posture_token(posture: QualificationPosture) -> &'static str {
    match posture {
        QualificationPosture::StableQualified => "stable_qualified",
        QualificationPosture::FileOnly => "file_only",
        QualificationPosture::InspectOnly => "inspect_only",
        QualificationPosture::HandoffOnly => "handoff_only",
        QualificationPosture::Downgraded => "downgraded",
    }
}

fn surface_row(
    surface: InfrastructureSurface,
    packet_refs: Vec<String>,
    required_proofs: Vec<InfrastructureProofClass>,
    satisfied_proofs: Vec<InfrastructureProofClass>,
    evidence_currency: EvidenceCurrency,
    scope_summary: impl Into<String>,
) -> InfrastructureSurfaceRow {
    let displayed_posture =
        derive_displayed_posture(&required_proofs, &satisfied_proofs, evidence_currency);
    let narrow_reasons =
        derive_narrow_reasons(&required_proofs, &satisfied_proofs, evidence_currency);
    let verdict = derive_verdict(displayed_posture);
    InfrastructureSurfaceRow {
        surface,
        claimed_posture: QualificationPosture::StableQualified,
        displayed_posture,
        verdict,
        evidence_currency,
        packet_refs,
        required_proofs,
        satisfied_proofs,
        narrow_reasons,
        scope_summary: scope_summary.into(),
    }
}

fn evidence_entry(
    entry_id: impl Into<String>,
    surface: InfrastructureSurface,
    schema_ref: impl Into<String>,
    doc_ref: impl Into<String>,
    artifact_ref: impl Into<String>,
    drill_refs: Vec<String>,
    support_summary: impl Into<String>,
) -> InfrastructureEvidenceIndexEntry {
    InfrastructureEvidenceIndexEntry {
        entry_id: entry_id.into(),
        surface,
        schema_ref: schema_ref.into(),
        doc_ref: doc_ref.into(),
        artifact_ref: artifact_ref.into(),
        drill_refs,
        support_summary: support_summary.into(),
    }
}

fn consumer_binding(
    consumer: InfrastructureEvidenceConsumer,
    consumer_ref: impl Into<String>,
) -> InfrastructureEvidenceConsumerBinding {
    InfrastructureEvidenceConsumerBinding {
        consumer,
        consumer_ref: consumer_ref.into(),
        surface_refs: InfrastructureSurface::ALL.to_vec(),
        uses_shared_index: true,
        shows_displayed_posture: true,
        shows_narrow_reasons: true,
        shows_handoff_boundary: true,
        support_summary:
            "Consumer renders infrastructure qualification directly from the shared evidence index."
                .to_owned(),
    }
}

fn derive_displayed_posture(
    required_proofs: &[InfrastructureProofClass],
    satisfied_proofs: &[InfrastructureProofClass],
    evidence_currency: EvidenceCurrency,
) -> QualificationPosture {
    if evidence_currency == EvidenceCurrency::Stale {
        return QualificationPosture::InspectOnly;
    }
    let satisfied = satisfied_proofs.iter().copied().collect::<BTreeSet<_>>();
    let missing = required_proofs
        .iter()
        .copied()
        .filter(|proof| !satisfied.contains(proof))
        .collect::<BTreeSet<_>>();

    if missing.is_empty() {
        QualificationPosture::StableQualified
    } else if missing.contains(&InfrastructureProofClass::HandoffBoundary) {
        QualificationPosture::HandoffOnly
    } else if missing.contains(&InfrastructureProofClass::RelationshipGraph) {
        QualificationPosture::FileOnly
    } else if missing.contains(&InfrastructureProofClass::ExportParity) {
        QualificationPosture::Downgraded
    } else {
        QualificationPosture::InspectOnly
    }
}

fn derive_narrow_reasons(
    required_proofs: &[InfrastructureProofClass],
    satisfied_proofs: &[InfrastructureProofClass],
    evidence_currency: EvidenceCurrency,
) -> Vec<InfrastructureNarrowReason> {
    let satisfied = satisfied_proofs.iter().copied().collect::<BTreeSet<_>>();
    let mut reasons = Vec::new();
    for proof in required_proofs {
        if satisfied.contains(proof) {
            continue;
        }
        reasons.push(match proof {
            InfrastructureProofClass::RelationshipGraph => {
                InfrastructureNarrowReason::MissingRelationshipProof
            }
            InfrastructureProofClass::TargetContext => {
                InfrastructureNarrowReason::MissingTargetContextProof
            }
            InfrastructureProofClass::LiveCounterpart => {
                InfrastructureNarrowReason::MissingLiveCounterpartProof
            }
            InfrastructureProofClass::PlanValidation => {
                InfrastructureNarrowReason::MissingPlanValidationProof
            }
            InfrastructureProofClass::HandoffBoundary => {
                InfrastructureNarrowReason::MissingHandoffBoundaryProof
            }
            InfrastructureProofClass::WrongTargetDrill => {
                InfrastructureNarrowReason::MissingWrongTargetDrill
            }
            InfrastructureProofClass::StaleLiveOverlayDrill => {
                InfrastructureNarrowReason::MissingStaleLiveOverlayDrill
            }
            InfrastructureProofClass::ExportParity => {
                InfrastructureNarrowReason::MissingExportParityProof
            }
        });
    }
    if evidence_currency == EvidenceCurrency::Stale {
        reasons.push(InfrastructureNarrowReason::EvidenceStale);
    }
    reasons
}

fn derive_verdict(displayed_posture: QualificationPosture) -> InfrastructureSurfaceVerdict {
    match displayed_posture {
        QualificationPosture::StableQualified => InfrastructureSurfaceVerdict::Certified,
        QualificationPosture::FileOnly
        | QualificationPosture::InspectOnly
        | QualificationPosture::HandoffOnly => InfrastructureSurfaceVerdict::Narrowed,
        QualificationPosture::Downgraded => InfrastructureSurfaceVerdict::Blocked,
    }
}

fn validate_source_contract_refs(
    packet: &InfrastructureSurfaceQualificationPacket,
    violations: &mut Vec<InfrastructureSurfaceQualificationViolation>,
) {
    let expected = [
        CONTROL_PLANE_BOUNDARY_SCHEMA_REF,
        CLUSTER_LIVE_RESOURCE_SCHEMA_REF,
        PLAN_AND_VALIDATION_VIEWER_SCHEMA_REF,
        PROVIDER_OVERLAY_HANDOFF_SCHEMA_REF,
        RELATION_GRAPH_PARITY_SCHEMA_REF,
        SOURCE_INTELLIGENCE_RELATIONSHIP_SCHEMA_REF,
        INFRASTRUCTURE_SURFACE_QUALIFICATION_SCHEMA_REF,
    ];
    if expected.iter().any(|required| {
        !packet
            .source_contract_refs
            .iter()
            .any(|value| value == required)
    }) {
        violations.push(InfrastructureSurfaceQualificationViolation::MissingSourceContracts);
    }
}

fn validate_surface_rows(
    packet: &InfrastructureSurfaceQualificationPacket,
    violations: &mut Vec<InfrastructureSurfaceQualificationViolation>,
) {
    let mut seen = BTreeSet::new();
    for row in &packet.surface_rows {
        if !seen.insert(row.surface) {
            violations.push(InfrastructureSurfaceQualificationViolation::SurfaceCoverageIncomplete);
        }
        if row.scope_summary.trim().is_empty() || row.required_proofs.is_empty() {
            violations.push(InfrastructureSurfaceQualificationViolation::SurfaceRowIncomplete);
        }
        if row.packet_refs.is_empty() || row.packet_refs.iter().any(|value| value.trim().is_empty())
        {
            violations.push(InfrastructureSurfaceQualificationViolation::SurfaceMissingPacketRefs);
        }
        let required = row.required_proofs.iter().copied().collect::<BTreeSet<_>>();
        let satisfied = row
            .satisfied_proofs
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        if !satisfied.is_subset(&required) {
            violations.push(InfrastructureSurfaceQualificationViolation::ProofCoverageMismatch);
        }
        let expected_posture = derive_displayed_posture(
            &row.required_proofs,
            &row.satisfied_proofs,
            row.evidence_currency,
        );
        if row.displayed_posture != expected_posture {
            violations.push(InfrastructureSurfaceQualificationViolation::DisplayedPostureMismatch);
        }
        let expected_reasons = derive_narrow_reasons(
            &row.required_proofs,
            &row.satisfied_proofs,
            row.evidence_currency,
        );
        if row.narrow_reasons != expected_reasons {
            violations.push(InfrastructureSurfaceQualificationViolation::ProofCoverageMismatch);
        }
        let expected_verdict = derive_verdict(row.displayed_posture);
        if row.verdict != expected_verdict {
            violations.push(InfrastructureSurfaceQualificationViolation::VerdictPostureMismatch);
        }
    }

    if seen
        != InfrastructureSurface::ALL
            .into_iter()
            .collect::<BTreeSet<_>>()
    {
        violations.push(InfrastructureSurfaceQualificationViolation::SurfaceCoverageIncomplete);
    }
}

fn validate_evidence_index(
    packet: &InfrastructureSurfaceQualificationPacket,
    violations: &mut Vec<InfrastructureSurfaceQualificationViolation>,
) {
    let mut seen = BTreeSet::new();
    for entry in &packet.evidence_index {
        if !seen.insert(entry.surface) {
            violations
                .push(InfrastructureSurfaceQualificationViolation::EvidenceIndexCoverageIncomplete);
        }
        if entry.entry_id.trim().is_empty()
            || entry.schema_ref.trim().is_empty()
            || entry.doc_ref.trim().is_empty()
            || entry.artifact_ref.trim().is_empty()
            || entry.support_summary.trim().is_empty()
        {
            violations.push(InfrastructureSurfaceQualificationViolation::EvidenceIndexRefMismatch);
        }

        let expected_schema = match entry.surface {
            InfrastructureSurface::SourceIntelligence => {
                SOURCE_INTELLIGENCE_RELATIONSHIP_SCHEMA_REF
            }
            InfrastructureSurface::LiveCounterpartGraph
            | InfrastructureSurface::IncidentSupportParity => RELATION_GRAPH_PARITY_SCHEMA_REF,
            InfrastructureSurface::PlanAndValidation => PLAN_AND_VALIDATION_VIEWER_SCHEMA_REF,
            InfrastructureSurface::LiveResourceContext => CLUSTER_LIVE_RESOURCE_SCHEMA_REF,
            InfrastructureSurface::ProviderOverlayHandoff => PROVIDER_OVERLAY_HANDOFF_SCHEMA_REF,
            InfrastructureSurface::PublicEvidenceIndex => {
                INFRASTRUCTURE_SURFACE_QUALIFICATION_SCHEMA_REF
            }
        };
        if entry.schema_ref != expected_schema {
            violations.push(InfrastructureSurfaceQualificationViolation::EvidenceIndexRefMismatch);
        }
    }

    if seen
        != InfrastructureSurface::ALL
            .into_iter()
            .collect::<BTreeSet<_>>()
    {
        violations
            .push(InfrastructureSurfaceQualificationViolation::EvidenceIndexCoverageIncomplete);
    }
}

fn validate_consumer_bindings(
    packet: &InfrastructureSurfaceQualificationPacket,
    violations: &mut Vec<InfrastructureSurfaceQualificationViolation>,
) {
    let required_surfaces = InfrastructureSurface::ALL.into_iter().collect::<Vec<_>>();
    let mut seen = BTreeSet::new();
    for binding in &packet.consumer_bindings {
        if !seen.insert(binding.consumer) {
            violations.push(InfrastructureSurfaceQualificationViolation::ConsumerBindingIncomplete);
        }
        if binding.consumer_ref.trim().is_empty()
            || binding.support_summary.trim().is_empty()
            || !binding.uses_shared_index
            || !binding.shows_displayed_posture
            || !binding.shows_narrow_reasons
            || !binding.shows_handoff_boundary
        {
            violations.push(InfrastructureSurfaceQualificationViolation::ConsumerBindingIncomplete);
        }
        if binding.surface_refs != required_surfaces {
            violations
                .push(InfrastructureSurfaceQualificationViolation::ConsumerBindingCoverageMismatch);
        }
    }

    if seen
        != InfrastructureEvidenceConsumer::ALL
            .into_iter()
            .collect::<BTreeSet<_>>()
    {
        violations.push(InfrastructureSurfaceQualificationViolation::ConsumerBindingIncomplete);
    }
}
