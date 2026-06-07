use super::*;

const CANDIDATE_ID: &str = "ai-test-candidate:retry-regression:0001";

fn lineage(
    object_id: &str,
    validation_status: AiTestGenerationValidationStatus,
) -> AiTestGenerationLineage {
    AiTestGenerationLineage {
        object_id: object_id.to_owned(),
        evidence_refs: vec![format!("evidence:{object_id}")],
        export_lineage_refs: vec![format!("export:{object_id}")],
        validation_status,
    }
}

fn brief() -> TestGenerationBrief {
    TestGenerationBrief {
        candidate_id: CANDIDATE_ID.to_owned(),
        lineage: lineage(
            "brief:retry-regression:0001",
            AiTestGenerationValidationStatus::Inspectable,
        ),
        trigger_class: TestProposalTriggerClass::BugReport,
        trigger_ref: "bug:retry-regression:0001".to_owned(),
        target_refs: vec![
            "symbol:retry-policy:should-retry".to_owned(),
            "file-ref:retry-tests".to_owned(),
        ],
        requested_test_type: "unit regression assertions".to_owned(),
        framework_target: "rust cargo test".to_owned(),
        confidence_class: TestCandidateConfidenceClass::Low,
        flaky_risk_class: TestCandidateFlakyRiskClass::PotentiallyFlaky,
        risk_labels_retained_after_pass: true,
    }
}

fn assumptions() -> AssumptionReviewSheet {
    AssumptionReviewSheet {
        candidate_id: CANDIDATE_ID.to_owned(),
        lineage: lineage(
            "assumption-sheet:retry-regression:0001",
            AiTestGenerationValidationStatus::Inspectable,
        ),
        rows: vec![
            AssumptionReviewRow {
                assumption_id: "assumption:retry-clock".to_owned(),
                assumption_class: AssumptionClass::ClockOrRandomness,
                source_ref: "helper:fake-clock".to_owned(),
                risk_class: AssumptionRiskClass::Medium,
                inspectable_before_apply: true,
                unsupported_note: None,
            },
            AssumptionReviewRow {
                assumption_id: "assumption:retry-fixture".to_owned(),
                assumption_class: AssumptionClass::FixtureCreation,
                source_ref: "fixture:retry-backoff".to_owned(),
                risk_class: AssumptionRiskClass::Low,
                inspectable_before_apply: true,
                unsupported_note: None,
            },
            AssumptionReviewRow {
                assumption_id: "assumption:real-network-not-exercised".to_owned(),
                assumption_class: AssumptionClass::UnsupportedPath,
                source_ref: "scope-limit:network-denied".to_owned(),
                risk_class: AssumptionRiskClass::High,
                inspectable_before_apply: true,
                unsupported_note: Some("real network retries are not exercised".to_owned()),
            },
        ],
    }
}

fn generated_diff() -> GeneratedTestDiffRecord {
    GeneratedTestDiffRecord {
        candidate_id: CANDIDATE_ID.to_owned(),
        lineage: lineage(
            "diff:retry-regression:0001",
            AiTestGenerationValidationStatus::Inspectable,
        ),
        patch_digest_ref: "sha256:1111111111111111111111111111111111111111111111111111111111111111"
            .to_owned(),
        write_scope_label: "retry test file, retry fixture, and one golden baseline".to_owned(),
        classes: vec![
            GeneratedTestDiffClassRow {
                diff_class_id: "diff-class:logic-assertion".to_owned(),
                diff_class: GeneratedTestDiffClass::LogicAssertion,
                change_refs: vec!["patch-hunk:retry-assertion".to_owned()],
                file_count: 1,
                separated_from_other_classes: true,
                snapshot_or_golden_baseline_ref: None,
            },
            GeneratedTestDiffClassRow {
                diff_class_id: "diff-class:helper-fixture".to_owned(),
                diff_class: GeneratedTestDiffClass::HelperOrFixtureAddition,
                change_refs: vec!["patch-hunk:retry-fixture".to_owned()],
                file_count: 1,
                separated_from_other_classes: true,
                snapshot_or_golden_baseline_ref: None,
            },
            GeneratedTestDiffClassRow {
                diff_class_id: "diff-class:snapshot-golden".to_owned(),
                diff_class: GeneratedTestDiffClass::SnapshotOrGoldenUpdate,
                change_refs: vec!["patch-hunk:retry-golden".to_owned()],
                file_count: 1,
                separated_from_other_classes: true,
                snapshot_or_golden_baseline_ref: Some(
                    "golden-baseline:retry-state:0001".to_owned(),
                ),
            },
        ],
    }
}

fn sandbox() -> SandboxValidationRecord {
    SandboxValidationRecord {
        candidate_id: CANDIDATE_ID.to_owned(),
        lineage: lineage(
            "sandbox-validation:retry-regression:0001",
            AiTestGenerationValidationStatus::Inspectable,
        ),
        target_class: SandboxTargetClass::LocalSandbox,
        target_environment_refs: vec![
            "target:local-cargo-sandbox".to_owned(),
            "env-lineage:toolchain-rust-stable".to_owned(),
        ],
        network_policy_ref: "policy:network-denied".to_owned(),
        file_policy_ref: "policy:temp-fs-allowed".to_owned(),
        secret_policy_ref: "policy:secrets-denied".to_owned(),
        outcome_class: SandboxOutcomeClass::Passed,
        blocked_by_trust_reason: None,
        executed_run_ref: "test-run:retry-regression-sandbox:0001".to_owned(),
        logs_ref: "logs:test-run:retry-regression-sandbox:0001".to_owned(),
        rerun_available: true,
        open_logs_available: true,
    }
}

fn coverage() -> CoverageImpactNote {
    CoverageImpactNote {
        candidate_id: CANDIDATE_ID.to_owned(),
        lineage: lineage(
            "coverage-impact:retry-regression:0001",
            AiTestGenerationValidationStatus::Inspectable,
        ),
        target_family: "rust-unit-tests".to_owned(),
        impact_class: CoverageImpactClass::Estimated,
        expected_covered_area_refs: vec!["branch:retry-policy:retry-after-timeout".to_owned()],
        measured_coverage_ref: None,
        delta_summary: Some("estimated to cover the timeout retry branch".to_owned()),
        counts_for_release_or_benchmark_truth: false,
    }
}

fn candidate() -> AiTestCandidateRow {
    AiTestCandidateRow {
        candidate_id: CANDIDATE_ID.to_owned(),
        review_state: TestCandidateReviewState::ReviewReadyDraft,
        gate_record_ref: "ai-test-generation-gate:retry-regression:0001".to_owned(),
        brief_id: "brief:retry-regression:0001".to_owned(),
        assumption_sheet_id: "assumption-sheet:retry-regression:0001".to_owned(),
        diff_record_id: "diff:retry-regression:0001".to_owned(),
        sandbox_record_id: "sandbox-validation:retry-regression:0001".to_owned(),
        coverage_note_id: "coverage-impact:retry-regression:0001".to_owned(),
        assumptions_inspectable: true,
        diff_classes_inspectable: true,
        sandbox_state_inspectable: true,
        bulk_apply_posture: BulkApplyPostureClass::UnavailableReviewRequired,
    }
}

fn consumer_projections() -> Vec<AiTestGenerationConsumerProjection> {
    AiTestGenerationConsumerSurface::required_surfaces()
        .into_iter()
        .map(|surface| AiTestGenerationConsumerProjection {
            surface,
            preserves_draft_class: true,
            distinguishes_ai_generated_tests: true,
            separates_diff_classes: true,
            preserves_sandbox_validation: true,
            preserves_coverage_impact_class: true,
            export_ref: format!("projection:ai-test-generation:{}", surface.as_str()),
        })
        .collect()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        AI_TEST_GENERATION_TRUTH_AI_DOC_REF.to_owned(),
        AI_TEST_GENERATION_TESTING_CONTRACT_REF.to_owned(),
        AI_TEST_GENERATION_GATE_SCHEMA_REF.to_owned(),
        AI_TEST_GENERATION_TRUTH_SCHEMA_REF.to_owned(),
    ]
}

fn packet() -> AiTestGenerationTruthPacket {
    AiTestGenerationTruthPacket::new(AiTestGenerationTruthPacketInput {
        packet_id: "ai-test-generation-truth:stable:0001".to_owned(),
        workflow_id: "workflow:ai-test-generation:stable".to_owned(),
        display_label: "AI test-generation assumption and sandbox truth".to_owned(),
        candidates: vec![candidate()],
        briefs: vec![brief()],
        assumption_sheets: vec![assumptions()],
        generated_diffs: vec![generated_diff()],
        sandbox_validations: vec![sandbox()],
        coverage_impact_notes: vec![coverage()],
        consumer_projections: consumer_projections(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T03:40:00Z".to_owned(),
    })
}

#[test]
fn ai_test_generation_truth_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn candidate_must_remain_draft_class() {
    let mut packet = packet();
    packet.candidates[0].review_state = TestCandidateReviewState::TrustedApplied;

    assert!(packet
        .validate()
        .contains(&AiTestGenerationTruthViolation::CandidateNotDraftClass));
}

#[test]
fn candidate_objects_must_belong_to_same_candidate() {
    let mut packet = packet();
    packet.briefs[0].candidate_id = "ai-test-candidate:other".to_owned();

    assert!(packet
        .validate()
        .contains(&AiTestGenerationTruthViolation::CandidateLinkageIncomplete));
}

#[test]
fn candidate_requires_current_client_inspection() {
    let mut packet = packet();
    packet.candidates[0].sandbox_state_inspectable = false;

    assert!(packet
        .validate()
        .contains(&AiTestGenerationTruthViolation::CandidateInspectionIncomplete));
}

#[test]
fn bulk_apply_is_rejected() {
    let mut packet = packet();
    packet.candidates[0].bulk_apply_posture = BulkApplyPostureClass::Available;

    assert!(packet
        .validate()
        .contains(&AiTestGenerationTruthViolation::BulkApplyAvailable));
}

#[test]
fn brief_requires_concrete_trigger() {
    let mut packet = packet();
    packet.briefs[0].trigger_ref.clear();

    assert!(packet
        .validate()
        .contains(&AiTestGenerationTruthViolation::MissingConcreteTrigger));
}

#[test]
fn low_confidence_or_flaky_labels_survive_pass() {
    let mut packet = packet();
    packet.briefs[0].risk_labels_retained_after_pass = false;

    assert!(packet
        .validate()
        .contains(&AiTestGenerationTruthViolation::RiskLabelsDroppedAfterPass));
}

#[test]
fn assumptions_must_be_inspectable() {
    let mut packet = packet();
    packet.assumption_sheets[0].rows[0].inspectable_before_apply = false;

    assert!(packet
        .validate()
        .contains(&AiTestGenerationTruthViolation::AssumptionReviewIncomplete));
}

#[test]
fn unsupported_assumption_requires_note() {
    let mut packet = packet();
    packet.assumption_sheets[0].rows[2].unsupported_note = None;

    assert!(packet
        .validate()
        .contains(&AiTestGenerationTruthViolation::AssumptionReviewIncomplete));
}

#[test]
fn diff_classes_must_remain_separated() {
    let mut packet = packet();
    packet.generated_diffs[0].classes[1].separated_from_other_classes = false;

    assert!(packet
        .validate()
        .contains(&AiTestGenerationTruthViolation::DiffClassNotSeparated));
}

#[test]
fn snapshot_diff_requires_baseline_ref() {
    let mut packet = packet();
    packet.generated_diffs[0].classes[2].snapshot_or_golden_baseline_ref = None;

    assert!(packet
        .validate()
        .contains(&AiTestGenerationTruthViolation::DiffClassIncomplete));
}

#[test]
fn sandbox_requires_target_and_policy_lineage() {
    let mut packet = packet();
    packet.sandbox_validations[0]
        .target_environment_refs
        .clear();

    assert!(packet
        .validate()
        .contains(&AiTestGenerationTruthViolation::SandboxValidationIncomplete));
}

#[test]
fn trust_blocked_sandbox_requires_reason() {
    let mut packet = packet();
    packet.sandbox_validations[0].outcome_class = SandboxOutcomeClass::BlockedByTrust;

    assert!(packet
        .validate()
        .contains(&AiTestGenerationTruthViolation::SandboxBlockedReasonMissing));
}

#[test]
fn measured_coverage_requires_measurement_ref() {
    let mut packet = packet();
    packet.coverage_impact_notes[0].impact_class = CoverageImpactClass::Measured;

    assert!(packet
        .validate()
        .contains(&AiTestGenerationTruthViolation::CoverageImpactIncomplete));
}

#[test]
fn candidate_coverage_cannot_promote_release_truth() {
    let mut packet = packet();
    packet.coverage_impact_notes[0].counts_for_release_or_benchmark_truth = true;

    assert!(packet
        .validate()
        .contains(&AiTestGenerationTruthViolation::CandidateCoveragePromoted));
}

#[test]
fn consumer_projection_must_preserve_coverage_impact_truth() {
    let mut packet = packet();
    packet.consumer_projections[0].preserves_coverage_impact_class = false;

    assert!(packet
        .validate()
        .contains(&AiTestGenerationTruthViolation::ConsumerProjectionIncomplete));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != AI_TEST_GENERATION_TRUTH_SCHEMA_REF);

    assert!(packet
        .validate()
        .contains(&AiTestGenerationTruthViolation::MissingSourceContracts));
}

#[test]
fn raw_boundary_material_is_rejected() {
    let mut packet = packet();
    packet.briefs[0].framework_target = "runner https://provider.example/run".to_owned();

    assert!(packet
        .validate()
        .contains(&AiTestGenerationTruthViolation::RawBoundaryMaterialInExport));
}

#[test]
#[ignore = "run manually to regenerate the checked artifact"]
fn emit_artifact() {
    let packet = packet();
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let dir = format!("{root}/artifacts/ai/m4/ai-test-generation-assumption-and-sandbox-truth");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        format!("{dir}/support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
    std::fs::write(
        format!("{dir}/summary.md"),
        packet.render_markdown_summary(),
    )
    .unwrap();
    let fixture_dir =
        format!("{root}/fixtures/ai/m4/ai-test-generation-assumption-and-sandbox-truth");
    std::fs::create_dir_all(&fixture_dir).unwrap();
    std::fs::write(
        format!("{fixture_dir}/truth_packet.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
}

#[test]
fn checked_artifact_validates() {
    let packet = current_stable_ai_test_generation_truth_export()
        .expect("checked ai test-generation truth export validates");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}
