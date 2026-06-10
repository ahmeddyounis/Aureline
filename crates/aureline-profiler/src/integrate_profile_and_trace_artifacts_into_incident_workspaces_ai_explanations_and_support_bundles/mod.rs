//! Integrate profile and trace artifacts into incident workspaces, AI explanations,
//! and support bundles.
//!
//! This module materializes the typed records that keep profile and trace artifacts
//! attributable when they cross into incident workspaces, AI explanation surfaces,
//! and support-bundle exports. The records and closed vocabularies here mirror the
//! boundary schema at
//! `/schemas/perf/integrate-profile-and-trace-artifacts-into-incident-workspaces-ai-explanations-and-support-bundles.schema.json`
//! and reuse the capture-class, provenance, mapping-quality, and environment-identity
//! axes already frozen in `/docs/performance/profiling_trace_replay_contract.md`.
//!
//! The module exposes:
//!
//! - the [`IncidentWorkspaceAttachmentRow`] record that binds a profile or trace
//!   artifact to an incident workspace, preserving build identity, environment
//!   fingerprint, capture mode, mapping quality, and freshness so the workspace
//!   always knows what evidence it is holding;
//! - the [`AiExplanationRow`] record that binds an AI-generated explanation to a
//!   profile or trace artifact, carrying confidence, comparison basis, and provenance
//!   so explanation claims remain honest about what they are derived from;
//! - the [`SupportBundleInclusionRow`] record that defines how a profile or trace
//!   artifact is included in a support bundle, carrying inclusion kind, redaction
//!   profile, and export posture so bundle contents are never opaque;
//! - the [`IntegrateProfileTraceQualificationPacket`] checked-in artifact that
//!   downstream docs, help, support, and CI surfaces ingest instead of cloning
//!   status text.
//!
//! Raw payload bytes, raw command lines, secrets, and ambient credentials MUST NOT
//! appear on any record carried here.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::build_the_regression_baseline_store_baseline_selection_ux_and_comparable_environment_guards::BaselineFreshness;

/// Schema version stamped on every integration qualification packet carried by this
/// module. Bumped only on breaking payload changes; additive-optional fields do not
/// bump this value.
pub const INTEGRATE_PROFILE_TRACE_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`IntegrateProfileTraceQualificationPacket`].
pub const INTEGRATE_PROFILE_TRACE_QUALIFICATION_RECORD_KIND: &str =
    "integrate_profile_and_trace_artifacts_into_incident_workspaces_ai_explanations_and_support_bundles";

/// Repo-relative path to the checked-in integration qualification packet JSON.
pub const INTEGRATE_PROFILE_TRACE_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/perf/m5/integrate-profile-and-trace-artifacts-into-incident-workspaces-ai-explanations-and-support-bundles.json";

/// Embedded checked-in qualification packet JSON.
pub const INTEGRATE_PROFILE_TRACE_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/perf/m5/integrate-profile-and-trace-artifacts-into-incident-workspaces-ai-explanations-and-support-bundles.json"
));

/// Qualification label shown on promoted integration surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegrateProfileTraceQualificationLabel {
    /// Surface has current proof and may be called stable for its declared scope.
    Stable,
    /// Surface is visible but below stable.
    Preview,
    /// Surface is an experiment or internal lab.
    Labs,
    /// Surface may inspect metadata but must not execute or export live data.
    InspectOnly,
    /// Surface may import or view captured files only.
    ImportOnly,
}

impl IntegrateProfileTraceQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Integration surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegrateProfileTraceSurfaceKind {
    /// Incident workspace attachment surface.
    IncidentWorkspaceAttachment,
    /// AI explanation surface for profile and trace artifacts.
    AiExplanation,
    /// Support bundle inclusion surface.
    SupportBundleInclusion,
    /// Export review surface for integrated evidence.
    ExportReview,
    /// Cross-reference integrity surface.
    CrossReferenceIntegrity,
}

/// Kind of artifact that can be integrated across incident workspaces, AI explanations,
/// and support bundles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactAttachmentKind {
    /// CPU or time profile.
    Profile,
    /// Execution trace.
    Trace,
    /// Replay or time-travel timeline.
    ReplayTimeline,
    /// Memory snapshot or heap sample.
    MemorySnapshot,
    /// Code coverage data.
    Coverage,
    /// Test execution result.
    TestResult,
    /// Debug session capture.
    DebugSession,
    /// Notebook cell or output artifact.
    NotebookOutput,
}

/// Kind of AI explanation generated for a profile or trace artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiExplanationKind {
    /// High-level summary of the artifact.
    Summary,
    /// Explanation focused on hotspot or bottleneck analysis.
    HotspotExplanation,
    /// Explanation focused on regression or baseline comparison.
    RegressionExplanation,
    /// Narrative walkthrough of a trace.
    TraceNarrative,
    /// Narrative comparison between two profiles or traces.
    ComparisonNarrative,
    /// Explanation of anomalous behavior detected in the artifact.
    AnomalyExplanation,
}

/// Kind of support bundle inclusion for a profile or trace artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportBundleInclusionKind {
    /// Full artifact bytes included in the bundle.
    FullArtifact,
    /// Metadata-only inclusion with reference to the original artifact.
    MetadataOnly,
    /// Redacted summary suitable for external sharing.
    RedactedSummary,
    /// Reference-only inclusion pointing to an external store.
    ReferenceOnly,
}

/// Confidence level for an AI explanation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplanationConfidence {
    /// High confidence explanation derived from strong mapping and complete data.
    High,
    /// Medium confidence explanation with partial or inferred data.
    Medium,
    /// Low confidence explanation with significant uncertainty.
    Low,
    /// Confidence cannot be determined.
    Uncertain,
}

impl ExplanationConfidence {
    /// Returns true when the confidence is high enough to be trustworthy.
    pub const fn is_trustworthy(self) -> bool {
        matches!(self, Self::High | Self::Medium)
    }
}

/// One incident workspace attachment row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentWorkspaceAttachmentRow {
    /// Stable attachment row id.
    pub attachment_id: String,
    /// Human-readable title.
    pub title: String,
    /// Kind of artifact being attached.
    pub artifact_kind: ArtifactAttachmentKind,
    /// Incident workspace ref.
    pub incident_workspace_ref: String,
    /// Artifact ref.
    pub artifact_ref: String,
    /// Lineage ref.
    pub lineage_ref: String,
    /// Build identity ref.
    pub build_identity_ref: String,
    /// Environment fingerprint ref.
    pub environment_fingerprint_ref: String,
    /// Capture mode ref.
    pub capture_mode_ref: String,
    /// Mapping quality ref.
    pub mapping_quality_ref: String,
    /// Freshness state.
    pub freshness: BaselineFreshness,
    /// True when the attachment row is present in the promoted build.
    pub promoted_build_surface: bool,
    /// True when the attachment row shows the artifact origin.
    pub shows_artifact_origin: bool,
    /// True when the attachment row shows the build identity.
    pub shows_build_identity: bool,
    /// True when the attachment row shows the environment fingerprint.
    pub shows_environment_fingerprint: bool,
    /// True when the attachment row shows the capture mode.
    pub shows_capture_mode: bool,
    /// True when the attachment row shows the mapping quality.
    pub shows_mapping_quality: bool,
    /// True when the attachment row shows the freshness state.
    pub shows_freshness: bool,
    /// True when the attachment row shows the incident workspace link.
    pub shows_incident_workspace_link: bool,
}

/// One AI explanation row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiExplanationRow {
    /// Stable explanation row id.
    pub explanation_id: String,
    /// Human-readable title.
    pub title: String,
    /// Kind of explanation.
    pub explanation_kind: AiExplanationKind,
    /// Artifact ref.
    pub artifact_ref: String,
    /// Lineage ref.
    pub lineage_ref: String,
    /// Confidence level.
    pub confidence: ExplanationConfidence,
    /// Build identity ref.
    pub build_identity_ref: String,
    /// Environment fingerprint ref.
    pub environment_fingerprint_ref: String,
    /// Capture mode ref.
    pub capture_mode_ref: String,
    /// Mapping quality ref.
    pub mapping_quality_ref: String,
    /// Comparison basis ref.
    pub comparison_basis_ref: String,
    /// True when the explanation row is present in the promoted build.
    pub promoted_build_surface: bool,
    /// True when the explanation row shows the confidence level.
    pub shows_confidence: bool,
    /// True when the explanation row shows the artifact origin.
    pub shows_artifact_origin: bool,
    /// True when the explanation row shows the build identity.
    pub shows_build_identity: bool,
    /// True when the explanation row shows the environment fingerprint.
    pub shows_environment_fingerprint: bool,
    /// True when the explanation row shows the capture mode.
    pub shows_capture_mode: bool,
    /// True when the explanation row shows the mapping quality.
    pub shows_mapping_quality: bool,
    /// True when the explanation row shows the comparison basis.
    pub shows_comparison_basis: bool,
}

/// One support bundle inclusion row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportBundleInclusionRow {
    /// Stable inclusion row id.
    pub inclusion_id: String,
    /// Human-readable title.
    pub title: String,
    /// Kind of inclusion.
    pub inclusion_kind: SupportBundleInclusionKind,
    /// Artifact ref.
    pub artifact_ref: String,
    /// Lineage ref.
    pub lineage_ref: String,
    /// Bundle ref.
    pub bundle_ref: String,
    /// Build identity ref.
    pub build_identity_ref: String,
    /// Environment fingerprint ref.
    pub environment_fingerprint_ref: String,
    /// Capture mode ref.
    pub capture_mode_ref: String,
    /// Mapping quality ref.
    pub mapping_quality_ref: String,
    /// Redaction profile ref.
    pub redaction_profile_ref: String,
    /// Export posture ref.
    pub export_posture_ref: String,
    /// True when the inclusion row is present in the promoted build.
    pub promoted_build_surface: bool,
    /// True when the inclusion row shows the inclusion kind.
    pub shows_inclusion_kind: bool,
    /// True when the inclusion row shows the artifact origin.
    pub shows_artifact_origin: bool,
    /// True when the inclusion row shows the build identity.
    pub shows_build_identity: bool,
    /// True when the inclusion row shows the environment fingerprint.
    pub shows_environment_fingerprint: bool,
    /// True when the inclusion row shows the capture mode.
    pub shows_capture_mode: bool,
    /// True when the inclusion row shows the mapping quality.
    pub shows_mapping_quality: bool,
    /// True when the inclusion row shows the redaction profile.
    pub shows_redaction_profile: bool,
    /// True when the inclusion row shows the export posture.
    pub shows_export_posture: bool,
}

/// Checked-in proof bundle for one surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegrateProfileTraceQualificationProof {
    /// Packet id.
    pub packet_id: String,
    /// Packet ref path.
    pub packet_ref: String,
    /// Proof index ref path.
    pub proof_index_ref: String,
    /// Captured-at timestamp.
    pub captured_at: String,
    /// Evidence refs.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

/// Summary projected onto help, release, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegrateProfileTraceQualificationSummary {
    /// Total number of incident workspace attachment rows.
    pub incident_workspace_attachment_count: usize,
    /// Total number of AI explanation rows.
    pub ai_explanation_count: usize,
    /// Total number of support bundle inclusion rows.
    pub support_bundle_inclusion_count: usize,
    /// Number of rows claiming stable.
    pub stable_count: usize,
    /// Number of rows below stable.
    pub below_stable_count: usize,
    /// True when every row has a non-empty disclosure ref if below stable.
    pub all_below_stable_have_disclosure: bool,
    /// Number of attachment rows that are usable.
    pub usable_attachment_count: usize,
    /// Number of explanation rows that show confidence and comparison basis.
    pub honest_explanation_count: usize,
    /// Number of inclusion rows that show inclusion kind, redaction profile, and export posture.
    pub honest_inclusion_count: usize,
}

/// Guard set for an integration surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegrateProfileTraceSurfaceGuardSet {
    /// Artifact origin is visible.
    pub artifact_origin_visible: bool,
    /// Build identity is visible.
    pub build_identity_visible: bool,
    /// Mapping quality is visible.
    pub mapping_quality_visible: bool,
    /// Comparison basis is visible.
    pub comparison_basis_visible: bool,
    /// Export posture is visible.
    pub export_posture_visible: bool,
    /// Incident workspace link is visible.
    pub incident_workspace_link_visible: bool,
    /// AI explanation is visible.
    pub ai_explanation_visible: bool,
    /// Support bundle link is visible.
    pub support_bundle_link_visible: bool,
    /// Degraded-state label is visible when applicable.
    pub degraded_state_label_visible: bool,
}

/// One surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegrateProfileTraceSurfaceQualificationRow {
    /// Surface id.
    pub surface_id: String,
    /// Surface title.
    pub title: String,
    /// Surface kind.
    pub surface_kind: IntegrateProfileTraceSurfaceKind,
    /// True when the surface is present in the promoted build.
    pub promoted_build_surface: bool,
    /// Claim label.
    pub claim_label: IntegrateProfileTraceQualificationLabel,
    /// Displayed label (may differ from claim when narrowed).
    pub displayed_label: String,
    /// Qualification proof bundle.
    pub qualification_packet: IntegrateProfileTraceQualificationProof,
    /// Guard set.
    pub guards: IntegrateProfileTraceSurfaceGuardSet,
    /// True when the surface downgrades if required guards are missing.
    pub downgrade_if_missing: bool,
    /// Rationale string.
    pub rationale: String,
}

/// The checked-in integration qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegrateProfileTraceQualificationPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record kind.
    pub record_kind: String,
    /// Packet id.
    pub packet_id: String,
    /// As-of timestamp.
    pub as_of: String,
    /// Release doc ref.
    pub release_doc_ref: String,
    /// Help doc ref.
    pub help_doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Surface qualification rows.
    pub surfaces: Vec<IntegrateProfileTraceSurfaceQualificationRow>,
    /// Incident workspace attachment rows.
    pub incident_workspace_attachments: Vec<IncidentWorkspaceAttachmentRow>,
    /// AI explanation rows.
    pub ai_explanations: Vec<AiExplanationRow>,
    /// Support bundle inclusion rows.
    pub support_bundle_inclusions: Vec<SupportBundleInclusionRow>,
    /// Summary.
    pub summary: IntegrateProfileTraceQualificationSummary,
}

impl IntegrateProfileTraceQualificationPacket {
    /// Computes the summary from current rows.
    pub fn computed_summary(&self) -> IntegrateProfileTraceQualificationSummary {
        let stable_count = self
            .surfaces
            .iter()
            .filter(|s| s.claim_label.is_stable())
            .count();
        let below_stable_count = self.surfaces.len().saturating_sub(stable_count);
        let all_below_stable_have_disclosure = self
            .surfaces
            .iter()
            .filter(|s| !s.claim_label.is_stable())
            .all(|s| !s.rationale.is_empty());
        let usable_attachment_count = self
            .incident_workspace_attachments
            .iter()
            .filter(|a| a.freshness.is_usable())
            .count();
        let honest_explanation_count = self
            .ai_explanations
            .iter()
            .filter(|e| e.shows_confidence && e.shows_comparison_basis)
            .count();
        let honest_inclusion_count = self
            .support_bundle_inclusions
            .iter()
            .filter(|i| {
                i.shows_inclusion_kind && i.shows_redaction_profile && i.shows_export_posture
            })
            .count();

        IntegrateProfileTraceQualificationSummary {
            incident_workspace_attachment_count: self.incident_workspace_attachments.len(),
            ai_explanation_count: self.ai_explanations.len(),
            support_bundle_inclusion_count: self.support_bundle_inclusions.len(),
            stable_count,
            below_stable_count,
            all_below_stable_have_disclosure,
            usable_attachment_count,
            honest_explanation_count,
            honest_inclusion_count,
        }
    }

    /// Validates the packet and returns any violations.
    pub fn validate(&self) -> Vec<IntegrateProfileTraceQualificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != INTEGRATE_PROFILE_TRACE_QUALIFICATION_SCHEMA_VERSION {
            violations.push(IntegrateProfileTraceQualificationViolation::SchemaVersion {
                expected: INTEGRATE_PROFILE_TRACE_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }

        if self.record_kind != INTEGRATE_PROFILE_TRACE_QUALIFICATION_RECORD_KIND {
            violations.push(IntegrateProfileTraceQualificationViolation::RecordKind {
                expected: INTEGRATE_PROFILE_TRACE_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let mut surface_ids = BTreeSet::new();
        for surface in &self.surfaces {
            if !surface_ids.insert(surface.surface_id.clone()) {
                violations.push(IntegrateProfileTraceQualificationViolation::DuplicateId {
                    kind: IntegrateProfileTraceQualificationViolationKind::Surface,
                    id: surface.surface_id.clone(),
                });
            }
            if surface.promoted_build_surface
                && surface.claim_label.is_stable()
                && (!surface.guards.artifact_origin_visible
                    || !surface.guards.build_identity_visible
                    || !surface.guards.mapping_quality_visible
                    || !surface.guards.comparison_basis_visible
                    || !surface.guards.export_posture_visible
                    || !surface.guards.incident_workspace_link_visible
                    || !surface.guards.ai_explanation_visible
                    || !surface.guards.support_bundle_link_visible
                    || !surface.guards.degraded_state_label_visible)
            {
                violations.push(
                    IntegrateProfileTraceQualificationViolation::IncompleteGuardSet {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        let mut attachment_ids = BTreeSet::new();
        for attachment in &self.incident_workspace_attachments {
            if !attachment_ids.insert(attachment.attachment_id.clone()) {
                violations.push(IntegrateProfileTraceQualificationViolation::DuplicateId {
                    kind: IntegrateProfileTraceQualificationViolationKind::IncidentWorkspaceAttachment,
                    id: attachment.attachment_id.clone(),
                });
            }
            if attachment.attachment_id.trim().is_empty()
                || attachment.title.trim().is_empty()
                || attachment.incident_workspace_ref.trim().is_empty()
                || attachment.artifact_ref.trim().is_empty()
                || attachment.lineage_ref.trim().is_empty()
                || attachment.build_identity_ref.trim().is_empty()
                || attachment.environment_fingerprint_ref.trim().is_empty()
                || attachment.capture_mode_ref.trim().is_empty()
                || attachment.mapping_quality_ref.trim().is_empty()
            {
                violations.push(
                    IntegrateProfileTraceQualificationViolation::IncompleteIncidentWorkspaceAttachment {
                        attachment_id: attachment.attachment_id.clone(),
                    },
                );
            }
            if !attachment.shows_artifact_origin
                || !attachment.shows_build_identity
                || !attachment.shows_environment_fingerprint
                || !attachment.shows_capture_mode
                || !attachment.shows_mapping_quality
                || !attachment.shows_freshness
                || !attachment.shows_incident_workspace_link
            {
                violations.push(
                    IntegrateProfileTraceQualificationViolation::IncidentWorkspaceAttachmentMissingTruthLabels {
                        attachment_id: attachment.attachment_id.clone(),
                    },
                );
            }
        }

        let mut explanation_ids = BTreeSet::new();
        for explanation in &self.ai_explanations {
            if !explanation_ids.insert(explanation.explanation_id.clone()) {
                violations.push(IntegrateProfileTraceQualificationViolation::DuplicateId {
                    kind: IntegrateProfileTraceQualificationViolationKind::AiExplanation,
                    id: explanation.explanation_id.clone(),
                });
            }
            if explanation.explanation_id.trim().is_empty()
                || explanation.title.trim().is_empty()
                || explanation.artifact_ref.trim().is_empty()
                || explanation.lineage_ref.trim().is_empty()
                || explanation.build_identity_ref.trim().is_empty()
                || explanation.environment_fingerprint_ref.trim().is_empty()
                || explanation.capture_mode_ref.trim().is_empty()
                || explanation.mapping_quality_ref.trim().is_empty()
                || explanation.comparison_basis_ref.trim().is_empty()
            {
                violations.push(
                    IntegrateProfileTraceQualificationViolation::IncompleteAiExplanation {
                        explanation_id: explanation.explanation_id.clone(),
                    },
                );
            }
            if !explanation.shows_confidence
                || !explanation.shows_artifact_origin
                || !explanation.shows_build_identity
                || !explanation.shows_environment_fingerprint
                || !explanation.shows_capture_mode
                || !explanation.shows_mapping_quality
                || !explanation.shows_comparison_basis
            {
                violations.push(
                    IntegrateProfileTraceQualificationViolation::AiExplanationMissingTruthLabels {
                        explanation_id: explanation.explanation_id.clone(),
                    },
                );
            }
        }

        let mut inclusion_ids = BTreeSet::new();
        for inclusion in &self.support_bundle_inclusions {
            if !inclusion_ids.insert(inclusion.inclusion_id.clone()) {
                violations.push(IntegrateProfileTraceQualificationViolation::DuplicateId {
                    kind: IntegrateProfileTraceQualificationViolationKind::SupportBundleInclusion,
                    id: inclusion.inclusion_id.clone(),
                });
            }
            if inclusion.inclusion_id.trim().is_empty()
                || inclusion.title.trim().is_empty()
                || inclusion.artifact_ref.trim().is_empty()
                || inclusion.lineage_ref.trim().is_empty()
                || inclusion.bundle_ref.trim().is_empty()
                || inclusion.build_identity_ref.trim().is_empty()
                || inclusion.environment_fingerprint_ref.trim().is_empty()
                || inclusion.capture_mode_ref.trim().is_empty()
                || inclusion.mapping_quality_ref.trim().is_empty()
                || inclusion.redaction_profile_ref.trim().is_empty()
                || inclusion.export_posture_ref.trim().is_empty()
            {
                violations.push(
                    IntegrateProfileTraceQualificationViolation::IncompleteSupportBundleInclusion {
                        inclusion_id: inclusion.inclusion_id.clone(),
                    },
                );
            }
            if !inclusion.shows_inclusion_kind
                || !inclusion.shows_artifact_origin
                || !inclusion.shows_build_identity
                || !inclusion.shows_environment_fingerprint
                || !inclusion.shows_capture_mode
                || !inclusion.shows_mapping_quality
                || !inclusion.shows_redaction_profile
                || !inclusion.shows_export_posture
            {
                violations.push(
                    IntegrateProfileTraceQualificationViolation::SupportBundleInclusionMissingTruthLabels {
                        inclusion_id: inclusion.inclusion_id.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(IntegrateProfileTraceQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in integration qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_integrate_profile_trace_qualification(
) -> Result<IntegrateProfileTraceQualificationPacket, serde_json::Error> {
    serde_json::from_str(INTEGRATE_PROFILE_TRACE_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrateProfileTraceQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Incident workspace attachment rows.
    IncidentWorkspaceAttachment,
    /// AI explanation rows.
    AiExplanation,
    /// Support bundle inclusion rows.
    SupportBundleInclusion,
}

impl fmt::Display for IntegrateProfileTraceQualificationViolationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Surface => write!(f, "surface"),
            Self::IncidentWorkspaceAttachment => write!(f, "incident_workspace_attachment"),
            Self::AiExplanation => write!(f, "ai_explanation"),
            Self::SupportBundleInclusion => write!(f, "support_bundle_inclusion"),
        }
    }
}

/// Validation failure for integration qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegrateProfileTraceQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion {
        /// Expected schema version.
        expected: u32,
        /// Actual schema version.
        actual: u32,
    },
    /// Record kind does not match the model.
    RecordKind {
        /// Expected record kind.
        expected: String,
        /// Actual record kind.
        actual: String,
    },
    /// IDs must be unique inside an object family.
    DuplicateId {
        /// Kind of object family.
        kind: IntegrateProfileTraceQualificationViolationKind,
        /// Duplicate id.
        id: String,
    },
    /// A surface with a stable claim has an incomplete guard set.
    IncompleteGuardSet {
        /// Surface id.
        surface_id: String,
    },
    /// An incident workspace attachment row is incomplete.
    IncompleteIncidentWorkspaceAttachment {
        /// Attachment id.
        attachment_id: String,
    },
    /// An incident workspace attachment row must show artifact origin, build identity, environment fingerprint, capture mode, mapping quality, freshness, and incident workspace link.
    IncidentWorkspaceAttachmentMissingTruthLabels {
        /// Attachment id.
        attachment_id: String,
    },
    /// An AI explanation row is incomplete.
    IncompleteAiExplanation {
        /// Explanation id.
        explanation_id: String,
    },
    /// An AI explanation row must show confidence, artifact origin, build identity, environment fingerprint, capture mode, mapping quality, and comparison basis.
    AiExplanationMissingTruthLabels {
        /// Explanation id.
        explanation_id: String,
    },
    /// A support bundle inclusion row is incomplete.
    IncompleteSupportBundleInclusion {
        /// Inclusion id.
        inclusion_id: String,
    },
    /// A support bundle inclusion row must show inclusion kind, artifact origin, build identity, environment fingerprint, capture mode, mapping quality, redaction profile, and export posture.
    SupportBundleInclusionMissingTruthLabels {
        /// Inclusion id.
        inclusion_id: String,
    },
    /// Computed summary does not match the stored summary.
    SummaryMismatch,
}

impl fmt::Display for IntegrateProfileTraceQualificationViolation {
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
            Self::DuplicateId { kind, id } => {
                write!(f, "duplicate {kind} id: {id}")
            }
            Self::IncompleteGuardSet { surface_id } => {
                write!(
                    f,
                    "surface {surface_id} claims stable but guard set is incomplete"
                )
            }
            Self::IncompleteIncidentWorkspaceAttachment { attachment_id } => {
                write!(
                    f,
                    "incomplete incident workspace attachment row: {attachment_id}"
                )
            }
            Self::IncidentWorkspaceAttachmentMissingTruthLabels { attachment_id } => {
                write!(
                    f,
                    "incident workspace attachment row {attachment_id} must show artifact origin, build identity, environment fingerprint, capture mode, mapping quality, freshness, and incident workspace link"
                )
            }
            Self::IncompleteAiExplanation { explanation_id } => {
                write!(f, "incomplete AI explanation row: {explanation_id}")
            }
            Self::AiExplanationMissingTruthLabels { explanation_id } => {
                write!(
                    f,
                    "AI explanation row {explanation_id} must show confidence, artifact origin, build identity, environment fingerprint, capture mode, mapping quality, and comparison basis"
                )
            }
            Self::IncompleteSupportBundleInclusion { inclusion_id } => {
                write!(f, "incomplete support bundle inclusion row: {inclusion_id}")
            }
            Self::SupportBundleInclusionMissingTruthLabels { inclusion_id } => {
                write!(
                    f,
                    "support bundle inclusion row {inclusion_id} must show inclusion kind, artifact origin, build identity, environment fingerprint, capture mode, mapping quality, redaction profile, and export posture"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "computed summary does not match stored summary")
            }
        }
    }
}

impl Error for IntegrateProfileTraceQualificationViolation {}
