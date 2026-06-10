use super::*;

const PACKET_ID: &str = "m5-ai-workflow-matrix:stable:0001";

fn lane_rows() -> Vec<M5AiWorkflowMatrixLaneRow> {
    vec![
        M5AiWorkflowMatrixLaneRow {
            lane: M5AiWorkflowLane::InlineAssist,
            qualification: M5AiWorkflowQualificationClass::Stable,
            scope_summary: "Composer inline assist with bounded scoped-apply, preview/approval, and revert".to_owned(),
            evidence_requirement: M5AiWorkflowEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:prompt-composer-conformance:m5".to_owned(),
                "evidence:ai-scoped-apply-hardening:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5AiWorkflowDowngradeTrigger::ProofStale,
                M5AiWorkflowDowngradeTrigger::TrustNarrowing,
                M5AiWorkflowDowngradeTrigger::ScopeExpansionUnqualified,
            ],
            rollback_posture: M5AiWorkflowRollbackPosture::FullyReversible,
            source_contract_refs: vec![
                M5_AI_WORKFLOW_MATRIX_INLINE_ASSIST_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5AiWorkflowConsumerSurface::DesktopComposer,
                M5AiWorkflowConsumerSurface::CliHeadless,
                M5AiWorkflowConsumerSurface::SupportExport,
            ],
        },
        M5AiWorkflowMatrixLaneRow {
            lane: M5AiWorkflowLane::PatchReview,
            qualification: M5AiWorkflowQualificationClass::Stable,
            scope_summary: "AI review-assist findings, publish-to-review sheets, and resolution memory with scoped diff analysis".to_owned(),
            evidence_requirement: M5AiWorkflowEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:ai-review-assist-truth:m5".to_owned(),
                "evidence:patch-review-summary:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5AiWorkflowDowngradeTrigger::ProofStale,
                M5AiWorkflowDowngradeTrigger::ProviderUnavailable,
                M5AiWorkflowDowngradeTrigger::PolicyBlocked,
            ],
            rollback_posture: M5AiWorkflowRollbackPosture::EvidencePreservedNoRevert,
            source_contract_refs: vec![
                M5_AI_WORKFLOW_MATRIX_PATCH_REVIEW_CONTRACT_REF.to_owned(),
                M5_AI_WORKFLOW_MATRIX_PATCH_SEQUENCE_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5AiWorkflowConsumerSurface::DesktopReviewWorkspace,
                M5AiWorkflowConsumerSurface::BrowserCompanion,
                M5AiWorkflowConsumerSurface::SupportExport,
            ],
        },
        M5AiWorkflowMatrixLaneRow {
            lane: M5AiWorkflowLane::BranchOrWorktreeAgent,
            qualification: M5AiWorkflowQualificationClass::Beta,
            scope_summary: "Background branch-agent lifecycle with isolated worktrees, checkpoints, operator takeover, and completion review".to_owned(),
            evidence_requirement: M5AiWorkflowEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:background-branch-agent-lifecycle:m4".to_owned(),
            ],
            downgrade_triggers: vec![
                M5AiWorkflowDowngradeTrigger::ProofStale,
                M5AiWorkflowDowngradeTrigger::TrustNarrowing,
                M5AiWorkflowDowngradeTrigger::UpstreamDependencyNarrowed,
                M5AiWorkflowDowngradeTrigger::ScopeExpansionUnqualified,
            ],
            rollback_posture: M5AiWorkflowRollbackPosture::CheckpointReversible,
            source_contract_refs: vec![
                M5_AI_WORKFLOW_MATRIX_BRANCH_AGENT_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5AiWorkflowConsumerSurface::DesktopComposer,
                M5AiWorkflowConsumerSurface::DesktopReviewWorkspace,
                M5AiWorkflowConsumerSurface::CliHeadless,
                M5AiWorkflowConsumerSurface::SupportExport,
            ],
        },
    ]
}

fn security_review() -> M5AiWorkflowMatrixSecurityReview {
    M5AiWorkflowMatrixSecurityReview {
        no_self_approved_mutating_tools: true,
        no_worktree_isolation_bypass: true,
        preview_approval_required_before_apply: true,
        evidence_packets_cite_source_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_proof_blocks_promotion: true,
    }
}

fn consumer_projection() -> M5AiWorkflowMatrixConsumerProjection {
    M5AiWorkflowMatrixConsumerProjection {
        desktop_composer_shows_qualification: true,
        desktop_review_shows_qualification: true,
        cli_headless_shows_qualification: true,
        browser_companion_shows_qualification: true,
        support_export_shows_qualification: true,
        diagnostics_shows_qualification: true,
        preview_labs_label_for_unqualified_lanes: true,
    }
}

fn proof_freshness() -> M5AiWorkflowMatrixProofFreshness {
    M5AiWorkflowMatrixProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF.to_owned(),
        M5_AI_WORKFLOW_MATRIX_DOC_REF.to_owned(),
        M5_AI_WORKFLOW_MATRIX_INLINE_ASSIST_CONTRACT_REF.to_owned(),
        M5_AI_WORKFLOW_MATRIX_PATCH_REVIEW_CONTRACT_REF.to_owned(),
        M5_AI_WORKFLOW_MATRIX_PATCH_SEQUENCE_REF.to_owned(),
        M5_AI_WORKFLOW_MATRIX_BRANCH_AGENT_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> M5AiWorkflowMatrixPacket {
    M5AiWorkflowMatrixPacket::new(M5AiWorkflowMatrixPacketInput {
        packet_id: PACKET_ID.to_owned(),
        matrix_label: "M5 AI Workflow Matrix".to_owned(),
        lane_rows: lane_rows(),
        security_review: security_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

#[test]
fn m5_ai_workflow_matrix_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_lane_fails_validation() {
    let mut packet = packet();
    packet
        .lane_rows
        .retain(|row| row.lane != M5AiWorkflowLane::PatchReview);
    assert!(packet
        .validate()
        .contains(&M5AiWorkflowMatrixViolation::RequiredLaneMissing));
}

#[test]
fn stable_lane_missing_evidence_fails() {
    let mut packet = packet();
    packet.lane_rows[0].required_evidence_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AiWorkflowMatrixViolation::StableLaneMissingEvidence));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.lane_rows[1].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5AiWorkflowMatrixViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.lane_rows[2].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5AiWorkflowMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AiWorkflowMatrixViolation::MissingSourceContracts));
}

#[test]
fn security_review_incomplete_fails() {
    let mut packet = packet();
    packet.security_review.no_worktree_isolation_bypass = false;
    assert!(packet
        .validate()
        .contains(&M5AiWorkflowMatrixViolation::SecurityReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .preview_labs_label_for_unqualified_lanes = false;
    assert!(packet
        .validate()
        .contains(&M5AiWorkflowMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5AiWorkflowMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_ai_workflow_matrix_export()
        .expect("checked M5 AI workflow matrix export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}
