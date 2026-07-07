//! Canonical seed builders for the M5 AI rerun-review-sheet / incomplete-replay-banner /
//! agent-status-card primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix,
//! the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical rerun-review / incomplete-replay / agent-status packet.
pub const M5_AI_BACKGROUND_AGENT_REPLAY_PRIMITIVE_PACKET_ID: &str =
    "m5-ai-rerun-review-sheet-incomplete-replay-banner-agent-status-card-primitive:stable:0001";

/// The canonical run identity threaded through a rerun-review example, an incomplete-replay
/// example, and an agent-status example so the same AI run appears consistently across
/// surfaces.
const SHARED_RUN_ID: &str = "run-2026-07-06-0007";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked rerun-review resolution case from a full rerun state.
#[allow(clippy::too_many_arguments)]
fn rr_case(
    rerun_review_id: &str,
    canonical_run_id: &str,
    original_lineage_label: &str,
    current_lineage_label: &str,
    provider_label: &str,
    model_label: &str,
    changed_dimensions: &[M5AiRerunDriftDimension],
    original_approvals_effective: bool,
) -> M5AiRerunReviewResolutionCase {
    M5AiRerunReviewResolutionCase::resolved(M5AiRerunReviewResolutionInput {
        rerun_review_id: rerun_review_id.to_owned(),
        canonical_run_id: canonical_run_id.to_owned(),
        original_lineage_label: original_lineage_label.to_owned(),
        current_lineage_label: current_lineage_label.to_owned(),
        provider_label: provider_label.to_owned(),
        model_label: model_label.to_owned(),
        changed_dimensions: changed_dimensions.to_vec(),
        original_approvals_effective,
    })
}

/// Builds a worked incomplete-replay resolution case from a full replay state.
fn ir_case(
    packet_id: &str,
    canonical_run_id: &str,
    replay_completeness: M5AiReplayCompleteness,
    retained_segments: &[M5AiReplaySegment],
    missing_segments: &[M5AiReplaySegment],
) -> M5AiIncompleteReplayResolutionCase {
    M5AiIncompleteReplayResolutionCase::resolved(M5AiIncompleteReplayResolutionInput {
        packet_id: packet_id.to_owned(),
        canonical_run_id: canonical_run_id.to_owned(),
        replay_completeness,
        retained_segments: retained_segments.to_vec(),
        missing_segments: missing_segments.to_vec(),
    })
}

/// Builds a worked agent-status resolution case from a full agent state.
#[allow(clippy::too_many_arguments)]
fn as_case(
    agent_id: &str,
    canonical_run_id: &str,
    lifecycle_state: M5AiAgentLifecycleState,
    checkpoint_label: &str,
    has_checkpoint: bool,
    blast_radius: M5AiAgentBlastRadius,
    last_successful_step_label: &str,
    pending_writes_count: u32,
    takeover_path: M5AiTakeoverPath,
) -> M5AiAgentStatusResolutionCase {
    M5AiAgentStatusResolutionCase::resolved(M5AiAgentStatusResolutionInput {
        agent_id: agent_id.to_owned(),
        canonical_run_id: canonical_run_id.to_owned(),
        lifecycle_state,
        checkpoint_label: checkpoint_label.to_owned(),
        has_checkpoint,
        blast_radius,
        last_successful_step_label: last_successful_step_label.to_owned(),
        pending_writes_count,
        takeover_path,
    })
}

/// A base row with the shared fields filled in and the full rerun-review, incomplete-replay,
/// and agent-status anatomy, continue-option, vocabulary, export-field, and accessibility
/// parity every surface carries.
#[allow(clippy::too_many_arguments)]
fn base_row(
    background_agent_surface: M5AiBackgroundAgentSurface,
    qualification: M5AiQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    rerun_review_examples: Vec<M5AiRerunReviewResolutionCase>,
    incomplete_replay_examples: Vec<M5AiIncompleteReplayResolutionCase>,
    agent_status_examples: Vec<M5AiAgentStatusResolutionCase>,
) -> M5AiBackgroundAgentReplayRow {
    M5AiBackgroundAgentReplayRow {
        background_agent_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5AiSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5AiDeploymentLine::ALL.to_vec(),
        rerun_review_anatomy_parts: M5AiRerunReviewAnatomyPart::ALL.to_vec(),
        incomplete_replay_anatomy_parts: M5AiIncompleteReplayAnatomyPart::ALL.to_vec(),
        agent_status_anatomy_parts: M5AiAgentStatusAnatomyPart::ALL.to_vec(),
        continue_options: M5AiAgentContinueOption::ALL.to_vec(),
        execution_modes: M5AiExecutionMode::ALL.to_vec(),
        run_outcomes: M5AiRunOutcome::ALL.to_vec(),
        approval_gates: M5AiApprovalGate::ALL.to_vec(),
        rerun_review_reasons: M5AiRerunReviewReason::ALL.to_vec(),
        rerun_admissions: M5AiRerunAdmission::ALL.to_vec(),
        rerun_drift_dimensions: M5AiRerunDriftDimension::ALL.to_vec(),
        replay_completeness_states: M5AiReplayCompleteness::ALL.to_vec(),
        replay_segments: M5AiReplaySegment::ALL.to_vec(),
        agent_lifecycle_states: M5AiAgentLifecycleState::ALL.to_vec(),
        takeover_paths: M5AiTakeoverPath::ALL.to_vec(),
        agent_blast_radii: M5AiAgentBlastRadius::ALL.to_vec(),
        rerun_review_export_fields: M5AiRerunReviewExportField::ALL.to_vec(),
        incomplete_replay_export_fields: M5AiIncompleteReplayExportField::ALL.to_vec(),
        agent_status_export_fields: M5AiAgentStatusExportField::ALL.to_vec(),
        accessibility_routes: M5AiAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5AiConsumerSurface::ReplayReviewUi,
            M5AiConsumerSurface::BranchAgentConsoleUi,
            M5AiConsumerSurface::RunHistoryUi,
            M5AiConsumerSurface::SupportExport,
            M5AiConsumerSurface::CliInspect,
        ],
        downgrade_triggers: vec![
            M5AiExecutionDowngradeTrigger::RerunReviewReasonUnstated,
            M5AiExecutionDowngradeTrigger::ReplayCompletenessOverstated,
            M5AiExecutionDowngradeTrigger::CheckpointLineageBroken,
            M5AiExecutionDowngradeTrigger::TakeoverPathHidden,
            M5AiExecutionDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_AI_BACKGROUND_AGENT_REPLAY_RERUN_REVIEW_REF,
            M5_AI_BACKGROUND_AGENT_REPLAY_REPLAY_PACKET_REF,
            M5_AI_BACKGROUND_AGENT_REPLAY_AGENT_RUN_REF,
        ]),
        rerun_review_examples,
        incomplete_replay_examples,
        agent_status_examples,
        masks_run_lineage_across_surfaces: false,
        presents_interrupted_agent_as_alive: false,
        overstates_replay_completeness: false,
        invents_parallel_rerun_or_agent_grammar: false,
    }
}

fn rows() -> Vec<M5AiBackgroundAgentReplayRow> {
    use M5AiAgentBlastRadius as Blast;
    use M5AiAgentLifecycleState as Life;
    use M5AiRerunDriftDimension as Drift;
    use M5AiReplayCompleteness as Complete;
    use M5AiReplaySegment as Seg;
    use M5AiTakeoverPath as Takeover;

    vec![
        // 1. Rerun-review surface — the shared run identity is anchored here (a rerun-review
        //    example, an incomplete-replay example, and an agent-status example all cite the
        //    same canonical id). A model-and-provider drift blocks the rerun, and a paused
        //    agent with pending writes is shown as not alive with safe continue options.
        base_row(
            M5AiBackgroundAgentSurface::RerunReview,
            M5AiQualificationClass::Stable,
            "Rerun-review surface owner",
            "The rerun-review surface renders the shared rerun-review sheet, incomplete-replay banner, and agent-status card so one canonical run id anchors the original-vs-current lineage compare, the drifted dimensions, the rerun reason, the approval-reuse verdict, and the safe takeover path, and a model-or-provider drift blocks the rerun rather than rerunning silently",
            "evidence:m5-ai-background-agent-replay-rerun-review:001",
            vec![
                rr_case(
                    "rerun-0007-a",
                    SHARED_RUN_ID,
                    "branch feature/auth @ base rev-a1",
                    "branch feature/auth @ base rev-c9",
                    "provider.managed-a",
                    "model.opus-4",
                    &[Drift::ModelVersion, Drift::ProviderRoute],
                    true,
                ),
                rr_case(
                    "rerun-0007-b",
                    "run-2026-07-06-0008",
                    "branch feature/parser @ base rev-b2",
                    "branch feature/parser @ base rev-b2",
                    "provider.local-oss",
                    "model.local-mixtral",
                    &[Drift::ToolContract],
                    true,
                ),
            ],
            vec![
                ir_case(
                    "replay-0007-a",
                    SHARED_RUN_ID,
                    Complete::IncompleteReplay,
                    &[Seg::PromptTranscript, Seg::ToolCallLog],
                    &[Seg::ApprovalLineage, Seg::ProviderResponse],
                ),
                ir_case(
                    "replay-0007-b",
                    "run-2026-07-06-0008",
                    Complete::PartiallyReplayable,
                    &[Seg::RouteReceipt, Seg::DiffPacket],
                    &[Seg::ProviderResponse],
                ),
            ],
            vec![
                as_case(
                    "agent-0007-a",
                    SHARED_RUN_ID,
                    Life::Paused,
                    "checkpoint:rerun-0007-a:step-3",
                    true,
                    Blast::WorktreeLocal,
                    "Applied lint fixes to auth module",
                    3,
                    Takeover::ResumeInPlace,
                ),
                as_case(
                    "agent-0007-b",
                    "run-2026-07-06-0008",
                    Life::BlockedOnApproval,
                    "checkpoint:rerun-0007-b:step-1",
                    true,
                    Blast::BranchScoped,
                    "Proposed parser patch",
                    2,
                    Takeover::TakeOverLocally,
                ),
            ],
        ),
        // 2. Branch-agent console surface — an input-context rerun that admits after
        //    re-review, an undrifted rerun that reuses approvals; a non-deterministic and a
        //    missing-inputs replay; an awaiting-takeover agent and a clean running agent.
        base_row(
            M5AiBackgroundAgentSurface::BranchAgentConsole,
            M5AiQualificationClass::Stable,
            "Branch-agent console surface owner",
            "The branch-agent console renders the shared components so a branch/worktree agent shows its lifecycle state, checkpoint, current blast radius, last successful step, pending writes, and safe continue-manually or restart options, and an awaiting-takeover agent never appears alive or safe to resume by implication",
            "evidence:m5-ai-background-agent-replay-branch-agent-console:001",
            vec![
                rr_case(
                    "rerun-0009-a",
                    "run-2026-07-06-0009",
                    "branch feature/incident @ base rev-d3",
                    "branch feature/incident @ base rev-e4",
                    "provider.managed-b",
                    "model.sonnet-4",
                    &[Drift::InputContext, Drift::OriginalBranch, Drift::BaseRevision],
                    true,
                ),
                rr_case(
                    "rerun-0009-b",
                    "run-2026-07-06-0010",
                    "branch feature/docs @ base rev-f5",
                    "branch feature/docs @ base rev-f5",
                    "provider.self-hosted",
                    "model.internal-7b",
                    &[],
                    true,
                ),
            ],
            vec![
                ir_case(
                    "replay-0009-a",
                    "run-2026-07-06-0009",
                    Complete::NonDeterministic,
                    &[Seg::PromptTranscript],
                    &[Seg::ProviderResponse],
                ),
                ir_case(
                    "replay-0009-b",
                    "run-2026-07-06-0010",
                    Complete::MissingInputs,
                    &[Seg::ToolCallLog],
                    &[Seg::PromptTranscript, Seg::RouteReceipt],
                ),
            ],
            vec![
                as_case(
                    "agent-0009-a",
                    "run-2026-07-06-0009",
                    Life::AwaitingTakeover,
                    "checkpoint:agent-0009-a:step-7",
                    true,
                    Blast::WorkspaceWide,
                    "Reproduced incident timeline",
                    5,
                    Takeover::BranchReviewHandoff,
                ),
                as_case(
                    "agent-0009-b",
                    "run-2026-07-06-0010",
                    Life::Running,
                    "checkpoint:agent-0009-b:step-2",
                    true,
                    Blast::NoWrites,
                    "Loaded docs pack context",
                    0,
                    Takeover::ResumeInPlace,
                ),
            ],
        ),
        // 3. Run-history surface — a route-only rerun blocked on provider drift, an undrifted
        //    reuse rerun; a provider-drifted replay and a fully-replayable one; a handed-off
        //    agent and a completed agent with external side effects and no takeover.
        base_row(
            M5AiBackgroundAgentSurface::RunHistory,
            M5AiQualificationClass::Stable,
            "Run-history surface owner",
            "The run-history surface renders the shared components so a prior run's rerun-review sheet, incomplete-replay banner, and agent-status card stay openable with the same run lineage, and a provider-drifted replay is never presented as fully replayable",
            "evidence:m5-ai-background-agent-replay-run-history:001",
            vec![
                rr_case(
                    "rerun-0011-a",
                    "run-2026-07-06-0011",
                    "branch feature/lint @ base rev-g6",
                    "branch feature/lint @ base rev-g6",
                    "provider.managed-a",
                    "model.haiku-4",
                    &[Drift::ProviderRoute],
                    true,
                ),
                rr_case(
                    "rerun-0011-b",
                    "run-2026-07-06-0012",
                    "branch feature/explain @ base rev-h7",
                    "branch feature/explain @ base rev-h7",
                    "provider.managed-a",
                    "model.opus-4",
                    &[],
                    true,
                ),
            ],
            vec![
                ir_case(
                    "replay-0011-a",
                    "run-2026-07-06-0011",
                    Complete::ProviderDrifted,
                    &[Seg::DiffPacket],
                    &[Seg::ProviderResponse],
                ),
                ir_case(
                    "replay-0011-b",
                    "run-2026-07-06-0012",
                    Complete::FullyReplayable,
                    &[
                        Seg::PromptTranscript,
                        Seg::ToolCallLog,
                        Seg::RouteReceipt,
                        Seg::ApprovalLineage,
                        Seg::DiffPacket,
                        Seg::ProviderResponse,
                    ],
                    &[],
                ),
            ],
            vec![
                as_case(
                    "agent-0011-a",
                    "run-2026-07-06-0011",
                    Life::HandedOff,
                    "checkpoint:agent-0011-a:step-4",
                    true,
                    Blast::BranchScoped,
                    "Handed batch lint fixes to reviewer",
                    0,
                    Takeover::EscalateToOwner,
                ),
                as_case(
                    "agent-0011-b",
                    "run-2026-07-06-0012",
                    Life::Completed,
                    "",
                    false,
                    Blast::ExternalSideEffects,
                    "Posted explanation to issue tracker",
                    0,
                    Takeover::NoTakeoverPossible,
                ),
            ],
        ),
        // 4. Support surface — a policy-drift rerun blocked pending approval, a tool-contract
        //    rerun blocked pending approval; two incomplete replays a support reviewer
        //    reconstructs from the export alone; an abandoned agent with a checkpoint and a
        //    blocked agent with pending writes.
        base_row(
            M5AiBackgroundAgentSurface::Support,
            M5AiQualificationClass::Stable,
            "Support-desk surface owner",
            "The support-desk surface renders the shared components so a support reviewer reconstructs the run lineage, the rerun-review verdict, the retained-versus-missing replay segments, and the agent's safe takeover path from the export alone, without inferring liveness or approval reuse",
            "evidence:m5-ai-background-agent-replay-support:001",
            vec![
                rr_case(
                    "rerun-0013-a",
                    "run-2026-07-06-0013",
                    "branch feature/crash @ base rev-i8",
                    "branch feature/crash @ base rev-i8",
                    "provider.managed-b",
                    "model.sonnet-4",
                    &[Drift::PolicyEpoch],
                    true,
                ),
                rr_case(
                    "rerun-0013-b",
                    "run-2026-07-06-0014",
                    "branch feature/dep @ base rev-j9",
                    "branch feature/dep @ base rev-j9",
                    "provider.local-oss",
                    "model.local-mixtral",
                    &[Drift::ToolContract, Drift::InputContext],
                    false,
                ),
            ],
            vec![
                ir_case(
                    "replay-0013-a",
                    "run-2026-07-06-0013",
                    Complete::IncompleteReplay,
                    &[Seg::PromptTranscript, Seg::RouteReceipt],
                    &[Seg::ApprovalLineage],
                ),
                ir_case(
                    "replay-0013-b",
                    "run-2026-07-06-0014",
                    Complete::PartiallyReplayable,
                    &[Seg::ToolCallLog, Seg::DiffPacket],
                    &[Seg::ProviderResponse, Seg::RouteReceipt],
                ),
            ],
            vec![
                as_case(
                    "agent-0013-a",
                    "run-2026-07-06-0013",
                    Life::Abandoned,
                    "checkpoint:agent-0013-a:step-6",
                    true,
                    Blast::WorktreeLocal,
                    "Captured crash reproduction",
                    0,
                    Takeover::AbortWithCheckpoint,
                ),
                as_case(
                    "agent-0013-b",
                    "run-2026-07-06-0014",
                    Life::BlockedOnApproval,
                    "checkpoint:agent-0013-b:step-2",
                    true,
                    Blast::WorkspaceWide,
                    "Staged dependency bump",
                    4,
                    Takeover::EscalateToOwner,
                ),
            ],
        ),
        // 5. CLI surface — a model-version rerun blocked on provider drift, an input-context
        //    rerun that admits after re-review; two more replays exercising the remaining
        //    completeness states; a paused agent and a running clean agent for headless
        //    inspection.
        base_row(
            M5AiBackgroundAgentSurface::Cli,
            M5AiQualificationClass::Stable,
            "CLI / headless surface owner",
            "The CLI inspect surface renders the shared components so a headless operator reads the same rerun-review verdict, replay completeness, and agent lifecycle / takeover truth as the UI, with every field present in the export packet",
            "evidence:m5-ai-background-agent-replay-cli:001",
            vec![
                rr_case(
                    "rerun-0015-a",
                    "run-2026-07-06-0015",
                    "branch feature/rereview @ base rev-k0",
                    "branch feature/rereview @ base rev-l1",
                    "provider.self-hosted",
                    "model.internal-7b",
                    &[Drift::ModelVersion],
                    true,
                ),
                rr_case(
                    "rerun-0015-b",
                    "run-2026-07-06-0016",
                    "branch feature/batch @ base rev-m2",
                    "branch feature/batch @ base rev-n3",
                    "provider.managed-a",
                    "model.haiku-4",
                    &[Drift::InputContext],
                    true,
                ),
            ],
            vec![
                ir_case(
                    "replay-0015-a",
                    "run-2026-07-06-0015",
                    Complete::NonDeterministic,
                    &[Seg::PromptTranscript, Seg::ProviderResponse],
                    &[Seg::ApprovalLineage],
                ),
                ir_case(
                    "replay-0015-b",
                    "run-2026-07-06-0016",
                    Complete::MissingInputs,
                    &[Seg::RouteReceipt],
                    &[Seg::PromptTranscript, Seg::DiffPacket],
                ),
            ],
            vec![
                as_case(
                    "agent-0015-a",
                    "run-2026-07-06-0015",
                    Life::Paused,
                    "checkpoint:agent-0015-a:step-5",
                    true,
                    Blast::BranchScoped,
                    "Re-reviewed parser patch",
                    1,
                    Takeover::TakeOverLocally,
                ),
                as_case(
                    "agent-0015-b",
                    "run-2026-07-06-0016",
                    Life::Running,
                    "checkpoint:agent-0015-b:step-1",
                    true,
                    Blast::NoWrites,
                    "Queued batch replay",
                    0,
                    Takeover::ResumeInPlace,
                ),
            ],
        ),
    ]
}

fn governance_review() -> M5AiBackgroundAgentReplayGovernanceReview {
    M5AiBackgroundAgentReplayGovernanceReview {
        one_primitive_carries_rerun_replay_and_agent_truth: true,
        run_lineage_consistent_across_surfaces: true,
        rerun_review_names_reason_and_drift: true,
        approval_reuse_only_when_no_relevant_drift: true,
        incomplete_replay_names_retained_and_missing: true,
        replay_completeness_never_overstated: true,
        interrupted_agent_never_presents_as_alive: true,
        interrupted_agent_offers_safe_continue: true,
        support_export_reconstructs_rerun_replay_and_agent_truth: true,
        no_surface_invents_parallel_vocabulary: true,
        every_row_declares_accessibility_route: true,
        descriptors_stable_across_ui_export_support: true,
    }
}

fn consumer_projection() -> M5AiBackgroundAgentReplayConsumerProjection {
    M5AiBackgroundAgentReplayConsumerProjection {
        background_agent_surfaces_consume_shared_primitive: true,
        run_lineage_reads_single_source: true,
        rerun_admission_reads_single_source: true,
        replay_completeness_reads_single_source: true,
        agent_liveness_reads_single_source: true,
    }
}

fn proof_freshness() -> M5AiBackgroundAgentReplayProofFreshness {
    M5AiBackgroundAgentReplayProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5AiBackgroundAgentReplayReleasePosture {
    M5AiBackgroundAgentReplayReleasePosture {
        release_packet_ref: M5_AI_BACKGROUND_AGENT_REPLAY_ARTIFACT_REF.to_owned(),
        ai_audit_ref: M5_AI_BACKGROUND_AGENT_REPLAY_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_AI_BACKGROUND_AGENT_REPLAY_SCHEMA_REF,
        M5_AI_BACKGROUND_AGENT_REPLAY_DOC_REF,
        M5_AI_BACKGROUND_AGENT_REPLAY_COMPONENT_MATRIX_REF,
        M5_AI_BACKGROUND_AGENT_REPLAY_RERUN_REVIEW_REF,
        M5_AI_BACKGROUND_AGENT_REPLAY_REPLAY_PACKET_REF,
        M5_AI_BACKGROUND_AGENT_REPLAY_AGENT_RUN_REF,
    ])
}

/// Builds the canonical M5 AI rerun-review / incomplete-replay / agent-status primitive
/// packet.
pub fn seeded_m5_ai_background_agent_replay_primitive_packet() -> M5AiBackgroundAgentReplayPrimitivePacket
{
    M5AiBackgroundAgentReplayPrimitivePacket::new(M5AiBackgroundAgentReplayPrimitivePacketInput {
        packet_id: M5_AI_BACKGROUND_AGENT_REPLAY_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 AI rerun-review sheet, incomplete-replay banner, and agent-status card primitive: original-vs-current lineage compare, named drift dimensions, rerun reason and approval-reuse verdict, retained-versus-missing replay segments, and honest agent lifecycle, blast radius, checkpoint, and safe continue-manually / restart / takeover options"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5AiBackgroundAgentReplayVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the rerun-review surface is narrowed to Preview pending
/// drift-disclosure parity proof across every headless rerun path; every surface stays
/// visible.
pub fn seeded_m5_ai_background_agent_replay_primitive_rerun_blocked_preview_narrowed(
) -> M5AiBackgroundAgentReplayPrimitivePacket {
    let mut packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    packet.packet_id =
        "m5-ai-rerun-review-sheet-incomplete-replay-banner-agent-status-card-primitive:rerun-preview:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.background_agent_surface == M5AiBackgroundAgentSurface::RerunReview)
        .expect("rerun-review row present");
    row.qualification = M5AiQualificationClass::Preview;
    packet
}

/// Narrowed variant: the support surface is held at Beta because a slice of support-desk
/// cards do not yet render the pending-writes cue on every profile; every surface stays
/// visible.
pub fn seeded_m5_ai_background_agent_replay_primitive_support_beta_narrowed(
) -> M5AiBackgroundAgentReplayPrimitivePacket {
    let mut packet = seeded_m5_ai_background_agent_replay_primitive_packet();
    packet.packet_id =
        "m5-ai-rerun-review-sheet-incomplete-replay-banner-agent-status-card-primitive:support-beta:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.background_agent_surface == M5AiBackgroundAgentSurface::Support)
        .expect("support row present");
    row.qualification = M5AiQualificationClass::Beta;
    packet
}
