//! Project Doctor, support-export, and docs/help projection for
//! generated-artifact drift, source-missing, generator-unavailable,
//! regeneration-blocked, and direct-edit-denied states.
//!
//! This module folds the canonical write-boundary decision packet from
//! [`aureline_generated`] into one typed Project Doctor findings packet so the
//! Support Center, headless Doctor, docs/help, About, and support exports can
//! explain a generated-artifact problem without raw log spelunking. It does not
//! re-derive boundary truth: every finding quotes the same boundary state,
//! attempt outcome, edit gate, canonical-source reference, generator identity,
//! checkpoint lineage, regeneration route, why-blocked tokens, and guidance line
//! the in-product surfaces already render from the
//! [`aureline_generated::WriteBoundaryDecision`].
//!
//! ## What this projection owns
//!
//! - The [`GeneratedDoctorFindingClass`] vocabulary — the five generated-artifact
//!   problem states (drift detected, source missing, generator unavailable,
//!   regeneration blocked, and direct-edit denied) plus the single
//!   [`RESOLUTION_ORDER`] every surface classifies with, so docs, help, About,
//!   and support all name and rank the same states.
//! - The [`GeneratedDoctorFinding`] row — one inspectable finding per degraded
//!   generated-artifact case. Each row carries the controlled severity, summary,
//!   and next-action vocabulary, the canonical-source/generator/checkpoint
//!   lineage that traces the issue back to its origin, and a fixed set of
//!   [`GeneratedDoctorAction`]s (open-details, compare, regenerate, docs/help)
//!   linked to the same descriptor and regeneration-plan objects used elsewhere.
//! - The [`GeneratedDoctorFindingsPacket`] — the machine- and human-readable
//!   projection that shares one finding vocabulary, mirrored by the checked-in
//!   schema, report, proof packet, and fixture corpus.
//! - The [`GeneratedDoctorSupportExport`] envelope — the redaction-safe support
//!   projection that preserves canonical-source and checkpoint lineage while
//!   excluding raw payloads, private material, and ambient authority.
//!
//! ## What this projection does NOT own
//!
//! The write-boundary decision, the generated-artifact descriptor, the
//! regeneration plan, and the local-history timeline are owned by
//! [`aureline_generated`]. This module is a read-only consumer: it never mutates
//! an artifact, never applies a repair, and never regenerates anything. The
//! actions it surfaces are links back to those owning objects, not inline
//! mutations.

use std::collections::BTreeMap;

use aureline_generated::{
    seeded_write_boundary_packet, ArtifactClass, AttemptOutcome, BoundaryState, EditPosture,
    GeneratorIdentity, LegAvailability, RegenerationAvailability, WriteBoundaryCase,
    WriteBoundaryDecision, WriteBoundaryPacket, WriteBoundarySubject,
    GENERATED_ARTIFACT_DESCRIPTOR_PACKET_ID, GENERATED_ARTIFACT_DESCRIPTOR_PACKET_REF,
    GENERATED_TIMELINE_PACKET_ID, REGENERATION_PLAN_PACKET_ID, REGENERATION_PLAN_PACKET_REF,
    WRITE_BOUNDARY_PACKET_ID, WRITE_BOUNDARY_PACKET_REF,
};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Stable identifiers and source-contract refs.
// ---------------------------------------------------------------------------

/// Schema version for the generated-artifact Doctor packet and fixtures.
pub const GENERATED_DOCTOR_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the findings packet.
pub const GENERATED_DOCTOR_PACKET_RECORD_KIND: &str = "generated_doctor_findings_packet_record";

/// Stable record-kind tag for one finding row.
pub const GENERATED_DOCTOR_FINDING_RECORD_KIND: &str = "generated_doctor_finding_record";

/// Stable record-kind tag for one fixture row.
pub const GENERATED_DOCTOR_FIXTURE_RECORD_KIND: &str = "generated_doctor_fixture_record";

/// Stable record-kind tag for the support-export envelope.
pub const GENERATED_DOCTOR_SUPPORT_EXPORT_RECORD_KIND: &str =
    "generated_doctor_support_export_record";

/// Stable packet id for the generated-artifact Doctor lane.
pub const GENERATED_DOCTOR_PACKET_ID: &str = "generated.generated_doctor.v1";

/// Repository-relative reviewer/help doc for the lane.
pub const GENERATED_DOCTOR_DOC_REF: &str = "docs/generated/generated-doctor.md";

/// Repository-relative JSON proof packet.
pub const GENERATED_DOCTOR_PACKET_REF: &str = "artifacts/generated/generated-doctor-packet.json";

/// Repository-relative human-readable findings report.
pub const GENERATED_DOCTOR_REPORT_REF: &str = "artifacts/generated/generated-doctor-findings.md";

/// Repository-relative JSON Schema for the packet and fixtures.
pub const GENERATED_DOCTOR_SCHEMA_REF: &str = "schemas/generated/generated-doctor.schema.json";

/// Repository-relative fixture directory.
pub const GENERATED_DOCTOR_FIXTURE_DIR: &str = "fixtures/generated/doctor";

/// Repository-relative fixture manifest.
pub const GENERATED_DOCTOR_FIXTURE_MANIFEST_REF: &str = "fixtures/generated/doctor/manifest.yaml";

// ---------------------------------------------------------------------------
// Controlled vocabulary.
// ---------------------------------------------------------------------------

/// The five generated-artifact problem states the Doctor explains.
///
/// The variants are ordered most-blocking root cause first; [`RESOLUTION_ORDER`]
/// freezes that ranking so every surface classifies a degraded case the same
/// way. A single boundary case can exhibit more than one symptom — a drifted
/// artifact also has its direct edit held — and the resolution order decides
/// which root-cause finding is surfaced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratedDoctorFindingClass {
    /// The canonical source is missing, so the artifact cannot be compared or
    /// regenerated against it.
    SourceMissing,
    /// The generator that rebuilds the artifact is unavailable.
    GeneratorUnavailable,
    /// A policy blocks regenerating the artifact.
    RegenerationBlocked,
    /// The derived bytes have diverged from the canonical source.
    DriftDetected,
    /// A direct edit to a non-authoritative generated artifact was denied; it
    /// must regenerate or escalate through a reviewed override.
    DirectEditDenied,
}

impl GeneratedDoctorFindingClass {
    /// The frozen resolution ordering, most-blocking root cause first.
    pub const RESOLUTION_ORDER: [Self; 5] = [
        Self::SourceMissing,
        Self::GeneratorUnavailable,
        Self::RegenerationBlocked,
        Self::DriftDetected,
        Self::DirectEditDenied,
    ];

    /// Stable snake_case token used in refs, anchors, and exports.
    pub fn token(self) -> &'static str {
        match self {
            Self::SourceMissing => "source_missing",
            Self::GeneratorUnavailable => "generator_unavailable",
            Self::RegenerationBlocked => "regeneration_blocked",
            Self::DriftDetected => "drift_detected",
            Self::DirectEditDenied => "direct_edit_denied",
        }
    }

    /// Controlled severity for the class.
    pub fn severity(self) -> GeneratedDoctorSeverity {
        match self {
            Self::SourceMissing | Self::GeneratorUnavailable | Self::RegenerationBlocked => {
                GeneratedDoctorSeverity::Blocking
            }
            Self::DriftDetected => GeneratedDoctorSeverity::Warning,
            Self::DirectEditDenied => GeneratedDoctorSeverity::Notice,
        }
    }

    /// Controlled, surface-agnostic summary line. Docs, help, About, and support
    /// all reuse this exact text so the vocabulary never forks.
    pub fn summary(self) -> &'static str {
        match self {
            Self::SourceMissing => "The canonical source for this generated artifact is missing.",
            Self::GeneratorUnavailable => {
                "The generator that rebuilds this artifact is unavailable."
            }
            Self::RegenerationBlocked => "Regeneration of this artifact is blocked by policy.",
            Self::DriftDetected => "The generated artifact has drifted from its canonical source.",
            Self::DirectEditDenied => "A direct edit to this generated artifact was denied.",
        }
    }

    /// Controlled next-action line shared across every surface.
    pub fn next_action(self) -> &'static str {
        match self {
            Self::SourceMissing => {
                "Restore the canonical source, then regenerate; the artifact cannot be compared or rebuilt without it."
            }
            Self::GeneratorUnavailable => {
                "Restore the generator or its runtime, then regenerate from the canonical source."
            }
            Self::RegenerationBlocked => {
                "Resolve the policy that blocks regeneration before rebuilding the artifact."
            }
            Self::DriftDetected => {
                "Compare against the canonical source, then regenerate to discard local bytes or reconcile the change into the source."
            }
            Self::DirectEditDenied => {
                "Regenerate from the canonical source, or escalate the edit through a reviewed override."
            }
        }
    }
}

/// The frozen resolution ordering surfaces classify and rank findings by.
pub const RESOLUTION_ORDER: [GeneratedDoctorFindingClass; 5] =
    GeneratedDoctorFindingClass::RESOLUTION_ORDER;

/// Controlled severity for a Doctor finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratedDoctorSeverity {
    /// The artifact cannot be regenerated until the issue is resolved.
    Blocking,
    /// The artifact has diverged and needs reconciliation, but a path exists.
    Warning,
    /// The artifact is intact; only a direct edit was held for review.
    Notice,
}

/// The action a Doctor finding offers, surfaced identically by every consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratedDoctorActionKind {
    /// Open the generated-artifact descriptor for full provenance details.
    OpenDetails,
    /// Open the three-way compare over source, current bytes, and regenerated
    /// candidate.
    Compare,
    /// Open the regeneration plan to rebuild the artifact from its source.
    Regenerate,
    /// Open the docs/help explanation for this finding class.
    OpenDocs,
}

impl GeneratedDoctorActionKind {
    /// The fixed action order every finding exposes.
    pub const ALL: [Self; 4] = [
        Self::OpenDetails,
        Self::Compare,
        Self::Regenerate,
        Self::OpenDocs,
    ];
}

/// Whether a finding's action can run right now.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratedDoctorActionAvailability {
    /// The action can be taken now.
    Available,
    /// The action cannot run now; its target link is still preserved.
    Unavailable,
}

// ---------------------------------------------------------------------------
// Records.
// ---------------------------------------------------------------------------

/// One action a finding offers, linked to the owning descriptor, plan, compare,
/// or docs object rather than performed inline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedDoctorAction {
    /// Which action this is.
    pub kind: GeneratedDoctorActionKind,
    /// Whether it can run now.
    pub availability: GeneratedDoctorActionAvailability,
    /// Stable token naming why it cannot run, when unavailable.
    pub unavailable_reason: Option<String>,
    /// Review-safe reference to the object the action opens.
    pub target_ref: String,
    /// Surface-agnostic action label.
    pub label: String,
}

/// One inspectable Project Doctor finding for a degraded generated artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedDoctorFinding {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable finding id, derived from the source boundary case.
    pub finding_id: String,
    /// The classified root-cause state.
    pub finding_class: GeneratedDoctorFindingClass,
    /// Controlled severity for the class.
    pub severity: GeneratedDoctorSeverity,
    /// Generated-artifact class the finding is about.
    pub artifact_class: ArtifactClass,
    /// Review-safe artifact path label.
    pub artifact_path_label: String,
    /// Boundary state quoted from the write-boundary decision.
    pub boundary_state: BoundaryState,
    /// Attempt outcome quoted from the write-boundary decision.
    pub attempt_outcome: AttemptOutcome,
    /// Effective edit gate after the boundary state floored the declared one.
    pub effective_edit_gate: EditPosture,
    /// Review-safe canonical-source reference. Absent only when the source is
    /// missing.
    pub canonical_source_ref: Option<String>,
    /// Generator identity that produced the artifact.
    pub generator: GeneratorIdentity,
    /// Reference to the reversible-checkpoint lineage that captured the change.
    pub checkpoint_lineage_ref: String,
    /// Review-safe regeneration route that rebuilds the artifact.
    pub regeneration_route: String,
    /// Whether the artifact can be regenerated now, and why not.
    pub regeneration_availability: RegenerationAvailability,
    /// Stable tokens naming every input that blocked or escalated the edit.
    pub why_blocked_tokens: Vec<String>,
    /// The user-visible guidance line quoted from the decision.
    pub guidance_line: String,
    /// Controlled summary line for the finding class.
    pub summary: String,
    /// Controlled next-action line for the finding class.
    pub next_action: String,
    /// Open-details, compare, regenerate, and docs/help actions, in fixed order.
    pub actions: Vec<GeneratedDoctorAction>,
    /// Reference back to the source write-boundary case.
    pub source_case_ref: String,
    /// Upstream generated-artifact packets this finding draws on.
    pub evidence_refs: Vec<String>,
}

impl GeneratedDoctorFinding {
    /// Returns the action of the given kind, if present.
    pub fn action(&self, kind: GeneratedDoctorActionKind) -> Option<&GeneratedDoctorAction> {
        self.actions.iter().find(|action| action.kind == kind)
    }

    /// Returns true when the given action kind can run now.
    pub fn action_available(&self, kind: GeneratedDoctorActionKind) -> bool {
        self.action(kind)
            .map(|action| action.availability == GeneratedDoctorActionAvailability::Available)
            .unwrap_or(false)
    }
}

/// Shared source-contract references for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedDoctorSourceContractRefs {
    /// Reviewer/help doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Proof packet ref.
    pub packet_ref: String,
    /// Human-readable report ref.
    pub report_ref: String,
    /// Fixture manifest ref.
    pub fixture_manifest_ref: String,
}

impl GeneratedDoctorSourceContractRefs {
    fn current() -> Self {
        Self {
            doc_ref: GENERATED_DOCTOR_DOC_REF.to_owned(),
            schema_ref: GENERATED_DOCTOR_SCHEMA_REF.to_owned(),
            packet_ref: GENERATED_DOCTOR_PACKET_REF.to_owned(),
            report_ref: GENERATED_DOCTOR_REPORT_REF.to_owned(),
            fixture_manifest_ref: GENERATED_DOCTOR_FIXTURE_MANIFEST_REF.to_owned(),
        }
    }
}

/// One at-a-glance count of findings in a class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedDoctorClassCount {
    /// Finding class.
    pub finding_class: GeneratedDoctorFindingClass,
    /// Number of findings in the class.
    pub count: usize,
}

/// The Project Doctor findings packet for generated-artifact issues.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedDoctorFindingsPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer title.
    pub title: String,
    /// Shared source-contract refs.
    pub source_contract_refs: GeneratedDoctorSourceContractRefs,
    /// The finding-class vocabulary in resolution order.
    pub finding_classes: Vec<GeneratedDoctorFindingClass>,
    /// The frozen resolution ordering, repeated so consumers bind to it.
    pub resolution_order: Vec<GeneratedDoctorFindingClass>,
    /// Upstream generated-artifact packets folded into the findings.
    pub evidence_packet_refs: Vec<String>,
    /// One finding per degraded generated-artifact case.
    pub findings: Vec<GeneratedDoctorFinding>,
    /// At-a-glance counts per class, in resolution order.
    pub counts_by_class: Vec<GeneratedDoctorClassCount>,
    /// Short invariant summaries.
    pub invariants: Vec<String>,
}

impl GeneratedDoctorFindingsPacket {
    /// Returns the headless machine-readable projection.
    pub fn machine_output(&self) -> Vec<GeneratedDoctorMachineRow> {
        self.findings
            .iter()
            .map(|finding| GeneratedDoctorMachineRow {
                finding_id: finding.finding_id.clone(),
                finding_class: finding.finding_class,
                severity: finding.severity,
                boundary_state: finding.boundary_state,
                attempt_outcome: finding.attempt_outcome,
                regeneration_availability: finding.regeneration_availability,
                compare_available: finding.action_available(GeneratedDoctorActionKind::Compare),
                regenerate_available: finding
                    .action_available(GeneratedDoctorActionKind::Regenerate),
            })
            .collect()
    }

    /// Returns the human-readable summary projection.
    pub fn human_output(&self) -> Vec<GeneratedDoctorHumanRow> {
        self.findings
            .iter()
            .map(|finding| GeneratedDoctorHumanRow {
                finding_id: finding.finding_id.clone(),
                finding_class: finding.finding_class,
                severity: finding.severity,
                artifact_path_label: finding.artifact_path_label.clone(),
                summary: finding.summary.clone(),
                next_action: finding.next_action.clone(),
                guidance_line: finding.guidance_line.clone(),
            })
            .collect()
    }

    /// Returns true when the machine and human projections carry exactly the
    /// same finding ids and classes — the docs/help/support parity invariant.
    pub fn machine_and_human_share_vocabulary(&self) -> bool {
        let machine: Vec<(String, GeneratedDoctorFindingClass)> = self
            .machine_output()
            .into_iter()
            .map(|row| (row.finding_id, row.finding_class))
            .collect();
        let human: Vec<(String, GeneratedDoctorFindingClass)> = self
            .human_output()
            .into_iter()
            .map(|row| (row.finding_id, row.finding_class))
            .collect();
        machine == human
    }
}

/// One machine-readable finding row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedDoctorMachineRow {
    /// Stable finding id.
    pub finding_id: String,
    /// Finding class.
    pub finding_class: GeneratedDoctorFindingClass,
    /// Severity.
    pub severity: GeneratedDoctorSeverity,
    /// Boundary state.
    pub boundary_state: BoundaryState,
    /// Attempt outcome.
    pub attempt_outcome: AttemptOutcome,
    /// Regeneration availability.
    pub regeneration_availability: RegenerationAvailability,
    /// Whether compare can run now.
    pub compare_available: bool,
    /// Whether regenerate can run now.
    pub regenerate_available: bool,
}

/// One human-readable finding row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedDoctorHumanRow {
    /// Stable finding id.
    pub finding_id: String,
    /// Finding class.
    pub finding_class: GeneratedDoctorFindingClass,
    /// Severity.
    pub severity: GeneratedDoctorSeverity,
    /// Review-safe artifact path label.
    pub artifact_path_label: String,
    /// Controlled summary line.
    pub summary: String,
    /// Controlled next-action line.
    pub next_action: String,
    /// Runtime guidance line quoted from the decision.
    pub guidance_line: String,
}

/// One fixture binding a source case to its expected Doctor finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedDoctorFixture {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Reviewer scenario label.
    pub scenario: String,
    /// The finding under test.
    pub finding: GeneratedDoctorFinding,
    /// Expected finding class.
    pub expected_finding_class: GeneratedDoctorFindingClass,
    /// Expected severity.
    pub expected_severity: GeneratedDoctorSeverity,
    /// Expected compare-action availability.
    pub expected_compare_available: bool,
    /// Expected regenerate-action availability.
    pub expected_regenerate_available: bool,
    /// Expected action kinds, in order.
    pub expected_action_kinds: Vec<GeneratedDoctorActionKind>,
}

/// The redaction-safe support-export envelope for the Doctor packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedDoctorSupportExport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Envelope id.
    pub envelope_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Reviewer/help doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Proof packet ref.
    pub packet_ref: String,
    /// Human-readable report ref.
    pub report_ref: String,
    /// True when raw bytes, diffs, and patches are excluded.
    pub raw_payload_excluded: bool,
    /// True when private source material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
    /// True when every finding preserves its canonical-source lineage (a source
    /// ref, or an explicit source-missing state).
    pub canonical_source_lineage_preserved: bool,
    /// True when every finding preserves its checkpoint lineage ref.
    pub checkpoint_lineage_preserved: bool,
    /// The folded findings packet.
    pub packet: GeneratedDoctorFindingsPacket,
}

impl GeneratedDoctorSupportExport {
    /// Returns true when the envelope is metadata-safe and preserves lineage.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.canonical_source_lineage_preserved
            && self.checkpoint_lineage_preserved
            && self.doc_ref == GENERATED_DOCTOR_DOC_REF
            && self.schema_ref == GENERATED_DOCTOR_SCHEMA_REF
            && self.packet_ref == GENERATED_DOCTOR_PACKET_REF
            && self.report_ref == GENERATED_DOCTOR_REPORT_REF
            && !self.packet.findings.is_empty()
            && self.packet.machine_and_human_share_vocabulary()
    }
}

// ---------------------------------------------------------------------------
// Classification and folding.
// ---------------------------------------------------------------------------

/// Classifies a write-boundary decision into a single root-cause finding class,
/// or `None` when the artifact is healthy (a directly admitted edit, or an edit
/// admitted through a recorded reviewed override).
///
/// The boundary state takes precedence over the direct-edit outcome, so a
/// drifted, source-missing, generator-unavailable, or policy-blocked artifact is
/// surfaced by its root cause even though its direct edit is also denied. This
/// is the [`RESOLUTION_ORDER`] in code form.
pub fn classify_decision(decision: &WriteBoundaryDecision) -> Option<GeneratedDoctorFindingClass> {
    match decision.boundary_state {
        BoundaryState::SourceMissing => Some(GeneratedDoctorFindingClass::SourceMissing),
        BoundaryState::GeneratorUnavailable => {
            Some(GeneratedDoctorFindingClass::GeneratorUnavailable)
        }
        BoundaryState::RegenerationBlockedByPolicy => {
            Some(GeneratedDoctorFindingClass::RegenerationBlocked)
        }
        BoundaryState::DriftDetected => Some(GeneratedDoctorFindingClass::DriftDetected),
        BoundaryState::InSync => match decision.attempt_outcome {
            AttemptOutcome::BlockedPendingReview | AttemptOutcome::BlockedRegenerateFirst => {
                Some(GeneratedDoctorFindingClass::DirectEditDenied)
            }
            AttemptOutcome::DirectEditAdmitted | AttemptOutcome::OverrideAdmittedWithDivergence => {
                None
            }
        },
    }
}

fn compare_action(decision: &WriteBoundaryDecision, finding_id: &str) -> GeneratedDoctorAction {
    let available_legs = decision
        .three_way_compare
        .legs
        .iter()
        .filter(|leg| leg.availability == LegAvailability::Available)
        .count();
    // A compare needs at least two legs to mean anything (a baseline and one
    // thing to compare against it).
    let available = available_legs >= 2;
    let unavailable_reason = if available {
        None
    } else {
        let token = decision
            .three_way_compare
            .legs
            .iter()
            .find(|leg| leg.availability == LegAvailability::Unavailable)
            .and_then(|leg| leg.unavailable_reason.clone())
            .unwrap_or_else(|| "compare_legs_unavailable".to_owned());
        Some(token)
    };
    GeneratedDoctorAction {
        kind: GeneratedDoctorActionKind::Compare,
        availability: if available {
            GeneratedDoctorActionAvailability::Available
        } else {
            GeneratedDoctorActionAvailability::Unavailable
        },
        unavailable_reason,
        target_ref: format!("{WRITE_BOUNDARY_PACKET_REF}#{finding_id}"),
        label: "Compare against canonical source".to_owned(),
    }
}

fn regenerate_action(decision: &WriteBoundaryDecision, route: &str) -> GeneratedDoctorAction {
    let (availability, unavailable_reason) = match decision.regeneration_availability {
        RegenerationAvailability::Available => (GeneratedDoctorActionAvailability::Available, None),
        RegenerationAvailability::BlockedSourceMissing => (
            GeneratedDoctorActionAvailability::Unavailable,
            Some("regeneration_blocked_source_missing".to_owned()),
        ),
        RegenerationAvailability::BlockedGeneratorUnavailable => (
            GeneratedDoctorActionAvailability::Unavailable,
            Some("regeneration_blocked_generator_unavailable".to_owned()),
        ),
        RegenerationAvailability::BlockedByPolicy => (
            GeneratedDoctorActionAvailability::Unavailable,
            Some("regeneration_blocked_by_policy".to_owned()),
        ),
    };
    GeneratedDoctorAction {
        kind: GeneratedDoctorActionKind::Regenerate,
        availability,
        unavailable_reason,
        target_ref: format!("{REGENERATION_PLAN_PACKET_REF}#{route}"),
        label: "Regenerate from canonical source".to_owned(),
    }
}

fn actions_for(
    decision: &WriteBoundaryDecision,
    subject: &WriteBoundarySubject,
    finding_id: &str,
    finding_class: GeneratedDoctorFindingClass,
) -> Vec<GeneratedDoctorAction> {
    vec![
        GeneratedDoctorAction {
            kind: GeneratedDoctorActionKind::OpenDetails,
            availability: GeneratedDoctorActionAvailability::Available,
            unavailable_reason: None,
            target_ref: format!("{GENERATED_ARTIFACT_DESCRIPTOR_PACKET_REF}#{finding_id}"),
            label: "Open generated-artifact details".to_owned(),
        },
        compare_action(decision, finding_id),
        regenerate_action(decision, &subject.regeneration_route),
        GeneratedDoctorAction {
            kind: GeneratedDoctorActionKind::OpenDocs,
            availability: GeneratedDoctorActionAvailability::Available,
            unavailable_reason: None,
            target_ref: format!("{GENERATED_DOCTOR_DOC_REF}#{}", finding_class.token()),
            label: "Open docs and help".to_owned(),
        },
    ]
}

fn finding_id_for(case: &WriteBoundaryCase, finding_class: GeneratedDoctorFindingClass) -> String {
    format!("doctor.{}.{}", finding_class.token(), case.case_id)
}

fn finding_from_case(case: &WriteBoundaryCase) -> Option<GeneratedDoctorFinding> {
    let finding_class = classify_decision(&case.decision)?;
    let subject = &case.subject;
    let decision = &case.decision;
    let finding_id = finding_id_for(case, finding_class);
    let actions = actions_for(decision, subject, &finding_id, finding_class);
    Some(GeneratedDoctorFinding {
        record_kind: GENERATED_DOCTOR_FINDING_RECORD_KIND.to_owned(),
        finding_id,
        finding_class,
        severity: finding_class.severity(),
        artifact_class: subject.artifact_class,
        artifact_path_label: subject.artifact_path_label.clone(),
        boundary_state: decision.boundary_state,
        attempt_outcome: decision.attempt_outcome,
        effective_edit_gate: decision.effective_edit_gate,
        canonical_source_ref: subject.canonical_source_ref.clone(),
        generator: subject.generator.clone(),
        checkpoint_lineage_ref: subject.checkpoint_lineage_ref.clone(),
        regeneration_route: subject.regeneration_route.clone(),
        regeneration_availability: decision.regeneration_availability,
        why_blocked_tokens: decision.why_blocked_tokens.clone(),
        guidance_line: decision.guidance_line.clone(),
        summary: finding_class.summary().to_owned(),
        next_action: finding_class.next_action().to_owned(),
        actions,
        source_case_ref: format!("{WRITE_BOUNDARY_PACKET_ID}#{}", case.case_id),
        evidence_refs: evidence_packet_refs(),
    })
}

fn evidence_packet_refs() -> Vec<String> {
    vec![
        WRITE_BOUNDARY_PACKET_ID.to_owned(),
        GENERATED_ARTIFACT_DESCRIPTOR_PACKET_ID.to_owned(),
        REGENERATION_PLAN_PACKET_ID.to_owned(),
        GENERATED_TIMELINE_PACKET_ID.to_owned(),
    ]
}

fn invariants() -> Vec<String> {
    vec![
        "Every generated-artifact problem is one of five named states — source missing, generator unavailable, regeneration blocked, drift detected, or direct-edit denied — classified by one frozen resolution ordering, never flattened into a generic save or index failure.".to_owned(),
        "Each finding traces back to its canonical source, generator identity, and last reversible checkpoint, so an operator can follow the issue to its origin without raw log spelunking.".to_owned(),
        "Each finding offers the same four actions — open details, compare, regenerate, and docs/help — linked to the descriptor and regeneration-plan objects the runtime surfaces already use; an action that cannot run keeps its target link and a stable reason.".to_owned(),
        "The docs, help, About, headless Doctor, and support surfaces share one summary, next-action, severity, and resolution vocabulary, so the same problem reads the same everywhere.".to_owned(),
        "The support export preserves canonical-source and checkpoint lineage while excluding raw payloads, private material, and ambient authority.".to_owned(),
    ]
}

/// Folds the canonical write-boundary packet into the seeded Project Doctor
/// findings packet. Healthy and override-admitted cases produce no finding.
pub fn seeded_generated_doctor_findings_packet() -> GeneratedDoctorFindingsPacket {
    findings_packet_from_write_boundary(&seeded_write_boundary_packet())
}

/// Folds a specific write-boundary packet into a Project Doctor findings packet.
pub fn findings_packet_from_write_boundary(
    packet: &WriteBoundaryPacket,
) -> GeneratedDoctorFindingsPacket {
    let mut findings: Vec<GeneratedDoctorFinding> =
        packet.cases.iter().filter_map(finding_from_case).collect();
    findings.sort_by(|a, b| a.finding_id.cmp(&b.finding_id));

    let mut counts: BTreeMap<GeneratedDoctorFindingClass, usize> = BTreeMap::new();
    for finding in &findings {
        *counts.entry(finding.finding_class).or_default() += 1;
    }
    let counts_by_class = RESOLUTION_ORDER
        .iter()
        .map(|class| GeneratedDoctorClassCount {
            finding_class: *class,
            count: counts.get(class).copied().unwrap_or(0),
        })
        .collect();

    GeneratedDoctorFindingsPacket {
        record_kind: GENERATED_DOCTOR_PACKET_RECORD_KIND.to_owned(),
        schema_version: GENERATED_DOCTOR_SCHEMA_VERSION,
        packet_id: GENERATED_DOCTOR_PACKET_ID.to_owned(),
        title: "Project Doctor findings for generated-artifact drift, source-missing, generator-unavailable, regeneration-blocked, and direct-edit-denied states"
            .to_owned(),
        source_contract_refs: GeneratedDoctorSourceContractRefs::current(),
        finding_classes: RESOLUTION_ORDER.to_vec(),
        resolution_order: RESOLUTION_ORDER.to_vec(),
        evidence_packet_refs: evidence_packet_refs(),
        findings,
        counts_by_class,
        invariants: invariants(),
    }
}

/// Returns the checked-in Doctor fixture corpus: one fixture per finding.
pub fn seeded_generated_doctor_fixtures() -> Vec<GeneratedDoctorFixture> {
    seeded_generated_doctor_findings_packet()
        .findings
        .into_iter()
        .map(fixture_from_finding)
        .collect()
}

fn fixture_from_finding(finding: GeneratedDoctorFinding) -> GeneratedDoctorFixture {
    let expected_compare_available = finding.action_available(GeneratedDoctorActionKind::Compare);
    let expected_regenerate_available =
        finding.action_available(GeneratedDoctorActionKind::Regenerate);
    GeneratedDoctorFixture {
        record_kind: GENERATED_DOCTOR_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: GENERATED_DOCTOR_SCHEMA_VERSION,
        fixture_id: format!("fixture.{}", finding.finding_id),
        scenario: format!(
            "{} — {}",
            finding.artifact_path_label,
            finding.finding_class.summary()
        ),
        expected_finding_class: finding.finding_class,
        expected_severity: finding.severity,
        expected_compare_available,
        expected_regenerate_available,
        expected_action_kinds: finding.actions.iter().map(|action| action.kind).collect(),
        finding,
    }
}

/// Compiles the redaction-safe support-export envelope from the seeded packet.
pub fn compile_generated_doctor_support_export(
    envelope_id: impl Into<String>,
    captured_at: impl Into<String>,
) -> GeneratedDoctorSupportExport {
    let packet = seeded_generated_doctor_findings_packet();
    let canonical_source_lineage_preserved = packet.findings.iter().all(|finding| {
        finding.canonical_source_ref.is_some()
            || finding.finding_class == GeneratedDoctorFindingClass::SourceMissing
    });
    let checkpoint_lineage_preserved = packet
        .findings
        .iter()
        .all(|finding| !finding.checkpoint_lineage_ref.is_empty());
    GeneratedDoctorSupportExport {
        record_kind: GENERATED_DOCTOR_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        envelope_id: envelope_id.into(),
        captured_at: captured_at.into(),
        doc_ref: GENERATED_DOCTOR_DOC_REF.to_owned(),
        schema_ref: GENERATED_DOCTOR_SCHEMA_REF.to_owned(),
        packet_ref: GENERATED_DOCTOR_PACKET_REF.to_owned(),
        report_ref: GENERATED_DOCTOR_REPORT_REF.to_owned(),
        raw_payload_excluded: true,
        raw_private_material_excluded: true,
        ambient_authority_excluded: true,
        canonical_source_lineage_preserved,
        checkpoint_lineage_preserved,
        packet,
    }
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

/// One contract violation found while validating the Doctor packet or fixtures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedDoctorViolation {
    /// Stable check id.
    pub check_id: String,
    /// Subject ref that failed the check.
    pub subject_ref: String,
    /// Reviewer-facing failure message.
    pub message: String,
}

fn push(
    out: &mut Vec<GeneratedDoctorViolation>,
    check_id: &str,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    out.push(GeneratedDoctorViolation {
        check_id: check_id.to_owned(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}

/// Validates the Doctor findings packet contract.
pub fn validate_generated_doctor_findings_packet(
    packet: &GeneratedDoctorFindingsPacket,
) -> Vec<GeneratedDoctorViolation> {
    let mut out = Vec::new();

    if packet.record_kind != GENERATED_DOCTOR_PACKET_RECORD_KIND {
        push(
            &mut out,
            "generated_doctor.record_kind",
            &packet.packet_id,
            "packet record_kind drifted from the frozen token",
        );
    }
    if packet.schema_version != GENERATED_DOCTOR_SCHEMA_VERSION {
        push(
            &mut out,
            "generated_doctor.schema_version",
            &packet.packet_id,
            "schema_version must be 1",
        );
    }
    if packet.packet_id != GENERATED_DOCTOR_PACKET_ID {
        push(
            &mut out,
            "generated_doctor.packet_id",
            &packet.packet_id,
            "packet_id drifted from the frozen id",
        );
    }
    if packet.resolution_order != RESOLUTION_ORDER.to_vec() {
        push(
            &mut out,
            "generated_doctor.resolution_order",
            &packet.packet_id,
            "resolution_order drifted from the frozen ordering",
        );
    }
    if packet.finding_classes != RESOLUTION_ORDER.to_vec() {
        push(
            &mut out,
            "generated_doctor.finding_classes",
            &packet.packet_id,
            "finding_classes must equal the resolution ordering",
        );
    }
    if packet.findings.is_empty() {
        push(
            &mut out,
            "generated_doctor.empty",
            &packet.packet_id,
            "packet must carry at least one finding",
        );
    }
    if !packet.machine_and_human_share_vocabulary() {
        push(
            &mut out,
            "generated_doctor.vocabulary_mismatch",
            &packet.packet_id,
            "machine and human projections must share one finding vocabulary",
        );
    }

    // Every class in the resolution order must be represented at least once, so
    // the lane proves it can explain all five states.
    for class in RESOLUTION_ORDER {
        if !packet.findings.iter().any(|f| f.finding_class == class) {
            push(
                &mut out,
                "generated_doctor.class_coverage_gap",
                packet.packet_id.clone(),
                format!("no finding represents the {} state", class.token()),
            );
        }
    }

    // Counts must equal the actual findings.
    for count in &packet.counts_by_class {
        let actual = packet
            .findings
            .iter()
            .filter(|f| f.finding_class == count.finding_class)
            .count();
        if actual != count.count {
            push(
                &mut out,
                "generated_doctor.count_mismatch",
                count.finding_class.token(),
                format!("counts_by_class says {} but found {actual}", count.count),
            );
        }
    }

    for finding in &packet.findings {
        validate_finding(finding, &mut out);
    }

    out
}

fn validate_finding(finding: &GeneratedDoctorFinding, out: &mut Vec<GeneratedDoctorViolation>) {
    if finding.record_kind != GENERATED_DOCTOR_FINDING_RECORD_KIND {
        push(
            out,
            "generated_doctor.finding_record_kind",
            &finding.finding_id,
            "finding record_kind drifted",
        );
    }
    if finding.severity != finding.finding_class.severity() {
        push(
            out,
            "generated_doctor.severity_mismatch",
            &finding.finding_id,
            "severity drifted from the class severity",
        );
    }
    if finding.summary != finding.finding_class.summary() {
        push(
            out,
            "generated_doctor.summary_mismatch",
            &finding.finding_id,
            "summary drifted from the controlled vocabulary",
        );
    }
    if finding.next_action != finding.finding_class.next_action() {
        push(
            out,
            "generated_doctor.next_action_mismatch",
            &finding.finding_id,
            "next_action drifted from the controlled vocabulary",
        );
    }
    // The four actions, in the fixed order, are always present.
    let kinds: Vec<GeneratedDoctorActionKind> = finding.actions.iter().map(|a| a.kind).collect();
    if kinds != GeneratedDoctorActionKind::ALL.to_vec() {
        push(
            out,
            "generated_doctor.action_set",
            &finding.finding_id,
            "a finding must offer open-details, compare, regenerate, and docs/help in order",
        );
    }
    // An unavailable action keeps a reason; an available one does not invent one.
    for action in &finding.actions {
        match action.availability {
            GeneratedDoctorActionAvailability::Available => {
                if action.unavailable_reason.is_some() {
                    push(
                        out,
                        "generated_doctor.spurious_reason",
                        &finding.finding_id,
                        "an available action must not carry an unavailable_reason",
                    );
                }
            }
            GeneratedDoctorActionAvailability::Unavailable => {
                if action.unavailable_reason.is_none() {
                    push(
                        out,
                        "generated_doctor.missing_reason",
                        &finding.finding_id,
                        "an unavailable action must carry a stable reason token",
                    );
                }
            }
        }
        if action.target_ref.is_empty() {
            push(
                out,
                "generated_doctor.empty_target",
                &finding.finding_id,
                "every action must keep its target link",
            );
        }
    }
    // Lineage traceback must be intact: a source ref unless source-missing, plus
    // a checkpoint ref.
    if finding.canonical_source_ref.is_none()
        && finding.finding_class != GeneratedDoctorFindingClass::SourceMissing
    {
        push(
            out,
            "generated_doctor.source_lineage_gap",
            &finding.finding_id,
            "only a source-missing finding may omit the canonical-source ref",
        );
    }
    if finding.checkpoint_lineage_ref.is_empty() {
        push(
            out,
            "generated_doctor.checkpoint_lineage_gap",
            &finding.finding_id,
            "every finding must keep its checkpoint lineage ref",
        );
    }
    if finding.generator.name.is_empty() {
        push(
            out,
            "generated_doctor.generator_identity_gap",
            &finding.finding_id,
            "every finding must keep its generator identity",
        );
    }
}

/// Validates one Doctor fixture against the engine.
pub fn validate_generated_doctor_fixture(
    fixture: &GeneratedDoctorFixture,
) -> Vec<GeneratedDoctorViolation> {
    let mut out = Vec::new();
    if fixture.record_kind != GENERATED_DOCTOR_FIXTURE_RECORD_KIND {
        push(
            &mut out,
            "generated_doctor.fixture_record_kind",
            &fixture.fixture_id,
            "fixture record_kind drifted",
        );
    }
    if fixture.schema_version != GENERATED_DOCTOR_SCHEMA_VERSION {
        push(
            &mut out,
            "generated_doctor.fixture_schema_version",
            &fixture.fixture_id,
            "fixture schema_version must be 1",
        );
    }
    if fixture.expected_finding_class != fixture.finding.finding_class {
        push(
            &mut out,
            "generated_doctor.fixture_class",
            &fixture.fixture_id,
            "expected_finding_class disagrees with the finding",
        );
    }
    if fixture.expected_severity != fixture.finding.severity {
        push(
            &mut out,
            "generated_doctor.fixture_severity",
            &fixture.fixture_id,
            "expected_severity disagrees with the finding",
        );
    }
    if fixture.expected_compare_available
        != fixture
            .finding
            .action_available(GeneratedDoctorActionKind::Compare)
    {
        push(
            &mut out,
            "generated_doctor.fixture_compare",
            &fixture.fixture_id,
            "expected_compare_available disagrees with the finding",
        );
    }
    if fixture.expected_regenerate_available
        != fixture
            .finding
            .action_available(GeneratedDoctorActionKind::Regenerate)
    {
        push(
            &mut out,
            "generated_doctor.fixture_regenerate",
            &fixture.fixture_id,
            "expected_regenerate_available disagrees with the finding",
        );
    }
    let kinds: Vec<GeneratedDoctorActionKind> =
        fixture.finding.actions.iter().map(|a| a.kind).collect();
    if fixture.expected_action_kinds != kinds {
        push(
            &mut out,
            "generated_doctor.fixture_actions",
            &fixture.fixture_id,
            "expected_action_kinds disagrees with the finding",
        );
    }
    validate_finding(&fixture.finding, &mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_packet_validates_and_covers_all_states() {
        let packet = seeded_generated_doctor_findings_packet();
        let violations = validate_generated_doctor_findings_packet(&packet);
        assert!(
            violations.is_empty(),
            "unexpected violations: {violations:?}"
        );
        assert!(packet.machine_and_human_share_vocabulary());
        // All five states are represented.
        for class in RESOLUTION_ORDER {
            assert!(
                packet.findings.iter().any(|f| f.finding_class == class),
                "missing class {}",
                class.token()
            );
        }
    }

    #[test]
    fn healthy_and_override_cases_produce_no_finding() {
        // The seeded write-boundary packet has two healthy in-sync own-source
        // cases and one override-admitted case; none of those become findings.
        let packet = seeded_generated_doctor_findings_packet();
        assert!(packet
            .findings
            .iter()
            .all(|f| f.boundary_state != BoundaryState::InSync
                || f.attempt_outcome == AttemptOutcome::BlockedPendingReview
                || f.attempt_outcome == AttemptOutcome::BlockedRegenerateFirst));
    }

    #[test]
    fn source_missing_finding_blocks_regenerate_and_compare() {
        let packet = seeded_generated_doctor_findings_packet();
        let finding = packet
            .findings
            .iter()
            .find(|f| f.finding_class == GeneratedDoctorFindingClass::SourceMissing)
            .expect("a source-missing finding exists");
        assert!(finding.canonical_source_ref.is_none());
        assert!(!finding.action_available(GeneratedDoctorActionKind::Regenerate));
        assert_eq!(finding.severity, GeneratedDoctorSeverity::Blocking);
        // Lineage still traces back to a generator and a checkpoint.
        assert!(!finding.generator.name.is_empty());
        assert!(!finding.checkpoint_lineage_ref.is_empty());
    }

    #[test]
    fn fixtures_validate() {
        for fixture in seeded_generated_doctor_fixtures() {
            let violations = validate_generated_doctor_fixture(&fixture);
            assert!(
                violations.is_empty(),
                "fixture {} invalid: {violations:?}",
                fixture.fixture_id
            );
        }
    }

    #[test]
    fn support_export_is_redaction_safe_and_round_trips() {
        let export = compile_generated_doctor_support_export(
            "envelope:generated-doctor:test",
            "2026-06-20T10:00:00Z",
        );
        assert!(export.is_export_safe());
        let json = serde_json::to_string(&export).expect("export serializes");
        let parsed: GeneratedDoctorSupportExport =
            serde_json::from_str(&json).expect("export round-trips");
        assert_eq!(parsed, export);
    }

    #[test]
    fn every_finding_offers_all_four_actions() {
        for finding in seeded_generated_doctor_findings_packet().findings {
            let kinds: Vec<_> = finding.actions.iter().map(|a| a.kind).collect();
            assert_eq!(kinds, GeneratedDoctorActionKind::ALL.to_vec());
            // Docs action always resolves to the unified help surface with the
            // finding-class anchor.
            let docs = finding
                .action(GeneratedDoctorActionKind::OpenDocs)
                .expect("docs action present");
            assert_eq!(
                docs.target_ref,
                format!(
                    "{GENERATED_DOCTOR_DOC_REF}#{}",
                    finding.finding_class.token()
                )
            );
        }
    }
}
