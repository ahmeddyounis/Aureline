//! Canonical seed builders for the M5 AI high-friction-approval-sheet /
//! tool-call-timeline-row primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical approval-sheet / tool-call-timeline-row packet.
pub const M5_AI_APPROVAL_TOOL_CALL_PRIMITIVE_PACKET_ID: &str =
    "m5-ai-high-friction-approval-sheet-tool-call-timeline-row-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked approval-sheet resolution case from a full requested-action state.
#[allow(clippy::too_many_arguments)]
fn appr_case(
    requested_action_label: &str,
    action_scope: M5AiActionScope,
    side_effect_class: M5AiSideEffectClass,
    tool_boundary: M5AiToolBoundary,
    friction_reasons: &[M5AiFrictionReason],
    rollback_posture: M5AiRollbackPosture,
    checkpoint_ref_present: bool,
    declared_approval_gate: M5AiApprovalGate,
) -> M5AiApprovalSheetResolutionCase {
    M5AiApprovalSheetResolutionCase::resolved(M5AiApprovalSheetResolutionInput {
        requested_action_label: requested_action_label.to_owned(),
        action_scope,
        side_effect_class,
        tool_boundary,
        friction_reasons: friction_reasons.to_vec(),
        rollback_posture,
        checkpoint_ref_present,
        declared_approval_gate,
    })
}

/// Builds a worked tool-call resolution case from a full tool-call state.
#[allow(clippy::too_many_arguments)]
fn call_case(
    occurred_at_label: &str,
    tool_label: &str,
    tool_boundary: M5AiToolBoundary,
    predicted_side_effect: M5AiSideEffectClass,
    observed_side_effect: M5AiSideEffectClass,
    run_outcome: M5AiRunOutcome,
    output_available: bool,
    in_active_context: bool,
) -> M5AiToolCallResolutionCase {
    M5AiToolCallResolutionCase::resolved(M5AiToolCallResolutionInput {
        occurred_at_label: occurred_at_label.to_owned(),
        tool_label: tool_label.to_owned(),
        tool_boundary,
        predicted_side_effect,
        observed_side_effect,
        run_outcome,
        output_available,
        in_active_context,
    })
}

/// A base row with the shared fields filled in and the full approval / tool-call
/// anatomy, scope, side-effect, boundary, rollback, gate, friction, outcome, control,
/// follow-up, export-field, and accessibility parity every lane carries.
fn base_row(
    tool_lane: M5AiToolLaneSurface,
    qualification: M5AiQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    approval_examples: Vec<M5AiApprovalSheetResolutionCase>,
    tool_call_examples: Vec<M5AiToolCallResolutionCase>,
) -> M5AiApprovalToolCallRow {
    M5AiApprovalToolCallRow {
        tool_lane,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5AiSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5AiDeploymentLine::ALL.to_vec(),
        approval_anatomy_parts: M5AiApprovalSheetAnatomyPart::ALL.to_vec(),
        tool_call_anatomy_parts: M5AiToolCallAnatomyPart::ALL.to_vec(),
        action_scopes: M5AiActionScope::ALL.to_vec(),
        side_effect_classes: M5AiSideEffectClass::ALL.to_vec(),
        tool_boundaries: M5AiToolBoundary::ALL.to_vec(),
        rollback_postures: M5AiRollbackPosture::ALL.to_vec(),
        approval_gates: M5AiApprovalGate::ALL.to_vec(),
        friction_reasons: M5AiFrictionReason::ALL.to_vec(),
        run_outcomes: M5AiRunOutcome::ALL.to_vec(),
        approval_controls: M5AiApprovalControl::ALL.to_vec(),
        follow_up_actions: M5AiToolCallFollowUp::ALL.to_vec(),
        approval_export_fields: M5AiApprovalSheetExportField::ALL.to_vec(),
        tool_call_export_fields: M5AiToolCallExportField::ALL.to_vec(),
        accessibility_routes: M5AiAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5AiConsumerSurface::AssistantPanelUi,
            M5AiConsumerSurface::PatchReviewUi,
            M5AiConsumerSurface::BranchAgentConsoleUi,
            M5AiConsumerSurface::SupportExport,
            M5AiConsumerSurface::CliInspect,
        ],
        downgrade_triggers: vec![
            M5AiExecutionDowngradeTrigger::ToolBoundaryUnstated,
            M5AiExecutionDowngradeTrigger::ApprovalGateHidden,
            M5AiExecutionDowngradeTrigger::CheckpointLineageBroken,
            M5AiExecutionDowngradeTrigger::ConnectorSideEffectUndisclosed,
            M5AiExecutionDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_AI_APPROVAL_TOOL_CALL_SCHEMA_REF,
            M5_AI_APPROVAL_TOOL_CALL_APPROVAL_ACTION_REF,
            M5_AI_APPROVAL_TOOL_CALL_TIMELINE_ENTRY_REF,
        ]),
        approval_examples,
        tool_call_examples,
        masks_mutation_or_boundary_as_status: false,
        buries_provenance_or_removal_in_logs: false,
        drops_rollback_or_checkpoint_vocabulary: false,
        invents_parallel_approval_or_tool_grammar: false,
    }
}

fn rows() -> Vec<M5AiApprovalToolCallRow> {
    use M5AiActionScope as Scope;
    use M5AiApprovalGate as Gate;
    use M5AiFrictionReason as Reason;
    use M5AiRollbackPosture as Rollback;
    use M5AiRunOutcome as Outcome;
    use M5AiSideEffectClass as Effect;
    use M5AiToolBoundary as Boundary;

    let mut rows = Vec::new();

    // 1. Read-only tool invocation — two benign read-only approval sheets that stay
    //    low-friction, and two read-only timeline rows that still keep provenance and
    //    removal controls visible.
    rows.push(base_row(
        M5AiToolLaneSurface::ReadOnlyToolInvocation,
        M5AiQualificationClass::Stable,
        "Read-only tool lane owner",
        "The read-only tool invocation lane renders the shared approval sheet and timeline row so a read-only single-file query stays auto-approved while still naming its scope, side effect, boundary, and rollback posture, and every read-only tool-call row keeps its provenance and remove-from-context controls visible instead of burying them in a raw log",
        "evidence:m5-ai-approval-tool-call-read-only:001",
        vec![
            appr_case(
                "read repository file",
                Scope::SingleFile,
                Effect::ReadOnly,
                Boundary::InProcess,
                &[],
                Rollback::ReversibleInPlace,
                false,
                Gate::AutoApproved,
            ),
            appr_case(
                "list workspace subtree",
                Scope::WorkspaceSubtree,
                Effect::ReadOnly,
                Boundary::InProcess,
                &[],
                Rollback::NoRollback,
                false,
                Gate::NotifyOnly,
            ),
        ],
        vec![
            call_case(
                "2026-07-06T10:00:00Z",
                "tool.repo-read",
                Boundary::InProcess,
                Effect::ReadOnly,
                Effect::ReadOnly,
                Outcome::Succeeded,
                true,
                true,
            ),
            call_case(
                "2026-07-06T10:01:00Z",
                "tool.sandbox-grep",
                Boundary::LocalSandbox,
                Effect::ReadOnly,
                Effect::ReadOnly,
                Outcome::AwaitingReview,
                false,
                false,
            ),
        ],
    ));

    // 2. Mutating tool run — a destructive whole-workspace write held at a typed
    //    high-friction gate, and an irreversible host-system action held at a two-person
    //    review; a tool-call row whose observed effect escalates past its prediction.
    rows.push(base_row(
        M5AiToolLaneSurface::MutatingToolRun,
        M5AiQualificationClass::Stable,
        "Mutating tool lane owner",
        "The mutating tool run lane renders the shared approval sheet and timeline row so a destructive whole-workspace write is held review-first at a typed high-friction gate with a restorable checkpoint, an irreversible host-system action escalates to a two-person review, and a tool call whose observed side effect escalates past its predicted class is flagged rather than shown as read-only",
        "evidence:m5-ai-approval-tool-call-mutating:001",
        vec![
            appr_case(
                "rewrite all workspace manifests",
                Scope::WholeWorkspace,
                Effect::FileWrite,
                Boundary::LocalShell,
                &[Reason::DestructiveFileChange],
                Rollback::CheckpointBacked,
                true,
                Gate::OneClickConfirm,
            ),
            appr_case(
                "reset host service state",
                Scope::HostSystem,
                Effect::Destructive,
                Boundary::HostDelegated,
                &[Reason::IrreversibleSideEffect],
                Rollback::IrreversibleExternal,
                false,
                Gate::TwoPersonReview,
            ),
        ],
        vec![
            call_case(
                "2026-07-06T11:00:00Z",
                "tool.workspace-rewrite",
                Boundary::LocalShell,
                Effect::FileWrite,
                Effect::Destructive,
                Outcome::PartiallyApplied,
                true,
                true,
            ),
            call_case(
                "2026-07-06T11:05:00Z",
                "tool.host-process",
                Boundary::HostDelegated,
                Effect::ProcessSpawn,
                Effect::ProcessSpawn,
                Outcome::Failed,
                false,
                false,
            ),
        ],
    ));

    // 3. Test-generation validation — a sandboxed file-write held at a one-click confirm
    //    with a checkpoint, and a policy-mandated review that escalates a read-only
    //    action to a two-person review; a sandbox tool-call row that mutates.
    rows.push(base_row(
        M5AiToolLaneSurface::TestGenerationValidation,
        M5AiQualificationClass::Stable,
        "Test-generation validation lane owner",
        "The test-generation validation lane renders the shared approval sheet and timeline row so a sandboxed generated-test write is held at a one-click confirm backed by a checkpoint, a policy-mandated review escalates even a read-only validation to a two-person review, and each validation tool-call row keeps its follow-up and provenance controls explicit",
        "evidence:m5-ai-approval-tool-call-test-generation:001",
        vec![
            appr_case(
                "write generated test files",
                Scope::WorkspaceSubtree,
                Effect::FileWrite,
                Boundary::LocalSandbox,
                &[],
                Rollback::CheckpointBacked,
                true,
                Gate::OneClickConfirm,
            ),
            appr_case(
                "validate coverage under policy hold",
                Scope::SingleFile,
                Effect::ReadOnly,
                Boundary::InProcess,
                &[Reason::PolicyMandatedReview],
                Rollback::ForwardFixOnly,
                false,
                Gate::OneClickConfirm,
            ),
        ],
        vec![
            call_case(
                "2026-07-06T12:00:00Z",
                "tool.testgen-write",
                Boundary::LocalSandbox,
                Effect::FileWrite,
                Effect::FileWrite,
                Outcome::Succeeded,
                true,
                true,
            ),
            call_case(
                "2026-07-06T12:02:00Z",
                "tool.coverage-probe",
                Boundary::InProcess,
                Effect::ReadOnly,
                Effect::StateMutation,
                Outcome::Cancelled,
                false,
                true,
            ),
        ],
    ));

    // 4. Branch-agent checkpoint — an external-resource state mutation held at a one-click
    //    confirm, and a cross-tenant credentialed network call held at a typed
    //    high-friction gate; remote and external tool-call rows.
    rows.push(base_row(
        M5AiToolLaneSurface::BranchAgentCheckpoint,
        M5AiQualificationClass::Stable,
        "Branch-agent checkpoint lane owner",
        "The branch-agent checkpoint lane renders the shared approval sheet and timeline row so an external-resource state mutation at a checkpoint is held at a one-click confirm, a cross-tenant credentialed network call is held review-first at a typed high-friction gate, and each remote or external tool-call row surfaces its boundary, observed effect, and governed follow-up actions",
        "evidence:m5-ai-approval-tool-call-branch-agent:001",
        vec![
            appr_case(
                "sync external resource state",
                Scope::ExternalResource,
                Effect::StateMutation,
                Boundary::RemoteConnector,
                &[Reason::ExternalNetworkEgress],
                Rollback::ReversibleInPlace,
                false,
                Gate::OneClickConfirm,
            ),
            appr_case(
                "call cross-tenant credentialed api",
                Scope::CrossTenant,
                Effect::NetworkCall,
                Boundary::ExternalService,
                &[Reason::CrossTenantScope, Reason::CredentialAccess],
                Rollback::NoRollback,
                false,
                Gate::HighFrictionTyped,
            ),
        ],
        vec![
            call_case(
                "2026-07-06T13:00:00Z",
                "tool.remote-sync",
                Boundary::RemoteConnector,
                Effect::NetworkCall,
                Effect::NetworkCall,
                Outcome::Superseded,
                true,
                false,
            ),
            call_case(
                "2026-07-06T13:04:00Z",
                "tool.external-api",
                Boundary::ExternalService,
                Effect::ReadOnly,
                Effect::NetworkCall,
                Outcome::Succeeded,
                true,
                true,
            ),
        ],
    ));

    // 5. CLI / support export — a policy-blocked mutating action that offers no
    //    approve-once affordance, and a benign read-only export action; tool-call rows a
    //    support or CLI reviewer reconstructs from the export alone.
    rows.push(base_row(
        M5AiToolLaneSurface::CliSupportExport,
        M5AiQualificationClass::Stable,
        "CLI / support export lane owner",
        "The CLI / support export lane renders the shared approval sheet and timeline row so a policy-blocked mutating action offers deny and open-plan but never an approve-once affordance, a read-only export action stays low-friction, and every approval and tool-call record — its scope, side effect, boundary, rollback posture, effective gate, and follow-up actions — is reconstructable from the support export alone",
        "evidence:m5-ai-approval-tool-call-cli:001",
        vec![
            appr_case(
                "publish blocked mutation",
                Scope::WorkspaceSubtree,
                Effect::FileWrite,
                Boundary::LocalShell,
                &[],
                Rollback::CheckpointBacked,
                true,
                Gate::PolicyBlocked,
            ),
            appr_case(
                "export read-only support bundle",
                Scope::SingleFile,
                Effect::ReadOnly,
                Boundary::InProcess,
                &[],
                Rollback::ReversibleInPlace,
                false,
                Gate::AutoApproved,
            ),
        ],
        vec![
            call_case(
                "2026-07-06T14:00:00Z",
                "tool.support-export",
                Boundary::InProcess,
                Effect::ReadOnly,
                Effect::ReadOnly,
                Outcome::Succeeded,
                true,
                true,
            ),
            call_case(
                "2026-07-06T14:03:00Z",
                "tool.cli-apply",
                Boundary::LocalShell,
                Effect::StateMutation,
                Effect::StateMutation,
                Outcome::PartiallyApplied,
                false,
                true,
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5AiApprovalToolCallGovernanceReview {
    M5AiApprovalToolCallGovernanceReview {
        one_primitive_carries_approval_and_tool_call_truth: true,
        requested_action_scope_side_effect_always_shown: true,
        mutating_action_never_ordinary_status: true,
        boundary_and_rollback_always_named: true,
        provenance_and_removal_always_visible: true,
        approval_controls_always_explicit: true,
        action_classes_match_policy_and_evidence: true,
        support_export_reconstructs_sheet_and_row_truth: true,
        no_surface_invents_parallel_vocabulary: true,
        every_row_declares_accessibility_route: true,
        descriptors_stable_across_ui_export_support: true,
    }
}

fn consumer_projection() -> M5AiApprovalToolCallConsumerProjection {
    M5AiApprovalToolCallConsumerProjection {
        tool_lanes_consume_shared_primitive: true,
        approval_gate_reads_single_source: true,
        side_effect_class_reads_single_source: true,
        follow_up_actions_read_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5AiApprovalToolCallProofFreshness {
    M5AiApprovalToolCallProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5AiApprovalToolCallReleasePosture {
    M5AiApprovalToolCallReleasePosture {
        release_packet_ref: M5_AI_APPROVAL_TOOL_CALL_ARTIFACT_REF.to_owned(),
        ai_audit_ref: M5_AI_APPROVAL_TOOL_CALL_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_AI_APPROVAL_TOOL_CALL_SCHEMA_REF,
        M5_AI_APPROVAL_TOOL_CALL_DOC_REF,
        M5_AI_APPROVAL_TOOL_CALL_COMPONENT_MATRIX_REF,
        M5_AI_APPROVAL_TOOL_CALL_APPROVAL_ACTION_REF,
        M5_AI_APPROVAL_TOOL_CALL_TIMELINE_ENTRY_REF,
    ])
}

/// Builds the canonical M5 AI approval-sheet / tool-call-timeline-row primitive packet.
pub fn seeded_m5_ai_approval_tool_call_primitive_packet() -> M5AiApprovalToolCallPrimitivePacket {
    M5AiApprovalToolCallPrimitivePacket::new(M5AiApprovalToolCallPrimitivePacketInput {
        packet_id: M5_AI_APPROVAL_TOOL_CALL_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 AI high-friction approval sheet and tool-call timeline row primitive: requested action, scope, side effect, boundary, rollback/checkpoint, effective approval gate, explicit approve-once/deny/open-plan controls, and governed open-output/remove-from-context/view-provenance follow-up actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5AiApprovalToolCallVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the mutating tool run lane is narrowed to Preview pending
/// high-friction-gate parity proof across every headless export path; every lane stays
/// visible.
pub fn seeded_m5_ai_approval_tool_call_primitive_mutating_tool_run_preview_narrowed(
) -> M5AiApprovalToolCallPrimitivePacket {
    let mut packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    packet.packet_id =
        "m5-ai-high-friction-approval-sheet-tool-call-timeline-row-primitive:mutating-tool-run-preview:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.tool_lane == M5AiToolLaneSurface::MutatingToolRun)
        .expect("mutating-tool-run row present");
    row.qualification = M5AiQualificationClass::Preview;
    packet
}

/// Narrowed variant: the branch-agent checkpoint lane is held at Beta because a slice of
/// branch-agent checkpoint rows do not yet render the follow-up provenance cue on every
/// profile; every lane stays visible.
pub fn seeded_m5_ai_approval_tool_call_primitive_branch_agent_checkpoint_beta_narrowed(
) -> M5AiApprovalToolCallPrimitivePacket {
    let mut packet = seeded_m5_ai_approval_tool_call_primitive_packet();
    packet.packet_id =
        "m5-ai-high-friction-approval-sheet-tool-call-timeline-row-primitive:branch-agent-checkpoint-beta:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.tool_lane == M5AiToolLaneSurface::BranchAgentCheckpoint)
        .expect("branch-agent-checkpoint row present");
    row.qualification = M5AiQualificationClass::Beta;
    packet
}
