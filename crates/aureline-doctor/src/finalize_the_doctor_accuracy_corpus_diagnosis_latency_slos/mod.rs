//! Finalized Doctor accuracy corpus, diagnosis-latency SLOs, and headless/UI
//! parity on stable profiles.
//!
//! This module promotes the stable Project Doctor lane from
//! [`stabilize_project_doctor_probes_finding_codes_explainability_and`] into a
//! measurable, benchmark-gated system. It owns three typed records:
//!
//! - [`DoctorAccuracyCorpus`] — seeded ground-truth records that define the
//!   expected finding for each supported scenario so accuracy can be measured
//!   against a frozen baseline rather than ad-hoc reviewer memory.
//! - [`DiagnosisLatencySloCatalog`] — percentile-based latency budgets per
//!   scenario and measurement surface, with explicit threshold states and waiver
//!   hooks.
//! - [`StableProfileParityAudit`] — exactly four parity rows per stable
//!   profile (one per support context) so headless/UI parity gaps are visible
//!   and testable rather than implied by omission.
//!
//! The [`ProjectDoctorFinalizeEvaluator`] validates the corpus, latency
//! catalog, and parity audits, and folds them into a metadata-safe
//! [`ProjectDoctorFinalizeSupportPacket`] that includes benchmark-lab trace
//! refs and corpus metadata.
//!
//! The boundary schema is at
//! `/schemas/support/finalize_the_doctor_accuracy_corpus_diagnosis_latency_slos.schema.json`.

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stabilize_project_doctor_probes_finding_codes_explainability_and::{
    ExplainabilityFactorClass, StableFindingConfidenceClass, StableFindingSeverityClass,
    StableProbePackClass, StableSupportContextClass, DOCTOR_FINDING_PREFIX,
};

/// Frozen schema version for finalized Doctor accuracy, latency, and parity
/// records.
pub const PROJECT_DOCTOR_FINALIZE_SCHEMA_VERSION: u32 = 1;

/// Record-kind tag for the accuracy corpus.
pub const PROJECT_DOCTOR_ACCURACY_CORPUS_RECORD_KIND: &str =
    "project_doctor_accuracy_corpus_record";

/// Record-kind tag for the latency SLO catalog.
pub const PROJECT_DOCTOR_LATENCY_SLO_CATALOG_RECORD_KIND: &str =
    "project_doctor_latency_slo_catalog_record";

/// Record-kind tag for the stable-profile parity audit.
pub const PROJECT_DOCTOR_STABLE_PROFILE_PARITY_AUDIT_RECORD_KIND: &str =
    "project_doctor_stable_profile_parity_audit_record";

/// Record-kind tag for the finalized support-export packet.
pub const PROJECT_DOCTOR_FINALIZE_SUPPORT_PACKET_RECORD_KIND: &str =
    "project_doctor_finalize_support_packet_record";

/// Repo-relative path of the boundary schema mirrored by this module.
pub const PROJECT_DOCTOR_FINALIZE_SCHEMA_REF: &str =
    "schemas/support/finalize_the_doctor_accuracy_corpus_diagnosis_latency_slos.schema.json";

/// Reviewer doc ref quoted by every emitted packet.
pub const PROJECT_DOCTOR_FINALIZE_DOC_REF: &str =
    "docs/support/m4/finalize_the_doctor_accuracy_corpus_diagnosis_latency_slos.md";

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// Closed scenario vocabulary. Each variant names one seeded support scenario
/// that the accuracy corpus covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccuracyCorpusScenarioClass {
    /// Missing or broken toolchain scenario.
    MissingToolchain,
    /// Trust state that blocks workspace open.
    BlockedTrustState,
    /// Filesystem watcher failure or reseed required.
    BrokenWatcher,
    /// Cache or profile incompatible with current build.
    IncompatibleCacheProfile,
    /// Extension regression causing degraded or blocked behavior.
    ExtensionRegression,
    /// Wrong target or route environment selected.
    WrongTargetEnvironment,
    /// Helper attach failure requiring re-approval.
    FailedHelperAttach,
    /// Degraded docs or mirror pack.
    DegradedDocsMirror,
}

impl AccuracyCorpusScenarioClass {
    /// Returns every scenario in catalog order.
    pub const fn all() -> [Self; 8] {
        [
            Self::MissingToolchain,
            Self::BlockedTrustState,
            Self::BrokenWatcher,
            Self::IncompatibleCacheProfile,
            Self::ExtensionRegression,
            Self::WrongTargetEnvironment,
            Self::FailedHelperAttach,
            Self::DegradedDocsMirror,
        ]
    }

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingToolchain => "missing_toolchain",
            Self::BlockedTrustState => "blocked_trust_state",
            Self::BrokenWatcher => "broken_watcher",
            Self::IncompatibleCacheProfile => "incompatible_cache_profile",
            Self::ExtensionRegression => "extension_regression",
            Self::WrongTargetEnvironment => "wrong_target_environment",
            Self::FailedHelperAttach => "failed_helper_attach",
            Self::DegradedDocsMirror => "degraded_docs_mirror",
        }
    }
}

impl fmt::Display for AccuracyCorpusScenarioClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Closed measurement-surface vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MeasurementSurfaceClass {
    /// Local interactive Doctor probe run.
    DoctorProbeRunLocal,
    /// Headless / CLI Doctor probe run.
    DoctorProbeRunHeadless,
    /// Inspector-driven local Doctor probe run.
    DoctorProbeRunInspector,
    /// Managed-profile headless Doctor probe run.
    DoctorProbeRunManaged,
}

impl MeasurementSurfaceClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DoctorProbeRunLocal => "doctor_probe_run_local",
            Self::DoctorProbeRunHeadless => "doctor_probe_run_headless",
            Self::DoctorProbeRunInspector => "doctor_probe_run_inspector",
            Self::DoctorProbeRunManaged => "doctor_probe_run_managed",
        }
    }
}

/// Closed metric-family vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricFamilyClass {
    /// Time-to-first-diagnosis latency.
    DiagnosisLatency,
    /// Confusion-matrix accuracy (false-positive / false-negative budgets).
    FindingAccuracy,
    /// Rate of over-claim or under-claim in repair safety.
    FalseSafeRepairRate,
    /// Escalation packet completeness pass rate.
    EscalationPacketCompleteness,
}

impl MetricFamilyClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiagnosisLatency => "diagnosis_latency",
            Self::FindingAccuracy => "finding_accuracy",
            Self::FalseSafeRepairRate => "false_safe_repair_rate",
            Self::EscalationPacketCompleteness => "escalation_packet_completeness",
        }
    }
}

/// Closed threshold-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThresholdStateClass {
    /// Threshold not yet set; shape-only until benchmark council defines it.
    ToBeSetByBenchmarkCouncil,
    /// Must complete within the stated diagnosis-latency budget.
    MustCompleteUnderDiagnosisLatencyBudget,
    /// Must not exceed the false-positive budget.
    MustNotExceedFalsePositiveBudget,
    /// Must not claim exact rollback without evidence.
    MustNotClaimExactRollbackWithoutEvidence,
    /// Must export a complete escalation packet.
    MustExportCompleteEscalationPacket,
}

impl ThresholdStateClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ToBeSetByBenchmarkCouncil => "to_be_set_by_benchmark_council",
            Self::MustCompleteUnderDiagnosisLatencyBudget => {
                "must_complete_under_diagnosis_latency_budget"
            }
            Self::MustNotExceedFalsePositiveBudget => "must_not_exceed_false_positive_budget",
            Self::MustNotClaimExactRollbackWithoutEvidence => {
                "must_not_claim_exact_rollback_without_evidence"
            }
            Self::MustExportCompleteEscalationPacket => "must_export_complete_escalation_packet",
        }
    }
}

/// Latency percentile vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LatencyPercentileClass {
    /// 50th percentile.
    P50,
    /// 90th percentile.
    P90,
    /// 95th percentile.
    P95,
}

impl LatencyPercentileClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::P50 => "p50",
            Self::P90 => "p90",
            Self::P95 => "p95",
        }
    }
}

/// Headless/UI parity-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParityClass {
    /// Full parity: all UI affordances and machine-readable fields present.
    FullParity,
    /// Machine-readable fields only; no UI rendered.
    MachineReadableOnlyNoUi,
    /// UI suppressed because consent is required.
    UiSuppressedConsentRequired,
    /// UI suppressed because the context is unsupported.
    UiSuppressedUnsupported,
    /// UI suppressed because managed authority is required.
    UiSuppressedManagedAuthorityRequired,
    /// Unavailable in this context.
    UnavailableInContext,
}

impl ParityClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullParity => "full_parity",
            Self::MachineReadableOnlyNoUi => "machine_readable_only_no_ui",
            Self::UiSuppressedConsentRequired => "ui_suppressed_consent_required",
            Self::UiSuppressedUnsupported => "ui_suppressed_unsupported",
            Self::UiSuppressedManagedAuthorityRequired => {
                "ui_suppressed_managed_authority_required"
            }
            Self::UnavailableInContext => "unavailable_in_context",
        }
    }
}

/// Headless exit-code vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeadlessExitCodeClass {
    /// Clean exit with no findings.
    ExitCleanNoFindings,
    /// Findings emitted but advisory only.
    ExitFindingsAdvisoryOnly,
    /// Actionable findings emitted.
    ExitFindingsActionable,
    /// Unsupported context.
    ExitUnsupportedContext,
    /// Blocked because consent is required.
    ExitBlockedConsentRequired,
    /// Probe runtime error.
    ExitProbeRuntimeError,
}

impl HeadlessExitCodeClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExitCleanNoFindings => "exit_clean_no_findings",
            Self::ExitFindingsAdvisoryOnly => "exit_findings_advisory_only",
            Self::ExitFindingsActionable => "exit_findings_actionable",
            Self::ExitUnsupportedContext => "exit_unsupported_context",
            Self::ExitBlockedConsentRequired => "exit_blocked_consent_required",
            Self::ExitProbeRuntimeError => "exit_probe_runtime_error",
        }
    }
}

/// Capability-lifecycle vocabulary for parity rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnimplementedCapabilityClass {
    /// Fully implemented.
    Implemented,
    /// Not yet implemented but planned.
    NotYetImplementedPlanned,
    /// Not yet implemented and descoped.
    NotYetImplementedDescoped,
    /// Deprecated and will be removed.
    DeprecatedWillRemove,
    /// Permanently unsupported.
    PermanentlyUnsupported,
}

impl UnimplementedCapabilityClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Implemented => "implemented",
            Self::NotYetImplementedPlanned => "not_yet_implemented_planned",
            Self::NotYetImplementedDescoped => "not_yet_implemented_descoped",
            Self::DeprecatedWillRemove => "deprecated_will_remove",
            Self::PermanentlyUnsupported => "permanently_unsupported",
        }
    }
}

/// Repair-class vocabulary for ground-truth records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroundTruthRepairClass {
    /// No repair; observation only.
    ObserveOnlyNoRepair,
    /// Reacquire trust approval.
    ReacquireTrustApproval,
    /// Restart watcher with reseed.
    RestartWatcherWithReseed,
    /// Reset ephemeral cache.
    ResetEphemeralCache,
    /// Install or repair toolchain.
    InstallOrRepairToolchain,
    /// Quarantine and bisect extension.
    QuarantineAndBisectExtension,
    /// Reapprove target or route.
    ReapproveTargetOrRoute,
    /// Reattach helper with new approval.
    ReattachHelperWithNewApproval,
    /// Refresh docs or mirror pack.
    RefreshDocsOrMirrorPack,
    /// Defer to escalation packet.
    DeferToEscalationPacket,
}

impl GroundTruthRepairClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ObserveOnlyNoRepair => "observe_only_no_repair",
            Self::ReacquireTrustApproval => "reacquire_trust_approval",
            Self::RestartWatcherWithReseed => "restart_watcher_with_reseed",
            Self::ResetEphemeralCache => "reset_ephemeral_cache",
            Self::InstallOrRepairToolchain => "install_or_repair_toolchain",
            Self::QuarantineAndBisectExtension => "quarantine_and_bisect_extension",
            Self::ReapproveTargetOrRoute => "reapprove_target_or_route",
            Self::ReattachHelperWithNewApproval => "reattach_helper_with_new_approval",
            Self::RefreshDocsOrMirrorPack => "refresh_docs_or_mirror_pack",
            Self::DeferToEscalationPacket => "defer_to_escalation_packet",
        }
    }
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// One explainability-factor assertion expected in a ground-truth record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroundTruthExplainabilityAssertion {
    /// The explainability factor class required.
    pub factor_class: ExplainabilityFactorClass,
    /// Reviewable sentence describing the expected assertion content.
    pub expected_assertion_summary: String,
}

/// One ground-truth record in the accuracy corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroundTruthRecord {
    /// Unique row id.
    pub row_id: String,
    /// Scenario this ground truth describes.
    pub scenario_class: AccuracyCorpusScenarioClass,
    /// Probe pack class responsible for this scenario.
    pub probe_pack_class: StableProbePackClass,
    /// Expected finding code (must start with `doctor.finding.`).
    pub expected_finding_code: String,
    /// Expected severity.
    pub expected_severity: StableFindingSeverityClass,
    /// Expected confidence.
    pub expected_confidence: StableFindingConfidenceClass,
    /// Expected repair class.
    pub expected_repair_class: GroundTruthRepairClass,
    /// Expected explainability factors.
    pub expected_explainability: Vec<GroundTruthExplainabilityAssertion>,
    /// Expected no-touch boundaries.
    pub expected_no_touch_boundaries: Vec<String>,
    /// True when the scenario includes at least one observe-only outcome.
    pub has_observe_only_outcome: bool,
    /// Reviewable notes.
    pub notes: String,
}

/// One diagnosis-latency budget entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosisLatencyBudget {
    /// Percentile for this budget.
    pub percentile: LatencyPercentileClass,
    /// Target in milliseconds.
    pub target_ms: u64,
    /// Yellow threshold in milliseconds.
    pub yellow_ms: u64,
    /// Red threshold in milliseconds.
    pub red_ms: u64,
}

/// One latency SLO row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LatencyBudgetRow {
    /// Unique row id.
    pub row_id: String,
    /// Metric family (always diagnosis_latency for rows in this catalog).
    pub metric_family: MetricFamilyClass,
    /// Scenario this budget covers.
    pub scenario_class: AccuracyCorpusScenarioClass,
    /// Measurement surface.
    pub measurement_surface: MeasurementSurfaceClass,
    /// Latency budgets (one or more percentiles).
    pub budgets: Vec<DiagnosisLatencyBudget>,
    /// Current threshold state.
    pub threshold_state: ThresholdStateClass,
    /// Primary owner lane.
    pub primary_dri_lane: String,
    /// Co-review lanes.
    pub co_review_lanes: Vec<String>,
    /// Reviewable notes.
    pub notes: String,
}

/// One headless/UI parity row for a single support context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParityAuditRow {
    /// Support context class for this row.
    pub support_context_class: StableSupportContextClass,
    /// Parity class.
    pub parity_class: ParityClass,
    /// Machine-readable result fields emitted in this context.
    pub machine_readable_result_fields: Vec<String>,
    /// UI affordances suppressed in this context.
    pub suppressed_ui_affordances: Vec<String>,
    /// Capability lifecycle status.
    pub unimplemented_capability_class: UnimplementedCapabilityClass,
    /// Headless exit code class.
    pub headless_exit_code_class: HeadlessExitCodeClass,
    /// Reviewable notes describing the parity decision.
    pub notes: String,
}

/// Parity audit for one stable profile. Must contain exactly four rows,
/// one per support context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableProfileParityAudit {
    /// Unique audit id.
    pub audit_id: String,
    /// Profile name (e.g., "desktop_stable", "cli_headless_stable").
    pub profile_name: String,
    /// Schema version.
    pub schema_version: u32,
    /// Record kind.
    pub record_kind: String,
    /// The four parity rows.
    pub parity_rows: Vec<ParityAuditRow>,
    /// Reviewable notes.
    pub notes: String,
}

/// Accuracy corpus top-level record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorAccuracyCorpus {
    /// Schema version.
    pub schema_version: u32,
    /// Record kind.
    pub record_kind: String,
    /// Corpus id.
    pub corpus_id: String,
    /// Corpus version.
    pub corpus_version: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Doc ref.
    pub doc_ref: String,
    /// Ground truth records.
    pub ground_truth_records: Vec<GroundTruthRecord>,
    /// Reviewable notes.
    pub notes: String,
}

/// Diagnosis-latency SLO catalog top-level record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosisLatencySloCatalog {
    /// Schema version.
    pub schema_version: u32,
    /// Record kind.
    pub record_kind: String,
    /// Catalog id.
    pub catalog_id: String,
    /// Catalog version.
    pub catalog_version: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Doc ref.
    pub doc_ref: String,
    /// Latency budget rows.
    pub latency_rows: Vec<LatencyBudgetRow>,
    /// Reviewable notes.
    pub notes: String,
}

/// Benchmark-lab trace reference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchmarkLabTraceRef {
    /// Trace id.
    pub trace_id: String,
    /// Lab run timestamp.
    pub run_at: String,
    /// Profile under test.
    pub profile_name: String,
    /// Scenario tested.
    pub scenario_class: AccuracyCorpusScenarioClass,
    /// Surface tested.
    pub measurement_surface: MeasurementSurfaceClass,
    /// Observed latency in milliseconds.
    pub observed_latency_ms: u64,
    /// Whether the observed latency passed the budget.
    pub passed: bool,
}

/// Corpus metadata for the support packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorpusMetadata {
    /// Total ground-truth records.
    pub ground_truth_count: usize,
    /// Count of observe-only outcomes.
    pub observe_only_count: usize,
    /// Count of repair-candidate outcomes.
    pub repair_candidate_count: usize,
    /// Scenarios covered.
    pub scenarios_covered: Vec<AccuracyCorpusScenarioClass>,
}

/// Combined finalized support packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorFinalizeSupportPacket {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Packet id.
    pub packet_id: String,
    /// Captured-at timestamp.
    pub captured_at: String,
    /// Doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Corpus id.
    pub corpus_id: String,
    /// Corpus version.
    pub corpus_version: String,
    /// Catalog id.
    pub catalog_id: String,
    /// Catalog version.
    pub catalog_version: String,
    /// Corpus metadata.
    pub corpus_metadata: CorpusMetadata,
    /// Latency row summaries.
    pub latency_row_summaries: Vec<LatencyBudgetRowSummary>,
    /// Parity audit refs.
    pub parity_audit_refs: Vec<String>,
    /// Benchmark-lab trace refs.
    pub benchmark_lab_traces: Vec<BenchmarkLabTraceRef>,
    /// Raw private material excluded.
    pub raw_private_material_excluded: bool,
    /// Ambient authority excluded.
    pub ambient_authority_excluded: bool,
}

/// Summary of one latency budget row for export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LatencyBudgetRowSummary {
    /// Row id.
    pub row_id: String,
    /// Scenario.
    pub scenario_class: AccuracyCorpusScenarioClass,
    /// Surface.
    pub measurement_surface: MeasurementSurfaceClass,
    /// Threshold state.
    pub threshold_state: ThresholdStateClass,
    /// Budget count.
    pub budget_count: usize,
}

/// One validation violation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorFinalizeViolation {
    /// Check id.
    pub check_id: String,
    /// Subject ref.
    pub subject_ref: String,
    /// Human-readable message.
    pub message: String,
}

/// Validation report emitted when a record fails validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorFinalizeValidationReport {
    /// Violations.
    pub violations: Vec<ProjectDoctorFinalizeViolation>,
}

impl ProjectDoctorFinalizeValidationReport {
    /// True when no violations were found.
    pub fn is_valid(&self) -> bool {
        self.violations.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Load helpers
// ---------------------------------------------------------------------------

/// Deserialize an accuracy corpus from YAML.
pub fn load_accuracy_corpus(yaml: &str) -> Result<DoctorAccuracyCorpus, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Deserialize a latency SLO catalog from YAML.
pub fn load_latency_slo_catalog(
    yaml: &str,
) -> Result<DiagnosisLatencySloCatalog, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Deserialize a stable-profile parity audit from YAML.
pub fn load_stable_profile_parity_audit(
    yaml: &str,
) -> Result<StableProfileParityAudit, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

// ---------------------------------------------------------------------------
// Evaluator
// ---------------------------------------------------------------------------

/// Validates the accuracy corpus, latency SLO catalog, and stable-profile
/// parity audits.
pub struct ProjectDoctorFinalizeEvaluator;

impl Default for ProjectDoctorFinalizeEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectDoctorFinalizeEvaluator {
    /// Creates a new evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Validates an accuracy corpus.
    pub fn validate_accuracy_corpus(
        &self,
        corpus: &DoctorAccuracyCorpus,
    ) -> ProjectDoctorFinalizeValidationReport {
        let mut violations = Vec::new();

        if corpus.schema_version != PROJECT_DOCTOR_FINALIZE_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "project_doctor_finalize.corpus_schema_version",
                &corpus.corpus_id,
                "corpus schema_version must be 1",
            );
        }
        if corpus.record_kind != PROJECT_DOCTOR_ACCURACY_CORPUS_RECORD_KIND {
            push_violation(
                &mut violations,
                "project_doctor_finalize.corpus_record_kind",
                &corpus.corpus_id,
                "corpus record_kind must be project_doctor_accuracy_corpus_record",
            );
        }
        if corpus.corpus_id.trim().is_empty() {
            push_violation(
                &mut violations,
                "project_doctor_finalize.corpus_id_empty",
                &corpus.corpus_id,
                "corpus_id must be non-empty",
            );
        }
        if corpus.corpus_version.trim().is_empty() {
            push_violation(
                &mut violations,
                "project_doctor_finalize.corpus_version_empty",
                &corpus.corpus_id,
                "corpus_version must be non-empty",
            );
        }
        if corpus.schema_ref != PROJECT_DOCTOR_FINALIZE_SCHEMA_REF {
            push_violation(
                &mut violations,
                "project_doctor_finalize.corpus_schema_ref",
                &corpus.corpus_id,
                format!("corpus schema_ref must equal {PROJECT_DOCTOR_FINALIZE_SCHEMA_REF}"),
            );
        }
        if corpus.doc_ref != PROJECT_DOCTOR_FINALIZE_DOC_REF {
            push_violation(
                &mut violations,
                "project_doctor_finalize.corpus_doc_ref",
                &corpus.corpus_id,
                format!("corpus doc_ref must equal {PROJECT_DOCTOR_FINALIZE_DOC_REF}"),
            );
        }
        if corpus.ground_truth_records.is_empty() {
            push_violation(
                &mut violations,
                "project_doctor_finalize.corpus_ground_truth_missing",
                &corpus.corpus_id,
                "corpus must contain at least one ground_truth_record",
            );
        }

        let mut seen_row_ids = BTreeMap::new();
        let mut seen_scenarios = std::collections::BTreeSet::new();
        for record in &corpus.ground_truth_records {
            if record.row_id.trim().is_empty() {
                push_violation(
                    &mut violations,
                    "project_doctor_finalize.ground_truth_row_id_empty",
                    &corpus.corpus_id,
                    "ground_truth row_id must be non-empty",
                );
            }
            if seen_row_ids.insert(record.row_id.clone(), ()).is_some() {
                push_violation(
                    &mut violations,
                    "project_doctor_finalize.ground_truth_row_id_duplicate",
                    &record.row_id,
                    "ground_truth row_id must be unique within the corpus",
                );
            }
            if !record
                .expected_finding_code
                .starts_with(DOCTOR_FINDING_PREFIX)
            {
                push_violation(
                    &mut violations,
                    "project_doctor_finalize.ground_truth_finding_code_prefix",
                    &record.row_id,
                    format!("expected_finding_code must start with {DOCTOR_FINDING_PREFIX}"),
                );
            }
            if record.expected_no_touch_boundaries.is_empty() {
                push_violation(
                    &mut violations,
                    "project_doctor_finalize.ground_truth_no_touch_missing",
                    &record.row_id,
                    "ground_truth record must declare at least one no_touch_boundary",
                );
            }
            if record.expected_explainability.is_empty() {
                push_violation(
                    &mut violations,
                    "project_doctor_finalize.ground_truth_explainability_missing",
                    &record.row_id,
                    "ground_truth record must declare at least one expected_explainability factor",
                );
            }
            seen_scenarios.insert(record.scenario_class);
        }

        let all_scenarios = AccuracyCorpusScenarioClass::all();
        for scenario in &all_scenarios {
            if !seen_scenarios.contains(scenario) {
                push_violation(
                    &mut violations,
                    "project_doctor_finalize.corpus_scenario_incomplete",
                    &corpus.corpus_id,
                    format!(
                        "corpus is missing ground truth for scenario {}",
                        scenario.as_str()
                    ),
                );
            }
        }

        ProjectDoctorFinalizeValidationReport { violations }
    }

    /// Validates a latency SLO catalog.
    pub fn validate_latency_slo_catalog(
        &self,
        catalog: &DiagnosisLatencySloCatalog,
    ) -> ProjectDoctorFinalizeValidationReport {
        let mut violations = Vec::new();

        if catalog.schema_version != PROJECT_DOCTOR_FINALIZE_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "project_doctor_finalize.latency_schema_version",
                &catalog.catalog_id,
                "latency catalog schema_version must be 1",
            );
        }
        if catalog.record_kind != PROJECT_DOCTOR_LATENCY_SLO_CATALOG_RECORD_KIND {
            push_violation(
                &mut violations,
                "project_doctor_finalize.latency_record_kind",
                &catalog.catalog_id,
                "latency catalog record_kind must be project_doctor_latency_slo_catalog_record",
            );
        }
        if catalog.catalog_id.trim().is_empty() {
            push_violation(
                &mut violations,
                "project_doctor_finalize.latency_catalog_id_empty",
                &catalog.catalog_id,
                "catalog_id must be non-empty",
            );
        }
        if catalog.catalog_version.trim().is_empty() {
            push_violation(
                &mut violations,
                "project_doctor_finalize.latency_catalog_version_empty",
                &catalog.catalog_id,
                "catalog_version must be non-empty",
            );
        }
        if catalog.schema_ref != PROJECT_DOCTOR_FINALIZE_SCHEMA_REF {
            push_violation(
                &mut violations,
                "project_doctor_finalize.latency_schema_ref",
                &catalog.catalog_id,
                format!(
                    "latency catalog schema_ref must equal {PROJECT_DOCTOR_FINALIZE_SCHEMA_REF}"
                ),
            );
        }
        if catalog.doc_ref != PROJECT_DOCTOR_FINALIZE_DOC_REF {
            push_violation(
                &mut violations,
                "project_doctor_finalize.latency_doc_ref",
                &catalog.catalog_id,
                format!("latency catalog doc_ref must equal {PROJECT_DOCTOR_FINALIZE_DOC_REF}"),
            );
        }
        if catalog.latency_rows.is_empty() {
            push_violation(
                &mut violations,
                "project_doctor_finalize.latency_rows_missing",
                &catalog.catalog_id,
                "latency catalog must contain at least one latency_row",
            );
        }

        let mut seen_row_ids = BTreeMap::new();
        for row in &catalog.latency_rows {
            if row.row_id.trim().is_empty() {
                push_violation(
                    &mut violations,
                    "project_doctor_finalize.latency_row_id_empty",
                    &catalog.catalog_id,
                    "latency row_id must be non-empty",
                );
            }
            if seen_row_ids.insert(row.row_id.clone(), ()).is_some() {
                push_violation(
                    &mut violations,
                    "project_doctor_finalize.latency_row_id_duplicate",
                    &row.row_id,
                    "latency row_id must be unique within the catalog",
                );
            }
            if row.budgets.is_empty() {
                push_violation(
                    &mut violations,
                    "project_doctor_finalize.latency_budgets_empty",
                    &row.row_id,
                    "latency row must declare at least one budget",
                );
            }
            for budget in &row.budgets {
                if budget.target_ms == 0 {
                    push_violation(
                        &mut violations,
                        "project_doctor_finalize.latency_budget_target_zero",
                        &row.row_id,
                        "latency budget target_ms must be non-zero",
                    );
                }
                if budget.yellow_ms <= budget.target_ms {
                    push_violation(
                        &mut violations,
                        "project_doctor_finalize.latency_budget_yellow_not_above_target",
                        &row.row_id,
                        "latency budget yellow_ms must be greater than target_ms",
                    );
                }
                if budget.red_ms <= budget.yellow_ms {
                    push_violation(
                        &mut violations,
                        "project_doctor_finalize.latency_budget_red_not_above_yellow",
                        &row.row_id,
                        "latency budget red_ms must be greater than yellow_ms",
                    );
                }
            }
        }

        ProjectDoctorFinalizeValidationReport { violations }
    }

    /// Validates a stable-profile parity audit.
    pub fn validate_stable_profile_parity_audit(
        &self,
        audit: &StableProfileParityAudit,
    ) -> ProjectDoctorFinalizeValidationReport {
        let mut violations = Vec::new();

        if audit.schema_version != PROJECT_DOCTOR_FINALIZE_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "project_doctor_finalize.parity_schema_version",
                &audit.audit_id,
                "parity audit schema_version must be 1",
            );
        }
        if audit.record_kind != PROJECT_DOCTOR_STABLE_PROFILE_PARITY_AUDIT_RECORD_KIND {
            push_violation(
                &mut violations,
                "project_doctor_finalize.parity_record_kind",
                &audit.audit_id,
                "parity audit record_kind must be project_doctor_stable_profile_parity_audit_record",
            );
        }
        if audit.audit_id.trim().is_empty() {
            push_violation(
                &mut violations,
                "project_doctor_finalize.parity_audit_id_empty",
                &audit.audit_id,
                "audit_id must be non-empty",
            );
        }
        if audit.profile_name.trim().is_empty() {
            push_violation(
                &mut violations,
                "project_doctor_finalize.parity_profile_name_empty",
                &audit.audit_id,
                "profile_name must be non-empty",
            );
        }
        if audit.parity_rows.len() != 4 {
            push_violation(
                &mut violations,
                "project_doctor_finalize.parity_row_count",
                &audit.audit_id,
                format!(
                    "parity audit must contain exactly 4 rows (one per support context), found {}",
                    audit.parity_rows.len()
                ),
            );
        }

        let expected_contexts = [
            StableSupportContextClass::Desktop,
            StableSupportContextClass::CliHeadless,
            StableSupportContextClass::RemoteManaged,
            StableSupportContextClass::OfflineLocal,
        ];
        let mut seen_contexts = std::collections::BTreeSet::new();
        for row in &audit.parity_rows {
            if !seen_contexts.insert(row.support_context_class) {
                push_violation(
                    &mut violations,
                    "project_doctor_finalize.parity_duplicate_context",
                    &audit.audit_id,
                    format!(
                        "duplicate parity row for context {}",
                        row.support_context_class.as_str()
                    ),
                );
            }
            let required_fields = [
                "finding_id",
                "finding_code",
                "probe_id",
                "probe_class",
                "diagnosis_posture",
                "exit_code_class",
            ];
            for required in &required_fields {
                if !row
                    .machine_readable_result_fields
                    .iter()
                    .any(|f| f == *required)
                {
                    push_violation(
                        &mut violations,
                        "project_doctor_finalize.parity_missing_machine_field",
                        &audit.audit_id,
                        format!(
                            "parity row for context {} must include machine_readable field {}",
                            row.support_context_class.as_str(),
                            required
                        ),
                    );
                }
            }
        }
        for expected in &expected_contexts {
            if !seen_contexts.contains(expected) {
                push_violation(
                    &mut violations,
                    "project_doctor_finalize.parity_missing_context",
                    &audit.audit_id,
                    format!("parity audit missing row for context {}", expected.as_str()),
                );
            }
        }

        ProjectDoctorFinalizeValidationReport { violations }
    }

    /// Builds a finalized support packet from a validated corpus, latency
    /// catalog, parity audits, and optional benchmark-lab traces.
    ///
    /// Returns [`ProjectDoctorFinalizeValidationReport`] when any input fails
    /// validation.
    pub fn finalize_support_packet(
        &self,
        packet_id: impl Into<String>,
        captured_at: impl Into<String>,
        corpus: &DoctorAccuracyCorpus,
        latency_catalog: &DiagnosisLatencySloCatalog,
        parity_audits: &[StableProfileParityAudit],
        benchmark_lab_traces: &[BenchmarkLabTraceRef],
    ) -> Result<ProjectDoctorFinalizeSupportPacket, ProjectDoctorFinalizeValidationReport> {
        let mut all_violations = Vec::new();

        let corpus_report = self.validate_accuracy_corpus(corpus);
        all_violations.extend(corpus_report.violations);

        let latency_report = self.validate_latency_slo_catalog(latency_catalog);
        all_violations.extend(latency_report.violations);

        for audit in parity_audits {
            let audit_report = self.validate_stable_profile_parity_audit(audit);
            all_violations.extend(audit_report.violations);
        }

        if !all_violations.is_empty() {
            return Err(ProjectDoctorFinalizeValidationReport {
                violations: all_violations,
            });
        }

        let observe_only_count = corpus
            .ground_truth_records
            .iter()
            .filter(|r| r.expected_repair_class == GroundTruthRepairClass::ObserveOnlyNoRepair)
            .count();
        let repair_candidate_count = corpus.ground_truth_records.len() - observe_only_count;
        let scenarios_covered: Vec<_> = corpus
            .ground_truth_records
            .iter()
            .map(|r| r.scenario_class)
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();

        let corpus_metadata = CorpusMetadata {
            ground_truth_count: corpus.ground_truth_records.len(),
            observe_only_count,
            repair_candidate_count,
            scenarios_covered,
        };

        let latency_row_summaries = latency_catalog
            .latency_rows
            .iter()
            .map(|row| LatencyBudgetRowSummary {
                row_id: row.row_id.clone(),
                scenario_class: row.scenario_class,
                measurement_surface: row.measurement_surface,
                threshold_state: row.threshold_state,
                budget_count: row.budgets.len(),
            })
            .collect();

        let parity_audit_refs = parity_audits.iter().map(|a| a.audit_id.clone()).collect();

        Ok(ProjectDoctorFinalizeSupportPacket {
            record_kind: PROJECT_DOCTOR_FINALIZE_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: PROJECT_DOCTOR_FINALIZE_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            captured_at: captured_at.into(),
            doc_ref: PROJECT_DOCTOR_FINALIZE_DOC_REF.to_owned(),
            schema_ref: PROJECT_DOCTOR_FINALIZE_SCHEMA_REF.to_owned(),
            corpus_id: corpus.corpus_id.clone(),
            corpus_version: corpus.corpus_version.clone(),
            catalog_id: latency_catalog.catalog_id.clone(),
            catalog_version: latency_catalog.catalog_version.clone(),
            corpus_metadata,
            latency_row_summaries,
            parity_audit_refs,
            benchmark_lab_traces: benchmark_lab_traces.to_vec(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        })
    }
}

fn push_violation(
    violations: &mut Vec<ProjectDoctorFinalizeViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(ProjectDoctorFinalizeViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}
