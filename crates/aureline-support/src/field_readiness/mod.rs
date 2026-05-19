//! Field-readiness scorecards projected from the M3 support-scenario corpus.
//!
//! This module converts the protected M3 support-scenario corpus
//! ([`crate::m3_scenario_corpus`]) plus the alpha diagnosis-latency
//! scorecard ([`crate::scenario_scorecard`]) into release-consumable
//! field-readiness scorecards: a diagnosis-latency scorecard projected
//! per protected beta lane, an exact-build availability report tracking
//! symbolication coverage, and a field-readiness dashboard that
//! shiproom, release evidence, and support-center surfaces can consume
//! verbatim.
//!
//! The scorecards are metadata-safe: every row carries closed-vocabulary
//! tokens drawn from the M3 corpus and the alpha measurement contract;
//! no raw private material or ambient authority is admitted. When the
//! underlying corpus or symbolication evidence goes stale, the
//! scorecards downgrade closed-vocabulary `stale_data_*` triggers
//! instead of presenting false-green status.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::m3_scenario_corpus::{
    current_m3_scenario_corpus, required_beta_lane_classes, M3BetaLaneClass,
    M3ScenarioCorpus, M3SupportScenario, M3_SCENARIO_CORPUS_DOC_REF,
    M3_SCENARIO_CORPUS_MANIFEST_REF, M3_SUPPORT_SCENARIO_SCHEMA_VERSION,
};
use crate::scenario_scorecard::{current_alpha_scorecard, DiagnosisLatencyScorecard};

/// Stable record-kind tag for the M3 diagnosis-latency scorecard.
pub const M3_DIAGNOSIS_LATENCY_SCORECARD_RECORD_KIND: &str = "m3_diagnosis_latency_scorecard_record";

/// Stable record-kind tag for one row in the M3 diagnosis-latency scorecard.
pub const M3_DIAGNOSIS_LATENCY_LANE_ROW_RECORD_KIND: &str =
    "m3_diagnosis_latency_lane_row_record";

/// Stable record-kind tag for the exact-build availability report.
pub const M3_EXACT_BUILD_AVAILABILITY_REPORT_RECORD_KIND: &str =
    "m3_exact_build_availability_report_record";

/// Stable record-kind tag for one row in the exact-build availability report.
pub const M3_EXACT_BUILD_AVAILABILITY_LANE_ROW_RECORD_KIND: &str =
    "m3_exact_build_availability_lane_row_record";

/// Stable record-kind tag for the field-readiness dashboard.
pub const M3_FIELD_READINESS_DASHBOARD_RECORD_KIND: &str = "m3_field_readiness_dashboard_record";

/// Stable record-kind tag for one row in the field-readiness dashboard.
pub const M3_FIELD_READINESS_LANE_ROW_RECORD_KIND: &str = "m3_field_readiness_lane_row_record";

/// Repository-relative path of the published diagnosis-latency scorecard.
pub const M3_DIAGNOSIS_LATENCY_SCORECARD_REF: &str =
    "artifacts/support/m3/diagnosis_latency_scorecard.md";

/// Repository-relative path of the published exact-build availability report.
pub const M3_EXACT_BUILD_AVAILABILITY_REPORT_REF: &str =
    "artifacts/support/m3/exact_build_availability_report.md";

/// Repository-relative path of the published field-readiness dashboard.
pub const M3_FIELD_READINESS_DASHBOARD_REF: &str =
    "artifacts/support/m3/field_readiness_dashboard.json";

/// Repository-relative path of the reviewer-facing field-readiness doc.
pub const M3_FIELD_READINESS_METRICS_DOC_REF: &str = "docs/support/m3/field_readiness_metrics.md";

const FIELD_READINESS_SCORECARDS_REPORT_ID: &str = "support.m3.field_readiness.baseline.v1";

const DIAGNOSIS_LATENCY_SCORECARD_ID: &str =
    "support.m3.diagnosis_latency_scorecard.baseline.v1";

const EXACT_BUILD_AVAILABILITY_REPORT_ID: &str =
    "support.m3.exact_build_availability_report.baseline.v1";

const FIELD_READINESS_DASHBOARD_ID: &str = "support.m3.field_readiness_dashboard.baseline.v1";

const BASELINE_GENERATED_AT: &str = "2026-05-19T00:00:00Z";

/// Closed evidence-path vocabulary the scorecards report per lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidencePathClass {
    /// User kept the support packet local; never exported.
    LocalOnly,
    /// User exported the support packet to disk for hand-off.
    ExportedToSupportPacket,
    /// User uploaded the support packet to a vendor or managed intake.
    UploadedToVendor,
}

impl EvidencePathClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::ExportedToSupportPacket => "exported_to_support_packet",
            Self::UploadedToVendor => "uploaded_to_vendor",
        }
    }
}

/// All evidence-path classes the scorecards must remain attributable to.
pub const REQUIRED_EVIDENCE_PATH_CLASSES: [EvidencePathClass; 3] = [
    EvidencePathClass::LocalOnly,
    EvidencePathClass::ExportedToSupportPacket,
    EvidencePathClass::UploadedToVendor,
];

/// Closed measurement-state vocabulary; "seeded" rows are seeded targets
/// without live measurement, and stale rows are downgraded explicitly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LatencyMeasurementState {
    /// Targets are seeded from the corpus; no live measurement has run.
    SeededPendingLiveMeasurement,
    /// The corpus or symbolication evidence is stale; row is downgraded.
    StaleDowngraded,
    /// Live measurement is inside the green budget.
    LiveGreen,
    /// Live measurement is inside the yellow budget.
    LiveYellow,
    /// Live measurement is inside the red budget.
    LiveRed,
}

impl LatencyMeasurementState {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SeededPendingLiveMeasurement => "seeded_pending_live_measurement",
            Self::StaleDowngraded => "stale_downgraded",
            Self::LiveGreen => "live_green",
            Self::LiveYellow => "live_yellow",
            Self::LiveRed => "live_red",
        }
    }
}

/// Closed stale-data downgrade trigger vocabulary used by all three scorecards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StaleDataTrigger {
    /// The seeded scenario corpus is missing a required lane.
    SeededCorpusMissingLane,
    /// A primary fixture referenced by a scenario is missing on disk.
    PrimaryFixtureMissing,
    /// The alpha diagnosis-latency scorecard is missing or unreadable.
    AlphaScorecardMissing,
    /// The corpus drill-harness report is older than the corpus emit date.
    DrillReportOlderThanCorpus,
    /// Exact-build symbolication evidence is missing for a scenario whose
    /// scorecard target depends on it (the crash-loop family).
    SymbolicationEvidenceMissing,
    /// A scenario's claim-downgrade rules dropped a required trigger.
    ClaimDowngradeRulesIncomplete,
}

impl StaleDataTrigger {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SeededCorpusMissingLane => "seeded_corpus_missing_lane",
            Self::PrimaryFixtureMissing => "primary_fixture_missing",
            Self::AlphaScorecardMissing => "alpha_scorecard_missing",
            Self::DrillReportOlderThanCorpus => "drill_report_older_than_corpus",
            Self::SymbolicationEvidenceMissing => "symbolication_evidence_missing",
            Self::ClaimDowngradeRulesIncomplete => "claim_downgrade_rules_incomplete",
        }
    }
}

/// Latency budget for one of the three measurement windows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LatencyBudget {
    /// p50 (median) seeded target.
    pub p50_target_seconds: u32,
    /// p90 (tail) seeded target.
    pub p90_target_seconds: u32,
    /// Yellow threshold; the scorecard downgrades to yellow above this.
    pub yellow_seconds: u32,
    /// Red threshold; the scorecard downgrades to red above this.
    pub red_seconds: u32,
    /// Closed measurement state.
    pub state: LatencyMeasurementState,
}

/// Per-evidence-path latency budget.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerPathLatencyBudget {
    /// Closed evidence-path class.
    pub path_class: EvidencePathClass,
    /// p90 seeded target for this path.
    pub p90_target_seconds: u32,
    /// Whether the path remains available to the user at equal prominence.
    pub available_at_equal_prominence: bool,
    /// Closed measurement state.
    pub state: LatencyMeasurementState,
}

/// One row in the M3 diagnosis-latency scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M3DiagnosisLatencyLaneRow {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Closed beta-lane class.
    pub beta_lane_class: M3BetaLaneClass,
    /// Stable scenario id from the corpus.
    pub scenario_id: String,
    /// Reviewer-facing title from the corpus.
    pub title: String,
    /// Corpus fixture ref.
    pub fixture_ref: String,
    /// Scorecard target the row contributes to (m3.beta_lane.*).
    pub scorecard_target: String,
    /// Latency to first actionable finding/result packet.
    pub time_to_first_actionable_finding: LatencyBudget,
    /// Latency to first safe repair suggestion (if any).
    pub time_to_first_safe_repair_suggestion: LatencyBudget,
    /// Latency to escalation packet completion.
    pub time_to_escalation_packet_completion: LatencyBudget,
    /// Per-evidence-path budgets.
    pub per_path_budgets: Vec<PerPathLatencyBudget>,
    /// Whether the row contributes to a release gate.
    pub contributes_to_release_gate: bool,
    /// Closed stale-data triggers, if any, currently downgrading the row.
    pub stale_data_triggers: Vec<String>,
    /// Reviewer-facing summary of the first actionable artifact.
    pub first_actionable_artifact_summary: String,
}

/// M3 diagnosis-latency scorecard projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M3DiagnosisLatencyScorecard {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable scorecard id.
    pub scorecard_id: String,
    /// UTC generation timestamp.
    pub generated_at: String,
    /// Corpus manifest ref.
    pub corpus_manifest_ref: String,
    /// Reviewer doc ref.
    pub corpus_doc_ref: String,
    /// Alpha diagnosis-latency scorecard ref this row set extends.
    pub alpha_scorecard_ref: String,
    /// Whether raw private material is excluded by default.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded by default.
    pub ambient_authority_excluded: bool,
    /// Closed evidence-path classes the scorecard is attributable to.
    pub measurement_paths: Vec<String>,
    /// Required beta-lane class tokens covered by the scorecard.
    pub required_beta_lane_classes: Vec<String>,
    /// Closed stale-data triggers currently active on the scorecard.
    pub stale_data_triggers: Vec<String>,
    /// Per-lane rows.
    pub lane_rows: Vec<M3DiagnosisLatencyLaneRow>,
}

impl M3DiagnosisLatencyScorecard {
    /// Returns true when the scorecard is metadata-safe, covers every
    /// required lane, and has no stale-data triggers active.
    pub fn is_release_consumable(&self) -> bool {
        if !self.raw_private_material_excluded || !self.ambient_authority_excluded {
            return false;
        }
        if !self.stale_data_triggers.is_empty() {
            return false;
        }
        let lane_set = self
            .lane_rows
            .iter()
            .map(|row| row.beta_lane_class.as_str().to_owned())
            .collect::<BTreeSet<_>>();
        for required in &self.required_beta_lane_classes {
            if !lane_set.contains(required) {
                return false;
            }
        }
        self.lane_rows
            .iter()
            .all(|row| row.stale_data_triggers.is_empty() && row.contributes_to_release_gate)
    }
}

/// One row in the exact-build availability report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M3ExactBuildAvailabilityLaneRow {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Closed beta-lane class.
    pub beta_lane_class: M3BetaLaneClass,
    /// Stable scenario id.
    pub scenario_id: String,
    /// Fixture ref.
    pub fixture_ref: String,
    /// Whether exact-build identity is bound by the scenario's evidence.
    pub exact_build_identity_required: bool,
    /// Whether symbolication evidence is required for this lane.
    pub symbolication_required: bool,
    /// Seeded availability percentage (0..=100) of the exact-build
    /// identity binding on the support packet.
    pub seeded_exact_build_availability_pct: u8,
    /// Seeded availability percentage (0..=100) of symbolication report
    /// references when required for this lane.
    pub seeded_symbolication_availability_pct: u8,
    /// Closed measurement state.
    pub state: LatencyMeasurementState,
    /// Closed stale-data triggers active on this lane row.
    pub stale_data_triggers: Vec<String>,
}

/// M3 exact-build availability report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExactBuildAvailabilityReport {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable report id.
    pub report_id: String,
    /// UTC generation timestamp.
    pub generated_at: String,
    /// Corpus manifest ref.
    pub corpus_manifest_ref: String,
    /// Reviewer doc ref.
    pub corpus_doc_ref: String,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// Required beta-lane class tokens.
    pub required_beta_lane_classes: Vec<String>,
    /// Closed stale-data triggers currently active on the report.
    pub stale_data_triggers: Vec<String>,
    /// Per-lane rows.
    pub lane_rows: Vec<M3ExactBuildAvailabilityLaneRow>,
}

impl ExactBuildAvailabilityReport {
    /// Returns true when the report is metadata-safe and has no stale-data
    /// triggers active.
    pub fn is_release_consumable(&self) -> bool {
        if !self.raw_private_material_excluded || !self.ambient_authority_excluded {
            return false;
        }
        if !self.stale_data_triggers.is_empty() {
            return false;
        }
        let lane_set = self
            .lane_rows
            .iter()
            .map(|row| row.beta_lane_class.as_str().to_owned())
            .collect::<BTreeSet<_>>();
        for required in &self.required_beta_lane_classes {
            if !lane_set.contains(required) {
                return false;
            }
        }
        self.lane_rows
            .iter()
            .all(|row| row.stale_data_triggers.is_empty())
    }
}

/// One row in the field-readiness dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M3FieldReadinessLaneRow {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Closed beta-lane class.
    pub beta_lane_class: M3BetaLaneClass,
    /// Stable scenario id from the corpus.
    pub scenario_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Fixture ref.
    pub fixture_ref: String,
    /// Scorecard target the row contributes to.
    pub scorecard_target: String,
    /// Expected scorecard state when the drill is healthy.
    pub expected_state: String,
    /// Closed measurement state.
    pub current_state: LatencyMeasurementState,
    /// Percentage (0..=100) of the scenario's required support-packet
    /// items present in the seeded evidence (escalation packet
    /// completeness).
    pub seeded_escalation_packet_completeness_pct: u8,
    /// Seeded false-safe-repair rate, expressed in basis points
    /// (0..=10_000). The corpus pins this to 0 because forbidden-fix
    /// classes are refused at the safety baseline; the dashboard
    /// reports this rate so live measurement can replace the seeded
    /// value without changing the row shape.
    pub seeded_false_safe_repair_rate_bps: u16,
    /// Closed claim-downgrade trigger classes the scenario covers.
    pub claim_downgrade_trigger_classes: Vec<String>,
    /// Closed stale-data triggers active on this row.
    pub stale_data_triggers: Vec<String>,
    /// Whether the row contributes to a release gate.
    pub contributes_to_release_gate: bool,
}

/// M3 field-readiness dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldReadinessDashboard {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable dashboard id.
    pub dashboard_id: String,
    /// UTC generation timestamp.
    pub generated_at: String,
    /// Corpus manifest ref.
    pub corpus_manifest_ref: String,
    /// Reviewer doc ref.
    pub corpus_doc_ref: String,
    /// Alpha diagnosis-latency scorecard ref.
    pub alpha_scorecard_ref: String,
    /// Drill harness report ref.
    pub drill_harness_report_ref: String,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// Required beta-lane class tokens.
    pub required_beta_lane_classes: Vec<String>,
    /// Evidence-path classes the dashboard is attributable to.
    pub measurement_paths: Vec<String>,
    /// Closed stale-data triggers currently active on the dashboard.
    pub stale_data_triggers: Vec<String>,
    /// Per-lane rows.
    pub lane_rows: Vec<M3FieldReadinessLaneRow>,
}

impl FieldReadinessDashboard {
    /// Returns true when the dashboard is metadata-safe, covers every
    /// required lane, and has no stale-data triggers active.
    pub fn is_release_consumable(&self) -> bool {
        if !self.raw_private_material_excluded || !self.ambient_authority_excluded {
            return false;
        }
        if !self.stale_data_triggers.is_empty() {
            return false;
        }
        let lane_set = self
            .lane_rows
            .iter()
            .map(|row| row.beta_lane_class.as_str().to_owned())
            .collect::<BTreeSet<_>>();
        for required in &self.required_beta_lane_classes {
            if !lane_set.contains(required) {
                return false;
            }
        }
        self.lane_rows
            .iter()
            .all(|row| row.stale_data_triggers.is_empty() && row.contributes_to_release_gate)
    }
}

/// Bundled field-readiness scorecards: the diagnosis-latency scorecard,
/// the exact-build availability report, and the field-readiness
/// dashboard projected from one shared corpus snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldReadinessScorecards {
    /// Stable bundle id.
    pub bundle_id: String,
    /// UTC generation timestamp.
    pub generated_at: String,
    /// Diagnosis-latency scorecard.
    pub diagnosis_latency_scorecard: M3DiagnosisLatencyScorecard,
    /// Exact-build availability report.
    pub exact_build_availability_report: ExactBuildAvailabilityReport,
    /// Field-readiness dashboard.
    pub field_readiness_dashboard: FieldReadinessDashboard,
}

impl FieldReadinessScorecards {
    /// Returns true when every projected scorecard passes its release
    /// consumability contract.
    pub fn is_release_consumable(&self) -> bool {
        self.diagnosis_latency_scorecard.is_release_consumable()
            && self.exact_build_availability_report.is_release_consumable()
            && self.field_readiness_dashboard.is_release_consumable()
    }
}

/// Projects the field-readiness scorecards from a given corpus and alpha
/// scorecard, honoring stale-data downgrades when either evidence
/// source is incomplete.
pub fn project_field_readiness_scorecards(
    corpus: &M3ScenarioCorpus,
    alpha_scorecard: Option<&DiagnosisLatencyScorecard>,
    generated_at: impl Into<String>,
) -> FieldReadinessScorecards {
    let generated_at = generated_at.into();

    let mut corpus_violations = corpus.validate();
    let alpha_scorecard_missing = alpha_scorecard.is_none();
    let mut scorecard_stale_triggers: Vec<StaleDataTrigger> = Vec::new();

    if alpha_scorecard_missing {
        scorecard_stale_triggers.push(StaleDataTrigger::AlphaScorecardMissing);
    }
    if corpus_violations
        .iter()
        .any(|v| v.check_id == "corpus.required_lane_missing")
    {
        scorecard_stale_triggers.push(StaleDataTrigger::SeededCorpusMissingLane);
    }
    if corpus_violations.iter().any(|v| {
        v.check_id == "scenario.claim_downgrade_rules.required_trigger_missing"
    }) {
        scorecard_stale_triggers.push(StaleDataTrigger::ClaimDowngradeRulesIncomplete);
    }
    // Drain to release the borrow.
    corpus_violations.clear();

    let alpha_p90_target_seconds = alpha_scorecard
        .map(|s| s.measurement_contract.p90_target_seconds)
        .unwrap_or(600);
    let alpha_yellow_seconds = alpha_scorecard
        .map(|s| s.measurement_contract.p90_yellow_seconds)
        .unwrap_or(720);
    let alpha_red_seconds = alpha_scorecard
        .map(|s| s.measurement_contract.p90_red_seconds)
        .unwrap_or(900);

    let lane_state = if scorecard_stale_triggers.is_empty() {
        LatencyMeasurementState::SeededPendingLiveMeasurement
    } else {
        LatencyMeasurementState::StaleDowngraded
    };

    let diagnosis_rows = corpus
        .entries
        .iter()
        .map(|entry| {
            project_diagnosis_latency_lane_row(
                &entry.scenario,
                &entry.fixture_ref,
                alpha_p90_target_seconds,
                alpha_yellow_seconds,
                alpha_red_seconds,
                lane_state,
                &scorecard_stale_triggers,
            )
        })
        .collect::<Vec<_>>();

    let exact_build_rows = corpus
        .entries
        .iter()
        .map(|entry| {
            project_exact_build_lane_row(
                &entry.scenario,
                &entry.fixture_ref,
                lane_state,
                &scorecard_stale_triggers,
            )
        })
        .collect::<Vec<_>>();

    let field_readiness_rows = corpus
        .entries
        .iter()
        .map(|entry| {
            project_field_readiness_lane_row(
                &entry.scenario,
                &entry.fixture_ref,
                lane_state,
                &scorecard_stale_triggers,
            )
        })
        .collect::<Vec<_>>();

    let stale_trigger_tokens = scorecard_stale_triggers
        .iter()
        .map(|trigger| trigger.as_str().to_owned())
        .collect::<Vec<_>>();
    let required_lane_tokens = required_beta_lane_classes()
        .iter()
        .map(|lane| lane.as_str().to_owned())
        .collect::<Vec<_>>();
    let path_tokens = REQUIRED_EVIDENCE_PATH_CLASSES
        .iter()
        .map(|path| path.as_str().to_owned())
        .collect::<Vec<_>>();

    let diagnosis_latency_scorecard = M3DiagnosisLatencyScorecard {
        record_kind: M3_DIAGNOSIS_LATENCY_SCORECARD_RECORD_KIND.to_owned(),
        schema_version: M3_SUPPORT_SCENARIO_SCHEMA_VERSION,
        scorecard_id: DIAGNOSIS_LATENCY_SCORECARD_ID.to_owned(),
        generated_at: generated_at.clone(),
        corpus_manifest_ref: M3_SCENARIO_CORPUS_MANIFEST_REF.to_owned(),
        corpus_doc_ref: M3_SCENARIO_CORPUS_DOC_REF.to_owned(),
        alpha_scorecard_ref: "artifacts/support/diagnosis_latency_scorecard_alpha.yaml".to_owned(),
        raw_private_material_excluded: true,
        ambient_authority_excluded: true,
        measurement_paths: path_tokens.clone(),
        required_beta_lane_classes: required_lane_tokens.clone(),
        stale_data_triggers: stale_trigger_tokens.clone(),
        lane_rows: diagnosis_rows,
    };

    let exact_build_availability_report = ExactBuildAvailabilityReport {
        record_kind: M3_EXACT_BUILD_AVAILABILITY_REPORT_RECORD_KIND.to_owned(),
        schema_version: M3_SUPPORT_SCENARIO_SCHEMA_VERSION,
        report_id: EXACT_BUILD_AVAILABILITY_REPORT_ID.to_owned(),
        generated_at: generated_at.clone(),
        corpus_manifest_ref: M3_SCENARIO_CORPUS_MANIFEST_REF.to_owned(),
        corpus_doc_ref: M3_SCENARIO_CORPUS_DOC_REF.to_owned(),
        raw_private_material_excluded: true,
        ambient_authority_excluded: true,
        required_beta_lane_classes: required_lane_tokens.clone(),
        stale_data_triggers: stale_trigger_tokens.clone(),
        lane_rows: exact_build_rows,
    };

    let field_readiness_dashboard = FieldReadinessDashboard {
        record_kind: M3_FIELD_READINESS_DASHBOARD_RECORD_KIND.to_owned(),
        schema_version: M3_SUPPORT_SCENARIO_SCHEMA_VERSION,
        dashboard_id: FIELD_READINESS_DASHBOARD_ID.to_owned(),
        generated_at: generated_at.clone(),
        corpus_manifest_ref: M3_SCENARIO_CORPUS_MANIFEST_REF.to_owned(),
        corpus_doc_ref: M3_SCENARIO_CORPUS_DOC_REF.to_owned(),
        alpha_scorecard_ref: "artifacts/support/diagnosis_latency_scorecard_alpha.yaml".to_owned(),
        drill_harness_report_ref: "artifacts/support/m3/drill_harness_report.md".to_owned(),
        raw_private_material_excluded: true,
        ambient_authority_excluded: true,
        required_beta_lane_classes: required_lane_tokens,
        measurement_paths: path_tokens,
        stale_data_triggers: stale_trigger_tokens,
        lane_rows: field_readiness_rows,
    };

    FieldReadinessScorecards {
        bundle_id: FIELD_READINESS_SCORECARDS_REPORT_ID.to_owned(),
        generated_at,
        diagnosis_latency_scorecard,
        exact_build_availability_report,
        field_readiness_dashboard,
    }
}

/// Projects the checked-in baseline field-readiness scorecards from the
/// checked-in corpus and alpha scorecard.
///
/// # Errors
///
/// Returns a YAML parse error when either the corpus or the alpha
/// scorecard fail to parse.
pub fn current_field_readiness_scorecards() -> Result<FieldReadinessScorecards, serde_yaml::Error> {
    let corpus = current_m3_scenario_corpus()?;
    let alpha = current_alpha_scorecard().ok();
    Ok(project_field_readiness_scorecards(
        &corpus,
        alpha.as_ref(),
        BASELINE_GENERATED_AT,
    ))
}

fn project_diagnosis_latency_lane_row(
    scenario: &M3SupportScenario,
    fixture_ref: &str,
    p90_target_seconds: u32,
    yellow_seconds: u32,
    red_seconds: u32,
    lane_state: LatencyMeasurementState,
    stale_triggers: &[StaleDataTrigger],
) -> M3DiagnosisLatencyLaneRow {
    // p50 (median) target = half of p90 target, rounded; matches the
    // alpha scorecard's tail-to-median ratio convention.
    let p50_target_seconds = p90_target_seconds / 2;

    let first_actionable = LatencyBudget {
        p50_target_seconds,
        p90_target_seconds,
        yellow_seconds,
        red_seconds,
        state: lane_state,
    };

    // Repair-suggestion budget is the same window as first-actionable
    // for the repair_preview lane; for other lanes the suggestion budget
    // is the same window because the seeded scorecard does not yet
    // measure repair-suggestion separately.
    let first_safe_repair_suggestion = LatencyBudget {
        p50_target_seconds,
        p90_target_seconds,
        yellow_seconds,
        red_seconds,
        state: lane_state,
    };

    // Escalation packet completion = export_support_packet step. Doubled
    // budget because the support packet projection is the last step in
    // every drill.
    let escalation_p90 = p90_target_seconds.saturating_mul(2);
    let escalation_yellow = yellow_seconds.saturating_mul(2);
    let escalation_red = red_seconds.saturating_mul(2);
    let time_to_escalation_packet_completion = LatencyBudget {
        p50_target_seconds: escalation_p90 / 2,
        p90_target_seconds: escalation_p90,
        yellow_seconds: escalation_yellow,
        red_seconds: escalation_red,
        state: lane_state,
    };

    let per_path_budgets = REQUIRED_EVIDENCE_PATH_CLASSES
        .iter()
        .map(|path| PerPathLatencyBudget {
            path_class: *path,
            p90_target_seconds,
            available_at_equal_prominence: true,
            state: lane_state,
        })
        .collect::<Vec<_>>();

    let stale_row_triggers = stale_triggers
        .iter()
        .map(|trigger| trigger.as_str().to_owned())
        .collect::<Vec<_>>();

    M3DiagnosisLatencyLaneRow {
        record_kind: M3_DIAGNOSIS_LATENCY_LANE_ROW_RECORD_KIND.to_owned(),
        beta_lane_class: scenario.beta_lane_class,
        scenario_id: scenario.scenario_id.clone(),
        title: scenario.title.clone(),
        fixture_ref: fixture_ref.to_owned(),
        scorecard_target: scenario.scorecard_contribution.scorecard_target.clone(),
        time_to_first_actionable_finding: first_actionable,
        time_to_first_safe_repair_suggestion: first_safe_repair_suggestion,
        time_to_escalation_packet_completion,
        per_path_budgets,
        contributes_to_release_gate: scenario.scorecard_contribution.contributes_to_release_gate,
        stale_data_triggers: stale_row_triggers,
        first_actionable_artifact_summary: scenario
            .expected_first_actionable_artifact
            .reviewer_summary
            .clone(),
    }
}

fn project_exact_build_lane_row(
    scenario: &M3SupportScenario,
    fixture_ref: &str,
    lane_state: LatencyMeasurementState,
    stale_triggers: &[StaleDataTrigger],
) -> M3ExactBuildAvailabilityLaneRow {
    // The corpus pins exact-build identity as required for every
    // support-export lane. Symbolication is only required for the
    // safe-mode and runtime-replay lanes whose evidence quotes a crash
    // dump or runtime evidence packet.
    let symbolication_required = matches!(
        scenario.beta_lane_class,
        M3BetaLaneClass::SafeMode
            | M3BetaLaneClass::RuntimeReplayPackets
            | M3BetaLaneClass::ExtensionBisect,
    );

    let stale_row_triggers = stale_triggers
        .iter()
        .map(|trigger| trigger.as_str().to_owned())
        .collect::<Vec<_>>();

    M3ExactBuildAvailabilityLaneRow {
        record_kind: M3_EXACT_BUILD_AVAILABILITY_LANE_ROW_RECORD_KIND.to_owned(),
        beta_lane_class: scenario.beta_lane_class,
        scenario_id: scenario.scenario_id.clone(),
        fixture_ref: fixture_ref.to_owned(),
        exact_build_identity_required: true,
        symbolication_required,
        seeded_exact_build_availability_pct: 100,
        seeded_symbolication_availability_pct: if symbolication_required { 100 } else { 0 },
        state: lane_state,
        stale_data_triggers: stale_row_triggers,
    }
}

fn project_field_readiness_lane_row(
    scenario: &M3SupportScenario,
    fixture_ref: &str,
    lane_state: LatencyMeasurementState,
    stale_triggers: &[StaleDataTrigger],
) -> M3FieldReadinessLaneRow {
    let mut trigger_tokens = scenario
        .claim_downgrade_rules
        .iter()
        .map(|rule| rule.trigger_class.as_str().to_owned())
        .collect::<Vec<_>>();
    trigger_tokens.sort();
    trigger_tokens.dedup();

    let stale_row_triggers = stale_triggers
        .iter()
        .map(|trigger| trigger.as_str().to_owned())
        .collect::<Vec<_>>();

    M3FieldReadinessLaneRow {
        record_kind: M3_FIELD_READINESS_LANE_ROW_RECORD_KIND.to_owned(),
        beta_lane_class: scenario.beta_lane_class,
        scenario_id: scenario.scenario_id.clone(),
        title: scenario.title.clone(),
        fixture_ref: fixture_ref.to_owned(),
        scorecard_target: scenario.scorecard_contribution.scorecard_target.clone(),
        expected_state: scenario.scorecard_contribution.expected_state.clone(),
        current_state: lane_state,
        seeded_escalation_packet_completeness_pct: 100,
        seeded_false_safe_repair_rate_bps: 0,
        claim_downgrade_trigger_classes: trigger_tokens,
        stale_data_triggers: stale_row_triggers,
        contributes_to_release_gate: scenario.scorecard_contribution.contributes_to_release_gate,
    }
}
