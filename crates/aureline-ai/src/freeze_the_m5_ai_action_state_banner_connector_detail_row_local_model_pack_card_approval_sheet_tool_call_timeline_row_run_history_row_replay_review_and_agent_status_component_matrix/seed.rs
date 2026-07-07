//! Canonical seed builders for the frozen M5 AI-execution/replay component matrix.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call them
//! so the in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical AI-execution/replay-component matrix.
pub const M5_AI_EXECUTION_COMPONENT_MATRIX_PACKET_ID: &str =
    "m5-ai-execution-replay-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-06T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5AiRequiredLabel> {
    M5AiRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5AiRequiredLabel]) -> Vec<M5AiRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every
/// family-specific vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5AiExecutionComponentFamily,
    qualification: M5AiQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
) -> M5AiExecutionComponentRow {
    M5AiExecutionComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5AiSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5AiDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        action_states: vec![],
        execution_modes: vec![],
        connector_capabilities: vec![],
        auth_postures: vec![],
        model_pack_states: vec![],
        approval_gates: vec![],
        friction_reasons: vec![],
        tool_boundaries: vec![],
        side_effect_classes: vec![],
        run_outcomes: vec![],
        replay_completeness: vec![],
        rerun_review_reasons: vec![],
        agent_lifecycle_states: vec![],
        takeover_paths: vec![],
        accessibility_routes: M5AiAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5AiConsumerSurface::AssistantPanelUi,
            M5AiConsumerSurface::SupportExport,
            M5AiConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5AiExecutionDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_AI_EXECUTION_COMPONENT_SCHEMA_REF,
            M5_AI_EXECUTION_COMPONENT_REPLAY_REF,
        ]),
        masks_execution_mode_or_route: false,
        overstates_replay_completeness: false,
        invents_private_ai_status_grammar: false,
        hides_approval_gate_or_takeover_path: false,
    }
}

fn component_rows() -> Vec<M5AiExecutionComponentRow> {
    use M5AiActionState as AS;
    use M5AiAgentLifecycleState as AL;
    use M5AiApprovalGate as AG;
    use M5AiAuthPosture as AP;
    use M5AiConnectorCapability as CAP;
    use M5AiConsumerSurface as C;
    use M5AiExecutionComponentFamily as F;
    use M5AiExecutionDowngradeTrigger as D;
    use M5AiExecutionMode as EM;
    use M5AiFrictionReason as FR;
    use M5AiModelPackState as MP;
    use M5AiQualificationClass as Q;
    use M5AiReplayCompleteness as RC;
    use M5AiRequiredLabel as L;
    use M5AiRerunReviewReason as RR;
    use M5AiRunOutcome as RO;
    use M5AiSideEffectClass as SE;
    use M5AiTakeoverPath as TP;
    use M5AiToolBoundary as TB;

    let mut rows = Vec::new();

    // 1. AI action-state banner.
    let mut row = base_row(
        F::AiActionStateBanner,
        Q::Stable,
        "AI action-state component owner",
        "One AI-action-state-banner model naming the live action state — idle, composing, streaming, tool-running, awaiting-approval, paused, boundary-blocked, completed, or failed — and the execution mode behind it, so a user never has to infer whether an assistant, guided-patch, or background branch agent is active or blocked",
        "evidence:m5-ai-action-state-banner-parity:001",
    );
    row.action_states = AS::ALL.to_vec();
    row.execution_modes = EM::ALL.to_vec();
    row.required_labels = labels_with(&[L::ExecutionMode, L::Route]);
    row.consumer_surfaces = vec![
        C::AssistantPanelUi,
        C::BranchAgentConsoleUi,
        C::PatchReviewUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ExecutionModeUnstated,
        D::RouteOrProviderMasked,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Connector / tool-server detail row.
    let mut row = base_row(
        F::ConnectorDetailRow,
        Q::Stable,
        "Connector component owner",
        "One connector-detail-row model naming the capability class an external connector or tool server exposes — read-only query, file mutation, network egress, shell execution, external service call, or credential-scoped access — and its auth posture, so a connector never hides what it can do or how it authenticates",
        "evidence:m5-ai-connector-detail-row-parity:001",
    );
    row.connector_capabilities = CAP::ALL.to_vec();
    row.auth_postures = AP::ALL.to_vec();
    row.required_labels = labels_with(&[L::Route]);
    row.consumer_surfaces = vec![
        C::ConnectorAdminConsole,
        C::AssistantPanelUi,
        C::PatchReviewUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::AuthPostureMasked,
        D::ConnectorSideEffectUndisclosed,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Local model pack card.
    let mut row = base_row(
        F::LocalModelPackCard,
        Q::Stable,
        "Local-model component owner",
        "One local-model-pack-card model naming whether a pack is installed, mirrored, offline-only, quarantined, hardware-unfit, update-available, or provenance-unverified, so a quarantined or provenance-unverified model pack is never presented as freely ready to route",
        "evidence:m5-ai-local-model-pack-card-parity:001",
    );
    row.model_pack_states = MP::ALL.to_vec();
    row.required_labels = labels_with(&[L::Route]);
    row.consumer_surfaces = vec![
        C::ModelManagerUi,
        C::ConnectorAdminConsole,
        C::AssistantPanelUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::LocalModelProvenanceMasked,
        D::RouteOrProviderMasked,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. High-friction approval sheet.
    let mut row = base_row(
        F::ApprovalSheet,
        Q::Stable,
        "Approval component owner",
        "One approval-sheet model naming the approval gate in effect — auto-approved, notify-only, one-click, high-friction-typed, two-person-review, or policy-blocked — and why the friction applies, so a high-friction or blocked action is never presented as a quiet auto-approval",
        "evidence:m5-ai-approval-sheet-parity:001",
    );
    row.approval_gates = AG::ALL.to_vec();
    row.friction_reasons = FR::ALL.to_vec();
    row.required_labels = labels_with(&[L::ApprovalGate]);
    row.consumer_surfaces = vec![
        C::AssistantPanelUi,
        C::PatchReviewUi,
        C::BranchAgentConsoleUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![D::ApprovalGateHidden, D::ProofStale];
    rows.push(row);

    // 5. Tool-call timeline row.
    let mut row = base_row(
        F::ToolCallTimelineRow,
        Q::Stable,
        "Tool-call component owner",
        "One tool-call-timeline-row model naming where a tool ran — in-process, local sandbox, local shell, remote connector, external service, or host-delegated — and its side-effect class, so a destructive or network tool call is never shown as a benign in-process read",
        "evidence:m5-ai-tool-call-timeline-row-parity:001",
    );
    row.tool_boundaries = TB::ALL.to_vec();
    row.side_effect_classes = SE::ALL.to_vec();
    row.required_labels = labels_with(&[L::ApprovalGate]);
    row.consumer_surfaces = vec![
        C::AssistantPanelUi,
        C::PatchReviewUi,
        C::BranchAgentConsoleUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ToolBoundaryUnstated,
        D::ConnectorSideEffectUndisclosed,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. AI run-history row.
    let mut row = base_row(
        F::RunHistoryRow,
        Q::Stable,
        "Run-history component owner",
        "One run-history-row model naming the outcome of a recorded AI run — succeeded, failed, cancelled, superseded, partially-applied, or awaiting-review — alongside the route and mode it ran in, so a partially-applied or superseded run is never listed as a clean success",
        "evidence:m5-ai-run-history-row-parity:001",
    );
    row.run_outcomes = RO::ALL.to_vec();
    row.required_labels = labels_with(&[L::ExecutionMode, L::Route]);
    row.consumer_surfaces = vec![
        C::RunHistoryUi,
        C::ReplayReviewUi,
        C::BranchAgentConsoleUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::RouteOrProviderMasked,
        D::ExecutionModeUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 7. Replay / rerun-review sheet.
    let mut row = base_row(
        F::ReplayReview,
        Q::Stable,
        "Replay-review component owner",
        "One replay-review model naming how completely a run can be replayed — fully-replayable, partially-replayable, incomplete, non-deterministic, missing-inputs, or provider-drifted — and why a rerun requires re-review, so an incomplete or drifted replay is never shown as a faithful re-run",
        "evidence:m5-ai-replay-review-parity:001",
    );
    row.replay_completeness = RC::ALL.to_vec();
    row.rerun_review_reasons = RR::ALL.to_vec();
    row.required_labels = labels_with(&[L::ExecutionMode, L::Route]);
    row.consumer_surfaces = vec![
        C::ReplayReviewUi,
        C::RunHistoryUi,
        C::PatchReviewUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ReplayCompletenessOverstated,
        D::RerunReviewReasonUnstated,
        D::CheckpointLineageBroken,
        D::ProofStale,
    ];
    rows.push(row);

    // 8. Agent-status card.
    let mut row = base_row(
        F::AgentStatus,
        Q::Stable,
        "Agent-status component owner",
        "One agent-status model naming the lifecycle state of a branch / worktree agent — running, paused, blocked-on-approval, awaiting-takeover, handed-off, completed, or abandoned — and the manual-takeover path offered, so an interrupted agent always names how a user can safely take it over",
        "evidence:m5-ai-agent-status-parity:001",
    );
    row.agent_lifecycle_states = AL::ALL.to_vec();
    row.takeover_paths = TP::ALL.to_vec();
    row.required_labels = labels_with(&[L::ExecutionMode, L::ApprovalGate]);
    row.consumer_surfaces = vec![
        C::BranchAgentConsoleUi,
        C::RunHistoryUi,
        C::AssistantPanelUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::TakeoverPathHidden,
        D::CheckpointLineageBroken,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5AiExecutionComponentGovernanceReview {
    M5AiExecutionComponentGovernanceReview {
        action_state_banner_shows_mode_and_state: true,
        connector_row_shows_capability_and_auth: true,
        local_model_card_shows_pack_state_and_provenance: true,
        approval_sheet_shows_gate_and_friction: true,
        tool_call_row_shows_boundary_and_side_effect: true,
        run_history_row_shows_outcome_and_route: true,
        replay_review_shows_completeness_and_rerun_reason: true,
        agent_status_shows_lifecycle_and_takeover_path: true,
        live_versus_replayed_never_conflated: true,
        no_component_invents_second_status_grammar: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5AiExecutionComponentConsumerProjection {
    M5AiExecutionComponentConsumerProjection {
        assistant_and_run_surfaces_consume_mode_vocabulary: true,
        connector_and_tool_surfaces_consume_boundary_vocabulary: true,
        model_surfaces_consume_pack_state_vocabulary: true,
        approval_surfaces_consume_gate_vocabulary: true,
        support_export_reads_single_source: true,
        replay_and_agent_surfaces_read_single_source: true,
    }
}

fn proof_freshness() -> M5AiExecutionComponentProofFreshness {
    M5AiExecutionComponentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5AiExecutionComponentReleasePosture {
    M5AiExecutionComponentReleasePosture {
        proof_packet_ref: M5_AI_EXECUTION_COMPONENT_ARTIFACT_REF.to_owned(),
        ai_audit_ref: M5_AI_EXECUTION_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_AI_EXECUTION_COMPONENT_SCHEMA_REF,
        M5_AI_EXECUTION_COMPONENT_DOC_REF,
        M5_AI_EXECUTION_COMPONENT_TOOL_REF,
        M5_AI_EXECUTION_COMPONENT_RUN_HISTORY_REF,
        M5_AI_EXECUTION_COMPONENT_REPLAY_REF,
        M5_AI_EXECUTION_COMPONENT_AGENT_REF,
    ])
}

/// Builds the canonical frozen M5 AI-execution/replay-component matrix packet.
pub fn seeded_m5_ai_execution_replay_component_matrix() -> M5AiExecutionComponentMatrixPacket {
    M5AiExecutionComponentMatrixPacket::new(M5AiExecutionComponentMatrixPacketInput {
        packet_id: M5_AI_EXECUTION_COMPONENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 AI action-state-banner, connector-detail-row, local-model-pack-card, approval-sheet, tool-call-timeline-row, run-history-row, replay-review, and agent-status component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5AiExecutionComponentVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the replay-review sheet is held at Beta because a slice of
/// replay-completeness states do not yet round-trip across every rerun surface;
/// every component stays visible.
pub fn seeded_m5_ai_execution_replay_component_matrix_replay_review_beta_narrowed(
) -> M5AiExecutionComponentMatrixPacket {
    let mut packet = seeded_m5_ai_execution_replay_component_matrix();
    packet.packet_id = "m5-ai-execution-replay-components:replay-review-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5AiExecutionComponentFamily::ReplayReview)
        .expect("replay-review row present");
    row.qualification = M5AiQualificationClass::Beta;
    packet
}

/// Narrowed variant: the agent-status card is narrowed to Preview pending
/// manual-takeover-path parity proof across every branch-agent surface; every
/// component stays visible.
pub fn seeded_m5_ai_execution_replay_component_matrix_agent_status_preview_narrowed(
) -> M5AiExecutionComponentMatrixPacket {
    let mut packet = seeded_m5_ai_execution_replay_component_matrix();
    packet.packet_id = "m5-ai-execution-replay-components:agent-status-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5AiExecutionComponentFamily::AgentStatus)
        .expect("agent-status row present");
    row.qualification = M5AiQualificationClass::Preview;
    packet
}
