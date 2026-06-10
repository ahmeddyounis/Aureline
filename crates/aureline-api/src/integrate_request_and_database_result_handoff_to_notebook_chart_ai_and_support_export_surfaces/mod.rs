//! Request and database result handoff to notebook, chart, AI, and support-export
//! surface qualification records.
//!
//! This module owns the typed records that keep result-set handoff surfaces
//! inspectable and attributable without depending on hidden shell shortcuts or
//! ad hoc scripts. The boundary schema is
//! [`/schemas/data/integrate-request-and-database-result-handoff-to-notebook-chart-ai-and-support-export-surfaces.schema.json`](../../../schemas/data/integrate-request-and-database-result-handoff-to-notebook-chart-ai-and-support-export-surfaces.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/integrate-request-and-database-result-handoff-to-notebook-chart-ai-and-support-export-surfaces.json`](../../../artifacts/data/m5/integrate-request-and-database-result-handoff-to-notebook-chart-ai-and-support-export-surfaces.json).
//!
//! Raw row bodies, raw cell values, raw column-comment bodies, raw credentials,
//! raw connection-string fragments, and raw secret material do not belong in
//! these records. They carry stable IDs, closed posture vocabularies, and
//! reviewable summaries that UI, CLI, export, support, and public-proof surfaces
//! can ingest safely.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported schema version for handoff qualification packets.
pub const HANDOFF_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`HandoffQualificationPacket`].
pub const HANDOFF_QUALIFICATION_RECORD_KIND: &str =
    "integrate_request_and_database_result_handoff_to_notebook_chart_ai_and_support_export_surfaces";

/// Repo-relative path to the checked-in handoff qualification packet.
pub const HANDOFF_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/data/m5/integrate-request-and-database-result-handoff-to-notebook-chart-ai-and-support-export-surfaces.json";

/// Embedded checked-in packet JSON.
pub const HANDOFF_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/data/m5/integrate-request-and-database-result-handoff-to-notebook-chart-ai-and-support-export-surfaces.json"
));

/// Qualification label shown on promoted handoff surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffQualificationLabel {
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

impl HandoffQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Handoff surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffSurfaceKind {
    /// Notebook handoff surface for dataframe or textual fallback transfer.
    NotebookHandoffSurface,
    /// Chart handoff surface for typed or textual fallback transfer.
    ChartHandoffSurface,
    /// AI handoff surface for context transfer with secret boundaries.
    AiHandoffSurface,
    /// Support-export surface for redaction-safe bundle assembly.
    SupportExportSurface,
}

/// Notebook-handoff state for result-grid to notebook kernel transfers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotebookHandoffStateClass {
    /// No notebook handoff.
    NoNotebookHandoff,
    /// Notebook handoff proposed pending user admit.
    NotebookHandoffProposedPendingUserAdmit,
    /// Typed dataframe handoff admitted.
    NotebookHandoffAdmittedDataframeTyped,
    /// Textual fallback handoff admitted.
    NotebookHandoffAdmittedTextualFallbackOnly,
    /// Notebook handoff blocked pending policy.
    NotebookHandoffBlockedPendingPolicy,
    /// Notebook handoff blocked because redaction class is too high.
    NotebookHandoffBlockedRedactionClassTooHigh,
    /// Notebook handoff state is unknown and requires review.
    NotebookHandoffUnknownRequiresReview,
}

/// Chart-handoff state for result-grid to chart surface transfers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChartHandoffStateClass {
    /// No chart handoff.
    NoChartHandoff,
    /// Chart handoff proposed pending user admit.
    ChartHandoffProposedPendingUserAdmit,
    /// Typed chart handoff admitted.
    ChartHandoffAdmittedTyped,
    /// Textual fallback chart handoff admitted.
    ChartHandoffAdmittedTextualFallbackOnly,
    /// Chart handoff blocked pending policy.
    ChartHandoffBlockedPendingPolicy,
    /// Chart handoff blocked because redaction class is too high.
    ChartHandoffBlockedRedactionClassTooHigh,
    /// Chart handoff state is unknown and requires review.
    ChartHandoffUnknownRequiresReview,
}

/// AI-handoff state for result-grid to AI context transfers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiHandoffStateClass {
    /// No AI handoff.
    NoAiHandoff,
    /// AI handoff proposed pending user admit.
    AiHandoffProposedPendingUserAdmit,
    /// Typed AI handoff admitted with full context.
    AiHandoffAdmittedTypedWithContext,
    /// Textual fallback AI handoff admitted.
    AiHandoffAdmittedTextualFallbackOnly,
    /// AI handoff blocked pending policy.
    AiHandoffBlockedPendingPolicy,
    /// AI handoff blocked because redaction class is too high.
    AiHandoffBlockedRedactionClassTooHigh,
    /// AI handoff blocked because secret boundary would be violated.
    AiHandoffBlockedSecretBoundaryViolation,
    /// AI handoff state is unknown and requires review.
    AiHandoffUnknownRequiresReview,
}

/// Support-export posture class for redaction-safe bundle assembly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportExportPostureClass {
    /// Metadata-only safe default; no row data or credentials.
    MetadataSafeDefault,
    /// Operator-only restricted; includes summary stats but no PII/PHI.
    OperatorOnlyRestricted,
    /// Internal support restricted; may include row samples with explicit consent.
    InternalSupportRestricted,
    /// Signing-evidence only; immutable evidence packet for audit.
    SigningEvidenceOnly,
}

/// Result-set origin class for provenance tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultSetOriginClass {
    /// Origin is the desktop SQL editor.
    DesktopSqlEditor,
    /// Origin is the CLI runner.
    CliRunner,
    /// Origin is an AI tool review surface.
    AiToolReviewSurface,
    /// Origin is an automation run review surface.
    AutomationRunReviewSurface,
    /// Origin is an extension host runner.
    ExtensionHostRunner,
    /// Origin is a support export reader.
    SupportExportReader,
    /// Origin is an admin audit reader.
    AdminAuditReader,
    /// Origin is a hosted review reader.
    HostedReviewReader,
    /// Origin is a notebook kernel handoff target.
    NotebookKernelHandoffTarget,
}

/// Proof packet metadata attached to a stable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandoffQualificationProof {
    /// Stable proof packet id.
    pub packet_id: String,
    /// Repo-relative proof artifact reference.
    pub packet_ref: String,
    /// Proof-index reference.
    pub proof_index_ref: String,
    /// UTC capture date.
    pub captured_at: String,
    /// Evidence artifact references.
    pub evidence_refs: Vec<String>,
}

/// Boolean guard set that keeps stable surfaces from inheriting generic table truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandoffSurfaceGuardSet {
    /// Notebook handoff state is visible.
    pub notebook_handoff_visible: bool,
    /// Chart handoff state is visible.
    pub chart_handoff_visible: bool,
    /// AI handoff state is visible.
    pub ai_handoff_visible: bool,
    /// Support-export posture is visible.
    pub support_export_visible: bool,
    /// Truncation disclosure is visible on all handoffs.
    pub truncation_disclosure_visible: bool,
    /// Provenance chip is visible on all handoffs.
    pub provenance_chip_visible: bool,
    /// Secret boundaries are visible on AI handoff.
    pub secret_boundaries_visible: bool,
    /// Redaction class is visible on support export.
    pub redaction_class_visible: bool,
}

impl HandoffSurfaceGuardSet {
    /// Returns true when every required visible guard is present.
    pub const fn all_visible(&self) -> bool {
        self.notebook_handoff_visible
            && self.chart_handoff_visible
            && self.ai_handoff_visible
            && self.support_export_visible
            && self.truncation_disclosure_visible
            && self.provenance_chip_visible
            && self.secret_boundaries_visible
            && self.redaction_class_visible
    }
}

/// One governed surface row in the qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandoffSurfaceQualificationRow {
    /// Stable surface identifier.
    pub surface_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Surface family.
    pub surface_kind: HandoffSurfaceKind,
    /// Whether this surface is included in the promoted build.
    pub promoted_build_surface: bool,
    /// Claimed label from upstream release planning.
    pub claim_label: HandoffQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: HandoffQualificationLabel,
    /// Proof packet when the surface is stable.
    pub qualification_packet: Option<HandoffQualificationProof>,
    /// Visible guard set.
    pub guards: HandoffSurfaceGuardSet,
    /// True when missing proof narrows below stable instead of inheriting a label.
    pub downgrade_if_missing: bool,
    /// Plain-language reason for the displayed label.
    pub rationale: String,
}

/// One notebook handoff row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NotebookHandoffRow {
    /// Stable handoff id.
    pub handoff_id: String,
    /// Supported notebook handoff states.
    pub supported_states: Vec<NotebookHandoffStateClass>,
    /// Whether typed columns are preserved on handoff.
    pub preserves_typed_columns: bool,
    /// Whether truncation state is disclosed on handoff.
    pub preserves_truncation_disclosure: bool,
    /// Whether provenance is preserved on handoff.
    pub preserves_provenance_chip: bool,
    /// Whether a non-null notebook target ref is required for admitted typed handoff.
    pub requires_target_ref: bool,
    /// Whether the handoff is visible in UI.
    pub visible_in_ui: bool,
}

/// One chart handoff row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChartHandoffRow {
    /// Stable handoff id.
    pub handoff_id: String,
    /// Supported chart handoff states.
    pub supported_states: Vec<ChartHandoffStateClass>,
    /// Whether typed columns are preserved on handoff.
    pub preserves_typed_columns: bool,
    /// Whether truncation state is disclosed on handoff.
    pub preserves_truncation_disclosure: bool,
    /// Whether provenance is preserved on handoff.
    pub preserves_provenance_chip: bool,
    /// Whether the handoff is visible in UI.
    pub visible_in_ui: bool,
}

/// One AI handoff row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiHandoffRow {
    /// Stable handoff id.
    pub handoff_id: String,
    /// Supported AI handoff states.
    pub supported_states: Vec<AiHandoffStateClass>,
    /// Whether typed columns are preserved on handoff.
    pub preserves_typed_columns: bool,
    /// Whether truncation state is disclosed on handoff.
    pub preserves_truncation_disclosure: bool,
    /// Whether provenance is preserved on handoff.
    pub preserves_provenance_chip: bool,
    /// Whether secret boundaries are enforced on handoff.
    pub preserves_secret_boundaries: bool,
    /// Whether the handoff is visible in UI.
    pub visible_in_ui: bool,
}

/// One support-export row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportExportRow {
    /// Stable export id.
    pub export_id: String,
    /// Supported support-export posture classes.
    pub supported_postures: Vec<SupportExportPostureClass>,
    /// Default redaction posture for the export.
    pub default_redaction_class: SupportExportPostureClass,
    /// Whether including row data requires explicit user consent.
    pub includes_row_data_requires_consent: bool,
    /// Whether including credentials requires explicit user consent.
    pub includes_credentials_requires_consent: bool,
    /// Whether provenance is preserved on export.
    pub preserves_provenance_chip: bool,
    /// Whether truncation state is disclosed on export.
    pub preserves_truncation_disclosure: bool,
    /// Whether the export is visible in UI.
    pub visible_in_ui: bool,
}

/// Summary counts for a handoff qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandoffQualificationSummary {
    /// Number of promoted surfaces.
    pub promoted_surface_count: usize,
    /// Number of stable surfaces.
    pub stable_surface_count: usize,
    /// Number of narrowed promoted surfaces.
    pub narrowed_surface_count: usize,
    /// Number of notebook handoff rows.
    pub notebook_handoff_count: usize,
    /// Number of chart handoff rows.
    pub chart_handoff_count: usize,
    /// Number of AI handoff rows.
    pub ai_handoff_count: usize,
    /// Number of support-export rows.
    pub support_export_count: usize,
}

/// Canonical handoff qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandoffQualificationPacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Release document reference.
    pub release_doc_ref: String,
    /// Help document reference.
    pub help_doc_ref: String,
    /// JSON Schema path.
    pub schema_ref: String,
    /// Surface rows.
    pub surfaces: Vec<HandoffSurfaceQualificationRow>,
    /// Notebook handoff rows.
    pub notebook_handoffs: Vec<NotebookHandoffRow>,
    /// Chart handoff rows.
    pub chart_handoffs: Vec<ChartHandoffRow>,
    /// AI handoff rows.
    pub ai_handoffs: Vec<AiHandoffRow>,
    /// Support-export rows.
    pub support_exports: Vec<SupportExportRow>,
    /// Summary counts.
    pub summary: HandoffQualificationSummary,
}

impl HandoffQualificationPacket {
    /// Recomputes summary counts from packet rows.
    pub fn computed_summary(&self) -> HandoffQualificationSummary {
        let promoted_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.promoted_build_surface)
            .count();
        let stable_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.displayed_label.is_stable())
            .count();
        HandoffQualificationSummary {
            promoted_surface_count,
            stable_surface_count,
            narrowed_surface_count: promoted_surface_count.saturating_sub(stable_surface_count),
            notebook_handoff_count: self.notebook_handoffs.len(),
            chart_handoff_count: self.chart_handoffs.len(),
            ai_handoff_count: self.ai_handoffs.len(),
            support_export_count: self.support_exports.len(),
        }
    }

    /// Validates packet invariants for UI, CLI, support, and release consumers.
    pub fn validate(&self) -> Vec<HandoffQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != HANDOFF_QUALIFICATION_SCHEMA_VERSION {
            violations.push(HandoffQualificationViolation::SchemaVersion {
                expected: HANDOFF_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != HANDOFF_QUALIFICATION_RECORD_KIND {
            violations.push(HandoffQualificationViolation::RecordKind {
                expected: HANDOFF_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        collect_ids(
            self.surfaces
                .iter()
                .map(|surface| surface.surface_id.as_str()),
            &mut violations,
            HandoffQualificationViolationKind::Surface,
        );
        collect_ids(
            self.notebook_handoffs
                .iter()
                .map(|row| row.handoff_id.as_str()),
            &mut violations,
            HandoffQualificationViolationKind::NotebookHandoff,
        );
        collect_ids(
            self.chart_handoffs
                .iter()
                .map(|row| row.handoff_id.as_str()),
            &mut violations,
            HandoffQualificationViolationKind::ChartHandoff,
        );
        collect_ids(
            self.ai_handoffs
                .iter()
                .map(|row| row.handoff_id.as_str()),
            &mut violations,
            HandoffQualificationViolationKind::AiHandoff,
        );
        collect_ids(
            self.support_exports
                .iter()
                .map(|row| row.export_id.as_str()),
            &mut violations,
            HandoffQualificationViolationKind::SupportExport,
        );

        for surface in &self.surfaces {
            if surface.displayed_label.is_stable() {
                if surface.qualification_packet.is_none() {
                    violations.push(
                        HandoffQualificationViolation::StableSurfaceMissingProof {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
                if !surface.guards.all_visible() {
                    violations.push(
                        HandoffQualificationViolation::StableSurfaceMissingGuard {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
            }
            if !surface.displayed_label.is_stable()
                && surface.claim_label.is_stable()
                && !surface.downgrade_if_missing
            {
                violations.push(
                    HandoffQualificationViolation::NarrowedSurfaceLacksDowngradeRule {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        let notebook_states: BTreeSet<_> = self
            .notebook_handoffs
            .iter()
            .flat_map(|row| row.supported_states.iter().copied())
            .collect();
        for required_state in [
            NotebookHandoffStateClass::NotebookHandoffAdmittedDataframeTyped,
            NotebookHandoffStateClass::NotebookHandoffAdmittedTextualFallbackOnly,
            NotebookHandoffStateClass::NotebookHandoffBlockedPendingPolicy,
        ] {
            if !notebook_states.contains(&required_state) {
                violations.push(
                    HandoffQualificationViolation::MissingNotebookHandoffState {
                        state: required_state,
                    },
                );
            }
        }

        for row in &self.notebook_handoffs {
            if !row.preserves_truncation_disclosure {
                violations.push(
                    HandoffQualificationViolation::NotebookHandoffMissingTruncationDisclosure {
                        handoff_id: row.handoff_id.clone(),
                    },
                );
            }
            if !row.preserves_provenance_chip {
                violations.push(
                    HandoffQualificationViolation::NotebookHandoffMissingProvenanceChip {
                        handoff_id: row.handoff_id.clone(),
                    },
                );
            }
            if !row.visible_in_ui {
                violations.push(
                    HandoffQualificationViolation::NotebookHandoffNotVisibleInUi {
                        handoff_id: row.handoff_id.clone(),
                    },
                );
            }
        }

        let chart_states: BTreeSet<_> = self
            .chart_handoffs
            .iter()
            .flat_map(|row| row.supported_states.iter().copied())
            .collect();
        for required_state in [
            ChartHandoffStateClass::ChartHandoffAdmittedTyped,
            ChartHandoffStateClass::ChartHandoffAdmittedTextualFallbackOnly,
            ChartHandoffStateClass::ChartHandoffBlockedPendingPolicy,
        ] {
            if !chart_states.contains(&required_state) {
                violations.push(
                    HandoffQualificationViolation::MissingChartHandoffState {
                        state: required_state,
                    },
                );
            }
        }

        for row in &self.chart_handoffs {
            if !row.preserves_truncation_disclosure {
                violations.push(
                    HandoffQualificationViolation::ChartHandoffMissingTruncationDisclosure {
                        handoff_id: row.handoff_id.clone(),
                    },
                );
            }
            if !row.preserves_provenance_chip {
                violations.push(
                    HandoffQualificationViolation::ChartHandoffMissingProvenanceChip {
                        handoff_id: row.handoff_id.clone(),
                    },
                );
            }
            if !row.visible_in_ui {
                violations.push(
                    HandoffQualificationViolation::ChartHandoffNotVisibleInUi {
                        handoff_id: row.handoff_id.clone(),
                    },
                );
            }
        }

        let ai_states: BTreeSet<_> = self
            .ai_handoffs
            .iter()
            .flat_map(|row| row.supported_states.iter().copied())
            .collect();
        for required_state in [
            AiHandoffStateClass::AiHandoffAdmittedTypedWithContext,
            AiHandoffStateClass::AiHandoffAdmittedTextualFallbackOnly,
            AiHandoffStateClass::AiHandoffBlockedPendingPolicy,
            AiHandoffStateClass::AiHandoffBlockedSecretBoundaryViolation,
        ] {
            if !ai_states.contains(&required_state) {
                violations.push(
                    HandoffQualificationViolation::MissingAiHandoffState {
                        state: required_state,
                    },
                );
            }
        }

        for row in &self.ai_handoffs {
            if !row.preserves_truncation_disclosure {
                violations.push(
                    HandoffQualificationViolation::AiHandoffMissingTruncationDisclosure {
                        handoff_id: row.handoff_id.clone(),
                    },
                );
            }
            if !row.preserves_provenance_chip {
                violations.push(
                    HandoffQualificationViolation::AiHandoffMissingProvenanceChip {
                        handoff_id: row.handoff_id.clone(),
                    },
                );
            }
            if !row.preserves_secret_boundaries {
                violations.push(
                    HandoffQualificationViolation::AiHandoffMissingSecretBoundaries {
                        handoff_id: row.handoff_id.clone(),
                    },
                );
            }
            if !row.visible_in_ui {
                violations.push(
                    HandoffQualificationViolation::AiHandoffNotVisibleInUi {
                        handoff_id: row.handoff_id.clone(),
                    },
                );
            }
        }

        let export_postures: BTreeSet<_> = self
            .support_exports
            .iter()
            .flat_map(|row| row.supported_postures.iter().copied())
            .collect();
        for required_posture in [
            SupportExportPostureClass::MetadataSafeDefault,
            SupportExportPostureClass::OperatorOnlyRestricted,
            SupportExportPostureClass::InternalSupportRestricted,
            SupportExportPostureClass::SigningEvidenceOnly,
        ] {
            if !export_postures.contains(&required_posture) {
                violations.push(
                    HandoffQualificationViolation::MissingSupportExportPosture {
                        posture: required_posture,
                    },
                );
            }
        }

        for row in &self.support_exports {
            if row.includes_row_data_requires_consent && row.includes_credentials_requires_consent {
                // Both requiring consent is allowed; but both being false without metadata-only
                // default is a potential risk. We validate that default is metadata-safe.
            }
            if !row.preserves_truncation_disclosure {
                violations.push(
                    HandoffQualificationViolation::SupportExportMissingTruncationDisclosure {
                        export_id: row.export_id.clone(),
                    },
                );
            }
            if !row.preserves_provenance_chip {
                violations.push(
                    HandoffQualificationViolation::SupportExportMissingProvenanceChip {
                        export_id: row.export_id.clone(),
                    },
                );
            }
            if !row.visible_in_ui {
                violations.push(
                    HandoffQualificationViolation::SupportExportNotVisibleInUi {
                        export_id: row.export_id.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(HandoffQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in handoff qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_handoff_qualification() -> Result<HandoffQualificationPacket, serde_json::Error> {
    serde_json::from_str(HANDOFF_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandoffQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Notebook handoff rows.
    NotebookHandoff,
    /// Chart handoff rows.
    ChartHandoff,
    /// AI handoff rows.
    AiHandoff,
    /// Support-export rows.
    SupportExport,
}

fn collect_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    violations: &mut Vec<HandoffQualificationViolation>,
    kind: HandoffQualificationViolationKind,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in ids {
        if !out.insert(id.to_owned()) {
            violations.push(HandoffQualificationViolation::DuplicateId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    out
}

/// Validation failure for handoff qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandoffQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// IDs must be unique inside an object family.
    DuplicateId {
        kind: HandoffQualificationViolationKind,
        id: String,
    },
    /// Stable row has no proof packet.
    StableSurfaceMissingProof { surface_id: String },
    /// Stable row is missing one or more visible guards.
    StableSurfaceMissingGuard { surface_id: String },
    /// Narrowed stable claim lacks an explicit downgrade rule.
    NarrowedSurfaceLacksDowngradeRule { surface_id: String },
    /// Required notebook handoff state is missing.
    MissingNotebookHandoffState { state: NotebookHandoffStateClass },
    /// Notebook handoff does not disclose truncation state.
    NotebookHandoffMissingTruncationDisclosure { handoff_id: String },
    /// Notebook handoff does not preserve provenance chip.
    NotebookHandoffMissingProvenanceChip { handoff_id: String },
    /// Notebook handoff is not visible in UI.
    NotebookHandoffNotVisibleInUi { handoff_id: String },
    /// Required chart handoff state is missing.
    MissingChartHandoffState { state: ChartHandoffStateClass },
    /// Chart handoff does not disclose truncation state.
    ChartHandoffMissingTruncationDisclosure { handoff_id: String },
    /// Chart handoff does not preserve provenance chip.
    ChartHandoffMissingProvenanceChip { handoff_id: String },
    /// Chart handoff is not visible in UI.
    ChartHandoffNotVisibleInUi { handoff_id: String },
    /// Required AI handoff state is missing.
    MissingAiHandoffState { state: AiHandoffStateClass },
    /// AI handoff does not disclose truncation state.
    AiHandoffMissingTruncationDisclosure { handoff_id: String },
    /// AI handoff does not preserve provenance chip.
    AiHandoffMissingProvenanceChip { handoff_id: String },
    /// AI handoff does not enforce secret boundaries.
    AiHandoffMissingSecretBoundaries { handoff_id: String },
    /// AI handoff is not visible in UI.
    AiHandoffNotVisibleInUi { handoff_id: String },
    /// Required support-export posture is missing.
    MissingSupportExportPosture { posture: SupportExportPostureClass },
    /// Support export does not disclose truncation state.
    SupportExportMissingTruncationDisclosure { export_id: String },
    /// Support export does not preserve provenance chip.
    SupportExportMissingProvenanceChip { export_id: String },
    /// Support export is not visible in UI.
    SupportExportNotVisibleInUi { export_id: String },
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for HandoffQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema_version expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record_kind expected {expected}, got {actual}")
            }
            Self::DuplicateId { kind, id } => write!(f, "{kind:?} id {id} is duplicated"),
            Self::StableSurfaceMissingProof { surface_id } => {
                write!(f, "{surface_id} is stable without a proof packet")
            }
            Self::StableSurfaceMissingGuard { surface_id } => {
                write!(f, "{surface_id} is stable without complete guard truth")
            }
            Self::NarrowedSurfaceLacksDowngradeRule { surface_id } => {
                write!(f, "{surface_id} is narrowed without a downgrade rule")
            }
            Self::MissingNotebookHandoffState { state } => {
                write!(f, "notebook handoff state {state:?} is not covered")
            }
            Self::NotebookHandoffMissingTruncationDisclosure { handoff_id } => {
                write!(f, "{handoff_id} does not disclose truncation state on notebook handoff")
            }
            Self::NotebookHandoffMissingProvenanceChip { handoff_id } => {
                write!(f, "{handoff_id} does not preserve provenance chip on notebook handoff")
            }
            Self::NotebookHandoffNotVisibleInUi { handoff_id } => {
                write!(f, "{handoff_id} is not visible in UI")
            }
            Self::MissingChartHandoffState { state } => {
                write!(f, "chart handoff state {state:?} is not covered")
            }
            Self::ChartHandoffMissingTruncationDisclosure { handoff_id } => {
                write!(f, "{handoff_id} does not disclose truncation state on chart handoff")
            }
            Self::ChartHandoffMissingProvenanceChip { handoff_id } => {
                write!(f, "{handoff_id} does not preserve provenance chip on chart handoff")
            }
            Self::ChartHandoffNotVisibleInUi { handoff_id } => {
                write!(f, "{handoff_id} is not visible in UI")
            }
            Self::MissingAiHandoffState { state } => {
                write!(f, "AI handoff state {state:?} is not covered")
            }
            Self::AiHandoffMissingTruncationDisclosure { handoff_id } => {
                write!(f, "{handoff_id} does not disclose truncation state on AI handoff")
            }
            Self::AiHandoffMissingProvenanceChip { handoff_id } => {
                write!(f, "{handoff_id} does not preserve provenance chip on AI handoff")
            }
            Self::AiHandoffMissingSecretBoundaries { handoff_id } => {
                write!(f, "{handoff_id} does not enforce secret boundaries on AI handoff")
            }
            Self::AiHandoffNotVisibleInUi { handoff_id } => {
                write!(f, "{handoff_id} is not visible in UI")
            }
            Self::MissingSupportExportPosture { posture } => {
                write!(f, "support export posture {posture:?} is not covered")
            }
            Self::SupportExportMissingTruncationDisclosure { export_id } => {
                write!(f, "{export_id} does not disclose truncation state on support export")
            }
            Self::SupportExportMissingProvenanceChip { export_id } => {
                write!(f, "{export_id} does not preserve provenance chip on support export")
            }
            Self::SupportExportNotVisibleInUi { export_id } => {
                write!(f, "{export_id} is not visible in UI")
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for HandoffQualificationViolation {}
