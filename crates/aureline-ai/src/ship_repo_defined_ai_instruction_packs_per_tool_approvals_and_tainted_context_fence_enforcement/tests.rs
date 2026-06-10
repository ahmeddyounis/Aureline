use super::*;

const PACKET_ID: &str = "ai-governance:m5:0001";
const GOVERNANCE_RUN_ID: &str = "ai-governance-run:m5:0001";

fn instruction_packs() -> RepoInstructionPackBlock {
    RepoInstructionPackBlock {
        pack_set_id: "instruction-pack-set:m5:0001".to_owned(),
        repo_only: true,
        no_repo_policy_widening: true,
        precedence_disclosed: true,
        pack_rows: vec![
            RepoInstructionPackRow {
                pack_id: "pack:m5:0001:repo-guardrails".to_owned(),
                source_class: InstructionPackSourceClass::RepoCommitted,
                trust_posture: InstructionPackTrustClass::TrustedRepo,
                scope_effect: InstructionPackScopeEffectClass::NarrowsPolicy,
                precedence_rank: 0,
                applied: true,
                widen_blocked: false,
                disclosed: true,
            },
            RepoInstructionPackRow {
                pack_id: "pack:m5:0001:local-overrides".to_owned(),
                source_class: InstructionPackSourceClass::RepoLocalUncommitted,
                trust_posture: InstructionPackTrustClass::Restricted,
                scope_effect: InstructionPackScopeEffectClass::Neutral,
                precedence_rank: 1,
                applied: true,
                widen_blocked: false,
                disclosed: true,
            },
            RepoInstructionPackRow {
                pack_id: "pack:m5:0001:imported-widening-attempt".to_owned(),
                source_class: InstructionPackSourceClass::WorkspaceShared,
                trust_posture: InstructionPackTrustClass::UntrustedImported,
                scope_effect: InstructionPackScopeEffectClass::AttemptsWiden,
                precedence_rank: 2,
                applied: false,
                widen_blocked: true,
                disclosed: true,
            },
        ],
    }
}

fn tool_approvals() -> PerToolApprovalBlock {
    PerToolApprovalBlock {
        approval_set_id: "tool-approval-set:m5:0001".to_owned(),
        no_side_effect_without_approval: true,
        first_use_review_required: true,
        approval_rows: vec![
            PerToolApprovalRow {
                tool_id: "tool:m5:0001:read-context".to_owned(),
                capability_class: PerToolCapabilityClass::ReadOnly,
                side_effect_class: PerToolSideEffectClass::None,
                approval_posture: PerToolApprovalPostureClass::PreApprovedReadOnly,
                approved: true,
                approval_actor_class: PerToolApprovalActorClass::RepoPolicy,
                requires_human_first_use: false,
                disclosed: true,
            },
            PerToolApprovalRow {
                tool_id: "tool:m5:0001:apply-edit".to_owned(),
                capability_class: PerToolCapabilityClass::WorkspaceWrite,
                side_effect_class: PerToolSideEffectClass::LocalMutation,
                approval_posture: PerToolApprovalPostureClass::FirstUseReview,
                approved: true,
                approval_actor_class: PerToolApprovalActorClass::HumanOperator,
                requires_human_first_use: true,
                disclosed: true,
            },
            PerToolApprovalRow {
                tool_id: "tool:m5:0001:remote-call".to_owned(),
                capability_class: PerToolCapabilityClass::ExternalService,
                side_effect_class: PerToolSideEffectClass::NetworkCall,
                approval_posture: PerToolApprovalPostureClass::Denied,
                approved: false,
                approval_actor_class: PerToolApprovalActorClass::Unattributed,
                requires_human_first_use: true,
                disclosed: true,
            },
        ],
    }
}

fn context_fences() -> ContextFenceBlock {
    ContextFenceBlock {
        fence_set_id: "context-fence-set:m5:0001".to_owned(),
        all_tainted_fenced: true,
        no_fence_bypass: true,
        fence_rows: vec![
            ContextFenceRow {
                fence_id: "fence:m5:0001:imported-spec".to_owned(),
                source_class: ContextFenceSourceClass::ImportedExternal,
                enforcement: ContextFenceEnforcementClass::Quarantined,
                usage_constraint: ContextFenceUsageConstraintClass::NoPolicyWidening,
                widening_blocked: true,
                auto_approval_blocked: true,
                disclosed: true,
            },
            ContextFenceRow {
                fence_id: "fence:m5:0001:tool-output".to_owned(),
                source_class: ContextFenceSourceClass::ToolOutput,
                enforcement: ContextFenceEnforcementClass::DowngradedToAdvisory,
                usage_constraint: ContextFenceUsageConstraintClass::NoToolAutoApproval,
                widening_blocked: true,
                auto_approval_blocked: true,
                disclosed: true,
            },
            ContextFenceRow {
                fence_id: "fence:m5:0001:web-fetch".to_owned(),
                source_class: ContextFenceSourceClass::WebFetch,
                enforcement: ContextFenceEnforcementClass::Blocked,
                usage_constraint: ContextFenceUsageConstraintClass::DisplayOnly,
                widening_blocked: true,
                auto_approval_blocked: true,
                disclosed: true,
            },
        ],
    }
}

fn consumer_surface_parity() -> Vec<GovernanceSurfaceParityRow> {
    GovernanceConsumerSurface::ALL
        .into_iter()
        .map(|surface| GovernanceSurfaceParityRow {
            surface,
            shows_instruction_packs: true,
            shows_tool_approvals: true,
            shows_fences: true,
            reachable: true,
            qualification: GovernanceSurfaceQualificationClass::Stable,
            claimed_stable: true,
        })
        .collect()
}

fn downgrade_triggers() -> Vec<GovernanceDowngradeTrigger> {
    vec![
        GovernanceDowngradeTrigger::ProofStale,
        GovernanceDowngradeTrigger::PolicyBlocked,
        GovernanceDowngradeTrigger::ProviderUnavailable,
        GovernanceDowngradeTrigger::TrustNarrowing,
        GovernanceDowngradeTrigger::ScopeExpansionUnqualified,
        GovernanceDowngradeTrigger::UpstreamDependencyNarrowed,
        GovernanceDowngradeTrigger::RepoInstructionWidenedPolicy,
        GovernanceDowngradeTrigger::ToolSideEffectUnapproved,
        GovernanceDowngradeTrigger::TaintedContextBypassedFence,
        GovernanceDowngradeTrigger::ToolApprovalGrantedByTaintedContext,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        REPO_AI_GOVERNANCE_DOC_REF.to_owned(),
        REPO_AI_GOVERNANCE_SCHEMA_REF.to_owned(),
        REPO_AI_GOVERNANCE_CONTEXT_ASSEMBLY_CONTRACT_REF.to_owned(),
        REPO_AI_GOVERNANCE_REPO_INSTRUCTION_CONTRACT_REF.to_owned(),
        REPO_AI_GOVERNANCE_TAINT_CONTRACT_REF.to_owned(),
        REPO_AI_GOVERNANCE_FENCE_CONTRACT_REF.to_owned(),
        REPO_AI_GOVERNANCE_TOOL_REGISTRY_CONTRACT_REF.to_owned(),
        REPO_AI_GOVERNANCE_M5_MATRIX_CONTRACT_REF.to_owned(),
    ]
}

fn packet_input() -> RepoAiInstructionToolApprovalFencePacketInput {
    RepoAiInstructionToolApprovalFencePacketInput {
        packet_id: PACKET_ID.to_owned(),
        governance_run_id: GOVERNANCE_RUN_ID.to_owned(),
        display_label: "M5 repo-AI instruction, tool-approval, and fence governance run".to_owned(),
        trust_state_token: "restricted".to_owned(),
        policy_epoch_ref: "policy-epoch:m5:2026-06-01".to_owned(),
        fence_enforcement_interlocked: true,
        instruction_packs: instruction_packs(),
        tool_approvals: tool_approvals(),
        context_fences: context_fences(),
        consumer_surface_parity: consumer_surface_parity(),
        downgrade_triggers: downgrade_triggers(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-10T11:28:00Z".to_owned(),
    }
}

fn packet() -> RepoAiInstructionToolApprovalFencePacket {
    RepoAiInstructionToolApprovalFencePacket::new(packet_input())
}

#[test]
fn packet_constructs_and_serializes() {
    let packet = packet();
    assert_eq!(packet.record_kind, REPO_AI_GOVERNANCE_RECORD_KIND);
    assert_eq!(packet.schema_version, REPO_AI_GOVERNANCE_SCHEMA_VERSION);
    let json = packet.export_safe_json();
    assert!(json.contains("instruction_packs"));
}

#[test]
fn valid_packet_passes_validation() {
    assert!(packet().validate().is_empty());
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::WrongRecordKind));
}

#[test]
fn wrong_schema_version_fails() {
    let mut packet = packet();
    packet.schema_version = 99;
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::WrongSchemaVersion));
}

#[test]
fn missing_identity_fails() {
    let mut packet = packet();
    packet.packet_id = "  ".to_owned();
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::MissingIdentity));
}

#[test]
fn interlock_not_enforced_fails() {
    let mut packet = packet();
    packet.fence_enforcement_interlocked = false;
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::InterlockNotEnforced));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.pop();
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::MissingSourceContracts));
}

#[test]
fn instruction_pack_set_empty_fails() {
    let mut packet = packet();
    packet.instruction_packs.pack_rows.clear();
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::InstructionPackSetEmpty));
}

#[test]
fn non_repo_instruction_source_fails() {
    let mut packet = packet();
    packet.instruction_packs.repo_only = false;
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::NonRepoInstructionSource));
}

#[test]
fn instruction_precedence_not_disclosed_fails() {
    let mut packet = packet();
    packet.instruction_packs.precedence_disclosed = false;
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::InstructionPrecedenceNotDisclosed));
}

#[test]
fn hidden_instruction_pack_fails() {
    let mut packet = packet();
    packet.instruction_packs.pack_rows[0].disclosed = false;
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::HiddenInstructionPack));
}

#[test]
fn repo_instruction_widened_policy_block_flag_fails() {
    let mut packet = packet();
    packet.instruction_packs.no_repo_policy_widening = false;
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::RepoInstructionWidenedPolicy));
}

#[test]
fn repo_instruction_widening_applied_fails() {
    let mut packet = packet();
    let row = packet
        .instruction_packs
        .pack_rows
        .iter_mut()
        .find(|row| row.scope_effect.attempts_widen())
        .expect("widening pack present");
    row.applied = true;
    row.widen_blocked = false;
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::RepoInstructionWidenedPolicy));
}

#[test]
fn tool_approval_set_empty_fails() {
    let mut packet = packet();
    packet.tool_approvals.approval_rows.clear();
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::ToolApprovalSetEmpty));
}

#[test]
fn hidden_tool_approval_fails() {
    let mut packet = packet();
    packet.tool_approvals.approval_rows[0].disclosed = false;
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::HiddenToolApproval));
}

#[test]
fn denied_tool_approved_fails() {
    let mut packet = packet();
    let row = packet
        .tool_approvals
        .approval_rows
        .iter_mut()
        .find(|row| row.approval_posture.is_denied())
        .expect("denied tool present");
    row.approved = true;
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::DeniedToolApproved));
}

#[test]
fn side_effect_tool_without_human_approval_fails() {
    let mut packet = packet();
    let row = packet
        .tool_approvals
        .approval_rows
        .iter_mut()
        .find(|row| row.side_effect_class.has_side_effect() && row.approved)
        .expect("approved side-effecting tool present");
    row.approval_actor_class = PerToolApprovalActorClass::RepoPolicy;
    assert!(packet.validate().contains(
        &RepoAiInstructionToolApprovalFenceViolation::SideEffectToolWithoutHumanApproval
    ));
}

#[test]
fn tool_side_effect_unapproved_block_flag_fails() {
    let mut packet = packet();
    packet.tool_approvals.no_side_effect_without_approval = false;
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::ToolSideEffectUnapproved));
}

#[test]
fn first_use_review_not_required_fails() {
    let mut packet = packet();
    packet.tool_approvals.first_use_review_required = false;
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::FirstUseReviewNotRequired));
}

#[test]
fn fence_set_empty_fails() {
    let mut packet = packet();
    packet.context_fences.fence_rows.clear();
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::FenceSetEmpty));
}

#[test]
fn tainted_context_unfenced_fails() {
    let mut packet = packet();
    packet.context_fences.all_tainted_fenced = false;
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::TaintedContextUnfenced));
}

#[test]
fn fence_bypassed_fails() {
    let mut packet = packet();
    packet.context_fences.no_fence_bypass = false;
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::FenceBypassed));
}

#[test]
fn hidden_fence_fails() {
    let mut packet = packet();
    packet.context_fences.fence_rows[0].disclosed = false;
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::HiddenFence));
}

#[test]
fn tainted_context_widened_policy_fails() {
    let mut packet = packet();
    packet.context_fences.fence_rows[0].widening_blocked = false;
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::TaintedContextWidenedPolicy));
}

#[test]
fn tainted_context_granted_tool_approval_fails() {
    let mut packet = packet();
    packet.context_fences.fence_rows[0].auto_approval_blocked = false;
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::TaintedContextGrantedToolApproval));
}

#[test]
fn consumer_surface_coverage_missing_fails() {
    let mut packet = packet();
    packet.consumer_surface_parity.pop();
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::ConsumerSurfaceCoverageMissing));
}

#[test]
fn stable_claim_not_qualified_fails() {
    let mut packet = packet();
    packet.consumer_surface_parity[0].reachable = false;
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::StableClaimNotQualified));
}

#[test]
fn raw_boundary_material_detected() {
    let mut packet = packet();
    packet.instruction_packs.pack_rows[0].pack_id = "https://evil.example/pack".to_owned();
    assert!(packet
        .validate()
        .contains(&RepoAiInstructionToolApprovalFenceViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_renders() {
    let packet = packet();
    let md = packet.render_markdown_summary();
    assert!(md.contains("Instruction packs"));
    assert!(md.contains("Tool approvals"));
    assert!(md.contains("Context fences"));
}

#[test]
fn checked_in_export_loads_and_validates() {
    let result = current_stable_repo_ai_instruction_tool_approval_fence_export();
    assert!(
        result.is_ok(),
        "checked-in export must validate: {result:?}"
    );
}
