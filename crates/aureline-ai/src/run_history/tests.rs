use super::*;

use crate::routing::{DeploymentProfileClass, PolicyTrustState};

fn policy_context() -> RoutingPolicyContext {
    RoutingPolicyContext {
        policy_epoch_ref: "policy-epoch:0042".to_owned(),
        trust_state: PolicyTrustState::Trusted,
        deployment_profile_class: DeploymentProfileClass::IndividualLocal,
        execution_context_ref: Some("execution-context:ai.run-history:0001".to_owned()),
    }
}

fn applied_local_entry() -> AiRunHistoryEntry {
    AiRunHistoryEntry {
        record_kind: AI_RUN_HISTORY_ENTRY_RECORD_KIND.to_owned(),
        schema_version: AI_RUN_HISTORY_SCHEMA_VERSION,
        canonical_run_id: "ai-run:applied:local:0001".to_owned(),
        task_label: "Rename a private helper across the workspace.".to_owned(),
        actor_class: AiRunActorClass::LocalUser,
        actor_identity_ref: "actor:local-user:0001".to_owned(),
        provider_entry_ref: "provider-entry:local_native:0001".to_owned(),
        model_entry_ref: "model-entry:local_native:fast:0001".to_owned(),
        provider_label: "Aureline native local model".to_owned(),
        model_label: "Local fast model".to_owned(),
        execution_boundary_class: AiRunExecutionBoundaryClass::LocalInProcess,
        boundary_label: "Local in-process AI model boundary".to_owned(),
        outcome_class: AiRunOutcomeClass::SucceededWithLocalReversibleEdit,
        outcome_summary_label: "Reversible local edit landed after one approval.".to_owned(),
        cost_band_class: AiRunCostBandClass::NegligibleCost,
        quota_band_class: AiRunQuotaBandClass::QuotaHealthy,
        approval_timeline: vec![
            ApprovalTimelineEvent {
                event_id: "approval-event:applied:local:0001:pending".to_owned(),
                approval_ticket_ref: "approval-ticket:applied:local:0001".to_owned(),
                decision_class: ApprovalEventDecisionClass::PendingUserReview,
                scope_class: ApprovalScopeClass::OneTimePerInvocation,
                actor_class: ApprovalEventActorClass::LocalUser,
                actor_identity_ref: "actor:local-user:0001".to_owned(),
                object_class: ApprovalObjectClass::AiRun,
                object_identity_ref: "ai-run:applied:local:0001".to_owned(),
                policy_epoch_ref: "policy-epoch:0042".to_owned(),
                expires_at: Some("2026-05-18T00:30:00Z".to_owned()),
                revocation_note: None,
                decided_at: "2026-05-18T00:00:01Z".to_owned(),
                summary_label: "Approval prompted for one reversible local edit.".to_owned(),
            },
            ApprovalTimelineEvent {
                event_id: "approval-event:applied:local:0001:granted".to_owned(),
                approval_ticket_ref: "approval-ticket:applied:local:0001".to_owned(),
                decision_class: ApprovalEventDecisionClass::Granted,
                scope_class: ApprovalScopeClass::OneTimePerInvocation,
                actor_class: ApprovalEventActorClass::LocalUser,
                actor_identity_ref: "actor:local-user:0001".to_owned(),
                object_class: ApprovalObjectClass::AiRun,
                object_identity_ref: "ai-run:applied:local:0001".to_owned(),
                policy_epoch_ref: "policy-epoch:0042".to_owned(),
                expires_at: Some("2026-05-18T00:30:00Z".to_owned()),
                revocation_note: None,
                decided_at: "2026-05-18T00:00:02Z".to_owned(),
                summary_label: "Local user approved one reversible edit.".to_owned(),
            },
        ],
        evidence_lineage: AiRunEvidenceLineage {
            evidence_packet_ref: "evidence-packet:applied:local:0001".to_owned(),
            routing_packet_ref: "routing-packet:applied:local:0001".to_owned(),
            spend_receipt_ref: "spend-receipt:applied:local:0001".to_owned(),
            route_receipt_ref: "route-receipt:applied:local:0001".to_owned(),
            replay_packet_ref: "replay-packet:applied:local:0001".to_owned(),
            mutation_journal_ref: "mutation-journal:applied:local:0001".to_owned(),
            rollback_checkpoint_ref: "rollback-checkpoint:applied:local:0001".to_owned(),
            produced_artifact_refs: vec!["artifact:patch:applied:local:0001".to_owned()],
            diff_artifact_refs: vec!["diff:applied:local:0001".to_owned()],
            validation_summary_refs: vec!["validation-summary:applied:local:0001".to_owned()],
            validation_outcome_class: AiRunValidationOutcomeClass::ValidationPassed,
            running_build_identity_ref: "running-build-identity:0001".to_owned(),
        },
        thread_lineage: AiRunThreadLineage {
            composer_session_ref: "composer-session:applied:local:0001".to_owned(),
            turn_draft_ref: "turn-draft:applied:local:0001".to_owned(),
            request_workspace_ref: "request-workspace:applied:local:0001".to_owned(),
            assembly_ref: "context-assembly:applied:local:0001".to_owned(),
        },
        history_state_class: AiRunHistoryStateClass::Applied,
        evidence_completeness_class: EvidenceCompletenessClass::ReconstructibleFull,
        evidence_incompleteness_reason_class: None,
        evidence_incompleteness_note: None,
        tool_call_lineage_refs: vec!["tool-call:native-fs-snapshot:0001".to_owned()],
        rerun_review_refs: vec!["rerun-review:applied:local:0001".to_owned()],
        actions: AiRunHistoryActions {
            open_details_action_ref: "action:run-history:open-details:applied:local:0001"
                .to_owned(),
            open_evidence_packet_action_ref: "action:run-history:open-evidence:applied:local:0001"
                .to_owned(),
            open_thread_action_ref: "action:run-history:open-thread:applied:local:0001".to_owned(),
            open_replay_action_ref: "action:run-history:open-replay:applied:local:0001".to_owned(),
            open_support_export_action_ref:
                "action:run-history:open-support-export:applied:local:0001".to_owned(),
            open_rerun_review_action_ref: "action:run-history:open-rerun-review:applied:local:0001"
                .to_owned(),
            open_as_recipe_action_ref: "action:run-history:open-as-recipe:applied:local:0001"
                .to_owned(),
            share_action_ref: "action:run-history:share:applied:local:0001".to_owned(),
            export_action_ref: "action:run-history:export:applied:local:0001".to_owned(),
        },
        policy_context: policy_context(),
        redaction_class: AiRunHistoryRedactionClass::MetadataSafeDefault,
        minted_at: "2026-05-18T00:00:00Z".to_owned(),
        completed_at: Some("2026-05-18T00:00:05Z".to_owned()),
    }
}

fn denied_remote_entry() -> AiRunHistoryEntry {
    AiRunHistoryEntry {
        record_kind: AI_RUN_HISTORY_ENTRY_RECORD_KIND.to_owned(),
        schema_version: AI_RUN_HISTORY_SCHEMA_VERSION,
        canonical_run_id: "ai-run:denied:remote:0001".to_owned(),
        task_label: "Summarize issues from the enterprise issue tracker.".to_owned(),
        actor_class: AiRunActorClass::LocalUser,
        actor_identity_ref: "actor:local-user:0001".to_owned(),
        provider_entry_ref: "provider-entry:remote_vendor_hosted:0001".to_owned(),
        model_entry_ref: "model-entry:remote_vendor_hosted:reasoning:0001".to_owned(),
        provider_label: "Remote vendor hosted provider".to_owned(),
        model_label: "Remote reasoning model".to_owned(),
        execution_boundary_class: AiRunExecutionBoundaryClass::RemoteVendorManagedService,
        boundary_label: "Remote vendor managed service boundary".to_owned(),
        outcome_class: AiRunOutcomeClass::DeniedByApprovalMissing,
        outcome_summary_label:
            "Remote AI run denied because admin approval for the issues connector was missing."
                .to_owned(),
        cost_band_class: AiRunCostBandClass::NoCost,
        quota_band_class: AiRunQuotaBandClass::QuotaHealthy,
        approval_timeline: vec![
            ApprovalTimelineEvent {
                event_id: "approval-event:denied:remote:0001:pending".to_owned(),
                approval_ticket_ref: "approval-ticket:denied:remote:0001".to_owned(),
                decision_class: ApprovalEventDecisionClass::PendingUserReview,
                scope_class: ApprovalScopeClass::AdminTicket,
                actor_class: ApprovalEventActorClass::AdminPolicy,
                actor_identity_ref: "actor:admin-policy:0001".to_owned(),
                object_class: ApprovalObjectClass::ToolInvocation,
                object_identity_ref: "tool-invocation:denied:remote:0001".to_owned(),
                policy_epoch_ref: "policy-epoch:0042".to_owned(),
                expires_at: None,
                revocation_note: None,
                decided_at: "2026-05-18T01:00:00Z".to_owned(),
                summary_label: "Admin approval requested for issues connector.".to_owned(),
            },
            ApprovalTimelineEvent {
                event_id: "approval-event:denied:remote:0001:blocked".to_owned(),
                approval_ticket_ref: "approval-ticket:denied:remote:0001".to_owned(),
                decision_class: ApprovalEventDecisionClass::BlockedByPolicy,
                scope_class: ApprovalScopeClass::AdminTicket,
                actor_class: ApprovalEventActorClass::AutomatedPolicyGate,
                actor_identity_ref: "actor:automated-policy-gate:0001".to_owned(),
                object_class: ApprovalObjectClass::ToolInvocation,
                object_identity_ref: "tool-invocation:denied:remote:0001".to_owned(),
                policy_epoch_ref: "policy-epoch:0042".to_owned(),
                expires_at: None,
                revocation_note: None,
                decided_at: "2026-05-18T01:00:01Z".to_owned(),
                summary_label: "Policy gate blocked approval before any tool dispatch.".to_owned(),
            },
        ],
        evidence_lineage: AiRunEvidenceLineage {
            evidence_packet_ref: "evidence-packet:denied:remote:0001".to_owned(),
            routing_packet_ref: "routing-packet:denied:remote:0001".to_owned(),
            spend_receipt_ref: "spend-receipt:denied:remote:0001".to_owned(),
            route_receipt_ref: String::new(),
            replay_packet_ref: "replay-packet:denied:remote:0001".to_owned(),
            mutation_journal_ref: String::new(),
            rollback_checkpoint_ref: String::new(),
            produced_artifact_refs: Vec::new(),
            diff_artifact_refs: Vec::new(),
            validation_summary_refs: Vec::new(),
            validation_outcome_class: AiRunValidationOutcomeClass::ValidationNotRun,
            running_build_identity_ref: "running-build-identity:0001".to_owned(),
        },
        thread_lineage: AiRunThreadLineage {
            composer_session_ref: "composer-session:denied:remote:0001".to_owned(),
            turn_draft_ref: "turn-draft:denied:remote:0001".to_owned(),
            request_workspace_ref: "request-workspace:denied:remote:0001".to_owned(),
            assembly_ref: String::new(),
        },
        history_state_class: AiRunHistoryStateClass::Rejected,
        evidence_completeness_class: EvidenceCompletenessClass::ReconstructibleFull,
        evidence_incompleteness_reason_class: None,
        evidence_incompleteness_note: None,
        tool_call_lineage_refs: vec!["tool-call:remote-issues:0001".to_owned()],
        rerun_review_refs: vec!["rerun-review:denied:remote:0001".to_owned()],
        actions: AiRunHistoryActions {
            open_details_action_ref: "action:run-history:open-details:denied:remote:0001"
                .to_owned(),
            open_evidence_packet_action_ref: "action:run-history:open-evidence:denied:remote:0001"
                .to_owned(),
            open_thread_action_ref: "action:run-history:open-thread:denied:remote:0001".to_owned(),
            open_replay_action_ref: "action:run-history:open-replay:denied:remote:0001".to_owned(),
            open_support_export_action_ref:
                "action:run-history:open-support-export:denied:remote:0001".to_owned(),
            open_rerun_review_action_ref: "action:run-history:open-rerun-review:denied:remote:0001"
                .to_owned(),
            open_as_recipe_action_ref: "action:run-history:open-as-recipe:denied:remote:0001"
                .to_owned(),
            share_action_ref: String::new(),
            export_action_ref: "action:run-history:export:denied:remote:0001".to_owned(),
        },
        policy_context: policy_context(),
        redaction_class: AiRunHistoryRedactionClass::MetadataSafeDefault,
        minted_at: "2026-05-18T01:00:00Z".to_owned(),
        completed_at: Some("2026-05-18T01:00:02Z".to_owned()),
    }
}

fn revoked_branch_agent_entry() -> AiRunHistoryEntry {
    AiRunHistoryEntry {
        record_kind: AI_RUN_HISTORY_ENTRY_RECORD_KIND.to_owned(),
        schema_version: AI_RUN_HISTORY_SCHEMA_VERSION,
        canonical_run_id: "ai-run:revoked:branch-agent:0001".to_owned(),
        task_label: "Background branch-agent task for a long-running refactor.".to_owned(),
        actor_class: AiRunActorClass::BackgroundBranchAgent,
        actor_identity_ref: "actor:branch-agent:0001".to_owned(),
        provider_entry_ref: "provider-entry:remote_vendor_hosted:0002".to_owned(),
        model_entry_ref: "model-entry:remote_vendor_hosted:reasoning:0002".to_owned(),
        provider_label: "Withdrawn remote vendor".to_owned(),
        model_label: "Withdrawn remote model".to_owned(),
        execution_boundary_class: AiRunExecutionBoundaryClass::RemoteVendorManagedService,
        boundary_label: "Remote vendor managed service boundary".to_owned(),
        outcome_class: AiRunOutcomeClass::SucceededWithBranchAgentDispatch,
        outcome_summary_label: "Branch agent dispatched the refactor; later replay is degraded."
            .to_owned(),
        cost_band_class: AiRunCostBandClass::SmallCost,
        quota_band_class: AiRunQuotaBandClass::QuotaPressured,
        approval_timeline: vec![
            ApprovalTimelineEvent {
                event_id: "approval-event:revoked:branch-agent:0001:granted".to_owned(),
                approval_ticket_ref: "approval-ticket:revoked:branch-agent:0001".to_owned(),
                decision_class: ApprovalEventDecisionClass::Granted,
                scope_class: ApprovalScopeClass::AdminTicket,
                actor_class: ApprovalEventActorClass::AdminPolicy,
                actor_identity_ref: "actor:admin-policy:0001".to_owned(),
                object_class: ApprovalObjectClass::BranchAgentDispatch,
                object_identity_ref: "branch-agent-dispatch:0001".to_owned(),
                policy_epoch_ref: "policy-epoch:0041".to_owned(),
                expires_at: Some("2026-05-19T00:00:00Z".to_owned()),
                revocation_note: None,
                decided_at: "2026-05-17T00:00:00Z".to_owned(),
                summary_label: "Admin granted branch-agent dispatch.".to_owned(),
            },
            ApprovalTimelineEvent {
                event_id: "approval-event:revoked:branch-agent:0001:revoked".to_owned(),
                approval_ticket_ref: "approval-ticket:revoked:branch-agent:0001".to_owned(),
                decision_class: ApprovalEventDecisionClass::Revoked,
                scope_class: ApprovalScopeClass::AdminTicket,
                actor_class: ApprovalEventActorClass::AdminPolicy,
                actor_identity_ref: "actor:admin-policy:0001".to_owned(),
                object_class: ApprovalObjectClass::BranchAgentDispatch,
                object_identity_ref: "branch-agent-dispatch:0001".to_owned(),
                policy_epoch_ref: "policy-epoch:0042".to_owned(),
                expires_at: None,
                revocation_note: Some(
                    "Provider for the branch-agent path was withdrawn; admin revoked the ticket."
                        .to_owned(),
                ),
                decided_at: "2026-05-18T02:00:00Z".to_owned(),
                summary_label: "Admin revoked branch-agent dispatch after provider was withdrawn."
                    .to_owned(),
            },
        ],
        evidence_lineage: AiRunEvidenceLineage {
            evidence_packet_ref: "evidence-packet:revoked:branch-agent:0001".to_owned(),
            routing_packet_ref: "routing-packet:revoked:branch-agent:0001".to_owned(),
            spend_receipt_ref: "spend-receipt:revoked:branch-agent:0001".to_owned(),
            route_receipt_ref: "route-receipt:revoked:branch-agent:0001".to_owned(),
            replay_packet_ref: "replay-packet:revoked:branch-agent:0001".to_owned(),
            mutation_journal_ref: "mutation-journal:revoked:branch-agent:0001".to_owned(),
            rollback_checkpoint_ref: "rollback-checkpoint:revoked:branch-agent:0001".to_owned(),
            produced_artifact_refs: vec!["artifact:patch:revoked:branch-agent:0001".to_owned()],
            diff_artifact_refs: vec!["diff:revoked:branch-agent:0001".to_owned()],
            validation_summary_refs: vec!["validation-summary:revoked:branch-agent:0001".to_owned()],
            validation_outcome_class: AiRunValidationOutcomeClass::ValidationPartialUnreproducible,
            running_build_identity_ref: "running-build-identity:0001".to_owned(),
        },
        thread_lineage: AiRunThreadLineage {
            composer_session_ref: "composer-session:revoked:branch-agent:0001".to_owned(),
            turn_draft_ref: "turn-draft:revoked:branch-agent:0001".to_owned(),
            request_workspace_ref: "request-workspace:revoked:branch-agent:0001".to_owned(),
            assembly_ref: "context-assembly:revoked:branch-agent:0001".to_owned(),
        },
        history_state_class: AiRunHistoryStateClass::Applied,
        evidence_completeness_class: EvidenceCompletenessClass::EvidenceIncompleteDegradedReplay,
        evidence_incompleteness_reason_class: Some(
            EvidenceIncompletenessReasonClass::ProviderWithdrawn,
        ),
        evidence_incompleteness_note: Some(
            "Vendor provider was withdrawn; replay will not reproduce the exact model path."
                .to_owned(),
        ),
        tool_call_lineage_refs: vec![],
        rerun_review_refs: vec!["rerun-review:revoked:branch-agent:0001".to_owned()],
        actions: AiRunHistoryActions {
            open_details_action_ref: "action:run-history:open-details:revoked:branch-agent:0001"
                .to_owned(),
            open_evidence_packet_action_ref:
                "action:run-history:open-evidence:revoked:branch-agent:0001".to_owned(),
            open_thread_action_ref: "action:run-history:open-thread:revoked:branch-agent:0001"
                .to_owned(),
            open_replay_action_ref: "action:run-history:open-replay:revoked:branch-agent:0001"
                .to_owned(),
            open_support_export_action_ref:
                "action:run-history:open-support-export:revoked:branch-agent:0001".to_owned(),
            open_rerun_review_action_ref:
                "action:run-history:open-rerun-review:revoked:branch-agent:0001".to_owned(),
            open_as_recipe_action_ref:
                "action:run-history:open-as-recipe:revoked:branch-agent:0001".to_owned(),
            share_action_ref: String::new(),
            export_action_ref: "action:run-history:export:revoked:branch-agent:0001".to_owned(),
        },
        policy_context: policy_context(),
        redaction_class: AiRunHistoryRedactionClass::MetadataSafeDefault,
        minted_at: "2026-05-17T00:00:00Z".to_owned(),
        completed_at: Some("2026-05-17T00:30:00Z".to_owned()),
    }
}

fn admit_rerun_review() -> AiRerunReview {
    AiRerunReview {
        record_kind: AI_RERUN_REVIEW_RECORD_KIND.to_owned(),
        schema_version: AI_RUN_HISTORY_SCHEMA_VERSION,
        rerun_review_id: "rerun-review:applied:local:0001".to_owned(),
        canonical_run_id: "ai-run:applied:local:0001".to_owned(),
        original_run_entry_ref: "ai-run:applied:local:0001".to_owned(),
        drift_rows: vec![
            RerunDriftRow {
                axis_class: RerunDriftAxisClass::WorkspaceRevision,
                drift_class: RerunDriftClass::NoDrift,
                original_identity_ref: "workspace-revision:0001".to_owned(),
                current_identity_ref: "workspace-revision:0001".to_owned(),
                original_label: "Workspace revision 0001".to_owned(),
                current_label: "Workspace revision 0001".to_owned(),
                summary_label: "Workspace revision unchanged.".to_owned(),
            },
            RerunDriftRow {
                axis_class: RerunDriftAxisClass::PolicyEpoch,
                drift_class: RerunDriftClass::NoDrift,
                original_identity_ref: "policy-epoch:0042".to_owned(),
                current_identity_ref: "policy-epoch:0042".to_owned(),
                original_label: "Policy epoch 0042".to_owned(),
                current_label: "Policy epoch 0042".to_owned(),
                summary_label: "Policy epoch unchanged.".to_owned(),
            },
            RerunDriftRow {
                axis_class: RerunDriftAxisClass::ProviderLifecycle,
                drift_class: RerunDriftClass::NoDrift,
                original_identity_ref: "provider-entry:local_native:0001".to_owned(),
                current_identity_ref: "provider-entry:local_native:0001".to_owned(),
                original_label: "Aureline native local model".to_owned(),
                current_label: "Aureline native local model".to_owned(),
                summary_label: "Provider lifecycle unchanged.".to_owned(),
            },
            RerunDriftRow {
                axis_class: RerunDriftAxisClass::ModelLifecycle,
                drift_class: RerunDriftClass::NoDrift,
                original_identity_ref: "model-entry:local_native:fast:0001".to_owned(),
                current_identity_ref: "model-entry:local_native:fast:0001".to_owned(),
                original_label: "Local fast model".to_owned(),
                current_label: "Local fast model".to_owned(),
                summary_label: "Model lifecycle unchanged.".to_owned(),
            },
            RerunDriftRow {
                axis_class: RerunDriftAxisClass::ToolAvailability,
                drift_class: RerunDriftClass::NoDrift,
                original_identity_ref: "tool-gateway-descriptor:native-fs-snapshot:0001".to_owned(),
                current_identity_ref: "tool-gateway-descriptor:native-fs-snapshot:0001".to_owned(),
                original_label: "Native fs snapshot tool admitted.".to_owned(),
                current_label: "Native fs snapshot tool admitted.".to_owned(),
                summary_label: "Tool availability unchanged.".to_owned(),
            },
        ],
        approval_resolution: RerunApprovalResolution {
            resolution_class: RerunApprovalResolutionClass::AllRequiredFreshlyResolved,
            required_approval_refs: vec!["approval-ticket:applied:local:0001".to_owned()],
            freshly_resolved_approval_refs: vec![
                "approval-ticket:applied:local:0001:rerun".to_owned()
            ],
            missing_approval_refs: Vec::new(),
            expired_approval_refs: Vec::new(),
            summary_label: "Approval freshly re-resolved against the current policy epoch."
                .to_owned(),
        },
        action_offers: vec![
            RerunActionOffer::Rerun,
            RerunActionOffer::Cancel,
            RerunActionOffer::OpenAsRecipe,
            RerunActionOffer::OpenOriginalEvidence,
            RerunActionOffer::OpenOriginalThread,
        ],
        rerun_admission_class: RerunAdmissionClass::AdmitRerun,
        rerun_denied_reason_class: None,
        rerun_denied_reason_note: None,
        policy_context: policy_context(),
        redaction_class: AiRunHistoryRedactionClass::MetadataSafeDefault,
        minted_at: "2026-05-18T00:00:10Z".to_owned(),
    }
}

fn deny_rerun_review_for_remote() -> AiRerunReview {
    AiRerunReview {
        record_kind: AI_RERUN_REVIEW_RECORD_KIND.to_owned(),
        schema_version: AI_RUN_HISTORY_SCHEMA_VERSION,
        rerun_review_id: "rerun-review:denied:remote:0001".to_owned(),
        canonical_run_id: "ai-run:denied:remote:0001".to_owned(),
        original_run_entry_ref: "ai-run:denied:remote:0001".to_owned(),
        drift_rows: vec![
            RerunDriftRow {
                axis_class: RerunDriftAxisClass::WorkspaceRevision,
                drift_class: RerunDriftClass::NoDrift,
                original_identity_ref: "workspace-revision:0001".to_owned(),
                current_identity_ref: "workspace-revision:0001".to_owned(),
                original_label: "Workspace revision 0001".to_owned(),
                current_label: "Workspace revision 0001".to_owned(),
                summary_label: "Workspace revision unchanged.".to_owned(),
            },
            RerunDriftRow {
                axis_class: RerunDriftAxisClass::PolicyEpoch,
                drift_class: RerunDriftClass::NoDrift,
                original_identity_ref: "policy-epoch:0042".to_owned(),
                current_identity_ref: "policy-epoch:0042".to_owned(),
                original_label: "Policy epoch 0042".to_owned(),
                current_label: "Policy epoch 0042".to_owned(),
                summary_label: "Policy epoch unchanged.".to_owned(),
            },
            RerunDriftRow {
                axis_class: RerunDriftAxisClass::ProviderLifecycle,
                drift_class: RerunDriftClass::NoDrift,
                original_identity_ref: "provider-entry:remote_vendor_hosted:0001".to_owned(),
                current_identity_ref: "provider-entry:remote_vendor_hosted:0001".to_owned(),
                original_label: "Remote vendor hosted provider".to_owned(),
                current_label: "Remote vendor hosted provider".to_owned(),
                summary_label: "Provider lifecycle unchanged.".to_owned(),
            },
            RerunDriftRow {
                axis_class: RerunDriftAxisClass::ModelLifecycle,
                drift_class: RerunDriftClass::NoDrift,
                original_identity_ref:
                    "model-entry:remote_vendor_hosted:reasoning:0001".to_owned(),
                current_identity_ref:
                    "model-entry:remote_vendor_hosted:reasoning:0001".to_owned(),
                original_label: "Remote reasoning model".to_owned(),
                current_label: "Remote reasoning model".to_owned(),
                summary_label: "Model lifecycle unchanged.".to_owned(),
            },
            RerunDriftRow {
                axis_class: RerunDriftAxisClass::ToolAvailability,
                drift_class: RerunDriftClass::NoDrift,
                original_identity_ref:
                    "tool-gateway-descriptor:remote-issues:0001".to_owned(),
                current_identity_ref:
                    "tool-gateway-descriptor:remote-issues:0001".to_owned(),
                original_label: "Enterprise gateway issues tool admitted.".to_owned(),
                current_label: "Enterprise gateway issues tool admitted.".to_owned(),
                summary_label: "Tool availability unchanged.".to_owned(),
            },
            RerunDriftRow {
                axis_class: RerunDriftAxisClass::ApprovalValidity,
                drift_class: RerunDriftClass::MaterialDrift,
                original_identity_ref: "approval-ticket:denied:remote:0001".to_owned(),
                current_identity_ref: String::new(),
                original_label: "Admin approval was never granted.".to_owned(),
                current_label: "Admin approval still missing in current policy epoch.".to_owned(),
                summary_label: "Approval not yet granted; rerun cannot inherit absent approval."
                    .to_owned(),
            },
        ],
        approval_resolution: RerunApprovalResolution {
            resolution_class: RerunApprovalResolutionClass::MissingRequiredApproval,
            required_approval_refs: vec!["approval-ticket:denied:remote:0001".to_owned()],
            freshly_resolved_approval_refs: Vec::new(),
            missing_approval_refs: vec!["approval-ticket:denied:remote:0001".to_owned()],
            expired_approval_refs: Vec::new(),
            summary_label: "Admin ticket for issues connector is still missing.".to_owned(),
        },
        action_offers: vec![
            RerunActionOffer::Cancel,
            RerunActionOffer::OpenAsRecipe,
            RerunActionOffer::OpenOriginalEvidence,
            RerunActionOffer::RequestApprovalRenewal,
        ],
        rerun_admission_class: RerunAdmissionClass::DenyRerunApprovalUnresolved,
        rerun_denied_reason_class: Some(RerunDeniedReasonClass::ApprovalMissing),
        rerun_denied_reason_note: Some(
            "Admin ticket for the issues connector is still missing; cannot inherit absent approval."
                .to_owned(),
        ),
        policy_context: policy_context(),
        redaction_class: AiRunHistoryRedactionClass::MetadataSafeDefault,
        minted_at: "2026-05-18T01:00:10Z".to_owned(),
    }
}

fn deny_rerun_review_for_branch_agent() -> AiRerunReview {
    AiRerunReview {
        record_kind: AI_RERUN_REVIEW_RECORD_KIND.to_owned(),
        schema_version: AI_RUN_HISTORY_SCHEMA_VERSION,
        rerun_review_id: "rerun-review:revoked:branch-agent:0001".to_owned(),
        canonical_run_id: "ai-run:revoked:branch-agent:0001".to_owned(),
        original_run_entry_ref: "ai-run:revoked:branch-agent:0001".to_owned(),
        drift_rows: vec![
            RerunDriftRow {
                axis_class: RerunDriftAxisClass::WorkspaceRevision,
                drift_class: RerunDriftClass::MinorDrift,
                original_identity_ref: "workspace-revision:0001".to_owned(),
                current_identity_ref: "workspace-revision:0002".to_owned(),
                original_label: "Workspace revision 0001".to_owned(),
                current_label: "Workspace revision 0002".to_owned(),
                summary_label:
                    "Workspace advanced by one revision; minor drift recorded.".to_owned(),
            },
            RerunDriftRow {
                axis_class: RerunDriftAxisClass::PolicyEpoch,
                drift_class: RerunDriftClass::MinorDrift,
                original_identity_ref: "policy-epoch:0041".to_owned(),
                current_identity_ref: "policy-epoch:0042".to_owned(),
                original_label: "Policy epoch 0041".to_owned(),
                current_label: "Policy epoch 0042".to_owned(),
                summary_label: "Policy epoch advanced; minor drift recorded.".to_owned(),
            },
            RerunDriftRow {
                axis_class: RerunDriftAxisClass::ProviderLifecycle,
                drift_class: RerunDriftClass::RemovedOrWithdrawn,
                original_identity_ref: "provider-entry:remote_vendor_hosted:0002".to_owned(),
                current_identity_ref: String::new(),
                original_label: "Withdrawn remote vendor".to_owned(),
                current_label: String::new(),
                summary_label: "Original provider was withdrawn from the registry.".to_owned(),
            },
            RerunDriftRow {
                axis_class: RerunDriftAxisClass::ModelLifecycle,
                drift_class: RerunDriftClass::RemovedOrWithdrawn,
                original_identity_ref:
                    "model-entry:remote_vendor_hosted:reasoning:0002".to_owned(),
                current_identity_ref: String::new(),
                original_label: "Withdrawn remote model".to_owned(),
                current_label: String::new(),
                summary_label: "Original model revision is no longer available.".to_owned(),
            },
            RerunDriftRow {
                axis_class: RerunDriftAxisClass::ToolAvailability,
                drift_class: RerunDriftClass::NoDrift,
                original_identity_ref:
                    "tool-gateway-descriptor:native-fs-snapshot:0001".to_owned(),
                current_identity_ref:
                    "tool-gateway-descriptor:native-fs-snapshot:0001".to_owned(),
                original_label: "Native fs snapshot tool admitted.".to_owned(),
                current_label: "Native fs snapshot tool admitted.".to_owned(),
                summary_label: "Tool availability unchanged.".to_owned(),
            },
        ],
        approval_resolution: RerunApprovalResolution {
            resolution_class: RerunApprovalResolutionClass::ExpiredRequiredApproval,
            required_approval_refs: vec!["approval-ticket:revoked:branch-agent:0001".to_owned()],
            freshly_resolved_approval_refs: Vec::new(),
            missing_approval_refs: Vec::new(),
            expired_approval_refs: vec!["approval-ticket:revoked:branch-agent:0001".to_owned()],
            summary_label:
                "Original admin ticket expired with the previous policy epoch.".to_owned(),
        },
        action_offers: vec![
            RerunActionOffer::Cancel,
            RerunActionOffer::OpenAsRecipe,
            RerunActionOffer::OpenOriginalEvidence,
            RerunActionOffer::OpenOriginalThread,
        ],
        rerun_admission_class: RerunAdmissionClass::DenyRerunProviderUnavailable,
        rerun_denied_reason_class: Some(RerunDeniedReasonClass::ProviderWithdrawn),
        rerun_denied_reason_note: Some(
            "Original provider was withdrawn and replay is degraded; rerun cannot inherit removed authority."
                .to_owned(),
        ),
        policy_context: policy_context(),
        redaction_class: AiRunHistoryRedactionClass::MetadataSafeDefault,
        minted_at: "2026-05-18T02:00:10Z".to_owned(),
    }
}

fn surface_rows(
    canonical_run_ids: Vec<String>,
    rerun_review_refs: Vec<String>,
) -> Vec<AiRunHistorySurfaceRow> {
    vec![
        AiRunHistorySurfaceRow::new(
            AiRunHistorySurfaceClass::AiThread,
            "projection:ai-run-history:thread:0001",
            canonical_run_ids.clone(),
            rerun_review_refs.clone(),
        ),
        AiRunHistorySurfaceRow::new(
            AiRunHistorySurfaceClass::EvidencePanel,
            "projection:ai-run-history:evidence-panel:0001",
            canonical_run_ids.clone(),
            rerun_review_refs.clone(),
        ),
        AiRunHistorySurfaceRow::new(
            AiRunHistorySurfaceClass::SupportPacket,
            "projection:ai-run-history:support-packet:0001",
            canonical_run_ids.clone(),
            rerun_review_refs.clone(),
        ),
        AiRunHistorySurfaceRow::new(
            AiRunHistorySurfaceClass::ReplayView,
            "projection:ai-run-history:replay-view:0001",
            canonical_run_ids,
            rerun_review_refs,
        ),
    ]
}

fn build_packet() -> AiRunHistoryParityPacket {
    let entries = vec![
        applied_local_entry(),
        denied_remote_entry(),
        revoked_branch_agent_entry(),
    ];
    let rerun_reviews = vec![
        admit_rerun_review(),
        deny_rerun_review_for_remote(),
        deny_rerun_review_for_branch_agent(),
    ];
    let canonical_run_ids: Vec<String> = entries
        .iter()
        .map(|entry| entry.canonical_run_id.clone())
        .collect();
    let rerun_review_refs: Vec<String> = rerun_reviews
        .iter()
        .map(|rerun| rerun.rerun_review_id.clone())
        .collect();
    AiRunHistoryParityPacket::new(AiRunHistoryParityPacketInput {
        packet_id: "ai-run-history-parity:m3:0001".to_owned(),
        display_label: "AI run history M3 parity".to_owned(),
        entries,
        rerun_reviews,
        surface_rows: surface_rows(canonical_run_ids, rerun_review_refs),
        source_contract_refs: vec![
            AI_RUN_HISTORY_ENTRY_SCHEMA_REF.to_owned(),
            AI_RERUN_REVIEW_SCHEMA_REF.to_owned(),
            AI_RUN_HISTORY_PARITY_ARTIFACT_REF.to_owned(),
        ],
        policy_context: policy_context(),
        minted_at: "2026-05-18T00:00:00Z".to_owned(),
    })
}

#[test]
fn canonical_packet_validates_clean() {
    let packet = build_packet();
    let violations = packet.validate();
    assert!(violations.is_empty(), "{violations:?}");
}

#[test]
fn support_packet_preserves_canonical_run_ids() {
    let packet = build_packet();
    let support = packet.support_packet();
    let canonical_ids: Vec<String> = packet
        .entries
        .iter()
        .map(|entry| entry.canonical_run_id.clone())
        .collect();
    let support_ids: Vec<String> = support
        .entry_rows
        .iter()
        .map(|row| row.canonical_run_id.clone())
        .collect();
    assert_eq!(canonical_ids, support_ids);
    assert!(support
        .entry_rows
        .iter()
        .all(|row| !row.evidence_packet_ref.is_empty()));
}

#[test]
fn export_excludes_raw_boundary_material() {
    let packet = build_packet();
    let json = packet.export_safe_json();
    assert!(!json.contains("://"));
    assert!(!json.to_ascii_lowercase().contains("api_key="));
    assert!(!json.to_ascii_lowercase().contains("oauth_token"));
}

#[test]
fn markdown_summary_reports_outcome_approval_completeness_and_admission_coverage() {
    let packet = build_packet();
    let markdown = packet.render_markdown_summary();
    assert!(markdown.starts_with("# AI Run History Parity Report"));
    assert!(markdown.contains("succeeded_with_local_reversible_edit"));
    assert!(markdown.contains("denied_by_approval_missing"));
    assert!(markdown.contains("succeeded_with_branch_agent_dispatch"));
    assert!(markdown.contains("granted"));
    assert!(markdown.contains("revoked"));
    assert!(markdown.contains("blocked_by_policy"));
    assert!(markdown.contains("evidence_incomplete_degraded_replay"));
    assert!(markdown.contains("admit_rerun"));
    assert!(markdown.contains("deny_rerun_approval_unresolved"));
    assert!(markdown.contains("deny_rerun_provider_unavailable"));
}

#[test]
fn approval_timeline_must_preserve_each_event_not_just_final_status() {
    let packet = build_packet();
    let revoked_entry = packet
        .entries
        .iter()
        .find(|entry| entry.canonical_run_id == "ai-run:revoked:branch-agent:0001")
        .expect("revoked entry exists");
    let decisions: Vec<&'static str> = revoked_entry
        .approval_timeline
        .iter()
        .map(|event| event.decision_class.as_str())
        .collect();
    assert_eq!(decisions, vec!["granted", "revoked"]);
    assert!(revoked_entry.has_granted_approval());
    assert!(revoked_entry.has_revoked_or_expired_approval());
}

#[test]
fn revoked_event_without_note_is_rejected() {
    let mut packet = build_packet();
    let entry = packet
        .entries
        .iter_mut()
        .find(|entry| entry.canonical_run_id == "ai-run:revoked:branch-agent:0001")
        .expect("revoked entry exists");
    let revoked_event = entry
        .approval_timeline
        .iter_mut()
        .find(|event| event.decision_class == ApprovalEventDecisionClass::Revoked)
        .expect("revoked event exists");
    revoked_event.revocation_note = None;
    let violations = packet.validate();
    assert!(violations.contains(&AiRunHistoryViolation::ApprovalEventRevokedWithoutNote));
}

#[test]
fn incomplete_evidence_must_carry_typed_reason_and_note() {
    let mut packet = build_packet();
    let entry = packet
        .entries
        .iter_mut()
        .find(|entry| entry.canonical_run_id == "ai-run:revoked:branch-agent:0001")
        .expect("revoked entry exists");
    entry.evidence_incompleteness_reason_class = None;
    entry.evidence_incompleteness_note = None;
    let violations = packet.validate();
    assert!(violations.contains(&AiRunHistoryViolation::EntryIncompleteWithoutReason));
}

#[test]
fn applied_entry_must_carry_a_granted_approval_event() {
    let mut packet = build_packet();
    let entry = packet
        .entries
        .iter_mut()
        .find(|entry| entry.canonical_run_id == "ai-run:applied:local:0001")
        .expect("applied entry exists");
    for event in entry.approval_timeline.iter_mut() {
        if event.decision_class == ApprovalEventDecisionClass::Granted {
            event.decision_class = ApprovalEventDecisionClass::PendingUserReview;
        }
    }
    let violations = packet.validate();
    assert!(violations.contains(&AiRunHistoryViolation::EntryAppliedWithoutGrantedApproval));
}

#[test]
fn terminal_entry_must_carry_completed_at() {
    let mut packet = build_packet();
    let entry = packet
        .entries
        .iter_mut()
        .find(|entry| entry.canonical_run_id == "ai-run:applied:local:0001")
        .expect("applied entry exists");
    entry.completed_at = None;
    let violations = packet.validate();
    assert!(violations.contains(&AiRunHistoryViolation::EntryTerminalWithoutCompletedAt));
}

#[test]
fn rerun_must_offer_open_as_recipe() {
    let mut packet = build_packet();
    let rerun = packet
        .rerun_reviews
        .iter_mut()
        .find(|rerun| rerun.rerun_review_id == "rerun-review:applied:local:0001")
        .expect("admit rerun review exists");
    rerun
        .action_offers
        .retain(|offer| !matches!(offer, RerunActionOffer::OpenAsRecipe));
    let violations = packet.validate();
    assert!(violations.contains(&AiRunHistoryViolation::RerunMissingOpenAsRecipeOffer));
}

#[test]
fn rerun_must_not_admit_with_blocking_drift() {
    let mut packet = build_packet();
    let rerun = packet
        .rerun_reviews
        .iter_mut()
        .find(|rerun| rerun.rerun_review_id == "rerun-review:revoked:branch-agent:0001")
        .expect("denial rerun review exists");
    rerun.rerun_admission_class = RerunAdmissionClass::AdmitRerun;
    rerun.rerun_denied_reason_class = None;
    rerun.rerun_denied_reason_note = None;
    let violations = packet.validate();
    assert!(violations.contains(&AiRunHistoryViolation::RerunAdmitWithBlockingDrift));
}

#[test]
fn rerun_must_not_admit_with_unresolved_approval() {
    let mut packet = build_packet();
    let rerun = packet
        .rerun_reviews
        .iter_mut()
        .find(|rerun| rerun.rerun_review_id == "rerun-review:denied:remote:0001")
        .expect("denial rerun review exists");
    rerun.rerun_admission_class = RerunAdmissionClass::AdmitRerun;
    rerun.rerun_denied_reason_class = None;
    rerun.rerun_denied_reason_note = None;
    let violations = packet.validate();
    assert!(violations.contains(&AiRunHistoryViolation::RerunAdmitWithUnresolvedApproval));
}

#[test]
fn rerun_must_keep_canonical_run_id_aligned_with_original_entry() {
    let mut packet = build_packet();
    let rerun = packet
        .rerun_reviews
        .iter_mut()
        .find(|rerun| rerun.rerun_review_id == "rerun-review:applied:local:0001")
        .expect("admit rerun review exists");
    rerun.canonical_run_id = "ai-run:not-on-packet".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&AiRunHistoryViolation::RerunReferencesUnknownEntry));
}

#[test]
fn surface_must_preserve_canonical_run_ids() {
    let mut packet = build_packet();
    let surface_row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.surface_class == AiRunHistorySurfaceClass::EvidencePanel)
        .expect("evidence panel surface row exists");
    surface_row.preserves_canonical_run_id = false;
    let violations = packet.validate();
    assert!(violations.contains(&AiRunHistoryViolation::SurfaceProjectionDrift));
}

#[test]
fn surface_rejects_unknown_canonical_run_id() {
    let mut packet = build_packet();
    let surface_row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.surface_class == AiRunHistorySurfaceClass::ReplayView)
        .expect("replay view surface row exists");
    surface_row
        .canonical_run_ids
        .push("ai-run:not-on-packet".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&AiRunHistoryViolation::SurfaceMissingCanonicalRunId));
}

#[test]
fn missing_required_surface_is_rejected() {
    let mut packet = build_packet();
    packet
        .surface_rows
        .retain(|row| row.surface_class != AiRunHistorySurfaceClass::SupportPacket);
    let violations = packet.validate();
    assert!(violations.contains(&AiRunHistoryViolation::MissingSurfaceProjection));
}

#[test]
fn checked_in_fixture_matches_canonical_packet() {
    let packet = current_beta_ai_run_history_parity_packet()
        .expect("checked-in ai run history parity fixture validates");
    assert_eq!(packet.packet_id, "ai-run-history-parity:m3:0001");
    let canonical = build_packet();
    assert_eq!(packet, canonical);
}

#[test]
fn rerun_outcomes_cover_admit_drift_and_approval_denials() {
    let packet = build_packet();
    let admissions: Vec<&'static str> = packet
        .rerun_reviews
        .iter()
        .map(|rerun| rerun.rerun_admission_class.as_str())
        .collect();
    assert!(admissions.contains(&"admit_rerun"));
    assert!(admissions.contains(&"deny_rerun_approval_unresolved"));
    assert!(admissions.contains(&"deny_rerun_provider_unavailable"));
}
