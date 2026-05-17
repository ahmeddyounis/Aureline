//! Alpha probe runtime for Project Doctor.
//!
//! The alpha runtime is deliberately narrow. It does not inspect raw files,
//! execute project code, touch credentials, mutate indexes, run Git commands,
//! or reopen sessions. Callers pass typed evidence harvested by the owning
//! surfaces; Doctor validates that the packet is safe to diagnose and projects
//! one stable finding.
//!
//! The [`beta`] submodule promotes Project Doctor from alpha to a versioned,
//! attributable, confidence-labeled diagnosis system with a named probe-pack
//! catalog. Both modules share the alpha vocabulary; the beta lane adds the
//! pack catalog, supported-context and handoff vocabularies, and the
//! metadata-safe support packet.

pub mod beta;

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for an alpha probe scenario fixture.
pub const PROJECT_DOCTOR_ALPHA_PROBE_SCENARIO_RECORD_KIND: &str =
    "project_doctor_alpha_probe_scenario";

/// Stable record-kind tag for an emitted alpha finding.
pub const PROJECT_DOCTOR_ALPHA_FINDING_RECORD_KIND: &str = "project_doctor_alpha_finding";

/// Stable record-kind tag for a support/export packet derived from findings.
pub const PROJECT_DOCTOR_ALPHA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "project_doctor_alpha_support_export";

const REQUIRED_FORBIDDEN_SIDE_EFFECTS: [&str; 8] = [
    "activate_third_party_extension",
    "collect_high_risk_payload",
    "execute_repo_owned_code",
    "external_service_mutation",
    "mutate_cache_or_index",
    "mutate_target_or_route",
    "mutate_trust_policy_or_credentials",
    "mutate_user_files",
];

const ALLOWED_READ_ONLY_OPERATIONS: [&str; 2] =
    ["hash_existing_artifact", "read_existing_metadata"];

/// Loads one alpha probe scenario from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like an
/// [`AlphaProbeScenario`].
pub fn load_alpha_probe_scenario(yaml: &str) -> Result<AlphaProbeScenario, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Failure family covered by the external-alpha Doctor runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlphaProbeFamily {
    /// Opening, clone/import handoff, or recent-workspace entry failed.
    EntryOpen,
    /// Execution-context or toolchain selection failed.
    ToolchainDetection,
    /// Search or indexing is not ready enough for the requested scope.
    SearchIndexReadiness,
    /// Trust, identity, or policy blocked the requested capability.
    TrustPolicy,
    /// Local Git status or repository identity is unavailable or degraded.
    GitBaseline,
    /// Connected provider or credential authority cannot admit the action.
    ProviderAuth,
    /// Session restore, crash replay, or continuity hydration is unsafe.
    RestoreContinuity,
}

impl AlphaProbeFamily {
    /// Returns every alpha family in the acceptance order.
    pub const fn all() -> [Self; 7] {
        [
            Self::EntryOpen,
            Self::ToolchainDetection,
            Self::SearchIndexReadiness,
            Self::TrustPolicy,
            Self::GitBaseline,
            Self::ProviderAuth,
            Self::RestoreContinuity,
        ]
    }

    /// Returns the stable snake_case token for this family.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EntryOpen => "entry_open",
            Self::ToolchainDetection => "toolchain_detection",
            Self::SearchIndexReadiness => "search_index_readiness",
            Self::TrustPolicy => "trust_policy",
            Self::GitBaseline => "git_baseline",
            Self::ProviderAuth => "provider_auth",
            Self::RestoreContinuity => "restore_continuity",
        }
    }
}

impl fmt::Display for AlphaProbeFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Bounded state a probe can diagnose from existing evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlphaProblemClass {
    /// The requested entry target is missing, moved, or not admissible.
    EntryTargetUnavailable,
    /// A required toolchain component is missing.
    RequiredToolchainMissing,
    /// Search/index readiness stalled or fell below the requested scope.
    SearchIndexReadinessStalled,
    /// Active trust or policy denied the capability.
    TrustPolicyDenied,
    /// Local Git cannot identify a repository for the selected root.
    GitRepositoryUnavailable,
    /// Provider or credential authority requires renewal before use.
    ProviderCredentialExpired,
    /// Restore replay was blocked to avoid unsafe automatic rerun.
    RestoreReplayBlocked,
}

impl AlphaProblemClass {
    /// Returns the family that owns this problem class.
    pub const fn family(self) -> AlphaProbeFamily {
        match self {
            Self::EntryTargetUnavailable => AlphaProbeFamily::EntryOpen,
            Self::RequiredToolchainMissing => AlphaProbeFamily::ToolchainDetection,
            Self::SearchIndexReadinessStalled => AlphaProbeFamily::SearchIndexReadiness,
            Self::TrustPolicyDenied => AlphaProbeFamily::TrustPolicy,
            Self::GitRepositoryUnavailable => AlphaProbeFamily::GitBaseline,
            Self::ProviderCredentialExpired => AlphaProbeFamily::ProviderAuth,
            Self::RestoreReplayBlocked => AlphaProbeFamily::RestoreContinuity,
        }
    }
}

/// Machine severity emitted by an alpha finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    /// Informational finding.
    Info,
    /// Degraded but still usable.
    Degraded,
    /// Blocks the requested workflow until handled.
    Blocking,
    /// Unsupported in the current context.
    Unsupported,
}

/// Confidence class attached to a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceClass {
    /// Evidence directly proves the finding.
    ObservedAuthoritative,
    /// Evidence proves the finding but leaves a typed gap.
    ObservedWithGap,
    /// Evidence is sufficient for a bounded inference.
    InferredFromEvidence,
    /// More evidence is required before Doctor can prove the state.
    UnknownRequiresProbe,
}

/// Diagnosis posture shown in machine-readable and human-readable output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosisPosture {
    /// The finding is proven from current evidence.
    ProvingDiagnosis,
    /// The finding is inferred and keeps remaining unknowns visible.
    InferringFromPartialEvidence,
    /// Doctor refuses to diagnose beyond supported evidence.
    RefusingUnsupported,
}

/// Repair availability for the finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairAvailabilityClass {
    /// A reviewed repair path is available.
    ReviewedRepairAvailable,
    /// Doctor can show a repair preview but does not apply it.
    PreviewOnly,
    /// Doctor can only hand off to support or a governed external path.
    HandoffOnly,
    /// No supported local repair exists.
    Unsupported,
}

/// Headless exit semantics for an alpha finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeadlessExitCodeClass {
    /// No findings were emitted.
    ExitCleanNoFindings,
    /// Actionable findings were emitted.
    ExitFindingsActionable,
    /// The context is unsupported.
    ExitUnsupportedContext,
    /// Consent or reauthentication is required before continuing.
    ExitBlockedConsentRequired,
}

/// First governed action Doctor can offer for a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NextActionClass {
    /// Locate the missing target or open with minimal context.
    LocateMissingTarget,
    /// Re-resolve the toolchain from existing manifests.
    ReresolveToolchain,
    /// Open the search/index status surface.
    OpenIndexStatus,
    /// Open policy or trust details.
    OpenPolicyDetails,
    /// Open Git baseline details.
    OpenGitBaselineDetails,
    /// Reauthenticate or renew a provider handle.
    ReauthenticateProvider,
    /// Open without unsafe restore replay.
    OpenWithoutRestore,
    /// Create an escalation packet.
    CreateEscalationPacket,
}

/// Scope of an alpha probe finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProbeScope {
    /// Stable scope class.
    pub scope_class: String,
    /// Opaque scope reference.
    pub scope_ref: String,
}

/// Redaction-safe evidence available to Doctor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProbeEvidence {
    /// Opaque evidence reference.
    pub evidence_ref: String,
    /// Evidence role such as `primary` or `corroborating`.
    pub evidence_role: String,
    /// Source class from the support vocabulary.
    pub source_class: String,
    /// Signal class from the support vocabulary.
    pub signal_class: String,
    /// Diagnostic data class.
    pub data_class: String,
    /// Redaction class for support/export output.
    pub redaction_class: String,
    /// Support-pack inclusion posture.
    pub support_pack_inclusion_class: String,
    /// Replayability class for support and release evidence.
    pub replayability_class: String,
}

/// Read-only safety declaration for one scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadOnlySafety {
    /// Whether diagnosis is read-only by default.
    pub read_only_by_default: bool,
    /// Side effects allowed during diagnosis.
    #[serde(default)]
    pub allowed_side_effects: Vec<String>,
    /// Side effects explicitly forbidden during diagnosis.
    #[serde(default)]
    pub forbidden_side_effects: Vec<String>,
    /// Whether any mutation must move to a later repair flow.
    pub later_repair_flow_required_for_mutation: bool,
}

impl ReadOnlySafety {
    /// Returns true when the scenario satisfies the alpha read-only rule.
    pub fn is_read_only_diagnosis(&self) -> bool {
        self.read_only_by_default
            && self.later_repair_flow_required_for_mutation
            && self.disallowed_read_only_operations().is_empty()
    }

    fn disallowed_read_only_operations(&self) -> Vec<&str> {
        self.allowed_side_effects
            .iter()
            .filter_map(|effect| {
                let effect = effect.as_str();
                (!ALLOWED_READ_ONLY_OPERATIONS.contains(&effect)).then_some(effect)
            })
            .collect()
    }

    fn missing_forbidden_side_effects(&self) -> Vec<&'static str> {
        REQUIRED_FORBIDDEN_SIDE_EFFECTS
            .into_iter()
            .filter(|required| {
                !self
                    .forbidden_side_effects
                    .iter()
                    .any(|effect| effect == *required)
            })
            .collect()
    }
}

/// Observed bounded state from the owning surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservedState {
    /// Problem class Doctor should classify.
    pub problem_class: AlphaProblemClass,
    /// Opaque expected-state reference.
    pub expected_ref: String,
    /// Opaque observed-state reference.
    pub observed_ref: String,
    /// Human text key for expected state.
    pub expected_key: String,
    /// Human text key for observed state.
    pub observed_key: String,
}

/// Recovery or escalation path attached to the scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioRecoveryPath {
    /// Stable next-action id.
    pub recovery_action_id: String,
    /// Stable next-action class.
    pub recovery_action_class: NextActionClass,
    /// Repair availability for the path.
    pub repair_availability_class: RepairAvailabilityClass,
    /// Candidate repair ids, if any.
    #[serde(default)]
    pub repair_candidate_ids: Vec<String>,
    /// Exact recovery path or repair-preview ref.
    pub recovery_path_ref: Option<String>,
    /// Support runbook ref.
    pub runbook_ref: String,
    /// Support bundle ref.
    pub support_bundle_ref: String,
    /// Escalation packet ref.
    pub escalation_packet_ref: Option<String>,
}

impl ScenarioRecoveryPath {
    /// Returns true when the path has a concrete recovery or escalation ref.
    pub fn has_recovery_or_escalation(&self) -> bool {
        self.recovery_path_ref.is_some() || self.escalation_packet_ref.is_some()
    }
}

/// Expected finding fields used by protected fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpectedFinding {
    /// Stable finding id expected by the fixture.
    pub finding_id: String,
    /// Stable finding code expected by the fixture.
    pub finding_code: String,
    /// Expected first action class.
    pub recovery_action_class: NextActionClass,
}

/// One fixture or runtime input for an alpha Doctor probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaProbeScenario {
    /// Scenario schema version.
    pub schema_version: u32,
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Stable scenario id.
    pub scenario_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Alpha failure family.
    pub family: AlphaProbeFamily,
    /// Probe id.
    pub probe_id: String,
    /// Probe implementation version ref.
    pub probe_version: String,
    /// Support context class.
    pub support_context: String,
    /// Affected scope.
    pub scope: ProbeScope,
    /// Observed state from the owning surface.
    pub observed_state: ObservedState,
    /// Evidence refs Doctor can cite.
    pub evidence: Vec<ProbeEvidence>,
    /// Read-only safety declaration.
    pub safety: ReadOnlySafety,
    /// Recovery or escalation path.
    pub recovery: ScenarioRecoveryPath,
    /// Fixture assertion for the expected finding.
    pub expected: ExpectedFinding,
}

/// Project Doctor alpha runtime.
#[derive(Debug, Default, Clone, Copy)]
pub struct ProjectDoctorAlpha;

impl ProjectDoctorAlpha {
    /// Creates a new alpha Doctor runtime.
    pub const fn new() -> Self {
        Self
    }

    /// Diagnoses one read-only scenario into a stable finding.
    ///
    /// # Errors
    ///
    /// Returns [`DoctorProbeError`] when the input violates the read-only,
    /// evidence, family, or recovery-path contract.
    pub fn diagnose(
        &self,
        scenario: &AlphaProbeScenario,
    ) -> Result<DoctorFinding, DoctorProbeError> {
        validate_scenario(scenario)?;
        let template = FindingTemplate::for_problem(scenario.observed_state.problem_class);

        if scenario.recovery.recovery_action_class != template.recovery_action_class {
            return Err(DoctorProbeError::RecoveryActionMismatch {
                scenario_id: scenario.scenario_id.clone(),
                expected: template.recovery_action_class,
                actual: scenario.recovery.recovery_action_class,
            });
        }

        Ok(DoctorFinding {
            schema_version: 1,
            record_kind: PROJECT_DOCTOR_ALPHA_FINDING_RECORD_KIND.to_owned(),
            finding_id: format!("finding:{}", scenario.scenario_id.replace('.', ":")),
            finding_code: template.finding_code.to_owned(),
            rule_id: template.rule_id.to_owned(),
            probe_id: scenario.probe_id.clone(),
            probe_version: scenario.probe_version.clone(),
            family: scenario.family,
            severity: template.severity,
            confidence: template.confidence,
            diagnosis_posture: template.diagnosis_posture,
            scope: scenario.scope.clone(),
            expected_ref: scenario.observed_state.expected_ref.clone(),
            observed_ref: scenario.observed_state.observed_ref.clone(),
            evidence_refs: scenario.evidence.clone(),
            remaining_unknowns: template
                .remaining_unknowns
                .iter()
                .map(|s| s.to_string())
                .collect(),
            recovery: FindingRecovery {
                next_action_id: scenario.recovery.recovery_action_id.clone(),
                next_action_class: scenario.recovery.recovery_action_class,
                repair_availability_class: scenario.recovery.repair_availability_class,
                repair_candidate_ids: scenario.recovery.repair_candidate_ids.clone(),
                recovery_path_ref: scenario.recovery.recovery_path_ref.clone(),
                runbook_ref: scenario.recovery.runbook_ref.clone(),
                support_bundle_ref: scenario.recovery.support_bundle_ref.clone(),
                escalation_packet_ref: scenario.recovery.escalation_packet_ref.clone(),
            },
            read_only_attestation: scenario.safety.clone(),
            headless_exit_code_class: template.headless_exit_code_class,
        })
    }
}

/// Emitted alpha Doctor finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorFinding {
    /// Finding schema version.
    pub schema_version: u32,
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Stable finding id.
    pub finding_id: String,
    /// Stable finding code.
    pub finding_code: String,
    /// Stable rule id.
    pub rule_id: String,
    /// Probe id that emitted the finding.
    pub probe_id: String,
    /// Probe implementation version.
    pub probe_version: String,
    /// Alpha family this finding belongs to.
    pub family: AlphaProbeFamily,
    /// Machine severity.
    pub severity: FindingSeverity,
    /// Finding confidence class.
    pub confidence: ConfidenceClass,
    /// Diagnosis posture.
    pub diagnosis_posture: DiagnosisPosture,
    /// Affected scope.
    pub scope: ProbeScope,
    /// Expected-state ref.
    pub expected_ref: String,
    /// Observed-state ref.
    pub observed_ref: String,
    /// Evidence refs supporting the finding.
    pub evidence_refs: Vec<ProbeEvidence>,
    /// Typed unknowns still present after diagnosis.
    pub remaining_unknowns: Vec<String>,
    /// Recovery or escalation handoff.
    pub recovery: FindingRecovery,
    /// Read-only attestation inherited from the scenario.
    pub read_only_attestation: ReadOnlySafety,
    /// Headless exit-code class.
    pub headless_exit_code_class: HeadlessExitCodeClass,
}

impl DoctorFinding {
    /// Returns true when the finding cites evidence and a concrete next path.
    pub fn is_actionable_with_evidence(&self) -> bool {
        !self.evidence_refs.is_empty() && self.recovery.has_recovery_or_escalation()
    }

    /// Returns true when the finding did not require diagnosis-time mutation.
    pub fn is_read_only(&self) -> bool {
        self.read_only_attestation.is_read_only_diagnosis()
    }
}

/// Recovery or escalation handoff emitted with a finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FindingRecovery {
    /// Stable next-action id.
    pub next_action_id: String,
    /// Stable next-action class.
    pub next_action_class: NextActionClass,
    /// Repair availability.
    pub repair_availability_class: RepairAvailabilityClass,
    /// Candidate repair ids, if any.
    pub repair_candidate_ids: Vec<String>,
    /// Exact recovery path or repair-preview ref.
    pub recovery_path_ref: Option<String>,
    /// Runbook ref.
    pub runbook_ref: String,
    /// Support bundle ref.
    pub support_bundle_ref: String,
    /// Escalation packet ref.
    pub escalation_packet_ref: Option<String>,
}

impl FindingRecovery {
    /// Returns true when this recovery handoff carries a concrete route.
    pub fn has_recovery_or_escalation(&self) -> bool {
        self.recovery_path_ref.is_some() || self.escalation_packet_ref.is_some()
    }
}

/// Support/export packet derived from alpha Doctor findings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorSupportExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Export schema version.
    pub schema_version: u32,
    /// Opaque packet id.
    pub packet_id: String,
    /// Finding rows included in the export.
    pub rows: Vec<DoctorSupportExportRow>,
    /// Redaction class for all rows.
    pub redaction_class: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

impl DoctorSupportExport {
    /// Builds a support/export packet from alpha findings.
    pub fn from_findings(packet_id: impl Into<String>, findings: &[DoctorFinding]) -> Self {
        Self {
            record_kind: PROJECT_DOCTOR_ALPHA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: 1,
            packet_id: packet_id.into(),
            rows: findings.iter().map(DoctorSupportExportRow::from).collect(),
            redaction_class: "metadata_safe_default".to_owned(),
            raw_private_material_excluded: true,
        }
    }

    /// Returns true when every row remains metadata-only and actionable.
    pub fn is_export_safe(&self) -> bool {
        self.raw_private_material_excluded
            && self.redaction_class == "metadata_safe_default"
            && self.rows.iter().all(DoctorSupportExportRow::is_export_safe)
    }
}

/// One support/export row for an alpha finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorSupportExportRow {
    /// Stable finding id.
    pub finding_id: String,
    /// Stable finding code.
    pub finding_code: String,
    /// Alpha family token.
    pub family: AlphaProbeFamily,
    /// Probe version.
    pub probe_version: String,
    /// Evidence refs carried by the export.
    pub evidence_refs: Vec<String>,
    /// Next-action id.
    pub next_action_id: String,
    /// Recovery path ref, if available.
    pub recovery_path_ref: Option<String>,
    /// Support bundle ref.
    pub support_bundle_ref: String,
    /// Escalation packet ref, if available.
    pub escalation_packet_ref: Option<String>,
    /// Redaction class for the row.
    pub redaction_class: String,
}

impl DoctorSupportExportRow {
    /// Returns true when the row carries no raw private material.
    pub fn is_export_safe(&self) -> bool {
        self.redaction_class == "metadata_safe_default"
            && !self.evidence_refs.is_empty()
            && (self.recovery_path_ref.is_some() || self.escalation_packet_ref.is_some())
    }
}

impl From<&DoctorFinding> for DoctorSupportExportRow {
    fn from(finding: &DoctorFinding) -> Self {
        Self {
            finding_id: finding.finding_id.clone(),
            finding_code: finding.finding_code.clone(),
            family: finding.family,
            probe_version: finding.probe_version.clone(),
            evidence_refs: finding
                .evidence_refs
                .iter()
                .map(|evidence| evidence.evidence_ref.clone())
                .collect(),
            next_action_id: finding.recovery.next_action_id.clone(),
            recovery_path_ref: finding.recovery.recovery_path_ref.clone(),
            support_bundle_ref: finding.recovery.support_bundle_ref.clone(),
            escalation_packet_ref: finding.recovery.escalation_packet_ref.clone(),
            redaction_class: "metadata_safe_default".to_owned(),
        }
    }
}

/// Validation failure while classifying an alpha probe scenario.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoctorProbeError {
    /// The scenario schema version is not supported.
    UnsupportedSchemaVersion { scenario_id: String, actual: u32 },
    /// The record-kind discriminator is not the alpha probe scenario kind.
    WrongRecordKind { scenario_id: String, actual: String },
    /// The scenario family does not own the observed problem class.
    FamilyProblemMismatch {
        scenario_id: String,
        family: AlphaProbeFamily,
        problem_family: AlphaProbeFamily,
    },
    /// The scenario does not cite any evidence.
    MissingEvidence { scenario_id: String },
    /// The scenario would require mutation during diagnosis.
    UnsafeProbe { scenario_id: String, reason: String },
    /// The scenario lacks both a recovery ref and an escalation ref.
    MissingRecoveryOrEscalation { scenario_id: String },
    /// The recovery action class does not match the problem template.
    RecoveryActionMismatch {
        scenario_id: String,
        expected: NextActionClass,
        actual: NextActionClass,
    },
}

impl fmt::Display for DoctorProbeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion {
                scenario_id,
                actual,
            } => write!(
                f,
                "{scenario_id} uses unsupported schema_version {actual}; expected 1"
            ),
            Self::WrongRecordKind {
                scenario_id,
                actual,
            } => write!(
                f,
                "{scenario_id} has record_kind {actual}; expected {PROJECT_DOCTOR_ALPHA_PROBE_SCENARIO_RECORD_KIND}"
            ),
            Self::FamilyProblemMismatch {
                scenario_id,
                family,
                problem_family,
            } => write!(
                f,
                "{scenario_id} declares family {family} but problem belongs to {problem_family}"
            ),
            Self::MissingEvidence { scenario_id } => {
                write!(f, "{scenario_id} must cite at least one evidence ref")
            }
            Self::UnsafeProbe {
                scenario_id,
                reason,
            } => write!(f, "{scenario_id} is not a read-only diagnosis: {reason}"),
            Self::MissingRecoveryOrEscalation { scenario_id } => write!(
                f,
                "{scenario_id} must cite an exact recovery path or escalation packet"
            ),
            Self::RecoveryActionMismatch {
                scenario_id,
                expected,
                actual,
            } => write!(
                f,
                "{scenario_id} recovery action {actual:?} does not match expected {expected:?}"
            ),
        }
    }
}

impl Error for DoctorProbeError {}

struct FindingTemplate {
    finding_code: &'static str,
    rule_id: &'static str,
    severity: FindingSeverity,
    confidence: ConfidenceClass,
    diagnosis_posture: DiagnosisPosture,
    recovery_action_class: NextActionClass,
    headless_exit_code_class: HeadlessExitCodeClass,
    remaining_unknowns: &'static [&'static str],
}

impl FindingTemplate {
    fn for_problem(problem: AlphaProblemClass) -> Self {
        match problem {
            AlphaProblemClass::EntryTargetUnavailable => Self {
                finding_code: "doctor.finding.entry_open.target_unavailable",
                rule_id: "doctor.rule.entry_open.target_unavailable",
                severity: FindingSeverity::Blocking,
                confidence: ConfidenceClass::ObservedAuthoritative,
                diagnosis_posture: DiagnosisPosture::ProvingDiagnosis,
                recovery_action_class: NextActionClass::LocateMissingTarget,
                headless_exit_code_class: HeadlessExitCodeClass::ExitFindingsActionable,
                remaining_unknowns: &[],
            },
            AlphaProblemClass::RequiredToolchainMissing => Self {
                finding_code: "doctor.finding.toolchain_missing_required_component",
                rule_id: "doctor.rule.toolchain.required_component_missing",
                severity: FindingSeverity::Blocking,
                confidence: ConfidenceClass::ObservedWithGap,
                diagnosis_posture: DiagnosisPosture::InferringFromPartialEvidence,
                recovery_action_class: NextActionClass::ReresolveToolchain,
                headless_exit_code_class: HeadlessExitCodeClass::ExitFindingsActionable,
                remaining_unknowns: &["selected_toolchain_install_path_unknown"],
            },
            AlphaProblemClass::SearchIndexReadinessStalled => Self {
                finding_code: "doctor.finding.search_index.readiness_stalled",
                rule_id: "doctor.rule.search_index.readiness_stalled",
                severity: FindingSeverity::Degraded,
                confidence: ConfidenceClass::ObservedWithGap,
                diagnosis_posture: DiagnosisPosture::InferringFromPartialEvidence,
                recovery_action_class: NextActionClass::OpenIndexStatus,
                headless_exit_code_class: HeadlessExitCodeClass::ExitFindingsActionable,
                remaining_unknowns: &["cold_scope_index_completion_unknown"],
            },
            AlphaProblemClass::TrustPolicyDenied => Self {
                finding_code: "doctor.finding.trust_policy_blocked",
                rule_id: "doctor.rule.trust_policy.denied_capability",
                severity: FindingSeverity::Blocking,
                confidence: ConfidenceClass::ObservedAuthoritative,
                diagnosis_posture: DiagnosisPosture::ProvingDiagnosis,
                recovery_action_class: NextActionClass::OpenPolicyDetails,
                headless_exit_code_class: HeadlessExitCodeClass::ExitFindingsActionable,
                remaining_unknowns: &[],
            },
            AlphaProblemClass::GitRepositoryUnavailable => Self {
                finding_code: "doctor.finding.git_baseline.repository_unavailable",
                rule_id: "doctor.rule.git_baseline.repository_unavailable",
                severity: FindingSeverity::Degraded,
                confidence: ConfidenceClass::ObservedAuthoritative,
                diagnosis_posture: DiagnosisPosture::ProvingDiagnosis,
                recovery_action_class: NextActionClass::OpenGitBaselineDetails,
                headless_exit_code_class: HeadlessExitCodeClass::ExitFindingsActionable,
                remaining_unknowns: &[],
            },
            AlphaProblemClass::ProviderCredentialExpired => Self {
                finding_code: "doctor.finding.provider_auth.credential_expired",
                rule_id: "doctor.rule.provider_auth.credential_expired",
                severity: FindingSeverity::Blocking,
                confidence: ConfidenceClass::ObservedAuthoritative,
                diagnosis_posture: DiagnosisPosture::ProvingDiagnosis,
                recovery_action_class: NextActionClass::ReauthenticateProvider,
                headless_exit_code_class: HeadlessExitCodeClass::ExitBlockedConsentRequired,
                remaining_unknowns: &[],
            },
            AlphaProblemClass::RestoreReplayBlocked => Self {
                finding_code: "doctor.finding.restore_continuity.replay_blocked",
                rule_id: "doctor.rule.restore_continuity.replay_blocked",
                severity: FindingSeverity::Blocking,
                confidence: ConfidenceClass::InferredFromEvidence,
                diagnosis_posture: DiagnosisPosture::InferringFromPartialEvidence,
                recovery_action_class: NextActionClass::OpenWithoutRestore,
                headless_exit_code_class: HeadlessExitCodeClass::ExitFindingsActionable,
                remaining_unknowns: &["live_dependency_rebind_not_proven"],
            },
        }
    }
}

fn validate_scenario(scenario: &AlphaProbeScenario) -> Result<(), DoctorProbeError> {
    if scenario.schema_version != 1 {
        return Err(DoctorProbeError::UnsupportedSchemaVersion {
            scenario_id: scenario.scenario_id.clone(),
            actual: scenario.schema_version,
        });
    }
    if scenario.record_kind != PROJECT_DOCTOR_ALPHA_PROBE_SCENARIO_RECORD_KIND {
        return Err(DoctorProbeError::WrongRecordKind {
            scenario_id: scenario.scenario_id.clone(),
            actual: scenario.record_kind.clone(),
        });
    }
    let problem_family = scenario.observed_state.problem_class.family();
    if scenario.family != problem_family {
        return Err(DoctorProbeError::FamilyProblemMismatch {
            scenario_id: scenario.scenario_id.clone(),
            family: scenario.family,
            problem_family,
        });
    }
    if scenario.evidence.is_empty() {
        return Err(DoctorProbeError::MissingEvidence {
            scenario_id: scenario.scenario_id.clone(),
        });
    }
    if !scenario.safety.read_only_by_default
        || !scenario.safety.later_repair_flow_required_for_mutation
    {
        return Err(DoctorProbeError::UnsafeProbe {
            scenario_id: scenario.scenario_id.clone(),
            reason:
                "read_only_by_default and later_repair_flow_required_for_mutation must both be true"
                    .to_owned(),
        });
    }
    let disallowed = scenario.safety.disallowed_read_only_operations();
    if !disallowed.is_empty() {
        return Err(DoctorProbeError::UnsafeProbe {
            scenario_id: scenario.scenario_id.clone(),
            reason: format!(
                "allowed_side_effects contains non-read-only operations: {}",
                disallowed.join(", ")
            ),
        });
    }
    let missing = scenario.safety.missing_forbidden_side_effects();
    if !missing.is_empty() {
        return Err(DoctorProbeError::UnsafeProbe {
            scenario_id: scenario.scenario_id.clone(),
            reason: format!("missing forbidden side effects: {}", missing.join(", ")),
        });
    }
    if !scenario.recovery.has_recovery_or_escalation() {
        return Err(DoctorProbeError::MissingRecoveryOrEscalation {
            scenario_id: scenario.scenario_id.clone(),
        });
    }
    Ok(())
}
