//! Beta Project Doctor probe-pack catalog and finding contract.
//!
//! The beta lane promotes Project Doctor from the alpha probe runtime to a
//! versioned, attributable, confidence-labeled diagnosis system. It owns three
//! typed records mirroring the boundary schema at
//! `/schemas/support/project_doctor.schema.json`:
//!
//! - [`ProjectDoctorProbePackCatalog`] names the closed list of supported
//!   probe packs. Each pack is bound to a stable [`ProbePackClass`], carries a
//!   stable `pack_id` and `pack_version`, declares a [`ReadOnlyPostureClass`]
//!   posture and headless/support-guided admission classes, and enumerates the
//!   stable finding codes it may emit.
//! - [`ProjectDoctorProbePack`] wraps one pack record with a frozen
//!   schema-version and record-kind discriminator so a pack can be exchanged or
//!   stored on its own.
//! - [`ProjectDoctorBetaFinding`] is one typed finding emitted by a pack with a
//!   stable [`doctor_finding_code`], an attribution back to its pack and rule,
//!   evidence refs, a [`FindingConfidenceClass`], a
//!   [`FindingSeverityClass`], a [`RecoveryHandoffClass`], and the render
//!   surfaces (UI, CLI, headless, support export) that may carry it.
//!
//! [`ProjectDoctorBetaEvaluator`] validates the catalog and the finding shape,
//! refuses unsafe combinations (denied headless admission, unknown pack ref,
//! unknown finding code, mismatched pack class), and folds one catalog plus a
//! batch of findings into a metadata-safe [`ProjectDoctorBetaSupportPacket`]
//! the support-export pipeline can serialize verbatim.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Frozen schema version for the beta Project Doctor records.
pub const PROJECT_DOCTOR_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the probe-pack catalog record.
pub const PROJECT_DOCTOR_PROBE_PACK_CATALOG_RECORD_KIND: &str =
    "project_doctor_probe_pack_catalog_record";

/// Stable record-kind tag for a single probe-pack record.
pub const PROJECT_DOCTOR_PROBE_PACK_BETA_RECORD_KIND: &str = "project_doctor_probe_pack_record";

/// Stable record-kind tag for a beta finding record.
pub const PROJECT_DOCTOR_FINDING_BETA_RECORD_KIND: &str = "project_doctor_finding_record";

/// Stable record-kind tag for the support-export beta packet.
pub const PROJECT_DOCTOR_BETA_SUPPORT_PACKET_RECORD_KIND: &str =
    "project_doctor_beta_support_packet_record";

/// Repo-relative path of the boundary schema mirrored by this module.
pub const PROJECT_DOCTOR_BETA_SCHEMA_REF: &str = "schemas/support/project_doctor.schema.json";

/// Reviewer doc ref quoted by every emitted packet.
pub const PROJECT_DOCTOR_BETA_DOC_REF: &str = "docs/support/m3/project_doctor_beta.md";

/// Stable finding-code prefix every beta finding must use.
pub const DOCTOR_FINDING_PREFIX: &str = "doctor.finding.";

/// Closed probe-pack class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProbePackClass {
    /// Entry / open / clone-import / recent-workspace readiness pack.
    EntryOpenReadiness,
    /// Execution-context and toolchain resolution pack.
    ToolchainResolution,
    /// Search and index readiness pack.
    SearchIndexReadiness,
    /// Trust, identity, and policy pack.
    TrustPolicy,
    /// Local Git baseline pack.
    GitBaseline,
    /// Provider / credential authority pack.
    ProviderAuth,
    /// Session restore / crash replay continuity pack.
    RestoreContinuity,
    /// Crash and symbolication pack.
    CrashSymbolication,
    /// Support-bundle integrity pack.
    SupportBundleIntegrity,
}

impl ProbePackClass {
    /// Returns every probe-pack class in catalog order.
    pub const fn all() -> [Self; 9] {
        [
            Self::EntryOpenReadiness,
            Self::ToolchainResolution,
            Self::SearchIndexReadiness,
            Self::TrustPolicy,
            Self::GitBaseline,
            Self::ProviderAuth,
            Self::RestoreContinuity,
            Self::CrashSymbolication,
            Self::SupportBundleIntegrity,
        ]
    }

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EntryOpenReadiness => "entry_open_readiness",
            Self::ToolchainResolution => "toolchain_resolution",
            Self::SearchIndexReadiness => "search_index_readiness",
            Self::TrustPolicy => "trust_policy",
            Self::GitBaseline => "git_baseline",
            Self::ProviderAuth => "provider_auth",
            Self::RestoreContinuity => "restore_continuity",
            Self::CrashSymbolication => "crash_symbolication",
            Self::SupportBundleIntegrity => "support_bundle_integrity",
        }
    }
}

impl fmt::Display for ProbePackClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Closed probe-pack lifecycle status. The catalog only carries beta and
/// deprecated packs; alpha probes live in the alpha runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProbePackLifecycleStatus {
    /// Beta-grade pack admitted for headless and support-guided diagnosis.
    Beta,
    /// Deprecated pack retained for back-compat reads only.
    Deprecated,
}

impl ProbePackLifecycleStatus {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Beta => "beta",
            Self::Deprecated => "deprecated",
        }
    }
}

/// Closed read-only posture vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadOnlyPostureClass {
    /// The pack performs no mutation at all.
    ReadOnlyByDefaultNoMutation,
    /// The pack may write local evidence/preview rows only.
    MetadataLocalEvidenceOnly,
}

impl ReadOnlyPostureClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyByDefaultNoMutation => "read_only_by_default_no_mutation",
            Self::MetadataLocalEvidenceOnly => "metadata_local_evidence_only",
        }
    }
}

/// Headless admission class for a probe pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeadlessAdmissionClass {
    /// Admitted to run headless / non-interactively.
    AdmittedSafeForHeadless,
    /// Admitted only when a support-guided session attests to the run.
    AdmittedSafeForSupportGuidedOnly,
    /// Headless run is denied.
    Denied,
}

impl HeadlessAdmissionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdmittedSafeForHeadless => "admitted_safe_for_headless",
            Self::AdmittedSafeForSupportGuidedOnly => "admitted_safe_for_support_guided_only",
            Self::Denied => "denied",
        }
    }
}

/// Support-guided admission class for a probe pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportGuidedAdmissionClass {
    /// Admitted under support-guided mode.
    AdmittedInSupportGuidedMode,
    /// Support-guided run is denied.
    Denied,
}

impl SupportGuidedAdmissionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdmittedInSupportGuidedMode => "admitted_in_support_guided_mode",
            Self::Denied => "denied",
        }
    }
}

/// Closed finding severity class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverityClass {
    /// Informational finding.
    Info,
    /// Degraded but still usable.
    Degraded,
    /// Blocks the requested workflow until handled.
    Blocking,
    /// Unsupported in the current context.
    Unsupported,
}

impl FindingSeverityClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Degraded => "degraded",
            Self::Blocking => "blocking",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Closed finding confidence class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingConfidenceClass {
    /// Evidence directly proves the finding.
    ObservedAuthoritative,
    /// Evidence proves the finding but leaves a typed gap.
    ObservedWithGap,
    /// Evidence is sufficient for a bounded inference.
    InferredFromEvidence,
    /// More evidence is required before Doctor can prove the state.
    UnknownRequiresProbe,
}

impl FindingConfidenceClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ObservedAuthoritative => "observed_authoritative",
            Self::ObservedWithGap => "observed_with_gap",
            Self::InferredFromEvidence => "inferred_from_evidence",
            Self::UnknownRequiresProbe => "unknown_requires_probe",
        }
    }
}

/// Closed diagnosis posture class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosisPostureClass {
    /// The finding is proven from current evidence.
    ProvingDiagnosis,
    /// The finding is inferred and keeps remaining unknowns visible.
    InferringFromPartialEvidence,
    /// Doctor refuses to diagnose beyond supported evidence.
    RefusingUnsupported,
}

impl DiagnosisPostureClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProvingDiagnosis => "proving_diagnosis",
            Self::InferringFromPartialEvidence => "inferring_from_partial_evidence",
            Self::RefusingUnsupported => "refusing_unsupported",
        }
    }
}

/// Closed recovery handoff class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryHandoffClass {
    /// A reviewed repair path is available.
    ReviewedRepairAvailable,
    /// Doctor shows a repair preview but does not apply it.
    PreviewOnly,
    /// Doctor hands off to support or a governed external path.
    HandoffOnly,
    /// No supported local recovery exists.
    Unsupported,
}

impl RecoveryHandoffClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewedRepairAvailable => "reviewed_repair_available",
            Self::PreviewOnly => "preview_only",
            Self::HandoffOnly => "handoff_only",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Closed support-context class for beta findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportContextClass {
    /// Desktop interactive shell.
    Desktop,
    /// CLI / headless run.
    CliHeadless,
    /// Remote managed run.
    RemoteManaged,
    /// Offline local run.
    OfflineLocal,
    /// Support-guided session (live or asynchronous).
    SupportGuided,
}

impl SupportContextClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::CliHeadless => "cli_headless",
            Self::RemoteManaged => "remote_managed",
            Self::OfflineLocal => "offline_local",
            Self::SupportGuided => "support_guided",
        }
    }
}

/// Closed render-surface class. The same finding packet must render across
/// every surface listed here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenderSurfaceClass {
    /// Shell finding card / UI surface.
    UiFindingCard,
    /// Interactive CLI finding row.
    CliFindingRow,
    /// Support export row.
    SupportExportRow,
    /// Headless JSON row.
    HeadlessJsonRow,
}

impl RenderSurfaceClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UiFindingCard => "ui_finding_card",
            Self::CliFindingRow => "cli_finding_row",
            Self::SupportExportRow => "support_export_row",
            Self::HeadlessJsonRow => "headless_json_row",
        }
    }
}

/// Closed attribution-kind class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributionKindClass {
    /// Attribution back to the owning probe pack.
    ProbePackRef,
    /// Attribution back to a stable Doctor rule id.
    DoctorRuleRef,
    /// Attribution to an evidence row.
    EvidenceRef,
    /// Attribution to a support-bundle ref.
    SupportBundleRef,
    /// Attribution to a runbook ref.
    RunbookRef,
    /// Attribution to an escalation packet ref.
    EscalationPacketRef,
}

impl AttributionKindClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProbePackRef => "probe_pack_ref",
            Self::DoctorRuleRef => "doctor_rule_ref",
            Self::EvidenceRef => "evidence_ref",
            Self::SupportBundleRef => "support_bundle_ref",
            Self::RunbookRef => "runbook_ref",
            Self::EscalationPacketRef => "escalation_packet_ref",
        }
    }
}

/// Closed support-redaction class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportRedactionClass {
    /// Metadata-safe by default; raw private material excluded.
    MetadataSafeDefault,
    /// Operator-only restricted view.
    OperatorOnlyRestricted,
}

impl SupportRedactionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::OperatorOnlyRestricted => "operator_only_restricted",
        }
    }
}

/// One typed attribution row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttributionRef {
    /// Attribution kind.
    pub attribution_kind: AttributionKindClass,
    /// Opaque reference value.
    #[serde(rename = "ref")]
    pub ref_: String,
}

/// One typed probe-pack record stored inside the catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProbePackEntry {
    /// Stable pack identifier.
    pub pack_id: String,
    /// Pack class.
    pub pack_class: ProbePackClass,
    /// Pack implementation version.
    pub pack_version: String,
    /// Lifecycle status (beta or deprecated only).
    pub lifecycle_status: ProbePackLifecycleStatus,
    /// Read-only posture for the pack.
    pub read_only_posture: ReadOnlyPostureClass,
    /// Headless admission class.
    pub headless_admission: HeadlessAdmissionClass,
    /// Support-guided admission class.
    pub support_guided_admission: SupportGuidedAdmissionClass,
    /// Stable finding codes the pack may emit.
    pub supported_finding_codes: Vec<String>,
    /// Support contexts in which the pack may run.
    pub supported_support_contexts: Vec<SupportContextClass>,
    /// Recovery handoff classes the pack may surface.
    pub supported_recovery_handoffs: Vec<RecoveryHandoffClass>,
    /// Default redaction class for emitted findings.
    pub default_redaction_class: SupportRedactionClass,
    /// Reviewer-safe summary.
    pub summary: String,
}

impl ProbePackEntry {
    /// True when the pack is read-only and safe to run headlessly or under
    /// support-guided sessions.
    pub fn is_headless_or_support_guided_safe(&self) -> bool {
        matches!(
            self.read_only_posture,
            ReadOnlyPostureClass::ReadOnlyByDefaultNoMutation
                | ReadOnlyPostureClass::MetadataLocalEvidenceOnly
        ) && !matches!(self.headless_admission, HeadlessAdmissionClass::Denied)
                || !matches!(
                    self.support_guided_admission,
                    SupportGuidedAdmissionClass::Denied
                )
    }
}

/// Probe-pack catalog record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorProbePackCatalog {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable catalog id.
    pub catalog_id: String,
    /// Stable catalog version.
    pub catalog_version: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Pack entries.
    pub packs: Vec<ProbePackEntry>,
}

/// Single probe-pack record (wraps one entry with the discriminator).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorProbePack {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Pack entry.
    pub pack: ProbePackEntry,
}

/// Beta-grade Project Doctor finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorBetaFinding {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable finding id.
    pub finding_id: String,
    /// Stable finding code (must begin with `doctor.finding.`).
    pub finding_code: String,
    /// Pack id that emitted the finding.
    pub probe_pack_ref: String,
    /// Pack class that emitted the finding.
    pub probe_pack_class: ProbePackClass,
    /// Pack version that emitted the finding.
    pub probe_pack_version: String,
    /// Machine severity class.
    pub severity_class: FindingSeverityClass,
    /// Confidence class.
    pub confidence_class: FindingConfidenceClass,
    /// Diagnosis posture.
    pub diagnosis_posture: DiagnosisPostureClass,
    /// Recovery handoff class.
    pub recovery_handoff_class: RecoveryHandoffClass,
    /// Support context in which the finding was emitted.
    pub support_context_class: SupportContextClass,
    /// Render surfaces that may carry the finding.
    pub render_surfaces: Vec<RenderSurfaceClass>,
    /// Attribution refs (must include the owning pack).
    pub attribution_refs: Vec<AttributionRef>,
    /// Evidence refs.
    pub evidence_refs: Vec<String>,
    /// Reviewer-safe summary.
    pub summary: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Redaction class.
    pub redaction_class: SupportRedactionClass,
    /// Whether raw private material is excluded (must be true).
    pub raw_private_material_excluded: bool,
}

/// Metadata-safe support packet projecting one catalog and a batch of
/// findings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorBetaSupportPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Frozen schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Catalog id that backed the findings.
    pub catalog_id: String,
    /// Catalog version that backed the findings.
    pub catalog_version: String,
    /// Pack rows projected by the packet.
    pub pack_rows: Vec<ProjectDoctorBetaPackRow>,
    /// Finding rows projected by the packet.
    pub finding_rows: Vec<ProjectDoctorBetaFindingRow>,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
}

impl ProjectDoctorBetaSupportPacket {
    /// Returns true when the packet preserves the metadata-safe contract.
    pub fn is_export_safe(&self) -> bool {
        self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.pack_rows.is_empty()
            && self.finding_rows.iter().all(|row| {
                row.raw_private_material_excluded
                    && row.finding_code.starts_with(DOCTOR_FINDING_PREFIX)
            })
    }
}

/// Pack row in the support packet projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorBetaPackRow {
    /// Pack id.
    pub pack_id: String,
    /// Pack class.
    pub pack_class: ProbePackClass,
    /// Pack version.
    pub pack_version: String,
    /// Lifecycle status.
    pub lifecycle_status: ProbePackLifecycleStatus,
    /// Read-only posture.
    pub read_only_posture: ReadOnlyPostureClass,
    /// Headless admission.
    pub headless_admission: HeadlessAdmissionClass,
    /// Support-guided admission.
    pub support_guided_admission: SupportGuidedAdmissionClass,
    /// Default redaction class.
    pub default_redaction_class: SupportRedactionClass,
    /// Reviewer-safe summary.
    pub summary: String,
}

impl From<&ProbePackEntry> for ProjectDoctorBetaPackRow {
    fn from(pack: &ProbePackEntry) -> Self {
        Self {
            pack_id: pack.pack_id.clone(),
            pack_class: pack.pack_class,
            pack_version: pack.pack_version.clone(),
            lifecycle_status: pack.lifecycle_status,
            read_only_posture: pack.read_only_posture,
            headless_admission: pack.headless_admission,
            support_guided_admission: pack.support_guided_admission,
            default_redaction_class: pack.default_redaction_class,
            summary: pack.summary.clone(),
        }
    }
}

/// Finding row in the support packet projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorBetaFindingRow {
    /// Stable finding id.
    pub finding_id: String,
    /// Stable finding code.
    pub finding_code: String,
    /// Pack ref.
    pub probe_pack_ref: String,
    /// Pack class.
    pub probe_pack_class: ProbePackClass,
    /// Severity class.
    pub severity_class: FindingSeverityClass,
    /// Confidence class.
    pub confidence_class: FindingConfidenceClass,
    /// Recovery handoff class.
    pub recovery_handoff_class: RecoveryHandoffClass,
    /// Support context class.
    pub support_context_class: SupportContextClass,
    /// Render surfaces.
    pub render_surfaces: Vec<RenderSurfaceClass>,
    /// Attribution refs.
    pub attribution_refs: Vec<AttributionRef>,
    /// Evidence refs.
    pub evidence_refs: Vec<String>,
    /// Reviewer-safe summary.
    pub summary: String,
    /// Redaction class.
    pub redaction_class: SupportRedactionClass,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

impl From<&ProjectDoctorBetaFinding> for ProjectDoctorBetaFindingRow {
    fn from(finding: &ProjectDoctorBetaFinding) -> Self {
        Self {
            finding_id: finding.finding_id.clone(),
            finding_code: finding.finding_code.clone(),
            probe_pack_ref: finding.probe_pack_ref.clone(),
            probe_pack_class: finding.probe_pack_class,
            severity_class: finding.severity_class,
            confidence_class: finding.confidence_class,
            recovery_handoff_class: finding.recovery_handoff_class,
            support_context_class: finding.support_context_class,
            render_surfaces: finding.render_surfaces.clone(),
            attribution_refs: finding.attribution_refs.clone(),
            evidence_refs: finding.evidence_refs.clone(),
            summary: finding.summary.clone(),
            redaction_class: finding.redaction_class,
            raw_private_material_excluded: finding.raw_private_material_excluded,
        }
    }
}

/// One validation failure emitted by the evaluator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectDoctorBetaViolation {
    /// Stable check id.
    pub check_id: String,
    /// Subject ref that failed the check.
    pub subject_ref: String,
    /// Reviewer-facing failure message.
    pub message: String,
}

/// Validation report returned when one or more checks fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectDoctorBetaValidationReport {
    /// Validation failures.
    pub violations: Vec<ProjectDoctorBetaViolation>,
}

impl fmt::Display for ProjectDoctorBetaValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} project-doctor beta violation(s)",
            self.violations.len()
        )
    }
}

impl Error for ProjectDoctorBetaValidationReport {}

/// Loads a probe-pack catalog from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like a
/// [`ProjectDoctorProbePackCatalog`].
pub fn load_probe_pack_catalog(
    yaml: &str,
) -> Result<ProjectDoctorProbePackCatalog, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads a single probe-pack record from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like a
/// [`ProjectDoctorProbePack`].
pub fn load_probe_pack_record(yaml: &str) -> Result<ProjectDoctorProbePack, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads a beta finding record from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like a
/// [`ProjectDoctorBetaFinding`].
pub fn load_beta_finding(yaml: &str) -> Result<ProjectDoctorBetaFinding, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Beta Project Doctor evaluator.
#[derive(Debug, Default, Clone, Copy)]
pub struct ProjectDoctorBetaEvaluator;

impl ProjectDoctorBetaEvaluator {
    /// Creates a new beta evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Validates the catalog.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectDoctorBetaValidationReport`] when the catalog
    /// duplicates pack ids, omits the required schema/doc refs, or contains a
    /// pack that is not read-only, denied on both admission paths, or names
    /// duplicate / empty / mis-prefixed finding codes.
    pub fn validate_catalog(
        &self,
        catalog: &ProjectDoctorProbePackCatalog,
    ) -> Result<(), ProjectDoctorBetaValidationReport> {
        let violations = validate_catalog(catalog);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(ProjectDoctorBetaValidationReport { violations })
        }
    }

    /// Validates the finding shape without binding it to a catalog.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectDoctorBetaValidationReport`] when the finding has the
    /// wrong record-kind, the wrong schema version, an empty or mis-prefixed
    /// finding code, empty evidence/attribution lists, a redaction class that
    /// is not metadata-safe, or `raw_private_material_excluded` set to false.
    pub fn validate_finding(
        &self,
        finding: &ProjectDoctorBetaFinding,
    ) -> Result<(), ProjectDoctorBetaValidationReport> {
        let violations = validate_finding(finding, None);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(ProjectDoctorBetaValidationReport { violations })
        }
    }

    /// Validates the finding against its declared catalog.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectDoctorBetaValidationReport`] when the finding's
    /// `probe_pack_ref` is not in the catalog, when its class/version does not
    /// match the pack, when its finding code is not in the pack's supported
    /// finding codes, or when its support-context class is not declared by the
    /// pack.
    pub fn validate_finding_against_catalog(
        &self,
        catalog: &ProjectDoctorProbePackCatalog,
        finding: &ProjectDoctorBetaFinding,
    ) -> Result<(), ProjectDoctorBetaValidationReport> {
        let mut violations = validate_finding(finding, Some(catalog));
        if validate_catalog(catalog).is_empty() {
            // Already validated.
        } else {
            violations.extend(validate_catalog(catalog));
        }
        if violations.is_empty() {
            Ok(())
        } else {
            Err(ProjectDoctorBetaValidationReport { violations })
        }
    }

    /// Builds the metadata-safe support packet projection.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectDoctorBetaValidationReport`] when the catalog or any
    /// of the bound findings fail validation.
    pub fn support_packet(
        &self,
        packet_id: impl Into<String>,
        captured_at: impl Into<String>,
        catalog: &ProjectDoctorProbePackCatalog,
        findings: &[ProjectDoctorBetaFinding],
    ) -> Result<ProjectDoctorBetaSupportPacket, ProjectDoctorBetaValidationReport> {
        let mut violations = validate_catalog(catalog);
        for finding in findings {
            violations.extend(validate_finding(finding, Some(catalog)));
        }
        if !violations.is_empty() {
            return Err(ProjectDoctorBetaValidationReport { violations });
        }

        let pack_rows = catalog
            .packs
            .iter()
            .map(ProjectDoctorBetaPackRow::from)
            .collect::<Vec<_>>();
        let finding_rows = findings
            .iter()
            .map(ProjectDoctorBetaFindingRow::from)
            .collect::<Vec<_>>();

        Ok(ProjectDoctorBetaSupportPacket {
            record_kind: PROJECT_DOCTOR_BETA_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: PROJECT_DOCTOR_BETA_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            captured_at: captured_at.into(),
            doc_ref: PROJECT_DOCTOR_BETA_DOC_REF.to_owned(),
            schema_ref: PROJECT_DOCTOR_BETA_SCHEMA_REF.to_owned(),
            catalog_id: catalog.catalog_id.clone(),
            catalog_version: catalog.catalog_version.clone(),
            pack_rows,
            finding_rows,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        })
    }
}

fn validate_catalog(catalog: &ProjectDoctorProbePackCatalog) -> Vec<ProjectDoctorBetaViolation> {
    let mut violations = Vec::new();

    if catalog.schema_version != PROJECT_DOCTOR_BETA_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "project_doctor.catalog_schema_version",
            &catalog.catalog_id,
            "catalog schema_version must be 1",
        );
    }
    if catalog.record_kind != PROJECT_DOCTOR_PROBE_PACK_CATALOG_RECORD_KIND {
        push_violation(
            &mut violations,
            "project_doctor.catalog_record_kind",
            &catalog.catalog_id,
            "catalog record_kind must be project_doctor_probe_pack_catalog_record",
        );
    }
    if catalog.catalog_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "project_doctor.catalog_id_empty",
            &catalog.catalog_id,
            "catalog_id must be non-empty",
        );
    }
    if catalog.catalog_version.trim().is_empty() {
        push_violation(
            &mut violations,
            "project_doctor.catalog_version_empty",
            &catalog.catalog_id,
            "catalog_version must be non-empty",
        );
    }
    if catalog.schema_ref != PROJECT_DOCTOR_BETA_SCHEMA_REF {
        push_violation(
            &mut violations,
            "project_doctor.catalog_schema_ref",
            &catalog.catalog_id,
            format!(
                "catalog schema_ref must equal {PROJECT_DOCTOR_BETA_SCHEMA_REF}"
            ),
        );
    }
    if catalog.doc_ref != PROJECT_DOCTOR_BETA_DOC_REF {
        push_violation(
            &mut violations,
            "project_doctor.catalog_doc_ref",
            &catalog.catalog_id,
            format!("catalog doc_ref must equal {PROJECT_DOCTOR_BETA_DOC_REF}"),
        );
    }
    if catalog.packs.is_empty() {
        push_violation(
            &mut violations,
            "project_doctor.catalog_packs_missing",
            &catalog.catalog_id,
            "catalog must declare at least one probe pack",
        );
    }

    let mut pack_ids: BTreeSet<&str> = BTreeSet::new();
    for pack in &catalog.packs {
        if !pack_ids.insert(pack.pack_id.as_str()) {
            push_violation(
                &mut violations,
                "project_doctor.duplicate_pack_id",
                &pack.pack_id,
                "duplicate pack_id in catalog is forbidden",
            );
        }
        violations.extend(validate_pack(pack));
    }

    violations
}

fn validate_pack(pack: &ProbePackEntry) -> Vec<ProjectDoctorBetaViolation> {
    let mut violations = Vec::new();

    if pack.pack_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "project_doctor.pack_id_empty",
            &pack.pack_id,
            "pack_id must be non-empty",
        );
    }
    if pack.pack_version.trim().is_empty() {
        push_violation(
            &mut violations,
            "project_doctor.pack_version_empty",
            &pack.pack_id,
            "pack_version must be non-empty",
        );
    }
    if pack.summary.trim().is_empty() {
        push_violation(
            &mut violations,
            "project_doctor.pack_summary_empty",
            &pack.pack_id,
            "pack summary must be non-empty",
        );
    }
    if pack.supported_finding_codes.is_empty() {
        push_violation(
            &mut violations,
            "project_doctor.pack_finding_codes_empty",
            &pack.pack_id,
            "pack must declare at least one supported finding code",
        );
    }
    if pack.supported_support_contexts.is_empty() {
        push_violation(
            &mut violations,
            "project_doctor.pack_support_contexts_empty",
            &pack.pack_id,
            "pack must declare at least one supported support context",
        );
    }
    if pack.supported_recovery_handoffs.is_empty() {
        push_violation(
            &mut violations,
            "project_doctor.pack_recovery_handoffs_empty",
            &pack.pack_id,
            "pack must declare at least one supported recovery handoff",
        );
    }

    let mut seen_codes: BTreeSet<&str> = BTreeSet::new();
    for code in &pack.supported_finding_codes {
        if !code.starts_with(DOCTOR_FINDING_PREFIX) {
            push_violation(
                &mut violations,
                "project_doctor.pack_finding_code_prefix",
                &pack.pack_id,
                format!("finding code {code} must start with {DOCTOR_FINDING_PREFIX}"),
            );
        }
        if !seen_codes.insert(code.as_str()) {
            push_violation(
                &mut violations,
                "project_doctor.pack_duplicate_finding_code",
                &pack.pack_id,
                format!("duplicate finding code {code} in pack"),
            );
        }
    }

    if matches!(pack.headless_admission, HeadlessAdmissionClass::Denied)
        && matches!(
            pack.support_guided_admission,
            SupportGuidedAdmissionClass::Denied
        )
    {
        push_violation(
            &mut violations,
            "project_doctor.pack_admission_denied_everywhere",
            &pack.pack_id,
            "pack must be admitted under headless or support-guided mode",
        );
    }

    if pack.default_redaction_class != SupportRedactionClass::MetadataSafeDefault {
        push_violation(
            &mut violations,
            "project_doctor.pack_default_redaction_not_safe",
            &pack.pack_id,
            "pack default_redaction_class must be metadata_safe_default",
        );
    }

    violations
}

fn validate_finding(
    finding: &ProjectDoctorBetaFinding,
    catalog: Option<&ProjectDoctorProbePackCatalog>,
) -> Vec<ProjectDoctorBetaViolation> {
    let mut violations = Vec::new();

    if finding.schema_version != PROJECT_DOCTOR_BETA_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "project_doctor.finding_schema_version",
            &finding.finding_id,
            "finding schema_version must be 1",
        );
    }
    if finding.record_kind != PROJECT_DOCTOR_FINDING_BETA_RECORD_KIND {
        push_violation(
            &mut violations,
            "project_doctor.finding_record_kind",
            &finding.finding_id,
            "finding record_kind must be project_doctor_finding_record",
        );
    }
    if finding.finding_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "project_doctor.finding_id_empty",
            &finding.finding_id,
            "finding_id must be non-empty",
        );
    }
    if !finding.finding_code.starts_with(DOCTOR_FINDING_PREFIX) {
        push_violation(
            &mut violations,
            "project_doctor.finding_code_prefix",
            &finding.finding_id,
            format!("finding_code must start with {DOCTOR_FINDING_PREFIX}"),
        );
    }
    if finding.summary.trim().is_empty() {
        push_violation(
            &mut violations,
            "project_doctor.finding_summary_empty",
            &finding.finding_id,
            "finding summary must be non-empty",
        );
    }
    if finding.evidence_refs.is_empty() {
        push_violation(
            &mut violations,
            "project_doctor.finding_evidence_missing",
            &finding.finding_id,
            "finding must cite at least one evidence ref",
        );
    }
    if finding.render_surfaces.is_empty() {
        push_violation(
            &mut violations,
            "project_doctor.finding_render_surfaces_empty",
            &finding.finding_id,
            "finding must declare at least one render surface",
        );
    }
    if finding.redaction_class != SupportRedactionClass::MetadataSafeDefault {
        push_violation(
            &mut violations,
            "project_doctor.finding_redaction_not_safe",
            &finding.finding_id,
            "finding redaction_class must be metadata_safe_default",
        );
    }
    if !finding.raw_private_material_excluded {
        push_violation(
            &mut violations,
            "project_doctor.finding_raw_material_present",
            &finding.finding_id,
            "finding raw_private_material_excluded must be true",
        );
    }

    let has_pack_attribution = finding.attribution_refs.iter().any(|attribution| {
        attribution.attribution_kind == AttributionKindClass::ProbePackRef
            && attribution.ref_ == finding.probe_pack_ref
    });
    if !has_pack_attribution {
        push_violation(
            &mut violations,
            "project_doctor.finding_pack_attribution_missing",
            &finding.finding_id,
            "finding must carry a probe_pack_ref attribution back to probe_pack_ref",
        );
    }

    if let Some(catalog) = catalog {
        let pack_index: BTreeMap<&str, &ProbePackEntry> = catalog
            .packs
            .iter()
            .map(|pack| (pack.pack_id.as_str(), pack))
            .collect();
        match pack_index.get(finding.probe_pack_ref.as_str()) {
            None => push_violation(
                &mut violations,
                "project_doctor.finding_pack_ref_unknown",
                &finding.finding_id,
                format!(
                    "probe_pack_ref {} not present in catalog {}",
                    finding.probe_pack_ref, catalog.catalog_id
                ),
            ),
            Some(pack) => {
                if pack.pack_class != finding.probe_pack_class {
                    push_violation(
                        &mut violations,
                        "project_doctor.finding_pack_class_mismatch",
                        &finding.finding_id,
                        format!(
                            "finding probe_pack_class {} does not match catalog pack_class {} for {}",
                            finding.probe_pack_class,
                            pack.pack_class,
                            pack.pack_id
                        ),
                    );
                }
                if pack.pack_version != finding.probe_pack_version {
                    push_violation(
                        &mut violations,
                        "project_doctor.finding_pack_version_mismatch",
                        &finding.finding_id,
                        format!(
                            "finding probe_pack_version {} does not match catalog pack_version {} for {}",
                            finding.probe_pack_version,
                            pack.pack_version,
                            pack.pack_id
                        ),
                    );
                }
                if !pack
                    .supported_finding_codes
                    .iter()
                    .any(|code| code == &finding.finding_code)
                {
                    push_violation(
                        &mut violations,
                        "project_doctor.finding_code_not_supported",
                        &finding.finding_id,
                        format!(
                            "finding_code {} is not in supported codes of pack {}",
                            finding.finding_code, pack.pack_id
                        ),
                    );
                }
                if !pack
                    .supported_support_contexts
                    .iter()
                    .any(|context| *context == finding.support_context_class)
                {
                    push_violation(
                        &mut violations,
                        "project_doctor.finding_support_context_not_supported",
                        &finding.finding_id,
                        format!(
                            "support_context_class {} not declared by pack {}",
                            finding.support_context_class.as_str(),
                            pack.pack_id
                        ),
                    );
                }
                if !pack
                    .supported_recovery_handoffs
                    .iter()
                    .any(|handoff| *handoff == finding.recovery_handoff_class)
                {
                    push_violation(
                        &mut violations,
                        "project_doctor.finding_recovery_handoff_not_supported",
                        &finding.finding_id,
                        format!(
                            "recovery_handoff_class {} not declared by pack {}",
                            finding.recovery_handoff_class.as_str(),
                            pack.pack_id
                        ),
                    );
                }
                if matches!(finding.support_context_class, SupportContextClass::CliHeadless)
                    && matches!(pack.headless_admission, HeadlessAdmissionClass::Denied)
                {
                    push_violation(
                        &mut violations,
                        "project_doctor.finding_headless_denied",
                        &finding.finding_id,
                        format!(
                            "pack {} denies headless admission; cli_headless findings are forbidden",
                            pack.pack_id
                        ),
                    );
                }
                if matches!(finding.support_context_class, SupportContextClass::SupportGuided)
                    && matches!(
                        pack.support_guided_admission,
                        SupportGuidedAdmissionClass::Denied
                    )
                {
                    push_violation(
                        &mut violations,
                        "project_doctor.finding_support_guided_denied",
                        &finding.finding_id,
                        format!(
                            "pack {} denies support-guided admission; support_guided findings are forbidden",
                            pack.pack_id
                        ),
                    );
                }
            }
        }
    }

    violations
}

fn push_violation(
    violations: &mut Vec<ProjectDoctorBetaViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(ProjectDoctorBetaViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}
