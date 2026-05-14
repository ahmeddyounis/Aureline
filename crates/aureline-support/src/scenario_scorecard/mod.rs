//! External alpha support-scenario scorecard consumer.
//!
//! This module consumes the canonical diagnosis-latency scorecard at
//! `/artifacts/support/diagnosis_latency_scorecard_alpha.yaml` and the
//! protected scenario corpus under
//! `/fixtures/support/seeded_scenarios_alpha/`. It projects both support
//! packet rows and review-dashboard rows from the same scorecard so release
//! review does not depend on copied free-form status notes.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Stable record kind for the external alpha diagnosis-latency scorecard.
pub const SUPPORT_SCENARIO_SCORECARD_RECORD_KIND: &str =
    "support_scenario_diagnosis_latency_scorecard";

/// Stable record kind for one seeded external alpha scenario.
pub const SUPPORT_ALPHA_SEEDED_SCENARIO_RECORD_KIND: &str = "support_alpha_seeded_scenario";

/// Stable record kind for the support-packet projection.
pub const SUPPORT_SCENARIO_SUPPORT_PACKET_RECORD_KIND: &str =
    "support_scenario_scorecard_support_packet";

/// Stable record kind for the review-dashboard projection.
pub const SUPPORT_SCENARIO_DASHBOARD_RECORD_KIND: &str = "support_scenario_scorecard_dashboard";

/// Repository-relative path of the current scorecard artifact.
pub const CURRENT_ALPHA_SCORECARD_PATH: &str =
    "artifacts/support/diagnosis_latency_scorecard_alpha.yaml";

const CURRENT_ALPHA_SCORECARD_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/support/diagnosis_latency_scorecard_alpha.yaml"
));

const REQUIRED_FAMILIES: [SupportScenarioFamily; 6] = [
    SupportScenarioFamily::FirstRun,
    SupportScenarioFamily::SearchIndex,
    SupportScenarioFamily::TrustPolicy,
    SupportScenarioFamily::RestoreContinuity,
    SupportScenarioFamily::ProviderAuth,
    SupportScenarioFamily::CrashLoop,
];

const REQUIRED_FORBIDDEN_FIXES: [&str; 5] = [
    "destructive_reset_without_preview",
    "publish_route",
    "reenable_quarantined_extension_without_preview",
    "run_repo_owned_hook_for_diagnosis",
    "widen_workspace_trust",
];

const SCENARIO_FIXTURES: [(&str, &str); 6] = [
    (
        "fixtures/support/seeded_scenarios_alpha/first_run_entry_open_target_unavailable.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/seeded_scenarios_alpha/first_run_entry_open_target_unavailable.yaml"
        )),
    ),
    (
        "fixtures/support/seeded_scenarios_alpha/search_index_readiness_stalled.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/seeded_scenarios_alpha/search_index_readiness_stalled.yaml"
        )),
    ),
    (
        "fixtures/support/seeded_scenarios_alpha/trust_policy_denied_capability.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/seeded_scenarios_alpha/trust_policy_denied_capability.yaml"
        )),
    ),
    (
        "fixtures/support/seeded_scenarios_alpha/restore_replay_blocked.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/seeded_scenarios_alpha/restore_replay_blocked.yaml"
        )),
    ),
    (
        "fixtures/support/seeded_scenarios_alpha/provider_credential_expired.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/seeded_scenarios_alpha/provider_credential_expired.yaml"
        )),
    ),
    (
        "fixtures/support/seeded_scenarios_alpha/crash_loop_extension_quarantine.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/seeded_scenarios_alpha/crash_loop_extension_quarantine.yaml"
        )),
    ),
];

/// Loads the checked-in external alpha scorecard artifact.
///
/// # Errors
///
/// Returns a YAML parse error when the checked-in scorecard does not match
/// [`DiagnosisLatencyScorecard`].
pub fn current_alpha_scorecard() -> Result<DiagnosisLatencyScorecard, serde_yaml::Error> {
    serde_yaml::from_str(CURRENT_ALPHA_SCORECARD_YAML)
}

/// Loads the checked-in external alpha scenario corpus with fixture paths.
///
/// # Errors
///
/// Returns a YAML parse error when any checked-in scenario fixture does not
/// match [`SeededSupportScenario`].
pub fn current_alpha_seeded_scenario_corpus() -> Result<SeededScenarioCorpus, serde_yaml::Error> {
    let entries = SCENARIO_FIXTURES
        .into_iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str(yaml).map(|scenario| SeededScenarioEntry {
                fixture_ref: fixture_ref.to_owned(),
                scenario,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(SeededScenarioCorpus { entries })
}

/// Support scenario family covered by the external alpha corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportScenarioFamily {
    /// First-run, open, import, or entry admission failures.
    FirstRun,
    /// Search, index, and graph readiness failures.
    SearchIndex,
    /// Trust, policy, identity, or approval-denial failures.
    TrustPolicy,
    /// Restore, crash replay, and continuity failures.
    RestoreContinuity,
    /// Connected provider and credential-admission failures.
    ProviderAuth,
    /// Startup crash-loop, safe-mode, or quarantine failures.
    CrashLoop,
}

impl SupportScenarioFamily {
    /// Returns the stable snake-case token for the family.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstRun => "first_run",
            Self::SearchIndex => "search_index",
            Self::TrustPolicy => "trust_policy",
            Self::RestoreContinuity => "restore_continuity",
            Self::ProviderAuth => "provider_auth",
            Self::CrashLoop => "crash_loop",
        }
    }
}

/// Result packet type that can stop the diagnosis-latency timer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultPacketClass {
    /// Project Doctor finding packet.
    DoctorFinding,
    /// Recovery-ladder decision packet.
    RecoveryDecision,
}

/// Parsed scorecard artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosisLatencyScorecard {
    /// Scorecard schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable scorecard id consumed by support and dashboard projections.
    pub scorecard_id: String,
    /// Current measurement posture.
    pub status: String,
    /// Reviewer-facing scorecard title.
    pub title: String,
    /// Owning support or release lane.
    pub owner_lane: String,
    /// Source artifacts the scorecard consumes.
    pub source_refs: BTreeMap<String, String>,
    /// Consumer contract for support packet and dashboard projections.
    pub consumer_contract: ScorecardConsumerContract,
    /// Shared diagnosis-latency measurement window.
    pub measurement_contract: ScorecardMeasurementContract,
    /// Scenario families required by the alpha corpus.
    pub required_scenario_families: Vec<SupportScenarioFamily>,
    /// Launch wedges each scorecard row must be relevant to.
    pub required_launch_wedges: Vec<String>,
    /// Support contexts that must remain covered by the scorecard.
    pub required_support_contexts: Vec<String>,
    /// Per-scenario scorecard rows.
    pub scenario_rows: Vec<ScorecardScenarioRow>,
    /// Release or support gates backed by this scorecard.
    pub quality_gates: Vec<ScorecardQualityGate>,
    /// UTC timestamp when the scorecard artifact was emitted.
    pub emitted_at: String,
}

impl DiagnosisLatencyScorecard {
    /// Validates the scorecard against the checked-in scenario corpus.
    pub fn validate_with_corpus(&self, corpus: &SeededScenarioCorpus) -> Vec<ScorecardViolation> {
        let mut violations = Vec::new();

        if self.schema_version != 1 {
            push_violation(
                &mut violations,
                "scorecard.schema_version",
                &self.scorecard_id,
                "schema_version must be 1",
            );
        }
        if self.record_kind != SUPPORT_SCENARIO_SCORECARD_RECORD_KIND {
            push_violation(
                &mut violations,
                "scorecard.record_kind",
                &self.scorecard_id,
                "record_kind must be support_scenario_diagnosis_latency_scorecard",
            );
        }
        if self.consumer_contract.free_form_status_notes_allowed {
            push_violation(
                &mut violations,
                "scorecard.free_form_status",
                &self.scorecard_id,
                "support packet and dashboard consumers must use scorecard rows, not free-form notes",
            );
        }
        if self.consumer_contract.support_packet_projection
            != SUPPORT_SCENARIO_SUPPORT_PACKET_RECORD_KIND
        {
            push_violation(
                &mut violations,
                "scorecard.support_packet_projection",
                &self.scorecard_id,
                "support packet projection must match the support crate record kind",
            );
        }
        if self.consumer_contract.review_dashboard_projection
            != SUPPORT_SCENARIO_DASHBOARD_RECORD_KIND
        {
            push_violation(
                &mut violations,
                "scorecard.dashboard_projection",
                &self.scorecard_id,
                "review dashboard projection must match the support crate record kind",
            );
        }
        if self.measurement_contract.start_event != "support_scenario_started" {
            push_violation(
                &mut violations,
                "scorecard.measurement_start",
                &self.scorecard_id,
                "measurement must start at support_scenario_started",
            );
        }
        if self.measurement_contract.stop_event != "first_actionable_result_packet_emitted" {
            push_violation(
                &mut violations,
                "scorecard.measurement_stop",
                &self.scorecard_id,
                "measurement must stop at first_actionable_result_packet_emitted",
            );
        }
        if !self.measurement_contract.raw_private_material_excluded {
            push_violation(
                &mut violations,
                "scorecard.raw_private_material",
                &self.scorecard_id,
                "raw private material must be excluded by default",
            );
        }

        let expected_families = REQUIRED_FAMILIES.into_iter().collect::<BTreeSet<_>>();
        let declared_families = self
            .required_scenario_families
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        if declared_families != expected_families {
            push_violation(
                &mut violations,
                "scorecard.required_families",
                &self.scorecard_id,
                "required_scenario_families must match the alpha coverage contract",
            );
        }

        let row_family_set = self
            .scenario_rows
            .iter()
            .map(|row| row.scenario_family)
            .collect::<BTreeSet<_>>();
        for family in expected_families {
            if !row_family_set.contains(&family) {
                push_violation(
                    &mut violations,
                    "scorecard.family_coverage",
                    &self.scorecard_id,
                    format!("missing scorecard row for {}", family.as_str()),
                );
            }
        }

        let mut row_ids = BTreeSet::new();
        let mut scenario_ids = BTreeSet::new();
        let corpus_by_fixture = corpus.by_fixture_ref();
        let corpus_by_id = corpus.by_scenario_id();

        for row in &self.scenario_rows {
            if !row_ids.insert(row.row_id.as_str()) {
                push_violation(
                    &mut violations,
                    "scorecard.duplicate_row_id",
                    &row.row_id,
                    "scenario row ids must be unique",
                );
            }
            if !scenario_ids.insert(row.scenario_id.as_str()) {
                push_violation(
                    &mut violations,
                    "scorecard.duplicate_scenario_id",
                    &row.scenario_id,
                    "scenario ids must be unique",
                );
            }

            validate_measurement_window(
                &mut violations,
                &row.row_id,
                &row.measurement_start_event,
                &row.measurement_stop_event,
                row.target_seconds,
                row.yellow_seconds,
                row.red_seconds,
            );
            validate_support_bindings(&mut violations, &row.row_id, &row.support_packet_bindings);
            validate_dashboard_bindings(&mut violations, row);
            validate_no_touch_boundary(&mut violations, &row.row_id, &row.no_touch_boundary_set);
            validate_required_tokens(
                &mut violations,
                &row.row_id,
                "scorecard.launch_wedge_coverage",
                &self.required_launch_wedges,
                &row.launch_wedges,
            );

            if !row
                .support_contexts
                .iter()
                .any(|ctx| ctx == "cli_headless_doctor")
            {
                push_violation(
                    &mut violations,
                    "scorecard.headless_context",
                    &row.row_id,
                    "every row must include cli_headless_doctor",
                );
            }
            if row.scenario_family == SupportScenarioFamily::CrashLoop
                && !row
                    .support_contexts
                    .iter()
                    .any(|ctx| ctx == "safe_mode_support_center")
            {
                push_violation(
                    &mut violations,
                    "scorecard.safe_mode_context",
                    &row.row_id,
                    "crash-loop rows must include safe_mode_support_center",
                );
            }

            match corpus_by_fixture.get(row.fixture_ref.as_str()) {
                Some(entry) => validate_row_against_scenario(&mut violations, row, entry),
                None => push_violation(
                    &mut violations,
                    "scorecard.fixture_ref",
                    &row.row_id,
                    format!(
                        "fixture_ref {} is not in the seeded corpus",
                        row.fixture_ref
                    ),
                ),
            }
            if !corpus_by_id.contains_key(row.scenario_id.as_str()) {
                push_violation(
                    &mut violations,
                    "scorecard.scenario_id",
                    &row.row_id,
                    "scenario_id is not present in the seeded corpus",
                );
            }
        }

        violations
    }

    /// Projects this scorecard into a support packet.
    pub fn support_packet(
        &self,
        packet_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> SupportScenarioSupportPacket {
        SupportScenarioSupportPacket {
            record_kind: SUPPORT_SCENARIO_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: 1,
            packet_id: packet_id.into(),
            scorecard_id: self.scorecard_id.clone(),
            generated_at: generated_at.into(),
            raw_private_material_excluded: true,
            rows: self
                .scenario_rows
                .iter()
                .map(SupportScenarioSupportPacketRow::from)
                .collect(),
        }
    }

    /// Projects this scorecard into a review dashboard snapshot.
    pub fn review_dashboard(
        &self,
        dashboard_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> SupportScenarioDashboard {
        SupportScenarioDashboard {
            record_kind: SUPPORT_SCENARIO_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: 1,
            dashboard_id: dashboard_id.into(),
            scorecard_id: self.scorecard_id.clone(),
            generated_at: generated_at.into(),
            rows: self
                .scenario_rows
                .iter()
                .map(SupportScenarioDashboardRow::from)
                .collect(),
        }
    }
}

/// Projection contract fields in the scorecard header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScorecardConsumerContract {
    /// First Rust consumer that parses this artifact.
    pub first_runtime_consumer: String,
    /// Record kind emitted for support packet consumers.
    pub support_packet_projection: String,
    /// Record kind emitted for review dashboard consumers.
    pub review_dashboard_projection: String,
    /// Scorecard join used by support packet rows.
    pub support_packet_row_join: String,
    /// Scorecard join used by dashboard rows.
    pub dashboard_row_join: String,
    /// Whether consumers may replace scorecard state with prose notes.
    pub free_form_status_notes_allowed: bool,
}

/// Shared diagnosis-latency measurement window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScorecardMeasurementContract {
    /// Event that starts the latency timer.
    pub start_event: String,
    /// Event that stops the latency timer.
    pub stop_event: String,
    /// Green p90 budget in seconds.
    pub p90_target_seconds: u32,
    /// Yellow p90 threshold in seconds.
    pub p90_yellow_seconds: u32,
    /// Red p90 threshold in seconds.
    pub p90_red_seconds: u32,
    /// Whether raw private material is excluded from scorecard evidence.
    pub raw_private_material_excluded: bool,
    /// Fields required on the first actionable result packet.
    pub result_packet_minimum_fields: Vec<String>,
}

/// One scenario row in the scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScorecardScenarioRow {
    /// Stable row id.
    pub row_id: String,
    /// Stable scenario id.
    pub scenario_id: String,
    /// Scenario family.
    pub scenario_family: SupportScenarioFamily,
    /// Reviewer-facing title.
    pub title: String,
    /// Corpus fixture path.
    pub fixture_ref: String,
    /// Launch wedges covered by the row.
    pub launch_wedges: Vec<String>,
    /// Support contexts covered by the row.
    pub support_contexts: Vec<String>,
    /// Latency start event.
    pub measurement_start_event: String,
    /// Latency stop event.
    pub measurement_stop_event: String,
    /// Green target in seconds.
    pub target_seconds: u32,
    /// Yellow threshold in seconds.
    pub yellow_seconds: u32,
    /// Red threshold in seconds.
    pub red_seconds: u32,
    /// First actionable result expected for this row.
    pub expected_result: ExpectedResultPacket,
    /// Support packet join and redaction expectations.
    pub support_packet_bindings: SupportPacketBindings,
    /// Review dashboard join expectations.
    pub dashboard_bindings: DashboardBindings,
    /// State or authority boundaries diagnosis must not touch.
    pub no_touch_boundary_set: Vec<String>,
}

/// Result packet expected to stop the latency timer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpectedResultPacket {
    /// Result packet class.
    pub result_packet_class: ResultPacketClass,
    /// Finding code carried by the result packet.
    pub finding_code: String,
    /// Opaque result packet ref.
    pub result_packet_ref: String,
    /// Support bundle ref to include in support/export output.
    pub support_bundle_ref: String,
    /// First safe next action.
    pub next_action_class: String,
    /// Reviewer-facing explanation for why the result is useful.
    pub useful_result_summary: String,
}

/// Support packet bindings for one scorecard row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportPacketBindings {
    /// Pack class required for the scenario.
    pub pack_class: String,
    /// Support bundle manifest ref.
    pub support_bundle_manifest_ref: String,
    /// Required support item ids.
    pub required_item_ids: Vec<String>,
    /// Whether exact-build identity is required.
    pub exact_build_identity_required: bool,
    /// Redaction class used by the support packet row.
    pub redaction_class: String,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

/// Dashboard join metadata for one scorecard row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardBindings {
    /// Stable dashboard row ref.
    pub dashboard_row_ref: String,
    /// Dashboard grouping token.
    pub dashboard_family_group: String,
    /// Current display state.
    pub display_state: String,
    /// Backreference to the scorecard row.
    pub scorecard_row_ref: String,
}

/// Quality gate declared by the scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScorecardQualityGate {
    /// Stable gate id.
    pub gate_id: String,
    /// Gate class.
    pub gate_class: String,
    /// Required gate state.
    pub required_state: String,
}

/// Scenario corpus loaded from checked-in fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeededScenarioCorpus {
    /// Fixture entries with their repository-relative refs.
    pub entries: Vec<SeededScenarioEntry>,
}

impl SeededScenarioCorpus {
    fn by_fixture_ref(&self) -> BTreeMap<&str, &SeededScenarioEntry> {
        self.entries
            .iter()
            .map(|entry| (entry.fixture_ref.as_str(), entry))
            .collect()
    }

    fn by_scenario_id(&self) -> BTreeMap<&str, &SeededScenarioEntry> {
        self.entries
            .iter()
            .map(|entry| (entry.scenario.scenario_id.as_str(), entry))
            .collect()
    }

    /// Returns the scenarios without their fixture path wrappers.
    pub fn scenarios(&self) -> Vec<&SeededSupportScenario> {
        self.entries.iter().map(|entry| &entry.scenario).collect()
    }
}

/// One fixture entry in the scenario corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeededScenarioEntry {
    /// Repository-relative fixture path.
    pub fixture_ref: String,
    /// Parsed scenario record.
    pub scenario: SeededSupportScenario,
}

/// One external alpha support-scenario fixture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeededSupportScenario {
    /// Scenario schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable scenario id.
    pub scenario_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Scenario family.
    pub scenario_family: SupportScenarioFamily,
    /// Scorecard row reference.
    pub scorecard_row_ref: String,
    /// Launch wedges the scenario applies to.
    pub launch_wedges: Vec<String>,
    /// Starting condition.
    pub starting_condition: ScenarioStartingCondition,
    /// Scenario trigger.
    pub trigger: ScenarioTrigger,
    /// Expected first actionable result.
    pub expected_first_actionable_result: ScenarioExpectedResult,
    /// Latency measurement contract.
    pub measurement: ScenarioMeasurement,
    /// Evidence and support refs.
    pub evidence: ScenarioEvidence,
    /// Safety boundaries.
    pub safety: ScenarioSafety,
    /// Support packet expectations.
    pub support_packet_expectations: ScenarioSupportPacketExpectations,
    /// Dashboard expectations.
    pub dashboard_expectations: ScenarioDashboardExpectations,
    /// Companion refs.
    pub references: ScenarioReferences,
    /// UTC timestamp when the scenario fixture was emitted.
    pub emitted_at: String,
}

/// Starting condition for a seeded scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioStartingCondition {
    /// Opaque state ref.
    pub state_ref: String,
    /// Reviewer-facing state summary.
    pub summary: String,
}

/// Trigger that starts a seeded scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioTrigger {
    /// Start event token.
    pub start_event: String,
    /// Opaque event ref.
    pub event_ref: String,
    /// Support context where the trigger occurs.
    pub support_context: String,
}

/// Expected first actionable result for a seeded scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioExpectedResult {
    /// Stop event token.
    pub stop_event: String,
    /// Result packet class.
    pub result_packet_class: ResultPacketClass,
    /// Finding code.
    pub finding_code: String,
    /// Result packet ref.
    pub result_packet_ref: String,
    /// First safe next action.
    pub next_action_class: String,
    /// Reviewer-facing usefulness summary.
    pub useful_result_summary: String,
}

/// Latency measurement fields for a seeded scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioMeasurement {
    /// Start event token.
    pub start_event: String,
    /// Stop event token.
    pub stop_event: String,
    /// Green target in seconds.
    pub target_seconds: u32,
    /// Yellow threshold in seconds.
    pub yellow_seconds: u32,
    /// Red threshold in seconds.
    pub red_seconds: u32,
    /// Evidence fields required before the stop event is valid.
    pub evidence_required_before_stop: Vec<String>,
}

/// Evidence refs for one seeded scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioEvidence {
    /// Exact-build identity ref.
    pub exact_build_identity_ref: String,
    /// Support bundle ref.
    pub support_bundle_ref: String,
    /// Support packet refs.
    pub support_packet_refs: Vec<String>,
    /// Fixture refs consumed by this scenario.
    pub fixture_refs: Vec<String>,
    /// Support item ids expected in the packet.
    pub support_item_ids: Vec<String>,
}

/// Safety boundaries for one seeded scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioSafety {
    /// Whether diagnosis is read-only.
    pub read_only_diagnosis: bool,
    /// Whether raw private material is excluded by default.
    pub raw_private_material_excluded: bool,
    /// Fix classes that must not appear as first actions.
    pub forbidden_fix_classes: Vec<String>,
    /// State or authority boundaries diagnosis must not touch.
    pub no_touch_boundary_set: Vec<String>,
}

/// Support packet expectations for one seeded scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioSupportPacketExpectations {
    /// Pack class expected for the scenario.
    pub pack_class: String,
    /// Redaction class expected for support output.
    pub redaction_class: String,
    /// Whether exact-build identity is required.
    pub exact_build_identity_required: bool,
    /// Whether preview review is required.
    pub reviewed_preview_required: bool,
    /// Whether local-only save/review remains available.
    pub local_only_path_available: bool,
}

/// Dashboard expectations for one seeded scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioDashboardExpectations {
    /// Dashboard row ref.
    pub dashboard_row_ref: String,
    /// Dashboard consumer record kind.
    pub consumer_ref: String,
}

/// Companion refs for one seeded scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioReferences {
    /// Scorecard artifact ref.
    pub scorecard_ref: String,
    /// Doctor or drill fixture ref.
    pub doctor_fixture_ref: String,
    /// Support bundle alpha doc ref.
    pub support_bundle_alpha_ref: String,
    /// Escalation case ref when available.
    pub escalation_case_ref: Option<String>,
}

/// Validation violation emitted by the scorecard consumer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScorecardViolation {
    /// Stable check id.
    pub check_id: String,
    /// Scorecard row, scenario, or scorecard id that failed.
    pub target_ref: String,
    /// Reviewer-facing message.
    pub message: String,
}

/// Support packet projected from the scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportScenarioSupportPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Projection schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Scorecard id this packet consumes.
    pub scorecard_id: String,
    /// UTC generation timestamp.
    pub generated_at: String,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Projected rows.
    pub rows: Vec<SupportScenarioSupportPacketRow>,
}

impl SupportScenarioSupportPacket {
    /// Returns true when every row is metadata-safe and scorecard-backed.
    pub fn is_export_safe(&self) -> bool {
        self.raw_private_material_excluded
            && self
                .rows
                .iter()
                .all(SupportScenarioSupportPacketRow::is_export_safe)
    }
}

/// One support packet row projected from a scorecard row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportScenarioSupportPacketRow {
    /// Scorecard row id.
    pub scorecard_row_id: String,
    /// Scenario id.
    pub scenario_id: String,
    /// Scenario family.
    pub scenario_family: SupportScenarioFamily,
    /// Finding code.
    pub finding_code: String,
    /// Result packet ref.
    pub result_packet_ref: String,
    /// Support bundle ref.
    pub support_bundle_ref: String,
    /// Pack class.
    pub pack_class: String,
    /// Redaction class.
    pub redaction_class: String,
    /// Whether exact-build identity is required.
    pub exact_build_identity_required: bool,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Dashboard row ref.
    pub dashboard_row_ref: String,
    /// Required support item ids.
    pub required_item_ids: Vec<String>,
}

impl SupportScenarioSupportPacketRow {
    /// Returns true when this row can be included in metadata-safe support output.
    pub fn is_export_safe(&self) -> bool {
        self.exact_build_identity_required
            && self.raw_private_material_excluded
            && self.redaction_class == "metadata_safe_default"
            && !self.scorecard_row_id.is_empty()
            && !self.result_packet_ref.is_empty()
            && !self.support_bundle_ref.is_empty()
            && !self.required_item_ids.is_empty()
    }
}

impl From<&ScorecardScenarioRow> for SupportScenarioSupportPacketRow {
    fn from(row: &ScorecardScenarioRow) -> Self {
        Self {
            scorecard_row_id: row.row_id.clone(),
            scenario_id: row.scenario_id.clone(),
            scenario_family: row.scenario_family,
            finding_code: row.expected_result.finding_code.clone(),
            result_packet_ref: row.expected_result.result_packet_ref.clone(),
            support_bundle_ref: row.expected_result.support_bundle_ref.clone(),
            pack_class: row.support_packet_bindings.pack_class.clone(),
            redaction_class: row.support_packet_bindings.redaction_class.clone(),
            exact_build_identity_required: row
                .support_packet_bindings
                .exact_build_identity_required,
            raw_private_material_excluded: row
                .support_packet_bindings
                .raw_private_material_excluded,
            dashboard_row_ref: row.dashboard_bindings.dashboard_row_ref.clone(),
            required_item_ids: row.support_packet_bindings.required_item_ids.clone(),
        }
    }
}

/// Review dashboard projected from the scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportScenarioDashboard {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Projection schema version.
    pub schema_version: u32,
    /// Stable dashboard id.
    pub dashboard_id: String,
    /// Scorecard id this dashboard consumes.
    pub scorecard_id: String,
    /// UTC generation timestamp.
    pub generated_at: String,
    /// Dashboard rows.
    pub rows: Vec<SupportScenarioDashboardRow>,
}

/// One dashboard row projected from a scorecard row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportScenarioDashboardRow {
    /// Dashboard row ref.
    pub dashboard_row_ref: String,
    /// Scorecard row id.
    pub scorecard_row_id: String,
    /// Scenario id.
    pub scenario_id: String,
    /// Scenario family.
    pub scenario_family: SupportScenarioFamily,
    /// Display state from the scorecard.
    pub display_state: String,
    /// Green target in seconds.
    pub target_seconds: u32,
    /// Yellow threshold in seconds.
    pub yellow_seconds: u32,
    /// Red threshold in seconds.
    pub red_seconds: u32,
    /// Result packet ref surfaced by the dashboard.
    pub result_packet_ref: String,
    /// Support bundle ref surfaced by the dashboard.
    pub support_bundle_ref: String,
}

impl From<&ScorecardScenarioRow> for SupportScenarioDashboardRow {
    fn from(row: &ScorecardScenarioRow) -> Self {
        Self {
            dashboard_row_ref: row.dashboard_bindings.dashboard_row_ref.clone(),
            scorecard_row_id: row.row_id.clone(),
            scenario_id: row.scenario_id.clone(),
            scenario_family: row.scenario_family,
            display_state: row.dashboard_bindings.display_state.clone(),
            target_seconds: row.target_seconds,
            yellow_seconds: row.yellow_seconds,
            red_seconds: row.red_seconds,
            result_packet_ref: row.expected_result.result_packet_ref.clone(),
            support_bundle_ref: row.expected_result.support_bundle_ref.clone(),
        }
    }
}

fn validate_row_against_scenario(
    violations: &mut Vec<ScorecardViolation>,
    row: &ScorecardScenarioRow,
    entry: &SeededScenarioEntry,
) {
    let scenario = &entry.scenario;
    if scenario.schema_version != 1 {
        push_violation(
            violations,
            "scenario.schema_version",
            &scenario.scenario_id,
            "schema_version must be 1",
        );
    }
    if scenario.record_kind != SUPPORT_ALPHA_SEEDED_SCENARIO_RECORD_KIND {
        push_violation(
            violations,
            "scenario.record_kind",
            &scenario.scenario_id,
            "record_kind must be support_alpha_seeded_scenario",
        );
    }
    if scenario.scorecard_row_ref != row.row_id {
        push_violation(
            violations,
            "scenario.scorecard_row_ref",
            &scenario.scenario_id,
            "scenario must reference the matching scorecard row",
        );
    }
    if scenario.scenario_id != row.scenario_id {
        push_violation(
            violations,
            "scenario.id_mismatch",
            &row.row_id,
            "scorecard row scenario_id must match the scenario fixture",
        );
    }
    if scenario.scenario_family != row.scenario_family {
        push_violation(
            violations,
            "scenario.family_mismatch",
            &row.row_id,
            "scorecard row family must match the scenario fixture",
        );
    }
    validate_required_tokens(
        violations,
        &scenario.scenario_id,
        "scenario.launch_wedge_coverage",
        &row.launch_wedges,
        &scenario.launch_wedges,
    );
    if scenario.trigger.start_event != row.measurement_start_event
        || scenario.measurement.start_event != row.measurement_start_event
    {
        push_violation(
            violations,
            "scenario.measurement_start",
            &scenario.scenario_id,
            "scenario trigger and measurement must start at the scorecard start event",
        );
    }
    if scenario.expected_first_actionable_result.stop_event != row.measurement_stop_event
        || scenario.measurement.stop_event != row.measurement_stop_event
    {
        push_violation(
            violations,
            "scenario.measurement_stop",
            &scenario.scenario_id,
            "scenario result and measurement must stop at the scorecard stop event",
        );
    }
    if scenario.measurement.target_seconds != row.target_seconds
        || scenario.measurement.yellow_seconds != row.yellow_seconds
        || scenario.measurement.red_seconds != row.red_seconds
    {
        push_violation(
            violations,
            "scenario.measurement_budget",
            &scenario.scenario_id,
            "scenario measurement budgets must match the scorecard row",
        );
    }
    if scenario
        .expected_first_actionable_result
        .result_packet_class
        != row.expected_result.result_packet_class
        || scenario.expected_first_actionable_result.finding_code
            != row.expected_result.finding_code
        || scenario.expected_first_actionable_result.result_packet_ref
            != row.expected_result.result_packet_ref
        || scenario.expected_first_actionable_result.next_action_class
            != row.expected_result.next_action_class
    {
        push_violation(
            violations,
            "scenario.result_packet",
            &scenario.scenario_id,
            "scenario first actionable result must match the scorecard expected result",
        );
    }
    if scenario.evidence.support_bundle_ref != row.expected_result.support_bundle_ref {
        push_violation(
            violations,
            "scenario.support_bundle_ref",
            &scenario.scenario_id,
            "scenario support bundle ref must match the scorecard expected result",
        );
    }
    if !scenario.safety.read_only_diagnosis {
        push_violation(
            violations,
            "scenario.read_only",
            &scenario.scenario_id,
            "diagnosis must be read-only",
        );
    }
    if !scenario.safety.raw_private_material_excluded {
        push_violation(
            violations,
            "scenario.raw_private_material",
            &scenario.scenario_id,
            "raw private material must be excluded",
        );
    }
    validate_required_tokens(
        violations,
        &scenario.scenario_id,
        "scenario.forbidden_fix_coverage",
        &REQUIRED_FORBIDDEN_FIXES
            .iter()
            .map(|token| (*token).to_owned())
            .collect::<Vec<_>>(),
        &scenario.safety.forbidden_fix_classes,
    );
    validate_no_touch_boundary(
        violations,
        &scenario.scenario_id,
        &scenario.safety.no_touch_boundary_set,
    );
    if scenario.support_packet_expectations.pack_class != row.support_packet_bindings.pack_class
        || scenario.support_packet_expectations.redaction_class
            != row.support_packet_bindings.redaction_class
        || scenario
            .support_packet_expectations
            .exact_build_identity_required
            != row.support_packet_bindings.exact_build_identity_required
    {
        push_violation(
            violations,
            "scenario.support_packet_expectations",
            &scenario.scenario_id,
            "scenario support packet expectations must match the scorecard row",
        );
    }
    if !scenario
        .support_packet_expectations
        .local_only_path_available
    {
        push_violation(
            violations,
            "scenario.local_only_path",
            &scenario.scenario_id,
            "local-only support path must remain available",
        );
    }
    if scenario.dashboard_expectations.dashboard_row_ref != row.dashboard_bindings.dashboard_row_ref
        || scenario.dashboard_expectations.consumer_ref != SUPPORT_SCENARIO_DASHBOARD_RECORD_KIND
    {
        push_violation(
            violations,
            "scenario.dashboard_expectations",
            &scenario.scenario_id,
            "scenario dashboard expectations must match the scorecard dashboard projection",
        );
    }
}

fn validate_measurement_window(
    violations: &mut Vec<ScorecardViolation>,
    target_ref: &str,
    start_event: &str,
    stop_event: &str,
    target_seconds: u32,
    yellow_seconds: u32,
    red_seconds: u32,
) {
    if start_event != "support_scenario_started" {
        push_violation(
            violations,
            "scorecard.row_start_event",
            target_ref,
            "row latency must start at support_scenario_started",
        );
    }
    if stop_event != "first_actionable_result_packet_emitted" {
        push_violation(
            violations,
            "scorecard.row_stop_event",
            target_ref,
            "row latency must stop at first_actionable_result_packet_emitted",
        );
    }
    if !(target_seconds < yellow_seconds && yellow_seconds < red_seconds) {
        push_violation(
            violations,
            "scorecard.row_threshold_order",
            target_ref,
            "target, yellow, and red thresholds must be strictly increasing",
        );
    }
}

fn validate_support_bindings(
    violations: &mut Vec<ScorecardViolation>,
    target_ref: &str,
    bindings: &SupportPacketBindings,
) {
    if !bindings.exact_build_identity_required {
        push_violation(
            violations,
            "scorecard.exact_build_required",
            target_ref,
            "support packet binding must require exact-build identity",
        );
    }
    if !bindings.raw_private_material_excluded {
        push_violation(
            violations,
            "scorecard.raw_private_material_excluded",
            target_ref,
            "support packet binding must exclude raw private material",
        );
    }
    if bindings.redaction_class != "metadata_safe_default" {
        push_violation(
            violations,
            "scorecard.redaction_class",
            target_ref,
            "support packet binding must use metadata_safe_default",
        );
    }
    if bindings.required_item_ids.is_empty() {
        push_violation(
            violations,
            "scorecard.required_item_ids",
            target_ref,
            "support packet binding must name required item ids",
        );
    }
}

fn validate_dashboard_bindings(
    violations: &mut Vec<ScorecardViolation>,
    row: &ScorecardScenarioRow,
) {
    if row.dashboard_bindings.scorecard_row_ref != row.row_id {
        push_violation(
            violations,
            "scorecard.dashboard_scorecard_ref",
            &row.row_id,
            "dashboard binding must point back to the same scorecard row",
        );
    }
    if row.dashboard_bindings.dashboard_row_ref.is_empty()
        || row.dashboard_bindings.display_state.is_empty()
    {
        push_violation(
            violations,
            "scorecard.dashboard_binding_empty",
            &row.row_id,
            "dashboard binding must name a row ref and display state",
        );
    }
}

fn validate_no_touch_boundary(
    violations: &mut Vec<ScorecardViolation>,
    target_ref: &str,
    boundaries: &[String],
) {
    if !boundaries
        .iter()
        .any(|boundary| boundary == "user_authored_files")
    {
        push_violation(
            violations,
            "support.no_touch_user_files",
            target_ref,
            "no-touch boundary must include user_authored_files",
        );
    }
}

fn validate_required_tokens(
    violations: &mut Vec<ScorecardViolation>,
    target_ref: &str,
    check_id: &str,
    required: &[String],
    actual: &[String],
) {
    for token in required {
        if !actual.iter().any(|actual| actual == token) {
            push_violation(
                violations,
                check_id,
                target_ref,
                format!("missing required token {token}"),
            );
        }
    }
}

fn push_violation(
    violations: &mut Vec<ScorecardViolation>,
    check_id: impl Into<String>,
    target_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(ScorecardViolation {
        check_id: check_id.into(),
        target_ref: target_ref.into(),
        message: message.into(),
    });
}
