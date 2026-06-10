use super::*;

use crate::qualify_background_branch_agent_lifecycle::BranchAgentCleanupDisposition;

const PACKET_ID: &str = "ai-branch-agent:lifecycle:m5:0001";
const AGENT_RUN_ID: &str = "ai-agent-run:lifecycle:m5:0001";

fn lifecycle() -> AgentLifecycleBlock {
    AgentLifecycleBlock {
        execution_locus: BranchAgentExecutionLocus::IsolatedSideWorktree,
        current_stage: AgentLifecycleStage::MergeBackHandoff,
        base_ref: "ref:base:main@0001".to_owned(),
        branch_identity_ref: "ref:branch:agent-retry-refactor".to_owned(),
        worktree_identity_ref: Some("ref:worktree:agent-retry-refactor".to_owned()),
        launch_review_disclosed: true,
        isolation_verified: true,
        automation_bounded_by_stop_conditions: true,
        stage_rows: vec![
            AgentLifecycleStageRow {
                stage: AgentLifecycleStage::LaunchReview,
                checkpoint_ref: "checkpoint:lifecycle:m5:0001:launch".to_owned(),
                preview_shown: true,
                approval_required: true,
                approval_granted: true,
                review_artifacts_preserved: true,
                mutated_outside_isolation: false,
                operator_actions: vec![
                    BranchAgentOperatorAction::OpenReview,
                    BranchAgentOperatorAction::Cancel,
                ],
            },
            AgentLifecycleStageRow {
                stage: AgentLifecycleStage::IsolatedEditing,
                checkpoint_ref: "checkpoint:lifecycle:m5:0001:edit".to_owned(),
                preview_shown: true,
                approval_required: true,
                approval_granted: true,
                review_artifacts_preserved: true,
                mutated_outside_isolation: false,
                operator_actions: vec![
                    BranchAgentOperatorAction::Pause,
                    BranchAgentOperatorAction::CompareToBase,
                ],
            },
            AgentLifecycleStageRow {
                stage: AgentLifecycleStage::Validating,
                checkpoint_ref: "checkpoint:lifecycle:m5:0001:validate".to_owned(),
                preview_shown: true,
                approval_required: false,
                approval_granted: false,
                review_artifacts_preserved: true,
                mutated_outside_isolation: false,
                operator_actions: vec![BranchAgentOperatorAction::RerunValidation],
            },
            AgentLifecycleStageRow {
                stage: AgentLifecycleStage::ReviewReady,
                checkpoint_ref: "checkpoint:lifecycle:m5:0001:review".to_owned(),
                preview_shown: true,
                approval_required: true,
                approval_granted: false,
                review_artifacts_preserved: true,
                mutated_outside_isolation: false,
                operator_actions: vec![
                    BranchAgentOperatorAction::OpenReview,
                    BranchAgentOperatorAction::CompareToBase,
                ],
            },
            AgentLifecycleStageRow {
                stage: AgentLifecycleStage::MergeBackHandoff,
                checkpoint_ref: "checkpoint:lifecycle:m5:0001:handoff".to_owned(),
                preview_shown: true,
                approval_required: true,
                approval_granted: false,
                review_artifacts_preserved: true,
                mutated_outside_isolation: false,
                operator_actions: vec![
                    BranchAgentOperatorAction::CherryPick,
                    BranchAgentOperatorAction::DiscardBranch,
                ],
            },
        ],
    }
}

fn review_pack() -> SideBranchReviewPackBlock {
    SideBranchReviewPackBlock {
        pack_id: "review-pack:lifecycle:m5:0001".to_owned(),
        base_ref: "ref:base:main@0001".to_owned(),
        head_ref: "ref:head:agent-retry-refactor@0007".to_owned(),
        diff_packet_ref: "ai-patch-review:evidence-rich:m5:0001#diff".to_owned(),
        validation_receipt_ref: "ai-patch-review:evidence-rich:m5:0001#validation".to_owned(),
        rollback_handle_ref: "ai-patch-review:evidence-rich:m5:0001#rollback".to_owned(),
        evidence_packet_ref: "ai-evidence:finalized:m5:0001".to_owned(),
        finding_rows: vec![
            ReviewPackFindingRow {
                finding_id: "finding:lifecycle:m5:0001".to_owned(),
                severity: ReviewFindingSeverity::Major,
                file_ref: "file:retry-module".to_owned(),
                disclosed_in_pack: true,
                resolved: true,
            },
            ReviewPackFindingRow {
                finding_id: "finding:lifecycle:m5:0002".to_owned(),
                severity: ReviewFindingSeverity::Minor,
                file_ref: "file:retry-fixture".to_owned(),
                disclosed_in_pack: true,
                resolved: false,
            },
        ],
        compare_to_base_available: true,
        produced_in_isolation: true,
        review_required_before_merge: true,
    }
}

fn merge_back() -> MergeBackHandoffBlock {
    MergeBackHandoffBlock {
        handoff_id: "handoff:lifecycle:m5:0001".to_owned(),
        state: MergeBackState::ReadyForHumanMerge,
        destination_ref: "ref:dest:main".to_owned(),
        destination_protected: true,
        requires_human_approval: true,
        human_approval_granted: false,
        self_merge_forbidden: true,
        protected_destination_self_push_forbidden: true,
        available_actions: vec![
            MergeBackAction::CompareToBase,
            MergeBackAction::CherryPick,
            MergeBackAction::OpenPullRequest,
            MergeBackAction::RequestHumanReview,
            MergeBackAction::RerunValidation,
            MergeBackAction::DiscardBranch,
        ],
        cleanup_disposition: BranchAgentCleanupDisposition::RetainForReview,
        review_artifacts_survive_cleanup: true,
    }
}

fn consumer_surface_parity() -> Vec<ConsumerSurfaceParityRow> {
    ConsumerSurfaceClass::ALL
        .into_iter()
        .map(|surface| ConsumerSurfaceParityRow {
            surface,
            shows_lifecycle: true,
            shows_review_pack: true,
            shows_merge_back_handoff: true,
            reachable: true,
            qualification: SurfaceQualificationClass::Stable,
            claimed_stable: true,
        })
        .collect()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        BRANCH_WORKTREE_AGENT_LIFECYCLE_DOC_REF.to_owned(),
        BRANCH_WORKTREE_AGENT_LIFECYCLE_SCHEMA_REF.to_owned(),
        BRANCH_WORKTREE_AGENT_LIFECYCLE_BASE_CONTRACT_REF.to_owned(),
        BRANCH_WORKTREE_AGENT_LIFECYCLE_EVIDENCE_CONTRACT_REF.to_owned(),
        BRANCH_WORKTREE_AGENT_LIFECYCLE_M5_MATRIX_CONTRACT_REF.to_owned(),
    ]
}

fn packet_input() -> BranchWorktreeAgentLifecyclePacketInput {
    BranchWorktreeAgentLifecyclePacketInput {
        packet_id: PACKET_ID.to_owned(),
        agent_run_id: AGENT_RUN_ID.to_owned(),
        display_label: "M5 branch agent lifecycle for retry module refactor".to_owned(),
        trust_state_token: "restricted".to_owned(),
        policy_epoch_ref: "policy-epoch:m5:2026-06-01".to_owned(),
        lifecycle: lifecycle(),
        review_pack: review_pack(),
        merge_back: merge_back(),
        consumer_surface_parity: consumer_surface_parity(),
        downgrade_triggers: DowngradeTrigger::ALL.to_vec(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-10T09:55:12Z".to_owned(),
    }
}

#[test]
fn packet_constructs_and_serializes() {
    let packet = BranchWorktreeAgentLifecyclePacket::new(packet_input());
    let json = packet.export_safe_json();
    assert!(json.contains("branch_worktree_agent_lifecycle_implementation"));
    assert!(json.contains(PACKET_ID));
}

#[test]
fn valid_packet_passes_validation() {
    let packet = BranchWorktreeAgentLifecyclePacket::new(packet_input());
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = BranchWorktreeAgentLifecyclePacket::new(packet_input());
    packet.record_kind = "wrong_kind".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&BranchWorktreeAgentLifecycleViolation::WrongRecordKind));
}

#[test]
fn wrong_schema_version_fails() {
    let mut packet = BranchWorktreeAgentLifecyclePacket::new(packet_input());
    packet.schema_version = 999;
    let violations = packet.validate();
    assert!(violations.contains(&BranchWorktreeAgentLifecycleViolation::WrongSchemaVersion));
}

#[test]
fn missing_identity_fails() {
    let mut input = packet_input();
    input.agent_run_id = "   ".to_owned();
    let packet = BranchWorktreeAgentLifecyclePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&BranchWorktreeAgentLifecycleViolation::MissingIdentity));
}

#[test]
fn missing_source_contracts_fails() {
    let mut input = packet_input();
    input.source_contract_refs = vec![];
    let packet = BranchWorktreeAgentLifecyclePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&BranchWorktreeAgentLifecycleViolation::MissingSourceContracts));
}

#[test]
fn launch_review_missing_fails() {
    let mut input = packet_input();
    input.lifecycle.launch_review_disclosed = false;
    let packet = BranchWorktreeAgentLifecyclePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&BranchWorktreeAgentLifecycleViolation::LaunchReviewMissing));
}

#[test]
fn isolation_breach_fails() {
    let mut input = packet_input();
    input.lifecycle.stage_rows[1].mutated_outside_isolation = true;
    let packet = BranchWorktreeAgentLifecyclePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&BranchWorktreeAgentLifecycleViolation::IsolationBreached));
}

#[test]
fn mutating_stage_without_approval_fails() {
    let mut input = packet_input();
    input.lifecycle.stage_rows[1].approval_granted = false;
    let packet = BranchWorktreeAgentLifecyclePacket::new(input);
    let violations = packet.validate();
    assert!(violations
        .contains(&BranchWorktreeAgentLifecycleViolation::StageAppliedWithoutPreviewOrApproval));
}

#[test]
fn stage_does_not_preserve_artifacts_fails() {
    let mut input = packet_input();
    input.lifecycle.stage_rows[0].review_artifacts_preserved = false;
    let packet = BranchWorktreeAgentLifecyclePacket::new(input);
    let violations = packet.validate();
    assert!(
        violations.contains(&BranchWorktreeAgentLifecycleViolation::StageDoesNotPreserveArtifacts)
    );
}

#[test]
fn review_pack_incomplete_fails() {
    let mut input = packet_input();
    input.review_pack.diff_packet_ref = String::new();
    let packet = BranchWorktreeAgentLifecyclePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&BranchWorktreeAgentLifecycleViolation::ReviewPackIncomplete));
}

#[test]
fn review_pack_not_isolated_fails() {
    let mut input = packet_input();
    input.review_pack.produced_in_isolation = false;
    let packet = BranchWorktreeAgentLifecyclePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&BranchWorktreeAgentLifecycleViolation::ReviewPackNotIsolated));
}

#[test]
fn hidden_finding_fails() {
    let mut input = packet_input();
    input.review_pack.finding_rows[0].disclosed_in_pack = false;
    let packet = BranchWorktreeAgentLifecyclePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&BranchWorktreeAgentLifecycleViolation::HiddenFinding));
}

#[test]
fn merge_back_approval_not_required_fails() {
    let mut input = packet_input();
    input.merge_back.requires_human_approval = false;
    let packet = BranchWorktreeAgentLifecyclePacket::new(input);
    let violations = packet.validate();
    assert!(
        violations.contains(&BranchWorktreeAgentLifecycleViolation::MergeBackApprovalNotRequired)
    );
}

#[test]
fn merge_back_without_approval_fails() {
    let mut input = packet_input();
    input.merge_back.state = MergeBackState::MergedByHuman;
    input.merge_back.human_approval_granted = false;
    let packet = BranchWorktreeAgentLifecyclePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&BranchWorktreeAgentLifecycleViolation::MergeBackWithoutApproval));
}

#[test]
fn unsafe_landing_posture_fails() {
    let mut input = packet_input();
    input.merge_back.self_merge_forbidden = false;
    let packet = BranchWorktreeAgentLifecyclePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&BranchWorktreeAgentLifecycleViolation::UnsafeLandingPosture));
}

#[test]
fn cleanup_loses_review_artifacts_fails() {
    let mut input = packet_input();
    input.merge_back.review_artifacts_survive_cleanup = false;
    let packet = BranchWorktreeAgentLifecyclePacket::new(input);
    let violations = packet.validate();
    assert!(
        violations.contains(&BranchWorktreeAgentLifecycleViolation::CleanupLosesReviewArtifacts)
    );
}

#[test]
fn merge_back_actions_incomplete_fails() {
    let mut input = packet_input();
    input.merge_back.available_actions = vec![MergeBackAction::CompareToBase];
    let packet = BranchWorktreeAgentLifecyclePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&BranchWorktreeAgentLifecycleViolation::MergeBackActionsIncomplete));
}

#[test]
fn consumer_surface_coverage_missing_fails() {
    let mut input = packet_input();
    input.consumer_surface_parity = vec![input.consumer_surface_parity[0].clone()];
    let packet = BranchWorktreeAgentLifecyclePacket::new(input);
    let violations = packet.validate();
    assert!(
        violations.contains(&BranchWorktreeAgentLifecycleViolation::ConsumerSurfaceCoverageMissing)
    );
}

#[test]
fn stable_claim_not_qualified_fails() {
    let mut input = packet_input();
    input.consumer_surface_parity[0].claimed_stable = true;
    input.consumer_surface_parity[0].reachable = false;
    let packet = BranchWorktreeAgentLifecyclePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&BranchWorktreeAgentLifecycleViolation::StableClaimNotQualified));
}

#[test]
fn raw_boundary_material_detected() {
    let mut input = packet_input();
    input.review_pack.diff_packet_ref = "https://internal.aureline.dev/diff-secret".to_owned();
    let packet = BranchWorktreeAgentLifecyclePacket::new(input);
    let violations = packet.validate();
    assert!(
        violations.contains(&BranchWorktreeAgentLifecycleViolation::RawBoundaryMaterialInExport)
    );
}

#[test]
fn markdown_summary_renders() {
    let packet = BranchWorktreeAgentLifecyclePacket::new(packet_input());
    let md = packet.render_markdown_summary();
    assert!(md.starts_with("# Branch or Worktree Agent Lifecycle"));
    assert!(md.contains(PACKET_ID));
    assert!(md.contains(AGENT_RUN_ID));
}

#[test]
fn checked_in_export_loads_and_validates() {
    let result = current_stable_branch_worktree_agent_lifecycle_export();
    assert!(
        result.is_ok(),
        "checked-in export should load and validate: {:?}",
        result
    );
}
