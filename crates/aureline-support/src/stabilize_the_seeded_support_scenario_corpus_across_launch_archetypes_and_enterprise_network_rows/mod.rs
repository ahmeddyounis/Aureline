//! Stabilized seeded support-scenario corpus across launch archetypes and
//! enterprise-network rows.
//!
//! This module defines the canonical M4 support-scenario corpus that covers
//! blocked-user recovery, support-export verification, diagnosis routing, and
//! repair-preview validation across the full matrix of launch archetypes and
//! enterprise-network postures. Every scenario carries closed vocabulary,
//! exact-build identity, and a metadata-safe support projection.
//!
//! The [`StabilizedScenarioCorpus`] mirrors the boundary schema at
//! [`/schemas/support/stabilize_the_seeded_support_scenario_corpus_across_launch_archetypes_and_enterprise_network_rows.schema.json`].
//!
//! The [`StabilizedScenarioEvaluator`] validates the corpus and projects
//! a metadata-safe [`StabilizedScenarioSupportPacket`] and
//! [`StabilizedScenarioReport`].

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for a seeded stabilized support scenario.
pub const STABILIZED_SUPPORT_SCENARIO_RECORD_KIND: &str = "stabilized_support_scenario_record";

/// Stable record-kind tag for the stabilized scenario corpus report.
pub const STABILIZED_SCENARIO_REPORT_RECORD_KIND: &str = "stabilized_scenario_report_record";

/// Stable record-kind tag for one row in the stabilized scenario report.
pub const STABILIZED_SCENARIO_REPORT_ROW_RECORD_KIND: &str =
    "stabilized_scenario_report_row_record";

/// Stable record-kind tag for the metadata-safe support packet.
pub const STABILIZED_SCENARIO_SUPPORT_PACKET_RECORD_KIND: &str =
    "stabilized_scenario_support_packet_record";

/// Integer schema version for stabilized scenario records.
pub const STABILIZED_SCENARIO_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const STABILIZED_SCENARIO_SCHEMA_REF: &str =
    "schemas/support/stabilize_the_seeded_support_scenario_corpus_across_launch_archetypes_and_enterprise_network_rows.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const STABILIZED_SCENARIO_DOC_REF: &str =
    "docs/support/m4/stabilize-the-seeded-support-scenario-corpus-across-launch-archetypes-and-enterprise-network-rows.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const STABILIZED_SCENARIO_ARTIFACT_REF: &str =
    "artifacts/support/m4/stabilize-the-seeded-support-scenario-corpus-across-launch-archetypes-and-enterprise-network-rows.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const STABILIZED_SCENARIO_FIXTURE_DIR: &str =
    "fixtures/support/m4/stabilize-the-seeded-support-scenario-corpus-across-launch-archetypes-and-enterprise-network-rows";

// ---------------------------------------------------------------------------
// Fixture embeds
// ---------------------------------------------------------------------------

const SCENARIO_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/support/m4/stabilize-the-seeded-support-scenario-corpus-across-launch-archetypes-and-enterprise-network-rows/first_run_standard_internet.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m4/stabilize-the-seeded-support-scenario-corpus-across-launch-archetypes-and-enterprise-network-rows/first_run_standard_internet.yaml"
        )),
    ),
    (
        "fixtures/support/m4/stabilize-the-seeded-support-scenario-corpus-across-launch-archetypes-and-enterprise-network-rows/crash_recovery_air_gapped.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m4/stabilize-the-seeded-support-scenario-corpus-across-launch-archetypes-and-enterprise-network-rows/crash_recovery_air_gapped.yaml"
        )),
    ),
    (
        "fixtures/support/m4/stabilize-the-seeded-support-scenario-corpus-across-launch-archetypes-and-enterprise-network-rows/enterprise_managed_proxied.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m4/stabilize-the-seeded-support-scenario-corpus-across-launch-archetypes-and-enterprise-network-rows/enterprise_managed_proxied.yaml"
        )),
    ),
    (
        "fixtures/support/m4/stabilize-the-seeded-support-scenario-corpus-across-launch-archetypes-and-enterprise-network-rows/workspace_open_offline_first.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m4/stabilize-the-seeded-support-scenario-corpus-across-launch-archetypes-and-enterprise-network-rows/workspace_open_offline_first.yaml"
        )),
    ),
    (
        "fixtures/support/m4/stabilize-the-seeded-support-scenario-corpus-across-launch-archetypes-and-enterprise-network-rows/update_restart_restricted_enterprise.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m4/stabilize-the-seeded-support-scenario-corpus-across-launch-archetypes-and-enterprise-network-rows/update_restart_restricted_enterprise.yaml"
        )),
    ),
    (
        "fixtures/support/m4/stabilize-the-seeded-support-scenario-corpus-across-launch-archetypes-and-enterprise-network-rows/extension_install_standard_internet.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m4/stabilize-the-seeded-support-scenario-corpus-across-launch-archetypes-and-enterprise-network-rows/extension_install_standard_internet.yaml"
        )),
    ),
];

const REQUIRED_FORBIDDEN_FIX_CLASSES: &[&str] = &[
    "destructive_reset_without_preview",
    "publish_route",
    "reenable_quarantined_extension_without_preview",
    "run_repo_owned_hook_for_diagnosis",
    "widen_workspace_trust",
];

const REQUIRED_NO_TOUCH_BOUNDARY: &str = "user_authored_files";

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// Closed launch-archetype vocabulary covered by the corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchArchetypeClass {
    /// First-time launch with no prior workspace state.
    FirstRun,
    /// Launch after a product update or restart.
    UpdateRestart,
    /// Launch following a crash-loop recovery event.
    CrashRecovery,
    /// Launch under enterprise-managed policy or device posture.
    EnterpriseManaged,
    /// Launch triggered by an extension install or update.
    ExtensionInstall,
    /// Launch opening an existing workspace.
    WorkspaceOpen,
}

impl LaunchArchetypeClass {
    /// Every required launch archetype, in declaration order.
    pub const REQUIRED: [Self; 6] = [
        Self::FirstRun,
        Self::UpdateRestart,
        Self::CrashRecovery,
        Self::EnterpriseManaged,
        Self::ExtensionInstall,
        Self::WorkspaceOpen,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstRun => "first_run",
            Self::UpdateRestart => "update_restart",
            Self::CrashRecovery => "crash_recovery",
            Self::EnterpriseManaged => "enterprise_managed",
            Self::ExtensionInstall => "extension_install",
            Self::WorkspaceOpen => "workspace_open",
        }
    }
}

/// Closed enterprise-network-row vocabulary covered by the corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnterpriseNetworkRowClass {
    /// Standard internet-connected environment.
    StandardInternet,
    /// Air-gapped environment with no external network access.
    AirGapped,
    /// Network routed through an enterprise proxy.
    Proxied,
    /// Offline-first environment with intermittent connectivity.
    OfflineFirst,
    /// Restricted enterprise network with managed egress.
    RestrictedEnterprise,
}

impl EnterpriseNetworkRowClass {
    /// Every required enterprise network row, in declaration order.
    pub const REQUIRED: [Self; 5] = [
        Self::StandardInternet,
        Self::AirGapped,
        Self::Proxied,
        Self::OfflineFirst,
        Self::RestrictedEnterprise,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StandardInternet => "standard_internet",
            Self::AirGapped => "air_gapped",
            Self::Proxied => "proxied",
            Self::OfflineFirst => "offline_first",
            Self::RestrictedEnterprise => "restricted_enterprise",
        }
    }
}

/// Closed stabilized-scenario-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StabilizedScenarioClass {
    /// Blocked-user recovery scenario.
    BlockedUserRecovery,
    /// Support-export verification scenario.
    SupportExportVerification,
    /// Diagnosis routing scenario.
    DiagnosisRouting,
    /// Repair-preview validation scenario.
    RepairPreviewValidation,
    /// Safe-mode transition scenario.
    SafeModeTransition,
    /// Crash-loop center evidence scenario.
    CrashLoopCenterEvidence,
}

impl StabilizedScenarioClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlockedUserRecovery => "blocked_user_recovery",
            Self::SupportExportVerification => "support_export_verification",
            Self::DiagnosisRouting => "diagnosis_routing",
            Self::RepairPreviewValidation => "repair_preview_validation",
            Self::SafeModeTransition => "safe_mode_transition",
            Self::CrashLoopCenterEvidence => "crash_loop_center_evidence",
        }
    }
}

/// Closed claim-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScenarioClaimStateClass {
    /// Green — the scenario is healthy and contributes to the stable gate.
    Green,
    /// Yellow — the drill evidence is aging and needs refresh.
    YellowAging,
    /// Red — the scenario is blocked and the stable claim cannot promote.
    RedBlocked,
    /// Stale — the corpus or fixture is missing and blocks promotion.
    StaleCorpus,
}

impl ScenarioClaimStateClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::YellowAging => "yellow_aging",
            Self::RedBlocked => "red_blocked",
            Self::StaleCorpus => "stale_corpus",
        }
    }
}

/// Closed downgrade-trigger vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScenarioDowngradeTriggerClass {
    /// A primary fixture referenced by the scenario is missing.
    FixtureMissing,
    /// The reviewer doc is missing.
    DocMissing,
    /// The boundary schema is missing.
    SchemaMissing,
    /// The crate consumer module is missing.
    CrateConsumerMissing,
    /// The protected integration test is missing.
    IntegrationTestMissing,
    /// A declared validation step has no proving artifact reference.
    DrillStepUnproven,
    /// Drill replay produced a record the evaluator refused.
    DrillProvesRegression,
    /// A step proposed an unsafe recovery action.
    RecoveryActionUnsafe,
    /// The scenario exported raw private material or ambient authority.
    RawPrivateMaterialPresent,
    /// The launch archetype does not match the scenario's declared class.
    ArchetypeMismatch,
    /// The enterprise-network posture does not match the scenario's declared class.
    NetworkPostureMismatch,
}

impl ScenarioDowngradeTriggerClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FixtureMissing => "fixture_missing",
            Self::DocMissing => "doc_missing",
            Self::SchemaMissing => "schema_missing",
            Self::CrateConsumerMissing => "crate_consumer_missing",
            Self::IntegrationTestMissing => "integration_test_missing",
            Self::DrillStepUnproven => "drill_step_unproven",
            Self::DrillProvesRegression => "drill_proves_regression",
            Self::RecoveryActionUnsafe => "recovery_action_unsafe",
            Self::RawPrivateMaterialPresent => "raw_private_material_present",
            Self::ArchetypeMismatch => "archetype_mismatch",
            Self::NetworkPostureMismatch => "network_posture_mismatch",
        }
    }
}

// ---------------------------------------------------------------------------
// Row types
// ---------------------------------------------------------------------------

/// Consumer references quoted on every scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioConsumerRefs {
    /// Reviewer doc ref (repo-relative).
    pub doc_ref: String,
    /// Boundary schema ref (repo-relative).
    pub schema_ref: String,
    /// First-consumer crate module ref (repo-relative).
    pub crate_consumer: String,
    /// Protected integration test ref (repo-relative).
    pub integration_test: String,
}

/// Starting condition row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioStartingCondition {
    /// Opaque state ref describing the pre-validation condition.
    pub state_ref: String,
    /// Reviewer-safe summary.
    pub summary: String,
}

/// One validation step in a scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioValidationStep {
    /// Stable step id.
    pub step_id: String,
    /// Step class drawn from closed vocabulary.
    pub step_class: String,
    /// Reviewer-safe summary of what the step proves.
    pub summary: String,
    /// Expected artifact kind the step produces or validates.
    pub expected_artifact_kind: String,
    /// Opaque artifact ref the step expects to bind.
    pub expected_artifact_ref: String,
}

/// Expected first actionable artifact for a scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpectedFirstActionableArtifact {
    /// Artifact kind that resolves the scenario.
    pub artifact_kind: String,
    /// Opaque ref pointing at the artifact.
    pub artifact_ref: String,
    /// Recovery action the artifact routes the user to.
    pub recovery_action_class: String,
    /// Reviewer-safe summary describing why the artifact resolves the scenario.
    pub reviewer_summary: String,
}

/// Scorecard binding from the scenario into the report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioScorecardContribution {
    /// Stable scorecard target.
    pub scorecard_target: String,
    /// Coverage class summarising what the scenario re-proves.
    pub coverage_class: String,
    /// Expected scorecard state token when the validation is healthy.
    pub expected_state: String,
    /// Whether the contribution feeds a release gate.
    pub contributes_to_release_gate: bool,
}

/// Claim-downgrade rule attached to a scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioClaimDowngradeRule {
    /// Closed trigger class.
    pub trigger_class: ScenarioDowngradeTriggerClass,
    /// Claim state to apply when the trigger fires.
    pub claim_state_class: ScenarioClaimStateClass,
    /// Reviewer-safe note explaining the downgrade.
    pub reviewer_note: String,
}

/// Safety baseline for one scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioSafety {
    /// Whether the validation is read-only.
    pub read_only_diagnosis: bool,
    /// Whether raw private material is excluded by default.
    pub raw_private_material_excluded: bool,
    /// Whether the scenario contains destructive resets.
    pub destructive_resets_present: bool,
    /// Whether user-authored files are preserved.
    pub preserves_user_authored_files: bool,
    /// Fix classes that must not appear as first actions.
    pub forbidden_fix_classes: Vec<String>,
    /// State / authority boundaries the validation must not touch.
    pub no_touch_boundary_set: Vec<String>,
}

/// Companion references for one scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioReferences {
    /// Doc ref quoted by the support packet.
    pub lane_doc_ref: String,
    /// Recovery-ladder doc ref.
    pub recovery_ladder_ref: String,
    /// Diagnosis-latency scorecard ref.
    pub diagnosis_latency_scorecard_ref: String,
}

/// One seeded stabilized support scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeededSupportScenario {
    /// Frozen schema version (1).
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable scenario id.
    pub scenario_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Closed launch-archetype class the scenario covers.
    pub launch_archetype_class: LaunchArchetypeClass,
    /// Closed enterprise-network-row class the scenario covers.
    pub enterprise_network_row_class: EnterpriseNetworkRowClass,
    /// Closed stabilized-scenario class.
    pub stabilized_scenario_class: StabilizedScenarioClass,
    /// Project Doctor finding that justified the scenario.
    pub doctor_finding_ref: String,
    /// Consumer refs (doc, schema, crate consumer, integration test).
    pub consumer_refs: ScenarioConsumerRefs,
    /// Starting condition.
    pub starting_condition: ScenarioStartingCondition,
    /// Ordered validation steps.
    pub validation_steps: Vec<ScenarioValidationStep>,
    /// Expected first actionable artifact.
    pub expected_first_actionable_artifact: ExpectedFirstActionableArtifact,
    /// Primary fixture refs the validation replays.
    pub primary_fixture_refs: Vec<String>,
    /// Scorecard binding.
    pub scorecard_contribution: ScenarioScorecardContribution,
    /// Claim-downgrade rules.
    pub claim_downgrade_rules: Vec<ScenarioClaimDowngradeRule>,
    /// Safety baseline.
    pub safety: ScenarioSafety,
    /// Companion refs.
    pub references: ScenarioReferences,
    /// UTC timestamp when the scenario fixture was emitted.
    pub emitted_at: String,
}

/// One fixture-bound entry in the scenario corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizedScenarioCorpusEntry {
    /// Repository-relative fixture path.
    pub fixture_ref: String,
    /// Parsed scenario record.
    pub scenario: SeededSupportScenario,
}

/// Validation violation emitted by the evaluator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizedScenarioViolation {
    /// Stable check id.
    pub check_id: String,
    /// Scenario id, fixture ref, or corpus id that failed.
    pub target_ref: String,
    /// Reviewer-facing message.
    pub message: String,
}

/// Scenario corpus loaded from checked-in fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizedScenarioCorpus {
    /// Fixture-bound entries.
    pub entries: Vec<StabilizedScenarioCorpusEntry>,
}

impl StabilizedScenarioCorpus {
    /// Returns the parsed scenario records without their fixture path wrappers.
    pub fn scenarios(&self) -> impl Iterator<Item = &SeededSupportScenario> {
        self.entries.iter().map(|entry| &entry.scenario)
    }

    /// Validates the corpus against the launch-archetype and enterprise-network
    /// coverage contract.
    pub fn validate(&self) -> Vec<StabilizedScenarioViolation> {
        let mut violations = Vec::new();

        if self.entries.is_empty() {
            push_violation(
                &mut violations,
                "corpus.empty",
                STABILIZED_SCENARIO_FIXTURE_DIR,
                "corpus must contain at least one scenario",
            );
            return violations;
        }

        let mut scenario_ids = BTreeSet::new();
        let mut fixture_refs = BTreeSet::new();
        let mut scorecard_targets = BTreeSet::new();
        let mut archetype_seen: BTreeSet<LaunchArchetypeClass> = BTreeSet::new();
        let mut network_seen: BTreeSet<EnterpriseNetworkRowClass> = BTreeSet::new();

        for entry in &self.entries {
            if !fixture_refs.insert(entry.fixture_ref.clone()) {
                push_violation(
                    &mut violations,
                    "corpus.duplicate_fixture_ref",
                    &entry.fixture_ref,
                    "fixture_ref must be unique within the corpus",
                );
            }
            let scenario = &entry.scenario;
            if !scenario_ids.insert(scenario.scenario_id.clone()) {
                push_violation(
                    &mut violations,
                    "corpus.duplicate_scenario_id",
                    &scenario.scenario_id,
                    "scenario_id must be unique within the corpus",
                );
            }
            if !scorecard_targets.insert(scenario.scorecard_contribution.scorecard_target.clone()) {
                push_violation(
                    &mut violations,
                    "corpus.duplicate_scorecard_target",
                    &scenario.scorecard_contribution.scorecard_target,
                    "scorecard_target must be unique within the corpus",
                );
            }
            archetype_seen.insert(scenario.launch_archetype_class);
            network_seen.insert(scenario.enterprise_network_row_class);
            validate_scenario(&mut violations, scenario);
        }

        for required in LaunchArchetypeClass::REQUIRED {
            if !archetype_seen.contains(&required) {
                push_violation(
                    &mut violations,
                    "corpus.required_archetype_missing",
                    required.as_str(),
                    format!(
                        "required launch archetype {} has no seeded scenario",
                        required.as_str()
                    ),
                );
            }
        }

        for required in EnterpriseNetworkRowClass::REQUIRED {
            if !network_seen.contains(&required) {
                push_violation(
                    &mut violations,
                    "corpus.required_network_row_missing",
                    required.as_str(),
                    format!(
                        "required enterprise network row {} has no seeded scenario",
                        required.as_str()
                    ),
                );
            }
        }

        violations
    }

    /// Projects the corpus into a metadata-safe stabilized scenario report.
    pub fn report(
        &self,
        report_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> StabilizedScenarioReport {
        let rows = self
            .entries
            .iter()
            .map(StabilizedScenarioReportRow::from_entry)
            .collect::<Vec<_>>();
        StabilizedScenarioReport {
            record_kind: STABILIZED_SCENARIO_REPORT_RECORD_KIND.to_owned(),
            schema_version: STABILIZED_SCENARIO_SCHEMA_VERSION,
            report_id: report_id.into(),
            generated_at: generated_at.into(),
            corpus_doc_ref: STABILIZED_SCENARIO_DOC_REF.to_owned(),
            corpus_schema_ref: STABILIZED_SCENARIO_SCHEMA_REF.to_owned(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            required_launch_archetype_classes: LaunchArchetypeClass::REQUIRED
                .iter()
                .map(|a| a.as_str().to_owned())
                .collect(),
            required_enterprise_network_row_classes: EnterpriseNetworkRowClass::REQUIRED
                .iter()
                .map(|n| n.as_str().to_owned())
                .collect(),
            rows,
        }
    }
}

/// One projected row in the stabilized scenario report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizedScenarioReportRow {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Closed launch-archetype class.
    pub launch_archetype_class: LaunchArchetypeClass,
    /// Closed enterprise-network-row class.
    pub enterprise_network_row_class: EnterpriseNetworkRowClass,
    /// Closed stabilized-scenario class.
    pub stabilized_scenario_class: StabilizedScenarioClass,
    /// Stable scenario id.
    pub scenario_id: String,
    /// Reviewer-facing scenario title.
    pub title: String,
    /// Corpus fixture ref.
    pub fixture_ref: String,
    /// Step classes the scenario validates.
    pub validation_step_classes: Vec<String>,
    /// Artifact kinds the scenario re-proves.
    pub expected_artifact_kinds: Vec<String>,
    /// Recovery action class the first actionable artifact routes to.
    pub recovery_action_class: String,
    /// Scorecard target the row contributes to.
    pub scorecard_target: String,
    /// Coverage class.
    pub coverage_class: String,
    /// Expected scorecard state when the validation is healthy.
    pub expected_state: String,
    /// Whether the row contributes to a release gate.
    pub contributes_to_release_gate: bool,
    /// Declared downgrade triggers covered by the scenario.
    pub downgrade_trigger_classes: Vec<String>,
    /// Claim states the scenario can apply.
    pub claim_state_classes: Vec<String>,
    /// Primary doc ref.
    pub doc_ref: String,
    /// Primary schema ref.
    pub schema_ref: String,
    /// Primary crate consumer ref.
    pub crate_consumer_ref: String,
    /// Protected integration-test ref.
    pub integration_test_ref: String,
    /// Whether the row passes the metadata-safe baseline.
    pub metadata_safe_baseline_met: bool,
}

impl StabilizedScenarioReportRow {
    fn from_entry(entry: &StabilizedScenarioCorpusEntry) -> Self {
        let scenario = &entry.scenario;
        let validation_step_classes = scenario
            .validation_steps
            .iter()
            .map(|step| step.step_class.clone())
            .collect::<Vec<_>>();
        let expected_artifact_kinds = {
            let mut kinds = scenario
                .validation_steps
                .iter()
                .map(|step| step.expected_artifact_kind.clone())
                .collect::<Vec<_>>();
            kinds.push(
                scenario
                    .expected_first_actionable_artifact
                    .artifact_kind
                    .clone(),
            );
            kinds.sort();
            kinds.dedup();
            kinds
        };
        let downgrade_trigger_classes = {
            let mut tokens = scenario
                .claim_downgrade_rules
                .iter()
                .map(|rule| rule.trigger_class.as_str().to_owned())
                .collect::<Vec<_>>();
            tokens.sort();
            tokens.dedup();
            tokens
        };
        let claim_state_classes = {
            let mut tokens = scenario
                .claim_downgrade_rules
                .iter()
                .map(|rule| rule.claim_state_class.as_str().to_owned())
                .collect::<Vec<_>>();
            tokens.sort();
            tokens.dedup();
            tokens
        };
        let metadata_safe_baseline_met = scenario.safety.read_only_diagnosis
            && scenario.safety.raw_private_material_excluded
            && !scenario.safety.destructive_resets_present
            && scenario.safety.preserves_user_authored_files;
        Self {
            record_kind: STABILIZED_SCENARIO_REPORT_ROW_RECORD_KIND.to_owned(),
            launch_archetype_class: scenario.launch_archetype_class,
            enterprise_network_row_class: scenario.enterprise_network_row_class,
            stabilized_scenario_class: scenario.stabilized_scenario_class,
            scenario_id: scenario.scenario_id.clone(),
            title: scenario.title.clone(),
            fixture_ref: entry.fixture_ref.clone(),
            validation_step_classes,
            expected_artifact_kinds,
            recovery_action_class: scenario
                .expected_first_actionable_artifact
                .recovery_action_class
                .clone(),
            scorecard_target: scenario.scorecard_contribution.scorecard_target.clone(),
            coverage_class: scenario.scorecard_contribution.coverage_class.clone(),
            expected_state: scenario.scorecard_contribution.expected_state.clone(),
            contributes_to_release_gate: scenario
                .scorecard_contribution
                .contributes_to_release_gate,
            downgrade_trigger_classes,
            claim_state_classes,
            doc_ref: scenario.consumer_refs.doc_ref.clone(),
            schema_ref: scenario.consumer_refs.schema_ref.clone(),
            crate_consumer_ref: scenario.consumer_refs.crate_consumer.clone(),
            integration_test_ref: scenario.consumer_refs.integration_test.clone(),
            metadata_safe_baseline_met,
        }
    }
}

/// Stabilized scenario report projected from the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizedScenarioReport {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Projection schema version.
    pub schema_version: u32,
    /// Stable report id.
    pub report_id: String,
    /// UTC generation timestamp.
    pub generated_at: String,
    /// Corpus doc ref.
    pub corpus_doc_ref: String,
    /// Corpus schema ref.
    pub corpus_schema_ref: String,
    /// Whether raw private material is excluded by default.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded by default.
    pub ambient_authority_excluded: bool,
    /// Required launch-archetype class tokens covered by the report.
    pub required_launch_archetype_classes: Vec<String>,
    /// Required enterprise-network-row class tokens covered by the report.
    pub required_enterprise_network_row_classes: Vec<String>,
    /// Per-scenario rows.
    pub rows: Vec<StabilizedScenarioReportRow>,
}

impl StabilizedScenarioReport {
    /// Returns true when every required archetype and network row has a row
    /// and every row meets the metadata-safe baseline.
    pub fn is_export_safe(&self) -> bool {
        if !self.raw_private_material_excluded || !self.ambient_authority_excluded {
            return false;
        }
        let archetype_set: BTreeSet<_> = self
            .rows
            .iter()
            .map(|row| row.launch_archetype_class.as_str().to_owned())
            .collect();
        for required in &self.required_launch_archetype_classes {
            if !archetype_set.contains(required) {
                return false;
            }
        }
        let network_set: BTreeSet<_> = self
            .rows
            .iter()
            .map(|row| row.enterprise_network_row_class.as_str().to_owned())
            .collect();
        for required in &self.required_enterprise_network_row_classes {
            if !network_set.contains(required) {
                return false;
            }
        }
        self.rows.iter().all(|row| row.metadata_safe_baseline_met)
    }
}

/// Metadata-safe support projection for one stabilized scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizedScenarioSupportPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Packet schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Doc ref the packet quotes.
    pub doc_ref: String,
    /// Boundary schema ref the packet mirrors.
    pub schema_ref: String,
    /// Scenario id projected by the packet.
    pub scenario_id: String,
    /// Launch archetype class.
    pub launch_archetype_class: LaunchArchetypeClass,
    /// Enterprise network row class.
    pub enterprise_network_row_class: EnterpriseNetworkRowClass,
    /// Stabilized scenario class.
    pub stabilized_scenario_class: StabilizedScenarioClass,
    /// Project Doctor finding ref the packet cites.
    pub doctor_finding_ref: String,
    /// Validation step classes.
    pub validation_step_classes: Vec<String>,
    /// Expected artifact kinds.
    pub expected_artifact_kinds: Vec<String>,
    /// Recovery action class.
    pub recovery_action_class: String,
    /// Scorecard target.
    pub scorecard_target: String,
    /// Coverage class.
    pub coverage_class: String,
    /// Expected state.
    pub expected_state: String,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// Whether the projection records a destructive reset.
    pub destructive_resets_present: bool,
}

impl StabilizedScenarioSupportPacket {
    /// Returns true when the packet preserves the bounded scenario contract.
    pub fn is_export_safe(&self) -> bool {
        self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.destructive_resets_present
            && self.doctor_finding_ref.starts_with("doctor.finding.")
            && !self.validation_step_classes.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Evaluator
// ---------------------------------------------------------------------------

/// Stabilized scenario stable evaluator.
#[derive(Debug, Default, Clone, Copy)]
pub struct StabilizedScenarioEvaluator;

impl StabilizedScenarioEvaluator {
    /// Creates a new stabilized scenario evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Validates a [`StabilizedScenarioCorpus`].
    ///
    /// # Errors
    ///
    /// Returns [`StabilizedScenarioValidationReport`] when the corpus omits
    /// required launch archetypes, enterprise network rows, or fails scenario
    /// safety baselines.
    pub fn validate_corpus(
        &self,
        corpus: &StabilizedScenarioCorpus,
    ) -> Result<(), StabilizedScenarioValidationReport> {
        let violations = corpus.validate();
        if violations.is_empty() {
            Ok(())
        } else {
            Err(StabilizedScenarioValidationReport { violations })
        }
    }

    /// Builds the metadata-safe support packet projection for a scenario.
    ///
    /// # Errors
    ///
    /// Returns [`StabilizedScenarioValidationReport`] when the scenario fails
    /// validation.
    pub fn support_packet(
        &self,
        packet_id: impl Into<String>,
        captured_at: impl Into<String>,
        scenario: &SeededSupportScenario,
    ) -> Result<StabilizedScenarioSupportPacket, StabilizedScenarioValidationReport> {
        let mut violations = Vec::new();
        validate_scenario(&mut violations, scenario);
        if !violations.is_empty() {
            return Err(StabilizedScenarioValidationReport { violations });
        }

        Ok(StabilizedScenarioSupportPacket {
            record_kind: STABILIZED_SCENARIO_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: STABILIZED_SCENARIO_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            captured_at: captured_at.into(),
            doc_ref: STABILIZED_SCENARIO_DOC_REF.to_owned(),
            schema_ref: STABILIZED_SCENARIO_SCHEMA_REF.to_owned(),
            scenario_id: scenario.scenario_id.clone(),
            launch_archetype_class: scenario.launch_archetype_class,
            enterprise_network_row_class: scenario.enterprise_network_row_class,
            stabilized_scenario_class: scenario.stabilized_scenario_class,
            doctor_finding_ref: scenario.doctor_finding_ref.clone(),
            validation_step_classes: scenario
                .validation_steps
                .iter()
                .map(|s| s.step_class.clone())
                .collect(),
            expected_artifact_kinds: {
                let mut kinds = scenario
                    .validation_steps
                    .iter()
                    .map(|s| s.expected_artifact_kind.clone())
                    .collect::<Vec<_>>();
                kinds.push(
                    scenario
                        .expected_first_actionable_artifact
                        .artifact_kind
                        .clone(),
                );
                kinds.sort();
                kinds.dedup();
                kinds
            },
            recovery_action_class: scenario
                .expected_first_actionable_artifact
                .recovery_action_class
                .clone(),
            scorecard_target: scenario.scorecard_contribution.scorecard_target.clone(),
            coverage_class: scenario.scorecard_contribution.coverage_class.clone(),
            expected_state: scenario.scorecard_contribution.expected_state.clone(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            destructive_resets_present: false,
        })
    }

    /// Builds the metadata-safe report projection for a corpus.
    ///
    /// # Errors
    ///
    /// Returns [`StabilizedScenarioValidationReport`] when the corpus fails
    /// validation.
    pub fn report(
        &self,
        report_id: impl Into<String>,
        generated_at: impl Into<String>,
        corpus: &StabilizedScenarioCorpus,
    ) -> Result<StabilizedScenarioReport, StabilizedScenarioValidationReport> {
        let violations = corpus.validate();
        if !violations.is_empty() {
            return Err(StabilizedScenarioValidationReport { violations });
        }
        Ok(corpus.report(report_id, generated_at))
    }
}

/// Validation report returned when one or more checks fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StabilizedScenarioValidationReport {
    /// Validation failures.
    pub violations: Vec<StabilizedScenarioViolation>,
}

impl fmt::Display for StabilizedScenarioValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} stabilized scenario violation(s)",
            self.violations.len()
        )
    }
}

impl Error for StabilizedScenarioValidationReport {}

// ---------------------------------------------------------------------------
// Loaders
// ---------------------------------------------------------------------------

/// Loads one seeded support scenario from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text does not match
/// [`SeededSupportScenario`].
pub fn load_seeded_support_scenario(
    yaml: &str,
) -> Result<SeededSupportScenario, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads the checked-in stabilized scenario corpus.
///
/// # Errors
///
/// Returns a YAML parse error when a fixture does not match
/// [`SeededSupportScenario`].
pub fn current_stabilized_scenario_corpus() -> Result<StabilizedScenarioCorpus, serde_yaml::Error> {
    let entries = SCENARIO_FIXTURES
        .iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str::<SeededSupportScenario>(yaml).map(|scenario| {
                StabilizedScenarioCorpusEntry {
                    fixture_ref: (*fixture_ref).to_owned(),
                    scenario,
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(StabilizedScenarioCorpus { entries })
}

// ---------------------------------------------------------------------------
// Internal validation
// ---------------------------------------------------------------------------

fn validate_scenario(
    violations: &mut Vec<StabilizedScenarioViolation>,
    scenario: &SeededSupportScenario,
) {
    if scenario.schema_version != STABILIZED_SCENARIO_SCHEMA_VERSION {
        push_violation(
            violations,
            "scenario.schema_version",
            &scenario.scenario_id,
            "scenario schema_version must be 1",
        );
    }
    if scenario.record_kind != STABILIZED_SUPPORT_SCENARIO_RECORD_KIND {
        push_violation(
            violations,
            "scenario.record_kind",
            &scenario.scenario_id,
            "scenario record_kind must be stabilized_support_scenario_record",
        );
    }
    if scenario.scenario_id.trim().is_empty() {
        push_violation(
            violations,
            "scenario.scenario_id_empty",
            &scenario.scenario_id,
            "scenario_id must be non-empty",
        );
    }
    if scenario.title.trim().is_empty() {
        push_violation(
            violations,
            "scenario.title_empty",
            &scenario.scenario_id,
            "title must be non-empty",
        );
    }
    if !scenario.doctor_finding_ref.starts_with("doctor.finding.") {
        push_violation(
            violations,
            "scenario.doctor_finding_ref_missing",
            &scenario.scenario_id,
            "scenario must cite a Project Doctor finding ref",
        );
    }
    if scenario.validation_steps.is_empty() {
        push_violation(
            violations,
            "scenario.validation_steps_empty",
            &scenario.scenario_id,
            "validation_steps must not be empty",
        );
    }
    for step in &scenario.validation_steps {
        if step.step_id.trim().is_empty() {
            push_violation(
                violations,
                "scenario.step_id_empty",
                &scenario.scenario_id,
                "validation step step_id must be non-empty",
            );
        }
        if step.summary.trim().is_empty() {
            push_violation(
                violations,
                "scenario.step_summary_empty",
                &scenario.scenario_id,
                "validation step summary must be non-empty",
            );
        }
        if step.expected_artifact_ref.trim().is_empty() {
            push_violation(
                violations,
                "scenario.step_artifact_ref_empty",
                &scenario.scenario_id,
                "validation step expected_artifact_ref must be non-empty",
            );
        }
    }
    if scenario
        .expected_first_actionable_artifact
        .artifact_ref
        .trim()
        .is_empty()
    {
        push_violation(
            violations,
            "scenario.first_actionable_artifact_ref_empty",
            &scenario.scenario_id,
            "expected_first_actionable_artifact.artifact_ref must be non-empty",
        );
    }
    if scenario
        .expected_first_actionable_artifact
        .recovery_action_class
        .trim()
        .is_empty()
    {
        push_violation(
            violations,
            "scenario.recovery_action_class_empty",
            &scenario.scenario_id,
            "expected_first_actionable_artifact.recovery_action_class must be non-empty",
        );
    }
    if scenario.primary_fixture_refs.is_empty() {
        push_violation(
            violations,
            "scenario.primary_fixture_refs_empty",
            &scenario.scenario_id,
            "primary_fixture_refs must not be empty",
        );
    }
    if scenario
        .scorecard_contribution
        .scorecard_target
        .trim()
        .is_empty()
    {
        push_violation(
            violations,
            "scenario.scorecard_target_empty",
            &scenario.scenario_id,
            "scorecard_contribution.scorecard_target must be non-empty",
        );
    }
    if scenario.claim_downgrade_rules.is_empty() {
        push_violation(
            violations,
            "scenario.claim_downgrade_rules_empty",
            &scenario.scenario_id,
            "claim_downgrade_rules must not be empty",
        );
    }
    for rule in &scenario.claim_downgrade_rules {
        if rule.reviewer_note.trim().is_empty() {
            push_violation(
                violations,
                "scenario.downgrade_reviewer_note_empty",
                &scenario.scenario_id,
                "claim_downgrade_rule.reviewer_note must be non-empty",
            );
        }
    }

    if scenario.safety.destructive_resets_present {
        push_violation(
            violations,
            "scenario.destructive_resets_present",
            &scenario.scenario_id,
            "scenario must not declare destructive resets",
        );
    }
    if !scenario.safety.read_only_diagnosis {
        push_violation(
            violations,
            "scenario.read_only_diagnosis_false",
            &scenario.scenario_id,
            "scenario must declare read_only_diagnosis true",
        );
    }
    if !scenario.safety.raw_private_material_excluded {
        push_violation(
            violations,
            "scenario.raw_private_material_excluded_false",
            &scenario.scenario_id,
            "scenario must declare raw_private_material_excluded true",
        );
    }
    if !scenario.safety.preserves_user_authored_files {
        push_violation(
            violations,
            "scenario.preserves_user_authored_files_false",
            &scenario.scenario_id,
            "scenario must declare preserves_user_authored_files true",
        );
    }

    let forbidden: BTreeSet<_> = REQUIRED_FORBIDDEN_FIX_CLASSES.iter().copied().collect();
    for fix in &scenario.safety.forbidden_fix_classes {
        if !forbidden.contains(fix.as_str()) {
            push_violation(
                violations,
                "scenario.forbidden_fix_class_unknown",
                &scenario.scenario_id,
                format!("forbidden fix class {fix} is not in the required baseline"),
            );
        }
    }
    if !scenario
        .safety
        .no_touch_boundary_set
        .iter()
        .any(|b| b == REQUIRED_NO_TOUCH_BOUNDARY)
    {
        push_violation(
            violations,
            "scenario.no_touch_boundary_missing",
            &scenario.scenario_id,
            format!("no_touch_boundary_set must contain {REQUIRED_NO_TOUCH_BOUNDARY}"),
        );
    }
}

fn push_violation(
    violations: &mut Vec<StabilizedScenarioViolation>,
    check_id: &str,
    target_ref: impl fmt::Display,
    message: impl fmt::Display,
) {
    violations.push(StabilizedScenarioViolation {
        check_id: check_id.to_owned(),
        target_ref: target_ref.to_string(),
        message: message.to_string(),
    });
}
