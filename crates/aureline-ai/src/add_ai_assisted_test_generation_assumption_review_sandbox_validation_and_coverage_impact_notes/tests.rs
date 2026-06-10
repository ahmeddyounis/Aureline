use super::*;

const PACKET_ID: &str = "ai-test-generation:m5:0001";
const GENERATION_PASS_ID: &str = "ai-test-generation-pass:m5:0001";

fn anchor(id: &str, strategy: AnchorStrategy, state: AnchorState) -> DurableAnchor {
    let disturbed = state.is_disturbed();
    DurableAnchor {
        anchor_id: id.to_owned(),
        strategy,
        target_ref: "ref:target:retry-scheduler-fn".to_owned(),
        scope_ref: "ref:scope:retry-module".to_owned(),
        state,
        drift_detected: disturbed,
        rebind_disclosed: disturbed,
        durable: true,
    }
}

fn proposals() -> TestProposalsBlock {
    TestProposalsBlock {
        proposal_set_id: "test-proposal-set:m5:0001".to_owned(),
        uncited_proposals_count: 1,
        no_authority_beyond_evidence: true,
        produced_before_apply: true,
        never_auto_applied: true,
        proposal_rows: vec![
            TestProposalRow {
                proposal_id: "proposal:m5:0001:retry-regression".to_owned(),
                proposal_class: TestProposalClass::BugRegression,
                diff_risk: GeneratedDiffRiskClass::AdditiveTestOnly,
                review_state: ProposalReviewState::ReviewReadyDraft,
                anchor: anchor(
                    "anchor:m5:0001:retry-loop",
                    AnchorStrategy::SymbolPath,
                    AnchorState::Bound,
                ),
                sandbox_run_ref: "sandbox-run:m5:0001".to_owned(),
                cited_evidence_refs: vec![
                    "evidence:trace:retry-fanout".to_owned(),
                    "evidence:log:retry-storm".to_owned(),
                ],
                evidence_backed: true,
                requires_human_review: false,
                disclosed: true,
            },
            TestProposalRow {
                proposal_id: "proposal:m5:0001:uncovered-branch".to_owned(),
                proposal_class: TestProposalClass::UncoveredBranch,
                diff_risk: GeneratedDiffRiskClass::TouchesExistingTest,
                review_state: ProposalReviewState::DraftOnly,
                anchor: anchor(
                    "anchor:m5:0001:branch",
                    AnchorStrategy::StructuralNode,
                    AnchorState::Rebound,
                ),
                sandbox_run_ref: "sandbox-run:m5:0002".to_owned(),
                cited_evidence_refs: vec!["evidence:coverage:retry-module".to_owned()],
                evidence_backed: true,
                requires_human_review: false,
                disclosed: true,
            },
            TestProposalRow {
                proposal_id: "proposal:m5:0001:boundary-guess".to_owned(),
                proposal_class: TestProposalClass::BoundaryCondition,
                diff_risk: GeneratedDiffRiskClass::AdditiveTestOnly,
                review_state: ProposalReviewState::BlockedReviewOnly,
                anchor: anchor(
                    "anchor:m5:0001:boundary",
                    AnchorStrategy::LineRange,
                    AnchorState::Drifted,
                ),
                sandbox_run_ref: "sandbox-run:m5:0002".to_owned(),
                cited_evidence_refs: vec![],
                evidence_backed: false,
                requires_human_review: true,
                disclosed: true,
            },
        ],
    }
}

fn assumptions() -> AssumptionReviewBlock {
    AssumptionReviewBlock {
        assumption_sheet_id: "assumption-sheet:m5:0001".to_owned(),
        unvalidated_assumptions_count: 1,
        assumptions_surfaced: true,
        assumption_rows: vec![
            AssumptionRow {
                assumption_id: "assumption:m5:0001:input-shape".to_owned(),
                assumption_class: AssumptionClass::InputShape,
                confidence: AssumptionConfidenceClass::Grounded,
                validated: true,
                requires_human_confirmation: false,
                scope_ref: "ref:scope:retry-module".to_owned(),
                disclosed: true,
            },
            AssumptionRow {
                assumption_id: "assumption:m5:0001:dep-behavior".to_owned(),
                assumption_class: AssumptionClass::DependencyBehavior,
                confidence: AssumptionConfidenceClass::Speculative,
                validated: false,
                requires_human_confirmation: true,
                scope_ref: "ref:scope:retry-deps".to_owned(),
                disclosed: true,
            },
        ],
    }
}

fn sandbox() -> SandboxValidationBlock {
    SandboxValidationBlock {
        sandbox_session_id: "sandbox-session:m5:0001".to_owned(),
        runs_isolated: true,
        sandbox_is_not_release_truth: true,
        run_rows: vec![
            SandboxRunRow {
                run_id: "sandbox-run:m5:0001".to_owned(),
                profile: SandboxProfileClass::EphemeralContainer,
                outcome: SandboxOutcomeClass::Passed,
                isolated: true,
                leaked_outside_sandbox: false,
                validated_proposal_ids: vec!["proposal:m5:0001:retry-regression".to_owned()],
                disclosed: true,
            },
            SandboxRunRow {
                run_id: "sandbox-run:m5:0002".to_owned(),
                profile: SandboxProfileClass::NetworkDenied,
                outcome: SandboxOutcomeClass::Failed,
                isolated: true,
                leaked_outside_sandbox: false,
                validated_proposal_ids: vec![
                    "proposal:m5:0001:uncovered-branch".to_owned(),
                    "proposal:m5:0001:boundary-guess".to_owned(),
                ],
                disclosed: true,
            },
        ],
    }
}

fn coverage_impact() -> CoverageImpactBlock {
    CoverageImpactBlock {
        impact_set_id: "coverage-impact-set:m5:0001".to_owned(),
        estimated_notes_count: 1,
        no_estimate_as_measured: true,
        impact_rows: vec![
            CoverageImpactRow {
                note_id: "coverage-note:m5:0001:retry-module".to_owned(),
                target_ref: "ref:scope:retry-module".to_owned(),
                measurement_basis: CoverageMeasurementBasis::Measured,
                delta_direction: CoverageDeltaDirection::Increase,
                estimated_labeled: false,
                disclosed: true,
            },
            CoverageImpactRow {
                note_id: "coverage-note:m5:0001:edge-paths".to_owned(),
                target_ref: "ref:scope:retry-edges".to_owned(),
                measurement_basis: CoverageMeasurementBasis::Estimated,
                delta_direction: CoverageDeltaDirection::Increase,
                estimated_labeled: true,
                disclosed: true,
            },
        ],
    }
}

fn consumer_surface_parity() -> Vec<TestGenSurfaceParityRow> {
    TestGenConsumerSurface::ALL
        .into_iter()
        .map(|surface| TestGenSurfaceParityRow {
            surface,
            shows_proposals: true,
            shows_assumptions: true,
            shows_sandbox: true,
            shows_coverage: true,
            reachable: true,
            qualification: TestGenSurfaceQualificationClass::Stable,
            claimed_stable: true,
        })
        .collect()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        GENERATED_TEST_REVIEW_DOC_REF.to_owned(),
        GENERATED_TEST_REVIEW_SCHEMA_REF.to_owned(),
        GENERATED_TEST_REVIEW_CONTEXT_ASSEMBLY_CONTRACT_REF.to_owned(),
        GENERATED_TEST_REVIEW_TEST_GENERATION_CONTRACT_REF.to_owned(),
        GENERATED_TEST_REVIEW_TESTING_CONTRACT_REF.to_owned(),
        GENERATED_TEST_REVIEW_SANDBOX_CONTRACT_REF.to_owned(),
        GENERATED_TEST_REVIEW_M5_MATRIX_CONTRACT_REF.to_owned(),
    ]
}

fn packet_input() -> GeneratedTestReviewPacketInput {
    GeneratedTestReviewPacketInput {
        packet_id: PACKET_ID.to_owned(),
        generation_pass_id: GENERATION_PASS_ID.to_owned(),
        display_label: "M5 AI test-generation pass for retry-storm change".to_owned(),
        trust_state_token: "restricted".to_owned(),
        policy_epoch_ref: "policy-epoch:m5:2026-06-01".to_owned(),
        proposals: proposals(),
        assumptions: assumptions(),
        sandbox: sandbox(),
        coverage_impact: coverage_impact(),
        consumer_surface_parity: consumer_surface_parity(),
        downgrade_triggers: TestGenDowngradeTrigger::ALL.to_vec(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-10T11:09:00Z".to_owned(),
    }
}

#[test]
fn packet_constructs_and_serializes() {
    let packet = GeneratedTestReviewPacket::new(packet_input());
    let json = packet.export_safe_json();
    assert!(json.contains("ai_test_generation_sandbox_coverage_implementation"));
    assert!(json.contains(PACKET_ID));
}

#[test]
fn valid_packet_passes_validation() {
    let packet = GeneratedTestReviewPacket::new(packet_input());
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = GeneratedTestReviewPacket::new(packet_input());
    packet.record_kind = "wrong_kind".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::WrongRecordKind));
}

#[test]
fn wrong_schema_version_fails() {
    let mut packet = GeneratedTestReviewPacket::new(packet_input());
    packet.schema_version = 999;
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::WrongSchemaVersion));
}

#[test]
fn missing_identity_fails() {
    let mut input = packet_input();
    input.generation_pass_id = "   ".to_owned();
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::MissingIdentity));
}

#[test]
fn missing_source_contracts_fails() {
    let mut input = packet_input();
    input.source_contract_refs = vec![];
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::MissingSourceContracts));
}

#[test]
fn proposal_set_empty_fails() {
    let mut input = packet_input();
    input.proposals.proposal_rows = vec![];
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::ProposalSetEmpty));
}

#[test]
fn hidden_proposal_fails() {
    let mut input = packet_input();
    input.proposals.proposal_rows[0].disclosed = false;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::HiddenProposal));
}

#[test]
fn evidence_backed_flag_mismatch_fails() {
    let mut input = packet_input();
    input.proposals.proposal_rows[0].evidence_backed = false;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::EvidenceBackedFlagMismatch));
}

#[test]
fn uncited_proposal_not_flagged_fails() {
    let mut input = packet_input();
    input.proposals.proposal_rows[2].requires_human_review = false;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::UncitedProposalNotFlagged));
}

#[test]
fn uncited_count_mismatch_fails() {
    let mut input = packet_input();
    input.proposals.uncited_proposals_count = 0;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::UncitedCountMismatch));
}

#[test]
fn authority_beyond_evidence_fails() {
    let mut input = packet_input();
    input.proposals.no_authority_beyond_evidence = false;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::AuthorityBeyondEvidence));
}

#[test]
fn proposals_not_produced_before_apply_fails() {
    let mut input = packet_input();
    input.proposals.produced_before_apply = false;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::ProposalsNotProducedBeforeApply));
}

#[test]
fn generated_test_auto_applied_fails() {
    let mut input = packet_input();
    input.proposals.proposal_rows[0].review_state = ProposalReviewState::AutoApplied;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::GeneratedTestAutoApplied));
}

#[test]
fn auto_apply_allowed_fails() {
    let mut input = packet_input();
    input.proposals.never_auto_applied = false;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::AutoApplyAllowed));
}

#[test]
fn proposal_missing_sandbox_ref_fails() {
    let mut input = packet_input();
    input.proposals.proposal_rows[0].sandbox_run_ref = String::new();
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::ProposalMissingSandboxRef));
}

#[test]
fn dangling_sandbox_ref_fails() {
    let mut input = packet_input();
    input.proposals.proposal_rows[0].sandbox_run_ref = "sandbox-run:does-not-exist".to_owned();
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::DanglingSandboxRef));
}

#[test]
fn anchor_incomplete_fails() {
    let mut input = packet_input();
    input.proposals.proposal_rows[0].anchor.target_ref = String::new();
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::AnchorIncomplete));
}

#[test]
fn anchor_not_durable_fails() {
    let mut input = packet_input();
    input.proposals.proposal_rows[0].anchor.durable = false;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::AnchorNotDurable));
}

#[test]
fn anchor_drift_undisclosed_fails() {
    let mut input = packet_input();
    // The drifted boundary proposal leaves its rebind disposition undisclosed.
    input.proposals.proposal_rows[2].anchor.rebind_disclosed = false;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::AnchorDriftUndisclosed));
}

#[test]
fn assumptions_not_surfaced_fails() {
    let mut input = packet_input();
    input.assumptions.assumptions_surfaced = false;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::AssumptionsNotSurfaced));
}

#[test]
fn assumption_incomplete_fails() {
    let mut input = packet_input();
    input.assumptions.assumption_rows[0].scope_ref = String::new();
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::AssumptionIncomplete));
}

#[test]
fn hidden_assumption_fails() {
    let mut input = packet_input();
    input.assumptions.assumption_rows[0].disclosed = false;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::HiddenAssumption));
}

#[test]
fn unvalidated_assumption_not_flagged_fails() {
    let mut input = packet_input();
    input.assumptions.assumption_rows[1].requires_human_confirmation = false;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::UnvalidatedAssumptionNotFlagged));
}

#[test]
fn unvalidated_count_mismatch_fails() {
    let mut input = packet_input();
    input.assumptions.unvalidated_assumptions_count = 0;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::UnvalidatedCountMismatch));
}

#[test]
fn sandbox_session_empty_fails() {
    let mut input = packet_input();
    input.sandbox.run_rows = vec![];
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::SandboxSessionEmpty));
}

#[test]
fn sandbox_not_isolated_fails() {
    let mut input = packet_input();
    input.sandbox.runs_isolated = false;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::SandboxNotIsolated));
}

#[test]
fn sandbox_treated_as_release_truth_fails() {
    let mut input = packet_input();
    input.sandbox.sandbox_is_not_release_truth = false;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::SandboxTreatedAsReleaseTruth));
}

#[test]
fn sandbox_run_not_isolated_fails() {
    let mut input = packet_input();
    input.sandbox.run_rows[0].isolated = false;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::SandboxRunNotIsolated));
}

#[test]
fn sandbox_leak_fails() {
    let mut input = packet_input();
    input.sandbox.run_rows[0].leaked_outside_sandbox = true;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::SandboxLeak));
}

#[test]
fn hidden_sandbox_run_fails() {
    let mut input = packet_input();
    input.sandbox.run_rows[0].disclosed = false;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::HiddenSandboxRun));
}

#[test]
fn dangling_validated_proposal_fails() {
    let mut input = packet_input();
    input.sandbox.run_rows[0].validated_proposal_ids = vec!["proposal:does-not-exist".to_owned()];
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::DanglingValidatedProposal));
}

#[test]
fn coverage_impact_set_empty_fails() {
    let mut input = packet_input();
    input.coverage_impact.impact_rows = vec![];
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::CoverageImpactSetEmpty));
}

#[test]
fn coverage_note_incomplete_fails() {
    let mut input = packet_input();
    input.coverage_impact.impact_rows[0].target_ref = String::new();
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::CoverageNoteIncomplete));
}

#[test]
fn hidden_coverage_note_fails() {
    let mut input = packet_input();
    input.coverage_impact.impact_rows[0].disclosed = false;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::HiddenCoverageNote));
}

#[test]
fn estimated_coverage_unlabeled_fails() {
    let mut input = packet_input();
    input.coverage_impact.impact_rows[1].estimated_labeled = false;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::EstimatedCoverageUnlabeled));
}

#[test]
fn estimate_as_measured_fails() {
    let mut input = packet_input();
    input.coverage_impact.no_estimate_as_measured = false;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::EstimateAsMeasured));
}

#[test]
fn estimated_count_mismatch_fails() {
    let mut input = packet_input();
    input.coverage_impact.estimated_notes_count = 0;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::EstimatedCountMismatch));
}

#[test]
fn consumer_surface_coverage_missing_fails() {
    let mut input = packet_input();
    input.consumer_surface_parity = vec![input.consumer_surface_parity[0].clone()];
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::ConsumerSurfaceCoverageMissing));
}

#[test]
fn stable_claim_not_qualified_fails() {
    let mut input = packet_input();
    input.consumer_surface_parity[0].claimed_stable = true;
    input.consumer_surface_parity[0].reachable = false;
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::StableClaimNotQualified));
}

#[test]
fn raw_boundary_material_detected() {
    let mut input = packet_input();
    input.assumptions.assumption_rows[0].scope_ref = "owner@internal.aureline.dev".to_owned();
    let packet = GeneratedTestReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&GeneratedTestReviewViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_renders() {
    let packet = GeneratedTestReviewPacket::new(packet_input());
    let md = packet.render_markdown_summary();
    assert!(md.starts_with(
        "# AI Test Generation, Assumption Review, Sandbox Validation, and Coverage Impact"
    ));
    assert!(md.contains(PACKET_ID));
    assert!(md.contains(GENERATION_PASS_ID));
}

#[test]
fn checked_in_export_loads_and_validates() {
    let result = current_stable_generated_test_review_export();
    assert!(
        result.is_ok(),
        "checked-in export should load and validate: {:?}",
        result
    );
}
