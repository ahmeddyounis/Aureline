//! Project Doctor alpha probe-pack consumer.
//!
//! This module is the first support/export consumer for
//! `/artifacts/support/project_doctor_probe_pack_alpha.yaml`. It parses the
//! checked-in read-only probe pack, validates the contract invariants that
//! make diagnosis safe, and renders machine-readable and human-readable
//! projections from the same finding vocabulary.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for a Project Doctor probe pack.
pub const PROJECT_DOCTOR_PROBE_PACK_RECORD_KIND: &str = "project_doctor_probe_pack_record";

/// Stable record-kind tag for one Project Doctor probe row.
pub const PROJECT_DOCTOR_PROBE_RECORD_KIND: &str = "project_doctor_probe_record";

/// Stable record-kind tag for the headless Project Doctor projection.
pub const PROJECT_DOCTOR_HEADLESS_OUTPUT_RECORD_KIND: &str = "project_doctor_headless_output";

/// Stable record-kind tag for the human Project Doctor projection.
pub const PROJECT_DOCTOR_HUMAN_OUTPUT_RECORD_KIND: &str = "project_doctor_human_output";

/// Stable support/export packet id for executable alpha Doctor findings.
pub const PROJECT_DOCTOR_ALPHA_RUNTIME_SUPPORT_PACKET_ID: &str =
    "support.project_doctor.alpha_runtime";

/// Repository-relative path of the alpha probe-pack artifact.
pub const CURRENT_ALPHA_PROBE_PACK_PATH: &str =
    "artifacts/support/project_doctor_probe_pack_alpha.yaml";

const CURRENT_ALPHA_PROBE_PACK_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/support/project_doctor_probe_pack_alpha.yaml"
));

const EXPECTED_CONTEXTS: [&str; 4] = ["cli_headless", "desktop", "offline_local", "remote_managed"];

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

/// Loads the current alpha Project Doctor probe pack from the checked-in YAML.
///
/// The returned value is a parsed artifact only. Call
/// [`ProjectDoctorProbePack::validate`] before using it as release evidence.
///
/// # Errors
///
/// Returns a YAML parse error when the checked-in artifact is not shaped like
/// [`ProjectDoctorProbePack`].
pub fn current_alpha_probe_pack() -> Result<ProjectDoctorProbePack, serde_yaml::Error> {
    serde_yaml::from_str(CURRENT_ALPHA_PROBE_PACK_YAML)
}

/// Builds the support/export projection for executable alpha Doctor findings.
///
/// The support crate consumes the runtime projection without reclassifying
/// probe results or scraping rendered text. The `aureline-doctor` crate remains
/// the finding owner; this function only gives support bundles a stable packet
/// id and export-safe row shape.
pub fn alpha_runtime_support_output(
    findings: &[aureline_doctor::probes::DoctorFinding],
) -> aureline_doctor::probes::DoctorSupportExport {
    aureline_doctor::probes::DoctorSupportExport::from_findings(
        PROJECT_DOCTOR_ALPHA_RUNTIME_SUPPORT_PACKET_ID,
        findings,
    )
}

/// A parsed `project_doctor_probe_pack_record` artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorProbePack {
    /// Schema version for the pack record.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable pack id.
    pub pack_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Owning lane or service ref.
    pub owner_ref: String,
    /// Schema refs used by this pack.
    pub schema_refs: ProjectDoctorSchemaRefs,
    /// Source docs and contracts the pack consumes.
    pub source_contract_refs: Vec<String>,
    /// Default read-only diagnosis policy.
    pub default_execution_policy: ProjectDoctorExecutionPolicy,
    /// Vocabulary refs back to the support-owned schemas.
    pub vocabulary_bindings: BTreeMap<String, String>,
    /// Redaction-safe output routing.
    pub output_routing: ProjectDoctorOutputRouting,
    /// Shared finding vocabulary used by machine and human projections.
    pub finding_vocabulary: Vec<ProjectDoctorFindingVocabularyEntry>,
    /// Probe records in the alpha baseline.
    pub probes: Vec<ProjectDoctorProbe>,
    /// Reviewer-facing coverage counters.
    pub acceptance_proof: ProjectDoctorAcceptanceProof,
    /// Timestamp when the artifact was emitted.
    pub emitted_at: String,
}

impl ProjectDoctorProbePack {
    /// Validates the Project Doctor alpha read-only and parity invariants.
    pub fn validate(&self) -> Vec<ProjectDoctorPackViolation> {
        let mut violations = Vec::new();

        if self.schema_version != 1 {
            push_violation(
                &mut violations,
                "project_doctor.schema_version",
                &self.pack_id,
                "schema_version must be 1",
            );
        }
        if self.record_kind != PROJECT_DOCTOR_PROBE_PACK_RECORD_KIND {
            push_violation(
                &mut violations,
                "project_doctor.record_kind",
                &self.pack_id,
                "record_kind must be project_doctor_probe_pack_record",
            );
        }
        if !self.default_execution_policy.read_only_by_default {
            push_violation(
                &mut violations,
                "project_doctor.read_only_default",
                &self.pack_id,
                "default_execution_policy.read_only_by_default must be true",
            );
        }
        if self.output_routing.shared_finding_vocabulary_ref != "#/finding_vocabulary" {
            push_violation(
                &mut violations,
                "project_doctor.shared_finding_vocabulary",
                &self.pack_id,
                "machine and human output must bind to #/finding_vocabulary",
            );
        }
        if self.output_routing.default_redaction_class != "metadata_safe_default" {
            push_violation(
                &mut violations,
                "project_doctor.default_redaction",
                &self.pack_id,
                "default output redaction must be metadata_safe_default",
            );
        }

        for required in REQUIRED_FORBIDDEN_SIDE_EFFECTS {
            if !self
                .default_execution_policy
                .forbidden_during_diagnosis
                .iter()
                .any(|effect| effect == required)
            {
                push_violation(
                    &mut violations,
                    "project_doctor.missing_forbidden_side_effect",
                    &self.pack_id,
                    format!("default policy must forbid {required}"),
                );
            }
        }

        let mut finding_codes = BTreeSet::new();
        for row in &self.finding_vocabulary {
            if !finding_codes.insert(row.finding_code.as_str()) {
                push_violation(
                    &mut violations,
                    "project_doctor.duplicate_finding_code",
                    &row.finding_code,
                    "finding_vocabulary must not contain duplicate finding codes",
                );
            }
        }

        self.validate_repair_coverage(&mut violations);
        self.validate_acceptance_counts(&mut violations);

        for probe in &self.probes {
            validate_probe(probe, &finding_codes, &mut violations);
        }

        if !self.machine_and_human_outputs_share_vocabulary() {
            push_violation(
                &mut violations,
                "project_doctor.output_vocabulary_mismatch",
                &self.pack_id,
                "machine and human projections must use the same finding codes",
            );
        }

        violations
    }

    /// Renders the headless support/export projection from the shared
    /// finding vocabulary.
    pub fn machine_output(&self) -> ProjectDoctorMachineOutput {
        ProjectDoctorMachineOutput {
            record_kind: PROJECT_DOCTOR_HEADLESS_OUTPUT_RECORD_KIND.to_owned(),
            output_ref: self.output_routing.machine_readable_output_ref.clone(),
            shared_finding_vocabulary_ref: self
                .output_routing
                .shared_finding_vocabulary_ref
                .clone(),
            finding_rows: self
                .finding_vocabulary
                .iter()
                .map(|finding| ProjectDoctorMachineFindingRow {
                    finding_code: finding.finding_code.clone(),
                    severity_class: finding.severity_class.clone(),
                    confidence_class: finding.confidence_class.clone(),
                    repair_availability_class: finding.repair_availability_class.clone(),
                    unsupported_state_class: finding.unsupported_state_class.clone(),
                })
                .collect(),
        }
    }

    /// Renders the human summary projection from the shared finding
    /// vocabulary.
    pub fn human_output(&self) -> ProjectDoctorHumanOutput {
        ProjectDoctorHumanOutput {
            record_kind: PROJECT_DOCTOR_HUMAN_OUTPUT_RECORD_KIND.to_owned(),
            output_ref: self.output_routing.human_readable_output_ref.clone(),
            shared_finding_vocabulary_ref: self
                .output_routing
                .shared_finding_vocabulary_ref
                .clone(),
            summary_rows: self
                .finding_vocabulary
                .iter()
                .map(|finding| ProjectDoctorHumanSummaryRow {
                    finding_code: finding.finding_code.clone(),
                    display_severity_class: finding.display_severity_class.clone(),
                    summary_key: finding.summary_key.clone(),
                    next_action_key: finding.next_action_key.clone(),
                    repair_availability_class: finding.repair_availability_class.clone(),
                    unsupported_state_class: finding.unsupported_state_class.clone(),
                })
                .collect(),
        }
    }

    /// Returns true when headless and human projections carry exactly the
    /// same finding-code set.
    pub fn machine_and_human_outputs_share_vocabulary(&self) -> bool {
        let machine = self
            .machine_output()
            .finding_rows
            .into_iter()
            .map(|row| row.finding_code)
            .collect::<BTreeSet<_>>();
        let human = self
            .human_output()
            .summary_rows
            .into_iter()
            .map(|row| row.finding_code)
            .collect::<BTreeSet<_>>();
        machine == human
            && self.output_routing.shared_finding_vocabulary_ref == "#/finding_vocabulary"
    }

    fn validate_repair_coverage(&self, violations: &mut Vec<ProjectDoctorPackViolation>) {
        let states = self
            .finding_vocabulary
            .iter()
            .map(|row| row.repair_availability_class.as_str())
            .collect::<BTreeSet<_>>();
        for required in ["preview_only", "reviewed_repair_available", "unsupported"] {
            if !states.contains(required) {
                push_violation(
                    violations,
                    "project_doctor.repair_availability_gap",
                    &self.pack_id,
                    format!("finding vocabulary must include repair state {required}"),
                );
            }
        }
    }

    fn validate_acceptance_counts(&self, violations: &mut Vec<ProjectDoctorPackViolation>) {
        let read_only_probe_count = self
            .probes
            .iter()
            .filter(|probe| {
                probe.read_only_default
                    && matches!(
                        probe.mutability_class.as_str(),
                        "non_mutating_read_only" | "metadata_write_local_evidence_only"
                    )
            })
            .count();
        let repair_available_count = self
            .probes
            .iter()
            .filter(|probe| {
                probe.repair_handoff.repair_availability_class == "reviewed_repair_available"
            })
            .count();
        let preview_only_count = self
            .probes
            .iter()
            .filter(|probe| probe.repair_handoff.repair_availability_class == "preview_only")
            .count();
        let unsupported_state_count = self
            .finding_vocabulary
            .iter()
            .filter(|row| row.unsupported_state_class != "none")
            .count();

        let expected = [
            (
                "read_only_probe_count",
                self.acceptance_proof.read_only_probe_count,
                read_only_probe_count,
            ),
            (
                "repair_available_count",
                self.acceptance_proof.repair_available_count,
                repair_available_count,
            ),
            (
                "preview_only_count",
                self.acceptance_proof.preview_only_count,
                preview_only_count,
            ),
            (
                "unsupported_state_count",
                self.acceptance_proof.unsupported_state_count,
                unsupported_state_count,
            ),
        ];
        for (field, expected_count, actual_count) in expected {
            if expected_count != actual_count {
                push_violation(
                    violations,
                    "project_doctor.acceptance_count_mismatch",
                    &self.pack_id,
                    format!(
                        "acceptance_proof.{field} expected {expected_count} but computed {actual_count}"
                    ),
                );
            }
        }

        if !self.acceptance_proof.machine_human_vocab_shared {
            push_violation(
                violations,
                "project_doctor.acceptance_vocab_shared",
                &self.pack_id,
                "acceptance_proof.machine_human_vocab_shared must be true",
            );
        }
    }
}

/// Schema references carried by the probe pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorSchemaRefs {
    /// Project Doctor probe-pack schema ref.
    pub probe_schema: String,
    /// Project Doctor finding schema ref.
    pub finding_schema: String,
    /// Existing support probe-catalog schema ref.
    pub support_probe_catalog_schema: String,
    /// Existing support doctor-probe descriptor schema ref.
    pub support_doctor_probe_schema: String,
    /// Existing support doctor-finding schema ref.
    pub support_doctor_finding_schema: String,
}

/// Default execution policy for diagnosis probes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorExecutionPolicy {
    /// Whether all probes are read-only unless explicitly handed off.
    pub read_only_by_default: bool,
    /// Mutability classes that may run inside diagnosis.
    pub permitted_mutability_classes: Vec<String>,
    /// Admission classes that may run inside diagnosis.
    pub permitted_admission_classes: Vec<String>,
    /// Side effects forbidden during diagnosis.
    pub forbidden_during_diagnosis: Vec<String>,
    /// Local evidence or preview writes allowed by the pack.
    pub allowed_local_writes: Vec<String>,
    /// Policy for unknown or unsupported states.
    pub unknown_state_policy: String,
    /// Policy for repair handoff instead of inline mutation.
    pub repair_handoff_policy: String,
}

/// Output routing shared by headless, human, support, and escalation paths.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorOutputRouting {
    /// Headless JSON output ref.
    pub machine_readable_output_ref: String,
    /// Human summary output ref.
    pub human_readable_output_ref: String,
    /// Shared finding vocabulary ref.
    pub shared_finding_vocabulary_ref: String,
    /// Default redaction class for output rows.
    pub default_redaction_class: String,
    /// Default support-pack inclusion class for output rows.
    pub default_support_pack_inclusion_class: String,
    /// Route classes this pack may emit.
    pub route_classes: Vec<String>,
}

/// One shared finding-vocabulary row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorFindingVocabularyEntry {
    /// Stable finding code.
    pub finding_code: String,
    /// Machine-readable severity class.
    pub severity_class: String,
    /// Confidence class for the finding.
    pub confidence_class: String,
    /// Human-display severity class.
    pub display_severity_class: String,
    /// Repair availability state.
    pub repair_availability_class: String,
    /// Unsupported-state class, or `none`.
    pub unsupported_state_class: String,
    /// Text key for human summary copy.
    pub summary_key: String,
    /// Text key for human next-action copy.
    pub next_action_key: String,
}

/// One read-only Project Doctor probe row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorProbe {
    /// Schema version for the probe row.
    pub project_doctor_probe_schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable probe id.
    pub probe_id: String,
    /// Stable probe code.
    pub probe_code: String,
    /// Probe-family class from the support vocabulary.
    pub probe_family_class: String,
    /// Probe implementation version ref.
    pub probe_version: String,
    /// Lifecycle status.
    pub lifecycle_status: String,
    /// Explicit target scope for the probe.
    pub target_scope: ProjectDoctorTargetScope,
    /// Probe class from the support vocabulary.
    pub probe_class: String,
    /// Default diagnosis posture.
    pub diagnosis_posture_default: String,
    /// Invocation policy.
    pub invocation_policy: String,
    /// Whether this probe is read-only by default.
    pub read_only_default: bool,
    /// Mutability class.
    pub mutability_class: String,
    /// Doctor admission class.
    pub doctor_admission_class: String,
    /// Side effects allowed inside diagnosis.
    pub allowed_side_effects: Vec<String>,
    /// Side effects forbidden inside diagnosis.
    pub forbidden_side_effects: Vec<String>,
    /// Evidence routes produced by this probe.
    pub evidence_routes: Vec<ProjectDoctorEvidenceRoute>,
    /// Machine-readable output contract.
    pub output_contract: ProjectDoctorProbeOutputContract,
    /// Repair or escalation handoff.
    pub repair_handoff: ProjectDoctorRepairHandoff,
    /// Human-output text keys.
    pub human_output: ProjectDoctorHumanOutputContract,
    /// Source contract refs this row consumes.
    pub source_contract_refs: Vec<String>,
}

/// Target scope for a Project Doctor probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorTargetScope {
    /// Scope class.
    pub scope_class: String,
    /// Opaque scope ref.
    pub scope_ref: Option<String>,
    /// Support contexts in which this scope is valid.
    pub support_context_classes: Vec<String>,
}

/// Redaction-safe evidence route for one probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorEvidenceRoute {
    /// Stable evidence key.
    pub evidence_key: String,
    /// Evidence source class.
    pub source_class: String,
    /// Signal class.
    pub signal_class: String,
    /// Diagnostic data class.
    pub data_class: String,
    /// Support redaction class.
    pub redaction_class: String,
    /// Support-pack inclusion class.
    pub support_pack_inclusion_class: String,
    /// Replayability class.
    pub replayability_class: String,
    /// Output route classes that may carry this evidence.
    pub output_route_classes: Vec<String>,
    /// Schema or contract ref for the evidence.
    pub evidence_ref_contract: String,
}

/// Machine-readable output contract for one probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorProbeOutputContract {
    /// Finding codes the probe may emit.
    pub finding_codes: Vec<String>,
    /// Finding code used when evidence is insufficient.
    pub unknown_finding_code: String,
    /// Finding code used when the probe cannot run in the current context.
    pub unsupported_finding_code: String,
    /// Headless exit-code class.
    pub headless_exit_code_class: String,
    /// Fields emitted by machine-readable output.
    pub machine_readable_fields: Vec<String>,
}

/// Repair or escalation handoff for one probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorRepairHandoff {
    /// Repair availability class.
    pub repair_availability_class: String,
    /// Repair candidate ids.
    pub repair_candidate_ids: Vec<String>,
    /// Optional repair-preview ref.
    pub repair_preview_ref: Option<String>,
    /// Optional repair-transaction schema ref.
    pub repair_transaction_schema_ref: Option<String>,
    /// Optional runbook ref.
    pub runbook_ref: Option<String>,
    /// Optional support-bundle ref.
    pub support_bundle_ref: Option<String>,
    /// Optional escalation-packet ref.
    pub escalation_packet_ref: Option<String>,
}

/// Human-output text-key contract for one probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorHumanOutputContract {
    /// Title text key.
    pub title_key: String,
    /// Summary text key.
    pub summary_key: String,
    /// Next-action text key.
    pub next_action_key: String,
    /// Unsupported-state text key.
    pub unsupported_state_key: String,
}

/// Acceptance counters embedded in the pack artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorAcceptanceProof {
    /// Number of read-only probes.
    pub read_only_probe_count: usize,
    /// Number of probes with a reviewed repair available.
    pub repair_available_count: usize,
    /// Number of probes whose repair path is preview-only.
    pub preview_only_count: usize,
    /// Number of finding vocabulary rows with explicit unsupported state.
    pub unsupported_state_count: usize,
    /// Whether machine and human outputs share vocabulary.
    pub machine_human_vocab_shared: bool,
}

/// Headless support/export projection derived from the shared vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorMachineOutput {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Output ref.
    pub output_ref: String,
    /// Shared finding-vocabulary ref.
    pub shared_finding_vocabulary_ref: String,
    /// Machine finding rows.
    pub finding_rows: Vec<ProjectDoctorMachineFindingRow>,
}

/// One machine-readable finding row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorMachineFindingRow {
    /// Stable finding code.
    pub finding_code: String,
    /// Machine-readable severity class.
    pub severity_class: String,
    /// Confidence class.
    pub confidence_class: String,
    /// Repair availability class.
    pub repair_availability_class: String,
    /// Unsupported-state class.
    pub unsupported_state_class: String,
}

/// Human summary projection derived from the shared vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorHumanOutput {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Output ref.
    pub output_ref: String,
    /// Shared finding-vocabulary ref.
    pub shared_finding_vocabulary_ref: String,
    /// Human summary rows.
    pub summary_rows: Vec<ProjectDoctorHumanSummaryRow>,
}

/// One human-readable summary row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorHumanSummaryRow {
    /// Stable finding code.
    pub finding_code: String,
    /// Human-display severity class.
    pub display_severity_class: String,
    /// Summary text key.
    pub summary_key: String,
    /// Next-action text key.
    pub next_action_key: String,
    /// Repair availability class.
    pub repair_availability_class: String,
    /// Unsupported-state class.
    pub unsupported_state_class: String,
}

/// Validation failure emitted by [`ProjectDoctorProbePack::validate`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectDoctorPackViolation {
    /// Stable check id.
    pub check_id: String,
    /// Subject ref that failed the check.
    pub subject_ref: String,
    /// Reviewer-facing failure message.
    pub message: String,
}

fn validate_probe(
    probe: &ProjectDoctorProbe,
    finding_codes: &BTreeSet<&str>,
    violations: &mut Vec<ProjectDoctorPackViolation>,
) {
    if probe.record_kind != PROJECT_DOCTOR_PROBE_RECORD_KIND {
        push_violation(
            violations,
            "project_doctor.probe_record_kind",
            &probe.probe_id,
            "probe record_kind must be project_doctor_probe_record",
        );
    }
    if probe.project_doctor_probe_schema_version != 1 {
        push_violation(
            violations,
            "project_doctor.probe_schema_version",
            &probe.probe_id,
            "project_doctor_probe_schema_version must be 1",
        );
    }
    if !probe.read_only_default {
        push_violation(
            violations,
            "project_doctor.probe_read_only_default",
            &probe.probe_id,
            "probe.read_only_default must be true",
        );
    }
    if !matches!(
        probe.mutability_class.as_str(),
        "non_mutating_read_only" | "metadata_write_local_evidence_only"
    ) {
        push_violation(
            violations,
            "project_doctor.probe_mutability_not_read_only",
            &probe.probe_id,
            "probe mutability must be read-only or metadata evidence only",
        );
    }
    if !matches!(
        probe.doctor_admission_class.as_str(),
        "admitted_safe_probe" | "admitted_metadata_evidence_only"
    ) {
        push_violation(
            violations,
            "project_doctor.probe_admission_not_safe",
            &probe.probe_id,
            "probe admission must be safe or metadata evidence only",
        );
    }

    let mut contexts = probe
        .target_scope
        .support_context_classes
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    contexts.sort_unstable();
    if contexts != EXPECTED_CONTEXTS {
        push_violation(
            violations,
            "project_doctor.probe_context_coverage",
            &probe.probe_id,
            "probe target scope must name desktop, cli_headless, remote_managed, and offline_local",
        );
    }

    for required in REQUIRED_FORBIDDEN_SIDE_EFFECTS {
        if !probe
            .forbidden_side_effects
            .iter()
            .any(|effect| effect == required)
        {
            push_violation(
                violations,
                "project_doctor.probe_missing_forbidden_side_effect",
                &probe.probe_id,
                format!("probe must forbid {required}"),
            );
        }
    }

    for evidence in &probe.evidence_routes {
        if matches!(evidence.data_class.as_str(), "code_adjacent" | "high_risk") {
            push_violation(
                violations,
                "project_doctor.evidence_data_class_not_default_safe",
                &probe.probe_id,
                format!(
                    "{} must stay metadata_only or environment_adjacent",
                    evidence.evidence_key
                ),
            );
        }
        if evidence.redaction_class != "metadata_safe_default" {
            push_violation(
                violations,
                "project_doctor.evidence_redaction_not_default_safe",
                &probe.probe_id,
                format!(
                    "{} must route through metadata_safe_default redaction",
                    evidence.evidence_key
                ),
            );
        }
        if !evidence
            .output_route_classes
            .iter()
            .any(|route| route == "doctor_headless_json")
        {
            push_violation(
                violations,
                "project_doctor.evidence_missing_headless_route",
                &probe.probe_id,
                format!("{} must route to headless JSON", evidence.evidence_key),
            );
        }
        if !evidence
            .output_route_classes
            .iter()
            .any(|route| route == "support_bundle_manifest_ref")
        {
            push_violation(
                violations,
                "project_doctor.evidence_missing_support_route",
                &probe.probe_id,
                format!(
                    "{} must route through support-bundle refs",
                    evidence.evidence_key
                ),
            );
        }
    }

    for finding_code in probe
        .output_contract
        .finding_codes
        .iter()
        .chain(std::iter::once(&probe.output_contract.unknown_finding_code))
        .chain(std::iter::once(
            &probe.output_contract.unsupported_finding_code,
        ))
    {
        if !finding_codes.contains(finding_code.as_str()) {
            push_violation(
                violations,
                "project_doctor.finding_code_not_in_vocabulary",
                &probe.probe_id,
                format!("{finding_code} is not present in finding_vocabulary"),
            );
        }
    }

    if probe.repair_handoff.repair_availability_class == "unsupported"
        && !probe.repair_handoff.repair_candidate_ids.is_empty()
    {
        push_violation(
            violations,
            "project_doctor.unsupported_repair_has_candidate",
            &probe.probe_id,
            "unsupported repair handoff must not name repair candidates",
        );
    }
}

fn push_violation(
    violations: &mut Vec<ProjectDoctorPackViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(ProjectDoctorPackViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}
