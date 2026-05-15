use std::path::PathBuf;

use aureline_ai::routing::RegionPostureClass;
use aureline_ai::{
    AiRouteCandidate, AiRouteProviderClass, AiRoutingPacket, ComposerContextAlphaInput,
    ComposerContextAlphaSnapshot, ComposerContextReviewLock, ComposerDraft, CostEnvelopeClass,
    CostVisibilityClass, DeploymentProfileClass, ExecutionBoundaryClass, ExecutionLocusClass,
    ExhaustionStateClass, IntentModeClass, LatencyCostEnvelope, LatencyEnvelopeClass,
    PolicyTrustState, QuotaFamilyClass, QuotaInspector, QuotaScopeClass, QuotaStateClass,
    RetentionStanceClass, ReviewLockClass, RouteOriginClass, RouteSelectionOverrideReasonClass,
    RouteSelectionReasonClass, RoutingPolicyContext, RoutingRunStateClass, SelectedOutcomeClass,
    TokenCeilingClass, ToolCallCeilingClass, WallTimeCeilingClass,
};
use aureline_graph::{
    GraphCueSurface, GraphFactCuePacket, GraphFactTruthLane, GraphQueryRequest, GraphStore,
};
use aureline_graph_proto::{all_scenarios, Visibility};
use aureline_review::{
    DiffFileInput, DiffHunkInput, DiffLineInput, DiffLineKind, DiffOpenTarget, DiffViewMode,
    DiffViewSurfacePacket, ReviewWorkspaceSeedInput, ReviewWorkspaceSeedPacket,
};

#[test]
fn partial_index_graph_cue_survives_ai_context_and_review_with_epoch_parity() {
    let (ai_cue, review_cue) = partial_graph_cues();
    assert_eq!(ai_cue.consumer_surface, GraphCueSurface::AiContext);
    assert_eq!(review_cue.consumer_surface, GraphCueSurface::Review);

    let draft = ComposerDraft::new(
        "turn-draft:graph-cue:partial",
        "composer-session:graph-cue:partial",
        "ws:aureline",
        "Review the graph-backed symbol in the selected workset.",
    );
    let snapshot = ComposerContextAlphaSnapshot::project(
        &draft,
        &routing_packet(),
        ComposerContextAlphaInput {
            intent_mode: IntentModeClass::ReviewDiff,
            scope_label: "Selected workset".to_owned(),
            execution_boundary_class: ExecutionBoundaryClass::ManagedHosted,
            action_identity_ref: Some("cmd:ai.review_diff".to_owned()),
            mention_previews: Vec::new(),
            attachment_pills: Vec::new(),
            context_items: Vec::new(),
            graph_cue_packets: vec![ai_cue.clone()],
            review_lock: ComposerContextReviewLock {
                lock_class: ReviewLockClass::FrozenForReview,
                context_snapshot_ref: "context-snapshot:graph-cue:partial".to_owned(),
                route_snapshot_ref: "ai_routing_packet:graph-cue:partial".to_owned(),
                review_started_at: Some("2026-05-15T00:00:00Z".to_owned()),
            },
        },
    );

    assert!(snapshot.validate().is_empty(), "{:?}", snapshot.validate());
    let summary = snapshot.render_markdown_summary();
    assert!(summary.contains("partial_graph_fact"));
    assert!(summary.contains("readiness `partial`"));

    let handoff = snapshot.evidence_handoff("context-handoff:graph-cue:partial");
    assert!(handoff.validate().is_empty(), "{:?}", handoff.validate());
    assert_eq!(handoff.graph_cue_packets.len(), 1);
    assert_eq!(
        handoff.graph_cue_packets[0].truth_lanes,
        vec![GraphFactTruthLane::PartialGraphFact]
    );

    let review_seed = ReviewWorkspaceSeedPacket::from_diff_packet(
        review_seed_input(review_cue.clone()),
        &diff_packet(),
    );
    assert!(review_seed.preserves_graph_cue_epoch_parity());
    assert_eq!(review_seed.graph_cue_packets.len(), 1);
    assert_eq!(
        review_seed.inspection.graph_cue_packet_refs,
        vec![review_cue.packet_id.clone()]
    );
    assert_eq!(
        review_seed.inspection.graph_cue_readiness_tokens,
        vec!["partial".to_owned()]
    );

    let ai_packet = &handoff.graph_cue_packets[0];
    let review_packet = &review_seed.graph_cue_packets[0];
    assert_eq!(ai_packet.source_packet_ref, review_packet.source_packet_ref);
    assert_eq!(ai_packet.query_request_id, review_packet.query_request_id);
    assert_eq!(
        ai_packet.workspace_graph_id,
        review_packet.workspace_graph_id
    );
    assert_eq!(ai_packet.emitted_at, review_packet.emitted_at);
    assert_eq!(ai_packet.readiness, review_packet.readiness);
    assert_eq!(ai_packet.cues[0].cue_id, review_packet.cues[0].cue_id);
    assert_eq!(
        review_seed.inspection.graph_cue_epoch_refs,
        vec![ai_packet.emitted_at.clone()]
    );
}

fn partial_graph_cues() -> (GraphFactCuePacket, GraphFactCuePacket) {
    let mut graph = all_scenarios()
        .into_iter()
        .find(|scenario| scenario.label == "local_root_workspace")
        .expect("local graph scenario exists")
        .graph;
    let node = graph
        .nodes
        .iter_mut()
        .find(|node| node.node_id == "node:symbol:greet_fn")
        .expect("symbol node exists");
    for scope in &mut node.scope_refs {
        scope.visibility = Visibility::PartialVisible;
    }

    let store = GraphStore::persist_snapshot(graph).expect("partial graph validates");
    let envelope = store.query(GraphQueryRequest::symbol_lookup(
        "q:graph-cue:partial",
        "ws:aureline",
        "greet",
    ));
    assert_eq!(envelope.readiness.as_str(), "partial");

    let ai = GraphFactCuePacket::from_graph_query_envelope(
        "packet:graph-cue:partial:ai",
        GraphCueSurface::AiContext,
        &envelope,
    );
    let review = GraphFactCuePacket::from_graph_query_envelope(
        "packet:graph-cue:partial:review",
        GraphCueSurface::Review,
        &envelope,
    );
    assert_eq!(ai.truth_lanes, vec![GraphFactTruthLane::PartialGraphFact]);
    assert_eq!(
        review.truth_lanes,
        vec![GraphFactTruthLane::PartialGraphFact]
    );
    (ai, review)
}

fn routing_packet() -> AiRoutingPacket {
    let quota = QuotaInspector {
        quota_family_class: QuotaFamilyClass::VendorHostedEntitlementQuota,
        quota_state_class: QuotaStateClass::WithinLimit,
        quota_scope_class: QuotaScopeClass::VendorHostedEntitlement,
        budget_owner_ref: "quota-owner:graph-cue:partial".to_owned(),
        quota_meter_ref: Some("quota-meter:graph-cue:partial".to_owned()),
        quota_forecast_ref: Some("quota-forecast:graph-cue:partial".to_owned()),
        usage_export_ref: Some("usage-export:graph-cue:partial".to_owned()),
        explanation_label: "Hosted AI route is available.".to_owned(),
        local_continuity_label: "Local review stays available without hosted AI.".to_owned(),
        recovery_action_ref: Some("action:ai:view-quota".to_owned()),
    };
    let envelope = LatencyCostEnvelope {
        latency_envelope_class: LatencyEnvelopeClass::StreamingFirstTokenUnder500Ms,
        cost_envelope_class: CostEnvelopeClass::VendorHostedEntitlementBand,
        cost_visibility_class: CostVisibilityClass::BundledNoIncrementalCost,
        token_ceiling_class: TokenCeilingClass::TokensUnder32K,
        tool_call_ceiling_class: ToolCallCeilingClass::BoundedToolCallsUnder4,
        wall_time_ceiling_class: WallTimeCeilingClass::WallTimeUnder30S,
        budget_routing_policy_ref: "budget-policy:graph-cue:partial".to_owned(),
        graduation_packet_ref: "graduation-packet:graph-cue:partial".to_owned(),
        envelope_evidence_ref: "envelope-evidence:graph-cue:partial".to_owned(),
        explanation_label: "Route uses the hosted preview band.".to_owned(),
    };
    let candidate = AiRouteCandidate {
        candidate_id: "candidate:hosted:graph-cue:partial".to_owned(),
        provider_entry_ref: "provider-entry:first-party:graph-cue:partial".to_owned(),
        provider_label: "Aureline managed hosted AI".to_owned(),
        provider_class: AiRouteProviderClass::FirstPartyManaged,
        model_entry_ref: "model-entry:hosted:graph-cue:partial".to_owned(),
        model_label: "Hosted context review preview".to_owned(),
        execution_locus_class: ExecutionLocusClass::VendorHostedFirstPartyManaged,
        route_origin_class: RouteOriginClass::VendorHostedManaged,
        region_posture_class: RegionPostureClass::SingleRegionPinned,
        retention_stance_class: RetentionStanceClass::NoRetentionPromisedBodyDiscarded,
        quota,
        envelope,
        route_selection_reason_class: RouteSelectionReasonClass::NoCheaperQualifyingRouteExisted,
        route_selection_override_reason_class:
            RouteSelectionOverrideReasonClass::NoOverrideCheapestWasUsed,
        exhaustion_state_class: ExhaustionStateClass::NotExhaustedRouteAdmitted,
        selected_outcome_class: SelectedOutcomeClass::SelectedThisPath,
        route_selection_disclosure_ref: None,
        originating_approval_ticket_ref: Some("approval-ticket:graph-cue:partial".to_owned()),
        explanation_label: "Hosted route selected for context review.".to_owned(),
    };

    AiRoutingPacket::new(
        "ai_routing_packet:graph-cue:partial",
        "workflow.ai.graph_cue_review",
        "ws:aureline",
        RoutingRunStateClass::PreviewPreDispatch,
        RoutingPolicyContext {
            policy_epoch_ref: "policy-epoch:graph-cue:partial".to_owned(),
            trust_state: PolicyTrustState::Trusted,
            deployment_profile_class: DeploymentProfileClass::ManagedCloud,
            execution_context_ref: Some("execution-context:graph-cue:partial".to_owned()),
        },
        "capability-lifecycle:ai.graph-cue",
        None,
        vec![candidate.clone()],
        candidate.candidate_id,
        Vec::new(),
        vec!["docs/ai/context_assembly_contract.md".to_owned()],
        "2026-05-15T00:00:00Z",
    )
}

fn diff_packet() -> DiffViewSurfacePacket {
    let input = DiffFileInput {
        workspace_ref: "ws:aureline".to_owned(),
        truth_source_ref: "git-status:graph-cue:partial".to_owned(),
        repo_root: PathBuf::from("/workspace/aureline"),
        logical_root_ref: "root:local:aureline".to_owned(),
        worktree_ref: "worktree:local:aureline".to_owned(),
        group_token: "unstaged".to_owned(),
        path: PathBuf::from("crates/editor_core/src/lib.rs"),
        original_path: None,
        status_code: ".M".to_owned(),
        language_id: Some("rust".to_owned()),
        view_mode: DiffViewMode::Unified,
        generated_at: "2026-05-15T00:00:00Z".to_owned(),
        hunks: vec![DiffHunkInput {
            hunk_header: "@@ -42,2 +42,3 @@".to_owned(),
            old_start: 42,
            old_lines: 2,
            new_start: 42,
            new_lines: 3,
            lines: vec![
                DiffLineInput {
                    line_kind: DiffLineKind::Context,
                    old_line_number: Some(42),
                    new_line_number: Some(42),
                    raw_text: "pub fn apply_edit() {".to_owned(),
                },
                DiffLineInput {
                    line_kind: DiffLineKind::Addition,
                    old_line_number: None,
                    new_line_number: Some(43),
                    raw_text: "    trace_graph_cue();".to_owned(),
                },
            ],
        }],
    };
    let open_target = DiffOpenTarget::from_change_list_row_parts(
        &input.workspace_ref,
        &input.truth_source_ref,
        "git.change.row.graph-cue.partial",
        &input.group_token,
        input.path.clone(),
        input.original_path.clone(),
        &input.status_code,
        "modified",
    );
    DiffViewSurfacePacket::from_file_input(open_target, input)
}

fn review_seed_input(graph_cue_packet: GraphFactCuePacket) -> ReviewWorkspaceSeedInput {
    ReviewWorkspaceSeedInput {
        review_workspace_id: "review.workspace.graph-cue.partial".to_owned(),
        branch_or_worktree_ref: "worktree:local:aureline".to_owned(),
        base_revision_ref: Some("git.rev.base.graph-cue".to_owned()),
        head_revision_ref: Some("git.rev.head.graph-cue".to_owned()),
        actor_ref: "actor:local:reviewer".to_owned(),
        policy_epoch: "policy-epoch:graph-cue:partial".to_owned(),
        trust_state: "trusted".to_owned(),
        execution_context_id: Some("execution-context:graph-cue:partial".to_owned()),
        client_scopes: vec!["desktop_product".to_owned()],
        created_at: "2026-05-15T00:00:00Z".to_owned(),
        provider_overlay: None,
        work_item_links: Vec::new(),
        graph_cue_packets: vec![graph_cue_packet],
    }
}
