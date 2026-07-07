//! Canonical seed builders for the M5 AI action-state-banner primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical action-state-banner-primitive packet.
pub const M5_AI_ACTION_STATE_BANNER_PRIMITIVE_PACKET_ID: &str =
    "m5-ai-action-state-banner-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked resolution case from a full banner state.
#[allow(clippy::too_many_arguments)]
fn case(
    banner_label: &str,
    scope_repr: &str,
    execution_mode: M5AiExecutionMode,
    action_state: M5AiActionState,
    scope_reach: M5AiExecutionScopeReach,
    placement: M5AiActionPlacement,
    approval_gate: M5AiApprovalGate,
    blocked_boundary: Option<M5AiBlockedBoundary>,
    operator_controls: &[M5AiOperatorControl],
) -> M5AiBannerResolutionCase {
    M5AiBannerResolutionCase::resolved(M5AiBannerResolutionInput {
        banner_label: banner_label.to_owned(),
        scope_repr: scope_repr.to_owned(),
        execution_mode,
        action_state,
        scope_reach,
        placement,
        approval_gate,
        blocked_boundary,
        operator_controls: operator_controls.to_vec(),
    })
}

/// A base row with the shared fields filled in and the full anatomy, mode, action,
/// reach, placement, gate, posture, boundary, safe-alternative, control, export-field,
/// and accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5AiBannerConsumerSurface,
    qualification: M5AiQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    example_resolutions: Vec<M5AiBannerResolutionCase>,
) -> M5AiActionStateBannerRow {
    M5AiActionStateBannerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5AiSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5AiDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5AiBannerAnatomyPart::ALL.to_vec(),
        execution_modes: M5AiExecutionMode::ALL.to_vec(),
        action_states: M5AiActionState::ALL.to_vec(),
        scope_reaches: M5AiExecutionScopeReach::ALL.to_vec(),
        placements: M5AiActionPlacement::ALL.to_vec(),
        approval_gates: M5AiApprovalGate::ALL.to_vec(),
        banner_postures: M5AiBannerPosture::ALL.to_vec(),
        blocked_boundaries: M5AiBlockedBoundary::ALL.to_vec(),
        safe_alternatives: M5AiSafeAlternative::ALL.to_vec(),
        operator_controls: M5AiOperatorControl::ALL.to_vec(),
        export_fields: M5AiBannerExportField::ALL.to_vec(),
        accessibility_routes: M5AiAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5AiConsumerSurface::AssistantPanelUi,
            M5AiConsumerSurface::PatchReviewUi,
            M5AiConsumerSurface::BranchAgentConsoleUi,
            M5AiConsumerSurface::CliInspect,
            M5AiConsumerSurface::SupportExport,
            M5AiConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5AiExecutionDowngradeTrigger::ExecutionModeUnstated,
            M5AiExecutionDowngradeTrigger::ApprovalGateHidden,
            M5AiExecutionDowngradeTrigger::TakeoverPathHidden,
            M5AiExecutionDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_AI_ACTION_STATE_BANNER_SCHEMA_REF,
            M5_AI_ACTION_STATE_BANNER_TOOL_REF,
            M5_AI_ACTION_STATE_BANNER_AGENT_REF,
        ]),
        example_resolutions,
        masks_execution_mode_or_reach: false,
        shows_boundary_crossing_as_allowed: false,
        emits_generic_model_or_tool_error: false,
        hides_operator_controls_or_takeover: false,
    }
}

fn banner_rows() -> Vec<M5AiActionStateBannerRow> {
    use M5AiActionPlacement as Place;
    use M5AiActionState as State;
    use M5AiApprovalGate as Gate;
    use M5AiBlockedBoundary as Bound;
    use M5AiExecutionMode as Mode;
    use M5AiExecutionScopeReach as Reach;
    use M5AiOperatorControl as Ctl;

    let mut rows = Vec::new();

    // 1. Inline explain/fix — a foreground-assistant explanation streaming within a
    //    single selection (active within scope), and a guided-patch fix that would
    //    leave the reviewed file scope (boundary-blocked with a self-contained banner).
    rows.push(base_row(
        M5AiBannerConsumerSurface::InlineExplainFix,
        M5AiQualificationClass::Stable,
        "Inline explain/fix owner",
        "The inline explain/fix overlay renders the shared banner so a foreground-assistant explanation reads as active-within-scope with open-plan / pause / cancel controls, while a guided-patch fix that would write beyond the reviewed file scope reads as boundary-blocked with a banner naming the reviewed-file-scope boundary and a narrow-to-reviewed-scope safe next action rather than a generic model error",
        "evidence:m5-ai-banner-inline:001",
        vec![
            case(
                "inline explain selection",
                "single selection: lines 40-52",
                Mode::ForegroundAssistant,
                State::Streaming,
                Reach::SingleSelection,
                Place::InlineOverlay,
                Gate::AutoApproved,
                None,
                &[Ctl::OpenPlan, Ctl::Pause, Ctl::Cancel],
            ),
            case(
                "inline fix apply",
                "reviewed set: 3 files",
                Mode::GuidedPatch,
                State::BoundaryBlocked,
                Reach::CurrentFile,
                Place::InlineOverlay,
                Gate::OneClickConfirm,
                Some(Bound::ReviewedFileScope),
                &[Ctl::NarrowScope, Ctl::Cancel],
            ),
        ],
    ));

    // 2. Assistant panel — a foreground-assistant edit awaiting a high-friction typed
    //    confirmation (active-awaiting-approval), and a paused mid-run edit.
    rows.push(base_row(
        M5AiBannerConsumerSurface::AssistantPanel,
        M5AiQualificationClass::Stable,
        "Assistant-panel owner",
        "The assistant panel renders the shared banner so a foreground-assistant workspace edit behind a high-friction typed confirmation reads as active-awaiting-approval, while a paused mid-run edit reads as paused with resume / cancel controls — the mode, reach, and approval visible without a secondary inspector",
        "evidence:m5-ai-banner-panel:001",
        vec![
            case(
                "panel workspace edit",
                "workspace: current project",
                Mode::ForegroundAssistant,
                State::AwaitingApproval,
                Reach::WorkspaceScoped,
                Place::AssistantSidePanel,
                Gate::HighFrictionTyped,
                None,
                &[Ctl::OpenPlan, Ctl::Pause, Ctl::Cancel],
            ),
            case(
                "panel paused edit",
                "current file: main.rs",
                Mode::ForegroundAssistant,
                State::Paused,
                Reach::CurrentFile,
                Place::AssistantSidePanel,
                Gate::NotifyOnly,
                None,
                &[Ctl::Resume, Ctl::Cancel],
            ),
        ],
    ));

    // 3. Patch-review lane — a review-first placement patch that completed cleanly
    //    (completed-clear), and a guided-patch tool run blocked by a policy fence
    //    (boundary-blocked).
    rows.push(base_row(
        M5AiBannerConsumerSurface::PatchReview,
        M5AiQualificationClass::Stable,
        "Patch-review lane owner",
        "The patch-review lane renders the shared banner so a review-first patch that finished reads as completed-clear, while a guided-patch tool run that a policy fence blocks reads as boundary-blocked with a banner naming the policy-fence boundary and a split-into-approved-steps safe next action rather than a generic tool failure",
        "evidence:m5-ai-banner-review:001",
        vec![
            case(
                "review patch complete",
                "reviewed set: 5 files",
                Mode::ReviewFirstPlacement,
                State::Completed,
                Reach::ReviewedFileSet,
                Place::ReviewLane,
                Gate::OneClickConfirm,
                None,
                &[Ctl::OpenPlan],
            ),
            case(
                "review tool run",
                "reviewed set: 5 files",
                Mode::GuidedPatch,
                State::ToolRunning,
                Reach::ReviewedFileSet,
                Place::ReviewLane,
                Gate::PolicyBlocked,
                Some(Bound::PolicyFence),
                &[Ctl::NarrowScope, Ctl::Cancel],
            ),
        ],
    ));

    // 4. Branch / worktree agent — a background agent tool run behind a two-person
    //    review (active-awaiting-approval), and a background agent blocked at a
    //    connector boundary (boundary-blocked, with a take-over path).
    rows.push(base_row(
        M5AiBannerConsumerSurface::BranchWorktreeAgent,
        M5AiQualificationClass::Stable,
        "Branch / worktree agent owner",
        "The branch/worktree agent surface renders the shared banner so a background agent tool run behind a two-person review reads as active-awaiting-approval with pause / take-over / cancel controls, while a background agent that would cross a connector boundary reads as boundary-blocked with a banner naming the connector boundary and a request-connector-approval safe next action and an explicit take-over path",
        "evidence:m5-ai-banner-agent:001",
        vec![
            case(
                "agent connector run",
                "connector: code-search",
                Mode::BackgroundBranchAgent,
                State::ToolRunning,
                Reach::ConnectorScoped,
                Place::BackgroundBranchWorktree,
                Gate::TwoPersonReview,
                None,
                &[Ctl::Pause, Ctl::TakeOver, Ctl::Cancel],
            ),
            case(
                "agent connector blocked",
                "connector: code-search",
                Mode::BackgroundBranchAgent,
                State::BoundaryBlocked,
                Reach::ConnectorScoped,
                Place::BackgroundBranchWorktree,
                Gate::OneClickConfirm,
                Some(Bound::ConnectorBoundary),
                &[Ctl::TakeOver, Ctl::NarrowScope, Ctl::Cancel],
            ),
        ],
    ));

    // 5. CLI / support export — a headless run that failed across a cross-workspace
    //    reach (failed-needs-attention), and an idle headless banner (idle-ready) —
    //    the same banner vocabulary a support or CLI reviewer reads elsewhere.
    rows.push(base_row(
        M5AiBannerConsumerSurface::CliSupportExport,
        M5AiQualificationClass::Stable,
        "CLI / support export owner",
        "The CLI / support export renders the shared banner so a headless automation run that failed reads as failed-needs-attention with open-plan / cancel controls, and an idle headless banner reads as idle-ready — the mode, reach, action state, and approval reconstructable from the support export alone",
        "evidence:m5-ai-banner-cli:001",
        vec![
            case(
                "cli headless failed",
                "cross-workspace: 2 tenants",
                Mode::HeadlessAutomation,
                State::Failed,
                Reach::CrossWorkspaceScoped,
                Place::ToolRunTimeline,
                Gate::AutoApproved,
                None,
                &[Ctl::OpenPlan, Ctl::Cancel],
            ),
            case(
                "cli headless idle",
                "single selection: queued job",
                Mode::HeadlessAutomation,
                State::Idle,
                Reach::SingleSelection,
                Place::ToolRunTimeline,
                Gate::AutoApproved,
                None,
                &[Ctl::OpenPlan],
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5AiActionStateBannerGovernanceReview {
    M5AiActionStateBannerGovernanceReview {
        one_primitive_carries_banner_truth: true,
        mode_and_reach_always_shown: true,
        placement_and_approval_never_inferred: true,
        boundary_crossing_never_shown_as_allowed: true,
        operator_controls_always_present: true,
        boundary_blocked_always_shows_self_contained_banner: true,
        banner_names_boundary_and_safe_action: true,
        support_export_reconstructs_banner_truth: true,
        no_surface_invents_second_banner_grammar: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5AiActionStateBannerConsumerProjection {
    M5AiActionStateBannerConsumerProjection {
        banner_surfaces_consume_shared_primitive: true,
        posture_resolver_reads_single_source: true,
        scope_reach_reads_single_source: true,
        boundary_banner_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5AiActionStateBannerProofFreshness {
    M5AiActionStateBannerProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5AiActionStateBannerReleasePosture {
    M5AiActionStateBannerReleasePosture {
        release_packet_ref: M5_AI_ACTION_STATE_BANNER_ARTIFACT_REF.to_owned(),
        ai_audit_ref: M5_AI_ACTION_STATE_BANNER_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_AI_ACTION_STATE_BANNER_SCHEMA_REF,
        M5_AI_ACTION_STATE_BANNER_DOC_REF,
        M5_AI_ACTION_STATE_BANNER_COMPONENT_MATRIX_REF,
        M5_AI_ACTION_STATE_BANNER_TOOL_REF,
        M5_AI_ACTION_STATE_BANNER_AGENT_REF,
    ])
}

/// Builds the canonical M5 AI action-state-banner-primitive packet.
pub fn seeded_m5_ai_action_state_banner_primitive_packet() -> M5AiActionStateBannerPrimitivePacket {
    M5AiActionStateBannerPrimitivePacket::new(M5AiActionStateBannerPrimitivePacketInput {
        packet_id: M5_AI_ACTION_STATE_BANNER_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 AI action-state banner and boundary-blocked-banner primitive: execution mode, action state, scope reach, placement, approval posture, operator controls, and boundary-blocked safe alternatives"
                .to_owned(),
        banner_rows: banner_rows(),
        vocabulary_set: M5AiActionStateBannerVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the patch-review lane is held at Beta because a slice of
/// patch-review banners do not yet render the next-safe-action cue on every profile;
/// every consumer stays visible.
pub fn seeded_m5_ai_action_state_banner_primitive_patch_review_beta_narrowed(
) -> M5AiActionStateBannerPrimitivePacket {
    let mut packet = seeded_m5_ai_action_state_banner_primitive_packet();
    packet.packet_id = "m5-ai-action-state-banner-primitive:patch-review-beta:0001".to_owned();
    let row = packet
        .banner_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5AiBannerConsumerSurface::PatchReview)
        .expect("patch-review row present");
    row.qualification = M5AiQualificationClass::Beta;
    packet
}

/// Narrowed variant: the branch/worktree agent surface is narrowed to Preview pending
/// self-contained-banner parity proof across every headless export path; every consumer
/// stays visible.
pub fn seeded_m5_ai_action_state_banner_primitive_branch_worktree_agent_preview_narrowed(
) -> M5AiActionStateBannerPrimitivePacket {
    let mut packet = seeded_m5_ai_action_state_banner_primitive_packet();
    packet.packet_id =
        "m5-ai-action-state-banner-primitive:branch-worktree-agent-preview:0001".to_owned();
    let row = packet
        .banner_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5AiBannerConsumerSurface::BranchWorktreeAgent)
        .expect("branch/worktree agent row present");
    row.qualification = M5AiQualificationClass::Preview;
    packet
}
