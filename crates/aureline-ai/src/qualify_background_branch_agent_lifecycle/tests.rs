use super::*;

fn launch_review(run_id: &str) -> BranchAgentLaunchReviewSheet {
    BranchAgentLaunchReviewSheet {
        stable_run_id: run_id.to_owned(),
        requested_goal: "Refactor the request cache behind a side-worktree review packet"
            .to_owned(),
        base_ref: "base:main@sha256:base-reviewed".to_owned(),
        target_identity_ref: "change-object:side-worktree:request-cache".to_owned(),
        tool_connector_classes: vec![
            "repo_search_read_only".to_owned(),
            "patch_writer_side_worktree".to_owned(),
            "validation_runner_local".to_owned(),
        ],
        approval_gates: vec![
            "launch_review_acceptance".to_owned(),
            "mutating_tool_once_per_checkpoint".to_owned(),
            "completion_review_before_landing".to_owned(),
        ],
        estimated_cost_risk_band: "medium_cost_medium_risk".to_owned(),
        secret_scope: "read_only_redacted_handles".to_owned(),
        stop_conditions: vec![
            "secrets_found".to_owned(),
            "base_branch_advanced".to_owned(),
            "cross_worktree_write_attempt".to_owned(),
        ],
    }
}

fn active_rows(run_id: &str) -> Vec<BranchAgentActiveRunRow> {
    vec![
        BranchAgentActiveRunRow {
            stable_run_id: run_id.to_owned(),
            elapsed_time_label: "00:01:12".to_owned(),
            current_milestone: BranchAgentRunState::Planning,
            environment_assumptions: vec!["local validation runner available".to_owned()],
            pending_approvals: vec!["launch_review_acceptance".to_owned()],
            operator_actions: vec![
                BranchAgentOperatorAction::Pause,
                BranchAgentOperatorAction::Cancel,
                BranchAgentOperatorAction::OpenReview,
                BranchAgentOperatorAction::TakeOverManually,
            ],
            execution_locus: BranchAgentExecutionLocus::IsolatedSideWorktree,
        },
        BranchAgentActiveRunRow {
            stable_run_id: run_id.to_owned(),
            elapsed_time_label: "00:09:44".to_owned(),
            current_milestone: BranchAgentRunState::ReReviewRequired,
            environment_assumptions: vec![
                "policy epoch stable at dispatch".to_owned(),
                "base commit pinned".to_owned(),
            ],
            pending_approvals: vec!["fresh_review_after_drift".to_owned()],
            operator_actions: vec![
                BranchAgentOperatorAction::OpenReview,
                BranchAgentOperatorAction::ResumeFromCheckpoint,
                BranchAgentOperatorAction::TakeOverManually,
            ],
            execution_locus: BranchAgentExecutionLocus::IsolatedSideWorktree,
        },
    ]
}

fn checkpoint_rows(run_id: &str) -> Vec<BranchAgentCheckpointRow> {
    vec![
        BranchAgentCheckpointRow {
            stable_run_id: run_id.to_owned(),
            checkpoint_id: "checkpoint:launch-review".to_owned(),
            elapsed_time_label: "00:00:00".to_owned(),
            milestone: BranchAgentRunState::Planning,
            new_artifact_refs: vec![
                "artifacts/ai/m4/qualify-background-branch-agent-lifecycle/launch-review.md"
                    .to_owned(),
            ],
            evidence_refs: vec!["evidence:launch-sheet".to_owned()],
            pending_approvals: vec!["launch_review_acceptance".to_owned()],
            operator_actions: vec![
                BranchAgentOperatorAction::Cancel,
                BranchAgentOperatorAction::OpenReview,
            ],
            review_artifacts_preserved: true,
        },
        BranchAgentCheckpointRow {
            stable_run_id: run_id.to_owned(),
            checkpoint_id: "checkpoint:validation-complete".to_owned(),
            elapsed_time_label: "00:08:39".to_owned(),
            milestone: BranchAgentRunState::Validating,
            new_artifact_refs: vec![
                "diff:request-cache-side-worktree".to_owned(),
                "validation:request-cache-local-suite".to_owned(),
            ],
            evidence_refs: vec!["evidence:packet:branch-agent-request-cache".to_owned()],
            pending_approvals: vec!["completion_review_before_landing".to_owned()],
            operator_actions: vec![
                BranchAgentOperatorAction::OpenReview,
                BranchAgentOperatorAction::RerunValidation,
                BranchAgentOperatorAction::TakeOverManually,
            ],
            review_artifacts_preserved: true,
        },
    ]
}

fn drift_drills(run_id: &str) -> Vec<BranchAgentDriftDrillRow> {
    BranchAgentDriftTrigger::required_coverage()
        .into_iter()
        .map(|trigger| BranchAgentDriftDrillRow {
            stable_run_id: run_id.to_owned(),
            trigger,
            drift_summary: format!("{} pauses the run before further writes", trigger.as_str()),
            resulting_state: BranchAgentRunState::ReReviewRequired,
            further_writes_blocked: true,
            already_produced_artifacts_preserved: true,
            requires_re_review_or_takeover: true,
            pauses_or_narrows_safely: true,
        })
        .collect()
}

fn takeover(run_id: &str) -> BranchAgentTakeoverRow {
    BranchAgentTakeoverRow {
        stable_run_id: run_id.to_owned(),
        branch_identity_preserved: true,
        checkpoint_lineage_preserved: true,
        tool_call_history_preserved: true,
        validation_receipts_preserved: true,
        pending_writes_disclosed: true,
        safe_next_step: "Open the side worktree and continue from checkpoint:validation-complete"
            .to_owned(),
        rerun_options: vec![
            BranchAgentOperatorAction::ResumeFromCheckpoint,
            BranchAgentOperatorAction::TakeOverManually,
        ],
    }
}

fn completion_review(run_id: &str) -> BranchAgentCompletionReview {
    BranchAgentCompletionReview {
        stable_run_id: run_id.to_owned(),
        diff_summary_ref: "diff:request-cache-side-worktree".to_owned(),
        validation_summary_ref: "validation:request-cache-local-suite".to_owned(),
        evidence_packet_ref: "evidence:packet:branch-agent-request-cache".to_owned(),
        compare_to_base_available: true,
        cleanup_options_available: true,
        follow_up_actions: vec![
            BranchAgentOperatorAction::OpenReview,
            BranchAgentOperatorAction::CompareToBase,
            BranchAgentOperatorAction::CherryPick,
            BranchAgentOperatorAction::RerunValidation,
            BranchAgentOperatorAction::DiscardBranch,
        ],
        self_merge_forbidden: true,
        protected_destination_push_forbidden: true,
    }
}

fn cleanup_rows(run_id: &str) -> Vec<BranchAgentCleanupRow> {
    vec![BranchAgentCleanupRow {
        stable_run_id: run_id.to_owned(),
        disposition: BranchAgentCleanupDisposition::PreviewDeleteAvailable,
        evidence_retention: "minimum_evidence_lifetime_pinned".to_owned(),
        checkpoint_retention: "checkpoint_group_retained_until_expiry_or_explicit_delete"
            .to_owned(),
        preview_required_before_delete: true,
        review_artifacts_survive_cleanup: true,
        support_export_ref:
            "artifacts/ai/m4/qualify-background-branch-agent-lifecycle/support_export.json"
                .to_owned(),
    }]
}

fn valid_packet() -> BackgroundBranchAgentLifecyclePacket {
    let run_id = "branch-agent-run:stable:request-cache:0001";
    BackgroundBranchAgentLifecyclePacket {
        record_kind: BACKGROUND_BRANCH_AGENT_LIFECYCLE_RECORD_KIND.to_owned(),
        schema_version: BACKGROUND_BRANCH_AGENT_LIFECYCLE_SCHEMA_VERSION,
        packet_id: "background-branch-agent-lifecycle:stable:0001".to_owned(),
        stable_run_id: run_id.to_owned(),
        plan_version: "plan-v3".to_owned(),
        initiator: "user_explicit_launch_review".to_owned(),
        branch_identity_ref: "branch-ref:side/request-cache".to_owned(),
        worktree_identity_ref: "worktree-ref:isolated/request-cache".to_owned(),
        base_commit_ref: "base:main@sha256:base-reviewed".to_owned(),
        requested_goal: "Refactor the request cache behind a side-worktree review packet"
            .to_owned(),
        current_state: BranchAgentRunState::ReadyForReview,
        current_execution_locus: BranchAgentExecutionLocus::IsolatedSideWorktree,
        pending_approvals: vec!["completion_review_before_landing".to_owned()],
        evidence_refs: vec!["evidence:packet:branch-agent-request-cache".to_owned()],
        cancellation_posture: BranchAgentCancellationPosture::ContinueUntilSafeCheckpoint,
        cleanup_posture: BranchAgentCleanupDisposition::PreviewDeleteAvailable,
        launch_review: launch_review(run_id),
        active_run_rows: active_rows(run_id),
        checkpoint_rows: checkpoint_rows(run_id),
        drift_drills: drift_drills(run_id),
        takeover: takeover(run_id),
        completion_review: completion_review(run_id),
        cleanup_rows: cleanup_rows(run_id),
        security_review: BranchAgentSecurityReview {
            no_self_approved_mutating_tools: true,
            no_worktree_isolation_bypass: true,
            execution_loci_not_collapsed: true,
            local_side_and_managed_loci_distinct: true,
            protected_destination_self_push_blocked: true,
        },
        consumer_projection: BranchAgentConsumerProjection {
            ui_rows_use_stable_run_id: true,
            support_exports_use_stable_run_id: true,
            evidence_packets_use_stable_run_id: true,
            docs_help_cite_governed_contract: true,
            preview_labs_label_for_unqualified_lanes: true,
        },
        source_contract_refs: vec![
            BACKGROUND_BRANCH_AGENT_LIFECYCLE_AI_DOC_REF.to_owned(),
            BACKGROUND_BRANCH_AGENT_BASE_CONTRACT_REF.to_owned(),
            BACKGROUND_BRANCH_AGENT_LIFECYCLE_SCHEMA_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T01:55:00Z".to_owned(),
    }
}

#[test]
fn valid_packet_passes_validation() {
    let packet = valid_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn all_rows_must_keep_stable_run_id() {
    let mut packet = valid_packet();
    packet.checkpoint_rows[0].stable_run_id = "different-run".to_owned();

    let violations = packet.validate();
    assert!(violations.iter().any(|violation| matches!(
        violation,
        BackgroundBranchAgentLifecycleViolation::RunIdMismatch {
            row: "checkpoint_row",
            ..
        }
    )));
}

#[test]
fn drift_drills_require_all_triggers() {
    let mut packet = valid_packet();
    packet
        .drift_drills
        .retain(|row| row.trigger != BranchAgentDriftTrigger::PolicyEpochChange);

    let violations = packet.validate();
    assert!(violations.iter().any(|violation| matches!(
        violation,
        BackgroundBranchAgentLifecycleViolation::MissingDriftTrigger {
            trigger: BranchAgentDriftTrigger::PolicyEpochChange
        }
    )));
}

#[test]
fn drift_must_block_writes_and_preserve_artifacts() {
    let mut packet = valid_packet();
    let row = packet
        .drift_drills
        .iter_mut()
        .find(|row| row.trigger == BranchAgentDriftTrigger::BoundaryExpansion)
        .expect("boundary expansion drill exists");
    row.further_writes_blocked = false;
    row.already_produced_artifacts_preserved = false;

    let violations = packet.validate();
    assert!(violations.iter().any(|violation| matches!(
        violation,
        BackgroundBranchAgentLifecycleViolation::DriftAllowsFurtherWrites {
            trigger: BranchAgentDriftTrigger::BoundaryExpansion
        }
    )));
    assert!(violations.iter().any(|violation| matches!(
        violation,
        BackgroundBranchAgentLifecycleViolation::DriftLosesArtifacts {
            trigger: BranchAgentDriftTrigger::BoundaryExpansion
        }
    )));
}

#[test]
fn takeover_must_preserve_lineage() {
    let mut packet = valid_packet();
    packet.takeover.tool_call_history_preserved = false;

    let violations = packet.validate();
    assert!(violations.iter().any(|violation| matches!(
        violation,
        BackgroundBranchAgentLifecycleViolation::TakeoverLineageIncomplete
    )));
}

#[test]
fn completion_review_cannot_allow_self_land() {
    let mut packet = valid_packet();
    packet.completion_review.self_merge_forbidden = false;

    let violations = packet.validate();
    assert!(violations.iter().any(|violation| matches!(
        violation,
        BackgroundBranchAgentLifecycleViolation::UnsafeLandingPosture
    )));
}

#[test]
fn cleanup_must_keep_artifacts_reviewable() {
    let mut packet = valid_packet();
    packet.cleanup_rows[0].review_artifacts_survive_cleanup = false;

    let violations = packet.validate();
    assert!(violations.iter().any(|violation| matches!(
        violation,
        BackgroundBranchAgentLifecycleViolation::CleanupLosesReviewArtifacts
    )));
}

#[test]
fn checked_artifact_validates() {
    let packet = current_stable_background_branch_agent_lifecycle_export()
        .expect("checked branch-agent lifecycle export validates");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}
