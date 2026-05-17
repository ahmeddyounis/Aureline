//! M3 support-scenario corpus and drill-harness baseline.
//!
//! This module is the canonical loader and validator for the protected
//! M3 support-scenario corpus under
//! `/fixtures/support/m3/scenario_corpus/`. It folds the seeded
//! scenarios into a typed [`M3ScenarioCorpus`] and a
//! [`M3DrillHarnessReport`] that the support-export pipeline, QE
//! reviewers, and the release-evidence lane consume verbatim — every
//! row quotes the closed `beta_lane_class`, `drill_class`,
//! `drill_step_class`, `expected_artifact_kind`,
//! `claim_downgrade_class`, and `claim_downgrade_trigger_class`
//! vocabularies declared here, never free-form prose.
//!
//! ## What this seed owns
//!
//! - The seven protected M3 beta lanes a release-candidate must keep
//!   covered with at least one seeded scenario and one drill path
//!   ([`required_beta_lane_classes`]).
//! - The closed [`M3BetaLaneClass`], [`M3DrillClass`],
//!   [`M3DrillStepClass`], [`M3ExpectedArtifactKind`],
//!   [`M3ClaimDowngradeClass`], and
//!   [`M3ClaimDowngradeTriggerClass`] vocabularies the corpus,
//!   harness report, and downstream scorecards share.
//! - The [`M3ScenarioCorpus::validate`] entry point that refuses a
//!   corpus that is missing a lane, registers duplicates, or fails the
//!   metadata-safe baseline; and the
//!   [`M3DrillHarnessReport`] projection that converts the validated
//!   corpus into a reviewer-safe support row set with
//!   `raw_private_material_excluded = true` and
//!   `ambient_authority_excluded = true`.
//!
//! ## What this seed does NOT own
//!
//! - Live runtime probe execution, fixture mutation, or the apply
//!   side of any beta lane. Each scenario references the already-owning
//!   beta-lane crate/test pair and asks the drill harness to re-prove
//!   the lane's lifecycle, not to re-implement it.
//! - Live measurement of support-scenario latency. The alpha
//!   [`crate::scenario_scorecard`] module remains the source of truth
//!   for the alpha diagnosis-latency lane; the M3 corpus here is the
//!   beta-lane coverage layer that complements (rather than replaces)
//!   the alpha scorecard.
//! - Hosted ticket intake or cross-tenant escalation. The harness
//!   report stays metadata-safe and references support packets that
//!   already exist as governed projections in their owning crates.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for a seeded M3 support scenario.
pub const M3_SUPPORT_SCENARIO_RECORD_KIND: &str = "m3_support_scenario_record";

/// Stable record-kind tag for the drill-harness report projection.
pub const M3_DRILL_HARNESS_REPORT_RECORD_KIND: &str = "m3_drill_harness_report_record";

/// Stable record-kind tag for one row in the drill-harness report.
pub const M3_DRILL_HARNESS_LANE_ROW_RECORD_KIND: &str = "m3_drill_harness_lane_row_record";

/// Frozen schema version for the M3 scenario corpus.
pub const M3_SUPPORT_SCENARIO_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path of the protected corpus directory.
pub const M3_SCENARIO_CORPUS_DIR: &str = "fixtures/support/m3/scenario_corpus";

/// Repository-relative path of the protected corpus manifest.
pub const M3_SCENARIO_CORPUS_MANIFEST_REF: &str =
    "fixtures/support/m3/scenario_corpus/manifest.yaml";

/// Repository-relative path of the reviewer doc.
pub const M3_SCENARIO_CORPUS_DOC_REF: &str = "docs/support/m3/support_scenario_corpus.md";

/// Repository-relative path of the drill-harness report artifact.
pub const M3_DRILL_HARNESS_REPORT_REF: &str = "artifacts/support/m3/drill_harness_report.md";

const SCENARIO_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/support/m3/scenario_corpus/safe_mode_post_crash_loop_entry_and_exit.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/scenario_corpus/safe_mode_post_crash_loop_entry_and_exit.yaml"
        )),
    ),
    (
        "fixtures/support/m3/scenario_corpus/extension_bisect_single_suspect_attribution.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/scenario_corpus/extension_bisect_single_suspect_attribution.yaml"
        )),
    ),
    (
        "fixtures/support/m3/scenario_corpus/repair_preview_extension_quarantine_comparison.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/scenario_corpus/repair_preview_extension_quarantine_comparison.yaml"
        )),
    ),
    (
        "fixtures/support/m3/scenario_corpus/doctor_probe_pack_entry_open_routing.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/scenario_corpus/doctor_probe_pack_entry_open_routing.yaml"
        )),
    ),
    (
        "fixtures/support/m3/scenario_corpus/project_doctor_beta_finding_render_packet.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/scenario_corpus/project_doctor_beta_finding_render_packet.yaml"
        )),
    ),
    (
        "fixtures/support/m3/scenario_corpus/records_governance_held_support_bundle_chain.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/scenario_corpus/records_governance_held_support_bundle_chain.yaml"
        )),
    ),
    (
        "fixtures/support/m3/scenario_corpus/runtime_replay_pack_layout_only_mutating.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/scenario_corpus/runtime_replay_pack_layout_only_mutating.yaml"
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

const REQUIRED_DOWNGRADE_TRIGGERS: &[M3ClaimDowngradeTriggerClass] = &[
    M3ClaimDowngradeTriggerClass::FixtureMissing,
    M3ClaimDowngradeTriggerClass::DrillStepUnproven,
    M3ClaimDowngradeTriggerClass::DrillProvesRegression,
];

/// Closed M3 beta-lane vocabulary covered by the corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M3BetaLaneClass {
    /// Safe-mode runtime profile and entry/exit transitions.
    SafeMode,
    /// Extension bisect orchestration with attributable findings.
    ExtensionBisect,
    /// Repair-transaction preview skeleton with compare-before-apply.
    RepairTransactionPreview,
    /// Doctor probe-pack family catalog with recovery routing.
    DoctorProbePacks,
    /// Project Doctor beta finding contract (typed, attributable, confidence-labeled).
    ProjectDoctorFindingContract,
    /// Records-governance packets with chain-of-custody and hold awareness.
    RecordsGovernance,
    /// Runtime replay packs with fidelity, privilege gating, and reopen decisions.
    RuntimeReplayPackets,
}

impl M3BetaLaneClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeMode => "safe_mode",
            Self::ExtensionBisect => "extension_bisect",
            Self::RepairTransactionPreview => "repair_transaction_preview",
            Self::DoctorProbePacks => "doctor_probe_packs",
            Self::ProjectDoctorFindingContract => "project_doctor_finding_contract",
            Self::RecordsGovernance => "records_governance",
            Self::RuntimeReplayPackets => "runtime_replay_packets",
        }
    }
}

/// Beta lanes the corpus must cover before release-candidate promotion.
pub const REQUIRED_BETA_LANE_CLASSES: [M3BetaLaneClass; 7] = [
    M3BetaLaneClass::SafeMode,
    M3BetaLaneClass::ExtensionBisect,
    M3BetaLaneClass::RepairTransactionPreview,
    M3BetaLaneClass::DoctorProbePacks,
    M3BetaLaneClass::ProjectDoctorFindingContract,
    M3BetaLaneClass::RecordsGovernance,
    M3BetaLaneClass::RuntimeReplayPackets,
];

/// Returns the closed list of beta lanes the corpus must cover.
pub fn required_beta_lane_classes() -> &'static [M3BetaLaneClass] {
    &REQUIRED_BETA_LANE_CLASSES
}

/// Closed drill-class vocabulary; bounds how a scenario reproves a lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M3DrillClass {
    /// Reproves an entry/exit safe-mode or extension-bisect lifecycle.
    FailureRecoveryDrill,
    /// Reproves a repair-preview skeleton with cancellable comparison.
    RepairPreviewDrill,
    /// Reproves a Project Doctor probe-pack or finding-contract path.
    DiagnosisRoutingDrill,
    /// Reproves a records-governance packet, chain, and hold contract.
    GovernanceChainDrill,
    /// Reproves a runtime replay pack's fidelity and reopen decision.
    ReplayDecisionDrill,
}

impl M3DrillClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FailureRecoveryDrill => "failure_recovery_drill",
            Self::RepairPreviewDrill => "repair_preview_drill",
            Self::DiagnosisRoutingDrill => "diagnosis_routing_drill",
            Self::GovernanceChainDrill => "governance_chain_drill",
            Self::ReplayDecisionDrill => "replay_decision_drill",
        }
    }
}

/// Closed step-class vocabulary for drill steps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M3DrillStepClass {
    /// Bind the drill to a typed Project Doctor finding.
    BindDoctorFinding,
    /// Enter a typed safe-mode profile.
    EnterSafeMode,
    /// Exit a typed safe-mode profile.
    ExitSafeMode,
    /// Start (or extend) an extension-bisect session.
    StartExtensionBisect,
    /// Restore the prior extension state captured at session start.
    RestoreExtensionState,
    /// Compile a beta repair-preview skeleton from an alpha transaction.
    CompileRepairSkeleton,
    /// Bind or verify a repair-preview comparison record.
    CompareRepairSkeleton,
    /// Evaluate a Doctor probe pack against its prerequisites.
    EvaluateDoctorProbePack,
    /// Emit a typed beta Project Doctor finding.
    EmitDoctorBetaFinding,
    /// Mint or validate a records-governance packet.
    MintRecordsGovernancePacket,
    /// Compute or refuse a runtime-replay decision.
    ComputeReplayDecision,
    /// Project the lane's records into a metadata-safe support packet.
    ExportSupportPacket,
}

impl M3DrillStepClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BindDoctorFinding => "bind_doctor_finding",
            Self::EnterSafeMode => "enter_safe_mode",
            Self::ExitSafeMode => "exit_safe_mode",
            Self::StartExtensionBisect => "start_extension_bisect",
            Self::RestoreExtensionState => "restore_extension_state",
            Self::CompileRepairSkeleton => "compile_repair_skeleton",
            Self::CompareRepairSkeleton => "compare_repair_skeleton",
            Self::EvaluateDoctorProbePack => "evaluate_doctor_probe_pack",
            Self::EmitDoctorBetaFinding => "emit_doctor_beta_finding",
            Self::MintRecordsGovernancePacket => "mint_records_governance_packet",
            Self::ComputeReplayDecision => "compute_replay_decision",
            Self::ExportSupportPacket => "export_support_packet",
        }
    }
}

/// Closed expected-artifact-kind vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M3ExpectedArtifactKind {
    /// Safe-mode runtime profile record.
    SafeModeProfileRecord,
    /// Safe-mode entry/exit transition record.
    SafeModeTransitionRecord,
    /// Safe-mode support packet projection record.
    SafeModeSupportPacketRecord,
    /// Extension-bisect session record.
    ExtensionBisectSessionRecord,
    /// Extension-bisect step record (baseline, cohort activation, verification, exit).
    ExtensionBisectStepRecord,
    /// Extension-bisect user-visible finding record.
    ExtensionBisectFindingRecord,
    /// Extension-bisect restore record.
    ExtensionBisectRestoreRecord,
    /// Extension-bisect support packet record.
    ExtensionBisectSupportPacketRecord,
    /// Repair-transaction preview skeleton record.
    RepairPreviewSkeletonRecord,
    /// Repair-transaction preview comparison record.
    RepairPreviewComparisonRecord,
    /// Repair-preview skeleton support packet record.
    RepairPreviewSkeletonSupportPacketRecord,
    /// Doctor probe-pack record.
    DoctorProbePackRecord,
    /// Doctor probe-pack catalog record.
    DoctorProbePackCatalogRecord,
    /// Doctor probe-pack coverage scorecard projection.
    DoctorProbePackCoverageScorecard,
    /// Project Doctor beta finding record.
    ProjectDoctorFindingRecord,
    /// Project Doctor beta support packet projection.
    ProjectDoctorBetaSupportPacket,
    /// Records-governance packet record.
    RecordsGovernancePacketRecord,
    /// Records-governance support-bundle preview item.
    RecordsGovernanceSupportPreviewItem,
    /// Runtime replay pack record.
    RuntimeReplayPack,
    /// Runtime replay pack support-export projection.
    RuntimeReplayPackSupportExport,
}

impl M3ExpectedArtifactKind {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeModeProfileRecord => "safe_mode_profile_record",
            Self::SafeModeTransitionRecord => "safe_mode_transition_record",
            Self::SafeModeSupportPacketRecord => "safe_mode_support_packet_record",
            Self::ExtensionBisectSessionRecord => "extension_bisect_session_record",
            Self::ExtensionBisectStepRecord => "extension_bisect_step_record",
            Self::ExtensionBisectFindingRecord => "extension_bisect_finding_record",
            Self::ExtensionBisectRestoreRecord => "extension_bisect_restore_record",
            Self::ExtensionBisectSupportPacketRecord => "extension_bisect_support_packet_record",
            Self::RepairPreviewSkeletonRecord => "repair_preview_skeleton_record",
            Self::RepairPreviewComparisonRecord => "repair_preview_comparison_record",
            Self::RepairPreviewSkeletonSupportPacketRecord => {
                "repair_preview_skeleton_support_packet_record"
            }
            Self::DoctorProbePackRecord => "doctor_probe_pack_record",
            Self::DoctorProbePackCatalogRecord => "doctor_probe_pack_catalog_record",
            Self::DoctorProbePackCoverageScorecard => "doctor_probe_pack_coverage_scorecard",
            Self::ProjectDoctorFindingRecord => "project_doctor_finding_record",
            Self::ProjectDoctorBetaSupportPacket => "project_doctor_beta_support_packet",
            Self::RecordsGovernancePacketRecord => "records_governance_packet_record",
            Self::RecordsGovernanceSupportPreviewItem => "records_governance_support_preview_item",
            Self::RuntimeReplayPack => "runtime_replay_pack",
            Self::RuntimeReplayPackSupportExport => "runtime_replay_pack_support_export",
        }
    }
}

/// Closed claim-downgrade class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M3ClaimDowngradeClass {
    /// Red — the beta claim is blocked until the regression is fixed or
    /// the claim is rescoped.
    RedBlocksBetaClaim,
    /// Yellow — the drill evidence is aging and the scorecard moves to
    /// "needs refresh" until the proving artifact returns.
    YellowAgingDrillEvidence,
    /// Stale corpus — a required scenario or fixture is missing; the
    /// release candidate cannot promote until the corpus is restored.
    StaleCorpusBlocksReleaseCandidate,
}

impl M3ClaimDowngradeClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RedBlocksBetaClaim => "red_blocks_beta_claim",
            Self::YellowAgingDrillEvidence => "yellow_aging_drill_evidence",
            Self::StaleCorpusBlocksReleaseCandidate => "stale_corpus_blocks_release_candidate",
        }
    }
}

/// Closed downgrade-trigger vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M3ClaimDowngradeTriggerClass {
    /// A primary fixture referenced by the scenario is missing.
    FixtureMissing,
    /// The beta-lane reviewer doc is missing.
    DocMissing,
    /// The boundary schema is missing.
    SchemaMissing,
    /// The crate consumer module is missing.
    CrateConsumerMissing,
    /// The protected integration test is missing.
    IntegrationTestMissing,
    /// A declared drill step has no proving artifact reference.
    DrillStepUnproven,
    /// Drill replay produced a record the evaluator refused.
    DrillProvesRegression,
    /// A drill step proposed an unsafe recovery action (destructive
    /// reset, trust widen, re-enable without preview, ...).
    RecoveryActionUnsafe,
    /// The lane exported raw private material or ambient authority.
    RawPrivateMaterialPresent,
}

impl M3ClaimDowngradeTriggerClass {
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
        }
    }
}

/// Beta-lane consumer references quoted on every scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaLaneRefs {
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
pub struct M3ScenarioStartingCondition {
    /// Opaque state ref describing the pre-drill condition.
    pub state_ref: String,
    /// Reviewer-safe summary.
    pub summary: String,
}

/// One drill step in a scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M3DrillStep {
    /// Stable step id.
    pub step_id: String,
    /// Closed step class.
    pub step_class: M3DrillStepClass,
    /// Reviewer-safe summary of what the step proves.
    pub summary: String,
    /// Closed expected-artifact kind the step produces or validates.
    pub expected_artifact_kind: M3ExpectedArtifactKind,
    /// Opaque artifact ref the step expects to bind.
    pub expected_artifact_ref: String,
}

/// First actionable artifact for a scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M3ExpectedFirstActionableArtifact {
    /// Closed artifact kind that resolves the scenario.
    pub artifact_kind: M3ExpectedArtifactKind,
    /// Opaque ref pointing at the artifact.
    pub artifact_ref: String,
    /// Closed recovery action the artifact routes the user to.
    pub recovery_action_class: String,
    /// Reviewer-safe summary describing why the artifact resolves the lane.
    pub reviewer_summary: String,
}

/// Scorecard binding from the scenario into the harness report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M3ScenarioScorecardContribution {
    /// Stable scorecard target.
    pub scorecard_target: String,
    /// Closed coverage class summarising what the scenario re-proves.
    pub coverage_class: String,
    /// Expected scorecard state token when the drill is healthy.
    pub expected_state: String,
    /// Whether the contribution feeds a release gate.
    pub contributes_to_release_gate: bool,
}

/// Claim-downgrade rule attached to a scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M3ClaimDowngradeRule {
    /// Closed trigger class.
    pub trigger_class: M3ClaimDowngradeTriggerClass,
    /// Closed downgrade class to apply when the trigger fires.
    pub downgrade_class: M3ClaimDowngradeClass,
    /// Reviewer-safe note explaining the downgrade.
    pub reviewer_note: String,
}

/// Safety baseline for one scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M3ScenarioSafety {
    /// Whether the drill is read-only.
    pub read_only_diagnosis: bool,
    /// Whether raw private material is excluded by default.
    pub raw_private_material_excluded: bool,
    /// Whether the scenario contains destructive resets.
    pub destructive_resets_present: bool,
    /// Whether user-authored files are preserved.
    pub preserves_user_authored_files: bool,
    /// Fix classes that must not appear as first actions.
    pub forbidden_fix_classes: Vec<String>,
    /// State / authority boundaries the drill must not touch.
    pub no_touch_boundary_set: Vec<String>,
}

/// Companion refs for one scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M3ScenarioReferences {
    /// Beta-lane doc ref quoted by the support packet.
    pub beta_lane_doc_ref: String,
    /// Recovery-ladder alpha doc ref.
    pub recovery_ladder_alpha_ref: String,
    /// Alpha diagnosis-latency scorecard ref.
    pub diagnosis_latency_scorecard_ref: String,
}

/// One seeded M3 support scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M3SupportScenario {
    /// Frozen schema version (1).
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable scenario id.
    pub scenario_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Closed beta-lane class the scenario covers.
    pub beta_lane_class: M3BetaLaneClass,
    /// Beta-lane refs (doc, schema, crate consumer, integration test).
    pub beta_lane_refs: BetaLaneRefs,
    /// Closed drill class.
    pub drill_class: M3DrillClass,
    /// Owning support / release lane for the drill.
    pub drill_owner_lane: String,
    /// Starting condition.
    pub starting_condition: M3ScenarioStartingCondition,
    /// Ordered drill steps.
    pub drill_steps: Vec<M3DrillStep>,
    /// Expected first actionable artifact.
    pub expected_first_actionable_artifact: M3ExpectedFirstActionableArtifact,
    /// Primary fixture refs the drill replays.
    pub primary_fixture_refs: Vec<String>,
    /// Scorecard binding.
    pub scorecard_contribution: M3ScenarioScorecardContribution,
    /// Claim-downgrade rules.
    pub claim_downgrade_rules: Vec<M3ClaimDowngradeRule>,
    /// Safety baseline.
    pub safety: M3ScenarioSafety,
    /// Companion refs.
    pub references: M3ScenarioReferences,
    /// UTC timestamp when the scenario fixture was emitted.
    pub emitted_at: String,
}

/// One fixture-bound entry in the scenario corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M3ScenarioCorpusEntry {
    /// Repository-relative fixture path.
    pub fixture_ref: String,
    /// Parsed scenario record.
    pub scenario: M3SupportScenario,
}

/// Validation violation emitted by the harness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M3CorpusViolation {
    /// Stable check id.
    pub check_id: String,
    /// Scenario id, fixture ref, or corpus id that failed.
    pub target_ref: String,
    /// Reviewer-facing message.
    pub message: String,
}

/// Scenario corpus loaded from checked-in fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M3ScenarioCorpus {
    /// Fixture-bound entries.
    pub entries: Vec<M3ScenarioCorpusEntry>,
}

impl M3ScenarioCorpus {
    /// Returns the parsed scenario records without their fixture path wrappers.
    pub fn scenarios(&self) -> impl Iterator<Item = &M3SupportScenario> {
        self.entries.iter().map(|entry| &entry.scenario)
    }

    /// Validates the corpus against the beta-lane coverage contract.
    pub fn validate(&self) -> Vec<M3CorpusViolation> {
        let mut violations = Vec::new();

        if self.entries.is_empty() {
            push_violation(
                &mut violations,
                "corpus.empty",
                M3_SCENARIO_CORPUS_DIR,
                "corpus must contain at least one scenario",
            );
            return violations;
        }

        let mut scenario_ids = BTreeSet::new();
        let mut fixture_refs = BTreeSet::new();
        let mut scorecard_targets = BTreeSet::new();
        let mut lane_seen: BTreeSet<M3BetaLaneClass> = BTreeSet::new();

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
            if !scorecard_targets.insert(scenario.scorecard_contribution.scorecard_target.clone())
            {
                push_violation(
                    &mut violations,
                    "corpus.duplicate_scorecard_target",
                    &scenario.scorecard_contribution.scorecard_target,
                    "scorecard_target must be unique within the corpus",
                );
            }
            lane_seen.insert(scenario.beta_lane_class);
            validate_scenario(&mut violations, scenario);
        }

        for required in required_beta_lane_classes() {
            if !lane_seen.contains(required) {
                push_violation(
                    &mut violations,
                    "corpus.required_lane_missing",
                    required.as_str(),
                    format!(
                        "required beta lane {} has no seeded scenario",
                        required.as_str()
                    ),
                );
            }
        }

        violations
    }

    /// Projects the corpus into a metadata-safe drill-harness report.
    pub fn drill_harness_report(
        &self,
        report_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> M3DrillHarnessReport {
        let lane_rows = self
            .entries
            .iter()
            .map(M3DrillHarnessLaneRow::from_entry)
            .collect::<Vec<_>>();
        M3DrillHarnessReport {
            record_kind: M3_DRILL_HARNESS_REPORT_RECORD_KIND.to_owned(),
            schema_version: M3_SUPPORT_SCENARIO_SCHEMA_VERSION,
            report_id: report_id.into(),
            generated_at: generated_at.into(),
            corpus_manifest_ref: M3_SCENARIO_CORPUS_MANIFEST_REF.to_owned(),
            corpus_doc_ref: M3_SCENARIO_CORPUS_DOC_REF.to_owned(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            required_beta_lane_classes: required_beta_lane_classes()
                .iter()
                .map(|lane| lane.as_str().to_owned())
                .collect(),
            lane_rows,
        }
    }
}

/// Loads the checked-in M3 scenario corpus.
///
/// # Errors
///
/// Returns a YAML parse error when a fixture does not match
/// [`M3SupportScenario`].
pub fn current_m3_scenario_corpus() -> Result<M3ScenarioCorpus, serde_yaml::Error> {
    let entries = SCENARIO_FIXTURES
        .iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str::<M3SupportScenario>(yaml).map(|scenario| {
                M3ScenarioCorpusEntry {
                    fixture_ref: (*fixture_ref).to_owned(),
                    scenario,
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(M3ScenarioCorpus { entries })
}

/// One projected row in the drill-harness report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M3DrillHarnessLaneRow {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Closed beta-lane class.
    pub beta_lane_class: M3BetaLaneClass,
    /// Stable scenario id.
    pub scenario_id: String,
    /// Reviewer-facing scenario title.
    pub title: String,
    /// Corpus fixture ref.
    pub fixture_ref: String,
    /// Closed drill class.
    pub drill_class: M3DrillClass,
    /// Owning lane.
    pub drill_owner_lane: String,
    /// Tokens drawn from the scenario's drill steps.
    pub drill_step_classes: Vec<String>,
    /// Artifact kinds the scenario re-proves.
    pub expected_artifact_kinds: Vec<String>,
    /// Recovery action class the first actionable artifact routes to.
    pub recovery_action_class: String,
    /// Scorecard target the row contributes to.
    pub scorecard_target: String,
    /// Closed coverage class.
    pub coverage_class: String,
    /// Expected scorecard state when the drill is healthy.
    pub expected_state: String,
    /// Whether the row contributes to a release gate.
    pub contributes_to_release_gate: bool,
    /// Declared claim-downgrade triggers covered by the scenario.
    pub claim_downgrade_trigger_classes: Vec<String>,
    /// Closed claim-downgrade classes the scenario can apply.
    pub claim_downgrade_classes: Vec<String>,
    /// Primary beta-lane doc ref.
    pub beta_lane_doc_ref: String,
    /// Primary beta-lane schema ref.
    pub beta_lane_schema_ref: String,
    /// Primary crate consumer ref.
    pub crate_consumer_ref: String,
    /// Protected integration-test ref.
    pub integration_test_ref: String,
    /// Whether the row passes the metadata-safe baseline.
    pub metadata_safe_baseline_met: bool,
}

impl M3DrillHarnessLaneRow {
    fn from_entry(entry: &M3ScenarioCorpusEntry) -> Self {
        let scenario = &entry.scenario;
        let drill_step_classes = scenario
            .drill_steps
            .iter()
            .map(|step| step.step_class.as_str().to_owned())
            .collect::<Vec<_>>();
        let expected_artifact_kinds = {
            let mut kinds = scenario
                .drill_steps
                .iter()
                .map(|step| step.expected_artifact_kind.as_str().to_owned())
                .collect::<Vec<_>>();
            kinds.push(
                scenario
                    .expected_first_actionable_artifact
                    .artifact_kind
                    .as_str()
                    .to_owned(),
            );
            kinds.sort();
            kinds.dedup();
            kinds
        };
        let claim_downgrade_trigger_classes = {
            let mut tokens = scenario
                .claim_downgrade_rules
                .iter()
                .map(|rule| rule.trigger_class.as_str().to_owned())
                .collect::<Vec<_>>();
            tokens.sort();
            tokens.dedup();
            tokens
        };
        let claim_downgrade_classes = {
            let mut tokens = scenario
                .claim_downgrade_rules
                .iter()
                .map(|rule| rule.downgrade_class.as_str().to_owned())
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
            record_kind: M3_DRILL_HARNESS_LANE_ROW_RECORD_KIND.to_owned(),
            beta_lane_class: scenario.beta_lane_class,
            scenario_id: scenario.scenario_id.clone(),
            title: scenario.title.clone(),
            fixture_ref: entry.fixture_ref.clone(),
            drill_class: scenario.drill_class,
            drill_owner_lane: scenario.drill_owner_lane.clone(),
            drill_step_classes,
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
            claim_downgrade_trigger_classes,
            claim_downgrade_classes,
            beta_lane_doc_ref: scenario.beta_lane_refs.doc_ref.clone(),
            beta_lane_schema_ref: scenario.beta_lane_refs.schema_ref.clone(),
            crate_consumer_ref: scenario.beta_lane_refs.crate_consumer.clone(),
            integration_test_ref: scenario.beta_lane_refs.integration_test.clone(),
            metadata_safe_baseline_met,
        }
    }
}

/// Drill-harness report projected from the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M3DrillHarnessReport {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Projection schema version.
    pub schema_version: u32,
    /// Stable report id.
    pub report_id: String,
    /// UTC generation timestamp.
    pub generated_at: String,
    /// Corpus manifest ref.
    pub corpus_manifest_ref: String,
    /// Reviewer doc ref.
    pub corpus_doc_ref: String,
    /// Whether raw private material is excluded by default.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded by default.
    pub ambient_authority_excluded: bool,
    /// Required beta-lane class tokens covered by the report.
    pub required_beta_lane_classes: Vec<String>,
    /// Per-lane rows.
    pub lane_rows: Vec<M3DrillHarnessLaneRow>,
}

impl M3DrillHarnessReport {
    /// Returns true when every required lane has a row and every row
    /// meets the metadata-safe baseline.
    pub fn is_export_safe(&self) -> bool {
        if !self.raw_private_material_excluded || !self.ambient_authority_excluded {
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
            .all(|row| row.metadata_safe_baseline_met)
    }
}

/// Strongly typed error returned by [`load_m3_support_scenario`].
#[derive(Debug)]
pub enum M3ScenarioLoadError {
    /// YAML parse error.
    Yaml(serde_yaml::Error),
}

impl fmt::Display for M3ScenarioLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yaml(err) => write!(f, "scenario yaml parse error: {err}"),
        }
    }
}

impl Error for M3ScenarioLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Yaml(err) => Some(err),
        }
    }
}

impl From<serde_yaml::Error> for M3ScenarioLoadError {
    fn from(value: serde_yaml::Error) -> Self {
        Self::Yaml(value)
    }
}

/// Parses one M3 support scenario YAML record.
///
/// # Errors
///
/// Returns [`M3ScenarioLoadError::Yaml`] when the YAML does not match
/// [`M3SupportScenario`].
pub fn load_m3_support_scenario(yaml: &str) -> Result<M3SupportScenario, M3ScenarioLoadError> {
    serde_yaml::from_str::<M3SupportScenario>(yaml).map_err(M3ScenarioLoadError::from)
}

fn validate_scenario(violations: &mut Vec<M3CorpusViolation>, scenario: &M3SupportScenario) {
    let target = scenario.scenario_id.as_str();

    if scenario.schema_version != M3_SUPPORT_SCENARIO_SCHEMA_VERSION {
        push_violation(
            violations,
            "scenario.schema_version",
            target,
            "schema_version must be 1",
        );
    }
    if scenario.record_kind != M3_SUPPORT_SCENARIO_RECORD_KIND {
        push_violation(
            violations,
            "scenario.record_kind",
            target,
            "record_kind must be m3_support_scenario_record",
        );
    }
    if scenario.scenario_id.trim().is_empty() {
        push_violation(
            violations,
            "scenario.scenario_id",
            target,
            "scenario_id must be non-empty",
        );
    }
    if scenario.title.trim().is_empty() {
        push_violation(
            violations,
            "scenario.title",
            target,
            "title must be non-empty",
        );
    }
    if scenario.drill_owner_lane.trim().is_empty() {
        push_violation(
            violations,
            "scenario.drill_owner_lane",
            target,
            "drill_owner_lane must be non-empty",
        );
    }
    if scenario.starting_condition.state_ref.trim().is_empty()
        || scenario.starting_condition.summary.trim().is_empty()
    {
        push_violation(
            violations,
            "scenario.starting_condition",
            target,
            "starting_condition must declare a state_ref and summary",
        );
    }

    validate_beta_lane_refs(violations, target, &scenario.beta_lane_refs);
    validate_drill_steps(violations, target, &scenario.drill_steps);
    validate_expected_first_actionable_artifact(
        violations,
        target,
        &scenario.expected_first_actionable_artifact,
    );
    validate_primary_fixture_refs(violations, target, &scenario.primary_fixture_refs);
    validate_scorecard_contribution(violations, target, &scenario.scorecard_contribution);
    validate_claim_downgrade_rules(violations, target, &scenario.claim_downgrade_rules);
    validate_safety(violations, target, &scenario.safety);
    validate_references(violations, target, &scenario.references, scenario.beta_lane_class);
    validate_drill_class_matches_lane(violations, target, scenario.beta_lane_class, scenario.drill_class);
}

fn validate_beta_lane_refs(
    violations: &mut Vec<M3CorpusViolation>,
    target: &str,
    refs: &BetaLaneRefs,
) {
    for (field, value) in [
        ("doc_ref", refs.doc_ref.as_str()),
        ("schema_ref", refs.schema_ref.as_str()),
        ("crate_consumer", refs.crate_consumer.as_str()),
        ("integration_test", refs.integration_test.as_str()),
    ] {
        if value.trim().is_empty() {
            push_violation(
                violations,
                "scenario.beta_lane_refs.empty",
                target,
                format!("beta_lane_refs.{field} must be non-empty"),
            );
        }
    }
    if !refs.crate_consumer.starts_with("crates/") {
        push_violation(
            violations,
            "scenario.beta_lane_refs.crate_consumer",
            target,
            "beta_lane_refs.crate_consumer must be a repo-relative crates/* path",
        );
    }
    if !refs.integration_test.starts_with("crates/") {
        push_violation(
            violations,
            "scenario.beta_lane_refs.integration_test",
            target,
            "beta_lane_refs.integration_test must be a repo-relative crates/* path",
        );
    }
    if !refs.doc_ref.starts_with("docs/") {
        push_violation(
            violations,
            "scenario.beta_lane_refs.doc_ref",
            target,
            "beta_lane_refs.doc_ref must be a repo-relative docs/* path",
        );
    }
    if !refs.schema_ref.starts_with("schemas/") {
        push_violation(
            violations,
            "scenario.beta_lane_refs.schema_ref",
            target,
            "beta_lane_refs.schema_ref must be a repo-relative schemas/* path",
        );
    }
}

fn validate_drill_steps(
    violations: &mut Vec<M3CorpusViolation>,
    target: &str,
    steps: &[M3DrillStep],
) {
    if steps.is_empty() {
        push_violation(
            violations,
            "scenario.drill_steps.empty",
            target,
            "drill_steps must declare at least one step",
        );
        return;
    }
    let mut seen = BTreeSet::new();
    for step in steps {
        if !seen.insert(step.step_id.as_str()) {
            push_violation(
                violations,
                "scenario.drill_steps.duplicate_step_id",
                target,
                format!("duplicate drill_step.step_id {}", step.step_id),
            );
        }
        if step.step_id.trim().is_empty() {
            push_violation(
                violations,
                "scenario.drill_steps.empty_step_id",
                target,
                "drill_step.step_id must be non-empty",
            );
        }
        if step.summary.trim().is_empty() {
            push_violation(
                violations,
                "scenario.drill_steps.empty_summary",
                target,
                format!("drill_step {} summary must be non-empty", step.step_id),
            );
        }
        if step.expected_artifact_ref.trim().is_empty() {
            push_violation(
                violations,
                "scenario.drill_steps.empty_expected_artifact_ref",
                target,
                format!(
                    "drill_step {} expected_artifact_ref must be non-empty (drill_step_unproven)",
                    step.step_id
                ),
            );
        }
    }
    if !steps
        .iter()
        .any(|step| step.step_class == M3DrillStepClass::ExportSupportPacket)
    {
        push_violation(
            violations,
            "scenario.drill_steps.no_export_support_packet",
            target,
            "drill must include at least one export_support_packet step",
        );
    }
}

fn validate_expected_first_actionable_artifact(
    violations: &mut Vec<M3CorpusViolation>,
    target: &str,
    artifact: &M3ExpectedFirstActionableArtifact,
) {
    if artifact.artifact_ref.trim().is_empty()
        || artifact.recovery_action_class.trim().is_empty()
        || artifact.reviewer_summary.trim().is_empty()
    {
        push_violation(
            violations,
            "scenario.expected_first_actionable_artifact.empty",
            target,
            "expected_first_actionable_artifact must declare artifact_ref, recovery_action_class, and reviewer_summary",
        );
    }
}

fn validate_primary_fixture_refs(
    violations: &mut Vec<M3CorpusViolation>,
    target: &str,
    refs: &[String],
) {
    if refs.is_empty() {
        push_violation(
            violations,
            "scenario.primary_fixture_refs.empty",
            target,
            "primary_fixture_refs must name at least one fixture",
        );
    }
    let mut seen = BTreeSet::new();
    for fixture in refs {
        if !seen.insert(fixture.as_str()) {
            push_violation(
                violations,
                "scenario.primary_fixture_refs.duplicate",
                target,
                format!("duplicate primary_fixture_ref {fixture}"),
            );
        }
        let is_repo_relative_fixture = fixture.starts_with("fixtures/")
            || fixture.starts_with("artifacts/")
            || fixture.starts_with("docs/");
        if !is_repo_relative_fixture {
            push_violation(
                violations,
                "scenario.primary_fixture_refs.not_repo_relative",
                target,
                format!(
                    "primary_fixture_ref {fixture} must be a repo-relative fixtures/* or artifacts/* path"
                ),
            );
        }
    }
}

fn validate_scorecard_contribution(
    violations: &mut Vec<M3CorpusViolation>,
    target: &str,
    contribution: &M3ScenarioScorecardContribution,
) {
    if contribution.scorecard_target.trim().is_empty()
        || contribution.coverage_class.trim().is_empty()
        || contribution.expected_state.trim().is_empty()
    {
        push_violation(
            violations,
            "scenario.scorecard_contribution.empty",
            target,
            "scorecard_contribution must declare scorecard_target, coverage_class, and expected_state",
        );
    }
    if !contribution
        .scorecard_target
        .starts_with("m3.beta_lane.")
    {
        push_violation(
            violations,
            "scenario.scorecard_contribution.scorecard_target",
            target,
            "scorecard_target must start with m3.beta_lane.",
        );
    }
}

fn validate_claim_downgrade_rules(
    violations: &mut Vec<M3CorpusViolation>,
    target: &str,
    rules: &[M3ClaimDowngradeRule],
) {
    if rules.is_empty() {
        push_violation(
            violations,
            "scenario.claim_downgrade_rules.empty",
            target,
            "claim_downgrade_rules must declare at least one rule",
        );
        return;
    }
    let mut seen = BTreeSet::new();
    for rule in rules {
        if !seen.insert(rule.trigger_class) {
            push_violation(
                violations,
                "scenario.claim_downgrade_rules.duplicate_trigger",
                target,
                format!(
                    "duplicate claim_downgrade_rule.trigger_class {}",
                    rule.trigger_class.as_str()
                ),
            );
        }
        if rule.reviewer_note.trim().is_empty() {
            push_violation(
                violations,
                "scenario.claim_downgrade_rules.empty_reviewer_note",
                target,
                format!(
                    "claim_downgrade_rule for trigger {} must declare a reviewer_note",
                    rule.trigger_class.as_str()
                ),
            );
        }
    }
    for required in REQUIRED_DOWNGRADE_TRIGGERS {
        if !seen.contains(required) {
            push_violation(
                violations,
                "scenario.claim_downgrade_rules.required_trigger_missing",
                target,
                format!(
                    "claim_downgrade_rules must cover required trigger {}",
                    required.as_str()
                ),
            );
        }
    }
}

fn validate_safety(
    violations: &mut Vec<M3CorpusViolation>,
    target: &str,
    safety: &M3ScenarioSafety,
) {
    if !safety.read_only_diagnosis {
        push_violation(
            violations,
            "scenario.safety.read_only_diagnosis",
            target,
            "diagnosis must be read-only",
        );
    }
    if !safety.raw_private_material_excluded {
        push_violation(
            violations,
            "scenario.safety.raw_private_material_excluded",
            target,
            "raw private material must be excluded by default",
        );
    }
    if safety.destructive_resets_present {
        push_violation(
            violations,
            "scenario.safety.destructive_resets_present",
            target,
            "destructive_resets_present must be false",
        );
    }
    if !safety.preserves_user_authored_files {
        push_violation(
            violations,
            "scenario.safety.preserves_user_authored_files",
            target,
            "preserves_user_authored_files must be true",
        );
    }
    for token in REQUIRED_FORBIDDEN_FIX_CLASSES {
        if !safety
            .forbidden_fix_classes
            .iter()
            .any(|actual| actual == *token)
        {
            push_violation(
                violations,
                "scenario.safety.forbidden_fix_classes",
                target,
                format!("forbidden_fix_classes must include {token}"),
            );
        }
    }
    if !safety
        .no_touch_boundary_set
        .iter()
        .any(|boundary| boundary == REQUIRED_NO_TOUCH_BOUNDARY)
    {
        push_violation(
            violations,
            "scenario.safety.no_touch_user_files",
            target,
            "no_touch_boundary_set must include user_authored_files",
        );
    }
}

fn validate_references(
    violations: &mut Vec<M3CorpusViolation>,
    target: &str,
    references: &M3ScenarioReferences,
    lane: M3BetaLaneClass,
) {
    if references.beta_lane_doc_ref.trim().is_empty()
        || references.recovery_ladder_alpha_ref.trim().is_empty()
        || references.diagnosis_latency_scorecard_ref.trim().is_empty()
    {
        push_violation(
            violations,
            "scenario.references.empty",
            target,
            "references must declare beta_lane_doc_ref, recovery_ladder_alpha_ref, and diagnosis_latency_scorecard_ref",
        );
    }
    if references.recovery_ladder_alpha_ref != "docs/support/recovery_ladder_alpha.md" {
        push_violation(
            violations,
            "scenario.references.recovery_ladder_alpha_ref",
            target,
            "recovery_ladder_alpha_ref must pin docs/support/recovery_ladder_alpha.md",
        );
    }
    if references.diagnosis_latency_scorecard_ref
        != "artifacts/support/diagnosis_latency_scorecard_alpha.yaml"
    {
        push_violation(
            violations,
            "scenario.references.diagnosis_latency_scorecard_ref",
            target,
            "diagnosis_latency_scorecard_ref must pin artifacts/support/diagnosis_latency_scorecard_alpha.yaml",
        );
    }
    let expected_doc_ref = match lane {
        M3BetaLaneClass::SafeMode => "docs/support/m3/safe_mode_beta.md",
        M3BetaLaneClass::ExtensionBisect => "docs/support/m3/extension_bisect_beta.md",
        M3BetaLaneClass::RepairTransactionPreview => "docs/support/m3/repair_transaction_beta.md",
        M3BetaLaneClass::DoctorProbePacks => "docs/support/m3/doctor_probe_packs_beta.md",
        M3BetaLaneClass::ProjectDoctorFindingContract => "docs/support/m3/project_doctor_beta.md",
        M3BetaLaneClass::RecordsGovernance => "docs/support/m3/records_governance_beta.md",
        M3BetaLaneClass::RuntimeReplayPackets => "docs/support/m3/runtime_replay_packets.md",
    };
    if references.beta_lane_doc_ref != expected_doc_ref {
        push_violation(
            violations,
            "scenario.references.beta_lane_doc_ref",
            target,
            format!(
                "beta_lane_doc_ref must pin {expected_doc_ref} for lane {}",
                lane.as_str()
            ),
        );
    }
}

fn validate_drill_class_matches_lane(
    violations: &mut Vec<M3CorpusViolation>,
    target: &str,
    lane: M3BetaLaneClass,
    drill_class: M3DrillClass,
) {
    let allowed: &[M3DrillClass] = match lane {
        M3BetaLaneClass::SafeMode | M3BetaLaneClass::ExtensionBisect => {
            &[M3DrillClass::FailureRecoveryDrill]
        }
        M3BetaLaneClass::RepairTransactionPreview => &[M3DrillClass::RepairPreviewDrill],
        M3BetaLaneClass::DoctorProbePacks | M3BetaLaneClass::ProjectDoctorFindingContract => {
            &[M3DrillClass::DiagnosisRoutingDrill]
        }
        M3BetaLaneClass::RecordsGovernance => &[M3DrillClass::GovernanceChainDrill],
        M3BetaLaneClass::RuntimeReplayPackets => &[M3DrillClass::ReplayDecisionDrill],
    };
    if !allowed.contains(&drill_class) {
        push_violation(
            violations,
            "scenario.drill_class.mismatch",
            target,
            format!(
                "drill_class {} is not admitted for beta lane {}",
                drill_class.as_str(),
                lane.as_str()
            ),
        );
    }
}

fn push_violation(
    violations: &mut Vec<M3CorpusViolation>,
    check_id: impl Into<String>,
    target_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(M3CorpusViolation {
        check_id: check_id.into(),
        target_ref: target_ref.into(),
        message: message.into(),
    });
}
