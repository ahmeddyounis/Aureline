//! Certification qualification records for API, database, and browser-runtime
//! workflows with mutation, redaction, and scale drills.
//!
//! This module owns the typed records that certify the integration of API
//! workflows (request workspace, composer, response viewers), database
//! workflows (connection browsers, statement safety, explain plans, result
//! grids, staged row mutations, query history, handoff), and browser-runtime
//! workflows with explicit mutation drills, redaction drills, and scale
//! drills. The boundary schema is
//! [`/schemas/data/certify-api-database-and-browser-runtime-workflows-with-mutation-redaction-and-scale-drills.schema.json`](../../../schemas/data/certify-api-database-and-browser-runtime-workflows-with-mutation-redaction-and-scale-drills.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/certify-api-database-and-browser-runtime-workflows-with-mutation-redaction-and-scale-drills.json`](../../../artifacts/data/m5/certify-api-database-and-browser-runtime-workflows-with-mutation-redaction-and-scale-drills.json).
//!
//! Raw secrets, raw credentials, raw connection strings, and unbounded result
//! sets do not belong in these records. They carry stable IDs, closed posture
//! vocabularies, and reviewable summaries that UI, CLI, export, support, and
//! public-proof surfaces can ingest safely.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported schema version for certification qualification packets.
pub const CERTIFICATION_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`CertificationQualificationPacket`].
pub const CERTIFICATION_QUALIFICATION_RECORD_KIND: &str =
    "certify_api_database_and_browser_runtime_workflows_with_mutation_redaction_and_scale_drills";

/// Repo-relative path to the checked-in certification qualification packet.
pub const CERTIFICATION_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/data/m5/certify-api-database-and-browser-runtime-workflows-with-mutation-redaction-and-scale-drills.json";

/// Embedded checked-in packet JSON.
pub const CERTIFICATION_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/data/m5/certify-api-database-and-browser-runtime-workflows-with-mutation-redaction-and-scale-drills.json"
));

/// Qualification label shown on promoted certification surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationQualificationLabel {
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

impl CertificationQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Certification surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationSurfaceKind {
    /// Certification of API workflows (request workspace, composer, response viewers).
    ApiWorkflowCertification,
    /// Certification of database workflows (connection browsers, statement safety, result grids, etc.).
    DatabaseWorkflowCertification,
    /// Certification of browser-runtime workflows (trust classes, timing tabs, response previews).
    BrowserRuntimeCertification,
    /// Mutation drill verifying preview, confirmation, rollback, write posture, and step-up flows.
    MutationDrill,
    /// Redaction drill verifying auth source redaction, secret-safe storage, export redaction, etc.
    RedactionDrill,
    /// Scale drill verifying row-count bounds, result-set limits, virtualization, timeout, and memory caps.
    ScaleDrill,
}

/// Mutation drill class exercised during certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationDrillClass {
    /// All mutating flows require a preview step before execution.
    PreviewRequired,
    /// All mutating flows require explicit confirmation before apply.
    ConfirmationRequired,
    /// Rollback paths are visible from all mutating surfaces.
    RollbackPathVisible,
    /// Write posture is explicit on all mutating surfaces.
    WritePostureExplicit,
    /// Protected-target step-up flows are completed for high-impact mutations.
    StepUpFlowCompleted,
}

/// Redaction drill class exercised during certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionDrillClass {
    /// Auth sources show inspectable mode and provenance without raw secrets.
    AuthSourceRedacted,
    /// Database credentials and secrets are stored with secret-safe posture.
    SecretSafeStorage,
    /// Redaction-safe export applies approved redaction classes to all exports.
    ExportRedactionApplied,
    /// Browser-runtime trust classes and response previews do not leak credentials.
    SupportBundleSafe,
    /// Query history retains metadata while redacting raw secrets and full results.
    HistoryRedacted,
}

/// Scale drill class exercised during certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaleDrillClass {
    /// Row counts are bounded and disclosed on all result grids and query outputs.
    RowCountBounded,
    /// Result sets are hard-limited before streaming to viewers or handoff surfaces.
    ResultSetLimited,
    /// Result-grid virtualization is active for large result sets.
    VirtualizationActive,
    /// API requests enforce timeout boundaries and surface timeout state.
    TimeoutEnforced,
    /// Database result sets enforce memory caps before rendering or export.
    MemoryCapEnforced,
}

/// Proof packet metadata attached to a stable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationQualificationProof {
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
pub struct CertificationSurfaceGuardSet {
    /// API workflow certification surface is visible.
    pub api_workflow_visible: bool,
    /// Database workflow certification surface is visible.
    pub database_workflow_visible: bool,
    /// Browser-runtime certification surface is visible.
    pub browser_runtime_visible: bool,
    /// Mutation drill surface is visible.
    pub mutation_drill_visible: bool,
    /// Redaction drill surface is visible.
    pub redaction_drill_visible: bool,
    /// Scale drill surface is visible.
    pub scale_drill_visible: bool,
    /// Upstream packet references are visible.
    pub upstream_packet_refs_visible: bool,
    /// Downgrade rules are visible.
    pub downgrade_rules_visible: bool,
}

impl CertificationSurfaceGuardSet {
    /// Returns true when every required visible guard is present.
    pub const fn all_visible(&self) -> bool {
        self.api_workflow_visible
            && self.database_workflow_visible
            && self.browser_runtime_visible
            && self.mutation_drill_visible
            && self.redaction_drill_visible
            && self.scale_drill_visible
            && self.upstream_packet_refs_visible
            && self.downgrade_rules_visible
    }
}

/// One governed surface row in the qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationSurfaceQualificationRow {
    /// Stable surface identifier.
    pub surface_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Surface family.
    pub surface_kind: CertificationSurfaceKind,
    /// Whether this surface is included in the promoted build.
    pub promoted_build_surface: bool,
    /// Claimed label from upstream release planning.
    pub claim_label: CertificationQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: CertificationQualificationLabel,
    /// Proof packet when the surface is stable.
    pub qualification_packet: Option<CertificationQualificationProof>,
    /// Visible guard set.
    pub guards: CertificationSurfaceGuardSet,
    /// True when missing proof narrows below stable instead of inheriting a label.
    pub downgrade_if_missing: bool,
    /// Plain-language reason for the displayed label.
    pub rationale: String,
}

/// One mutation drill row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MutationDrillRow {
    /// Stable drill id.
    pub drill_id: String,
    /// Mutation drill class.
    pub drill_class: MutationDrillClass,
    /// Target surface kind this drill applies to.
    pub target_surface_kind: CertificationSurfaceKind,
    /// Whether the drill passed.
    pub pass: bool,
    /// Human-readable rationale.
    pub rationale: String,
}

/// One redaction drill row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RedactionDrillRow {
    /// Stable drill id.
    pub drill_id: String,
    /// Redaction drill class.
    pub drill_class: RedactionDrillClass,
    /// Target surface kind this drill applies to.
    pub target_surface_kind: CertificationSurfaceKind,
    /// Whether the drill passed.
    pub pass: bool,
    /// Human-readable rationale.
    pub rationale: String,
}

/// One scale drill row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScaleDrillRow {
    /// Stable drill id.
    pub drill_id: String,
    /// Scale drill class.
    pub drill_class: ScaleDrillClass,
    /// Target surface kind this drill applies to.
    pub target_surface_kind: CertificationSurfaceKind,
    /// Whether the drill passed.
    pub pass: bool,
    /// Human-readable rationale.
    pub rationale: String,
}

/// Reference to an upstream qualification packet integrated into this certification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpstreamPacketRefRow {
    /// Stable reference id.
    pub ref_id: String,
    /// Upstream record kind.
    pub upstream_record_kind: String,
    /// Repo-relative path to the upstream packet.
    pub upstream_packet_path: String,
    /// Repo-relative path to the upstream schema.
    pub upstream_schema_path: String,
    /// Whether integration has been verified.
    pub integration_verified: bool,
    /// Human-readable rationale.
    pub rationale: String,
}

/// Summary counts for a certification qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationQualificationSummary {
    /// Number of promoted surfaces.
    pub promoted_surface_count: usize,
    /// Number of stable surfaces.
    pub stable_surface_count: usize,
    /// Number of narrowed promoted surfaces.
    pub narrowed_surface_count: usize,
    /// Number of mutation drill rows.
    pub mutation_drill_count: usize,
    /// Number of redaction drill rows.
    pub redaction_drill_count: usize,
    /// Number of scale drill rows.
    pub scale_drill_count: usize,
    /// Number of upstream packet reference rows.
    pub upstream_packet_ref_count: usize,
    /// Number of upstream integrations that passed verification.
    pub integration_pass_count: usize,
}

/// Canonical certification qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationQualificationPacket {
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
    pub surfaces: Vec<CertificationSurfaceQualificationRow>,
    /// Mutation drill rows.
    pub mutation_drills: Vec<MutationDrillRow>,
    /// Redaction drill rows.
    pub redaction_drills: Vec<RedactionDrillRow>,
    /// Scale drill rows.
    pub scale_drills: Vec<ScaleDrillRow>,
    /// Upstream packet reference rows.
    pub upstream_packet_refs: Vec<UpstreamPacketRefRow>,
    /// Summary counts.
    pub summary: CertificationQualificationSummary,
}

impl CertificationQualificationPacket {
    /// Recomputes summary counts from packet rows.
    pub fn computed_summary(&self) -> CertificationQualificationSummary {
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
        let integration_pass_count = self
            .upstream_packet_refs
            .iter()
            .filter(|ref_row| ref_row.integration_verified)
            .count();
        CertificationQualificationSummary {
            promoted_surface_count,
            stable_surface_count,
            narrowed_surface_count: promoted_surface_count.saturating_sub(stable_surface_count),
            mutation_drill_count: self.mutation_drills.len(),
            redaction_drill_count: self.redaction_drills.len(),
            scale_drill_count: self.scale_drills.len(),
            upstream_packet_ref_count: self.upstream_packet_refs.len(),
            integration_pass_count,
        }
    }

    /// Validates packet invariants for UI, CLI, support, and release consumers.
    pub fn validate(&self) -> Vec<CertificationQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != CERTIFICATION_QUALIFICATION_SCHEMA_VERSION {
            violations.push(CertificationQualificationViolation::SchemaVersion {
                expected: CERTIFICATION_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != CERTIFICATION_QUALIFICATION_RECORD_KIND {
            violations.push(CertificationQualificationViolation::RecordKind {
                expected: CERTIFICATION_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        collect_ids(
            self.surfaces
                .iter()
                .map(|surface| surface.surface_id.as_str()),
            &mut violations,
            CertificationQualificationViolationKind::Surface,
        );
        collect_ids(
            self.mutation_drills.iter().map(|row| row.drill_id.as_str()),
            &mut violations,
            CertificationQualificationViolationKind::MutationDrill,
        );
        collect_ids(
            self.redaction_drills
                .iter()
                .map(|row| row.drill_id.as_str()),
            &mut violations,
            CertificationQualificationViolationKind::RedactionDrill,
        );
        collect_ids(
            self.scale_drills.iter().map(|row| row.drill_id.as_str()),
            &mut violations,
            CertificationQualificationViolationKind::ScaleDrill,
        );
        collect_ids(
            self.upstream_packet_refs
                .iter()
                .map(|row| row.ref_id.as_str()),
            &mut violations,
            CertificationQualificationViolationKind::UpstreamPacketRef,
        );

        for surface in &self.surfaces {
            if surface.displayed_label.is_stable() {
                if surface.qualification_packet.is_none() {
                    violations.push(
                        CertificationQualificationViolation::StableSurfaceMissingProof {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
                if !surface.guards.all_visible() {
                    violations.push(
                        CertificationQualificationViolation::StableSurfaceMissingGuard {
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
                    CertificationQualificationViolation::NarrowedSurfaceLacksDowngradeRule {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        let mutation_classes: BTreeSet<_> = self
            .mutation_drills
            .iter()
            .map(|row| row.drill_class)
            .collect();
        for required_class in [
            MutationDrillClass::PreviewRequired,
            MutationDrillClass::ConfirmationRequired,
            MutationDrillClass::RollbackPathVisible,
            MutationDrillClass::WritePostureExplicit,
            MutationDrillClass::StepUpFlowCompleted,
        ] {
            if !mutation_classes.contains(&required_class) {
                violations.push(
                    CertificationQualificationViolation::MissingMutationDrillClass {
                        drill_class: required_class,
                    },
                );
            }
        }

        for row in &self.mutation_drills {
            if row.rationale.is_empty() {
                violations.push(
                    CertificationQualificationViolation::IncompleteMutationDrill {
                        drill_id: row.drill_id.clone(),
                    },
                );
            }
        }

        let redaction_classes: BTreeSet<_> = self
            .redaction_drills
            .iter()
            .map(|row| row.drill_class)
            .collect();
        for required_class in [
            RedactionDrillClass::AuthSourceRedacted,
            RedactionDrillClass::SecretSafeStorage,
            RedactionDrillClass::ExportRedactionApplied,
            RedactionDrillClass::SupportBundleSafe,
            RedactionDrillClass::HistoryRedacted,
        ] {
            if !redaction_classes.contains(&required_class) {
                violations.push(
                    CertificationQualificationViolation::MissingRedactionDrillClass {
                        drill_class: required_class,
                    },
                );
            }
        }

        for row in &self.redaction_drills {
            if row.rationale.is_empty() {
                violations.push(
                    CertificationQualificationViolation::IncompleteRedactionDrill {
                        drill_id: row.drill_id.clone(),
                    },
                );
            }
        }

        let scale_classes: BTreeSet<_> = self
            .scale_drills
            .iter()
            .map(|row| row.drill_class)
            .collect();
        for required_class in [
            ScaleDrillClass::RowCountBounded,
            ScaleDrillClass::ResultSetLimited,
            ScaleDrillClass::VirtualizationActive,
            ScaleDrillClass::TimeoutEnforced,
            ScaleDrillClass::MemoryCapEnforced,
        ] {
            if !scale_classes.contains(&required_class) {
                violations.push(
                    CertificationQualificationViolation::MissingScaleDrillClass {
                        drill_class: required_class,
                    },
                );
            }
        }

        for row in &self.scale_drills {
            if row.rationale.is_empty() {
                violations.push(CertificationQualificationViolation::IncompleteScaleDrill {
                    drill_id: row.drill_id.clone(),
                });
            }
        }

        for row in &self.upstream_packet_refs {
            if row.upstream_record_kind.is_empty()
                || row.upstream_packet_path.is_empty()
                || row.upstream_schema_path.is_empty()
            {
                violations.push(
                    CertificationQualificationViolation::IncompleteUpstreamPacketRef {
                        ref_id: row.ref_id.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(CertificationQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in certification qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_certification_qualification(
) -> Result<CertificationQualificationPacket, serde_json::Error> {
    serde_json::from_str(CERTIFICATION_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificationQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Mutation drill rows.
    MutationDrill,
    /// Redaction drill rows.
    RedactionDrill,
    /// Scale drill rows.
    ScaleDrill,
    /// Upstream packet reference rows.
    UpstreamPacketRef,
}

fn collect_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    violations: &mut Vec<CertificationQualificationViolation>,
    kind: CertificationQualificationViolationKind,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in ids {
        if !out.insert(id.to_owned()) {
            violations.push(CertificationQualificationViolation::DuplicateId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    out
}

/// Validation failure for certification qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertificationQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// IDs must be unique inside an object family.
    DuplicateId {
        kind: CertificationQualificationViolationKind,
        id: String,
    },
    /// Stable row has no proof packet.
    StableSurfaceMissingProof { surface_id: String },
    /// Stable row is missing one or more visible guards.
    StableSurfaceMissingGuard { surface_id: String },
    /// Narrowed stable claim lacks an explicit downgrade rule.
    NarrowedSurfaceLacksDowngradeRule { surface_id: String },
    /// Required mutation drill class is missing.
    MissingMutationDrillClass { drill_class: MutationDrillClass },
    /// Mutation drill is incomplete.
    IncompleteMutationDrill { drill_id: String },
    /// Required redaction drill class is missing.
    MissingRedactionDrillClass { drill_class: RedactionDrillClass },
    /// Redaction drill is incomplete.
    IncompleteRedactionDrill { drill_id: String },
    /// Required scale drill class is missing.
    MissingScaleDrillClass { drill_class: ScaleDrillClass },
    /// Scale drill is incomplete.
    IncompleteScaleDrill { drill_id: String },
    /// Upstream packet reference is incomplete.
    IncompleteUpstreamPacketRef { ref_id: String },
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for CertificationQualificationViolation {
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
            Self::MissingMutationDrillClass { drill_class } => {
                write!(f, "mutation drill class {drill_class:?} is not covered")
            }
            Self::IncompleteMutationDrill { drill_id } => {
                write!(
                    f,
                    "{drill_id} does not project mutation drill truth everywhere"
                )
            }
            Self::MissingRedactionDrillClass { drill_class } => {
                write!(f, "redaction drill class {drill_class:?} is not covered")
            }
            Self::IncompleteRedactionDrill { drill_id } => {
                write!(
                    f,
                    "{drill_id} does not project redaction drill truth everywhere"
                )
            }
            Self::MissingScaleDrillClass { drill_class } => {
                write!(f, "scale drill class {drill_class:?} is not covered")
            }
            Self::IncompleteScaleDrill { drill_id } => {
                write!(
                    f,
                    "{drill_id} does not project scale drill truth everywhere"
                )
            }
            Self::IncompleteUpstreamPacketRef { ref_id } => {
                write!(
                    f,
                    "{ref_id} does not project upstream packet reference truth everywhere"
                )
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for CertificationQualificationViolation {}
