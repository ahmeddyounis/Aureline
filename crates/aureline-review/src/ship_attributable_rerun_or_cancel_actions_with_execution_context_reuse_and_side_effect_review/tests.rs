use super::*;

const PACKET_ID: &str = "rerun-cancel-review:stable:0001";

fn effect_summary(what: &str) -> ActionEffectSummary {
    ActionEffectSummary {
        what_will_fire_label: what.to_owned(),
        where_label: "On the failed jobs of the named run".to_owned(),
        under_whose_authority_label: "Signed-in human account".to_owned(),
        audit_row_label: "Audit row will record actor, control, and scope".to_owned(),
    }
}

fn action_rows() -> Vec<RerunCancelActionRow> {
    vec![
        RerunCancelActionRow {
            action_id: "action:rerun-failed-login".to_owned(),
            run_id: "run:feature-login".to_owned(),
            durable_anchor_id: "anchor:review:0001".to_owned(),
            target_identity_label: "Local branch feature/login vs base main".to_owned(),
            control_class: ActionControlClass::RerunFailedJobs,
            target_scope: ActionTargetScope::FailedJobsOnly,
            mutation_mode: ActionMutationMode::PublishNow,
            execution_context_id: "exec-ctx:feature-login:0001".to_owned(),
            context_reuse_decision: ExecutionContextReuseDecision::ReuseIdenticalContext,
            context_freshness: ContextFreshness::AuthoritativeLive,
            context_staleness_label: String::new(),
            blocked_class: ActionBlockedClass::NotBlocked,
            actor_attribution_label: "Signed-in human account".to_owned(),
            audit_row_ref: "audit:rerun-failed-login:0001".to_owned(),
            effect_summary: effect_summary("Reruns only the failed jobs"),
            attention_reasons: Vec::new(),
            review_summary: "Rerun failed jobs reuses the recorded execution context".to_owned(),
            approval_ticket_ref: Some("approval:rerun-failed-login".to_owned()),
            browser_handoff_ref: None,
            deferred_queue_ref: None,
            source_contract_refs: vec![
                RERUN_CANCEL_RUN_CONTROL_CONTRACT_REF.to_owned(),
                RERUN_CANCEL_PIPELINE_RUN_CONTRACT_REF.to_owned(),
            ],
        },
        RerunCancelActionRow {
            action_id: "action:cancel-deploy-staging".to_owned(),
            run_id: "run:deploy-staging".to_owned(),
            durable_anchor_id: "anchor:review:0002".to_owned(),
            target_identity_label: "Local branch release/2026.6 vs base release".to_owned(),
            control_class: ActionControlClass::CancelWorkflow,
            target_scope: ActionTargetScope::EntireWorkflowRun,
            mutation_mode: ActionMutationMode::OpenInProvider,
            execution_context_id: "exec-ctx:deploy-staging:0002".to_owned(),
            context_reuse_decision: ExecutionContextReuseDecision::ForkNewContext,
            context_freshness: ContextFreshness::WarmCached,
            context_staleness_label: String::new(),
            blocked_class: ActionBlockedClass::NotBlocked,
            actor_attribution_label: "Signed-in human account".to_owned(),
            audit_row_ref: "audit:cancel-deploy-staging:0002".to_owned(),
            effect_summary: ActionEffectSummary {
                what_will_fire_label: "Cancels the in-flight deployment workflow".to_owned(),
                where_label: "On the entire deployment workflow run".to_owned(),
                under_whose_authority_label: "Signed-in human account".to_owned(),
                audit_row_label: "Audit row will record the cancel and its scope".to_owned(),
            },
            attention_reasons: Vec::new(),
            review_summary: "Cancel hands off to the provider in the browser".to_owned(),
            approval_ticket_ref: None,
            browser_handoff_ref: Some("handoff:cancel-deploy-staging".to_owned()),
            deferred_queue_ref: None,
            source_contract_refs: vec![
                RERUN_CANCEL_RUN_CONTROL_CONTRACT_REF.to_owned(),
                RERUN_CANCEL_EXECUTION_CONTEXT_CONTRACT_REF.to_owned(),
            ],
        },
        RerunCancelActionRow {
            action_id: "action:rerun-release".to_owned(),
            run_id: "run:release-2026-6".to_owned(),
            durable_anchor_id: "anchor:review:0003".to_owned(),
            target_identity_label: "Local branch release/2026.6 vs base release".to_owned(),
            control_class: ActionControlClass::RerunWorkflow,
            target_scope: ActionTargetScope::EntireWorkflowRun,
            mutation_mode: ActionMutationMode::DeferredPublish,
            execution_context_id: "exec-ctx:release-2026-6:0003".to_owned(),
            context_reuse_decision: ExecutionContextReuseDecision::ReuseWithPinnedInputs,
            context_freshness: ContextFreshness::Stale,
            context_staleness_label:
                "Recorded execution context is 9 days old; pinned inputs need review".to_owned(),
            blocked_class: ActionBlockedClass::BlockedContextReuseStaleReviewRequired,
            actor_attribution_label: "Signed-in human account".to_owned(),
            audit_row_ref: "audit:rerun-release:0003".to_owned(),
            effect_summary: ActionEffectSummary {
                what_will_fire_label: "Reruns the entire release workflow".to_owned(),
                where_label: "On the entire release run".to_owned(),
                under_whose_authority_label: "Signed-in human account".to_owned(),
                audit_row_label: "Audit row will record the rerun and pinned inputs".to_owned(),
            },
            attention_reasons: vec![
                "Reused execution context is stale and requires review before rerun".to_owned(),
                "Rerun is queued for a later drain, not fired now".to_owned(),
            ],
            review_summary: "Rerun is blocked until the stale context reuse is reviewed".to_owned(),
            approval_ticket_ref: None,
            browser_handoff_ref: None,
            deferred_queue_ref: Some("queue:rerun-release".to_owned()),
            source_contract_refs: vec![
                RERUN_CANCEL_RUN_CONTROL_CONTRACT_REF.to_owned(),
                RERUN_CANCEL_EXECUTION_CONTEXT_CONTRACT_REF.to_owned(),
            ],
        },
    ]
}

fn side_effect_rows() -> Vec<SideEffectReviewRow> {
    vec![
        SideEffectReviewRow {
            action_id: "action:rerun-failed-login".to_owned(),
            side_effect_id: "side:rerun-failed-login:reeval".to_owned(),
            side_effect_label: "Re-evaluates the failed jobs in place".to_owned(),
            side_effect_class: SideEffectClass::NoExternalSideEffect,
            acknowledgment_requirement: SideEffectAckRequirement::NoAckRequired,
            disclosure_label: "No external write scope; reruns the failed jobs only".to_owned(),
        },
        SideEffectReviewRow {
            action_id: "action:cancel-deploy-staging".to_owned(),
            side_effect_id: "side:cancel-deploy-staging:provider-mutation".to_owned(),
            side_effect_label: "Mutates provider run state to cancel the deployment".to_owned(),
            side_effect_class: SideEffectClass::MutatesProviderRunState,
            acknowledgment_requirement: SideEffectAckRequirement::RequiresBrowserHandoff,
            disclosure_label: "Cancellation reaches provider state via browser handoff".to_owned(),
        },
        SideEffectReviewRow {
            action_id: "action:cancel-deploy-staging".to_owned(),
            side_effect_id: "side:cancel-deploy-staging:notify".to_owned(),
            side_effect_label: "Notifies the deployment channel of the cancel".to_owned(),
            side_effect_class: SideEffectClass::SendsNotifications,
            acknowledgment_requirement: SideEffectAckRequirement::RequiresExplicitConfirmation,
            disclosure_label: "Sends a cancellation notification on confirmation".to_owned(),
        },
        SideEffectReviewRow {
            action_id: "action:rerun-release".to_owned(),
            side_effect_id: "side:rerun-release:deploy".to_owned(),
            side_effect_label: "Triggers a fresh release deployment".to_owned(),
            side_effect_class: SideEffectClass::TriggersDeployment,
            acknowledgment_requirement: SideEffectAckRequirement::RequiresDeferredQueue,
            disclosure_label: "Deployment is queued for a later drain on review".to_owned(),
        },
        SideEffectReviewRow {
            action_id: "action:rerun-release".to_owned(),
            side_effect_id: "side:rerun-release:cost".to_owned(),
            side_effect_label: "Consumes release pipeline quota".to_owned(),
            side_effect_class: SideEffectClass::ConsumesQuotaOrCost,
            acknowledgment_requirement: SideEffectAckRequirement::RequiresExplicitConfirmation,
            disclosure_label: "Rerun consumes additional pipeline quota on confirmation".to_owned(),
        },
    ]
}

fn trust_review() -> RerunCancelTrustReview {
    RerunCancelTrustReview {
        action_control_class_explicit: true,
        target_scope_explicit: true,
        every_mutating_action_attributable: true,
        audit_row_recorded_for_every_action: true,
        execution_context_reuse_explicit: true,
        stale_context_reuse_flagged_not_hidden: true,
        side_effect_reviewed_before_invocation: true,
        non_inert_side_effect_requires_acknowledgment: true,
        no_hidden_write_scope: true,
        mutation_mode_cites_required_grant: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> RerunCancelConsumerProjection {
    RerunCancelConsumerProjection {
        runs_panel_shows_control_class: true,
        run_control_menu_shows_target_scope: true,
        run_control_menu_shows_attribution: true,
        side_effect_sheet_shows_effect_summary: true,
        side_effect_sheet_shows_acknowledgment: true,
        review_workspace_header_shows_context_reuse: true,
        command_palette_shows_effect_summary: true,
        cli_headless_shows_truth: true,
        support_export_shows_truth: true,
        diagnostics_shows_truth: true,
        help_about_shows_truth: true,
        preview_labs_label_for_unqualified: true,
    }
}

fn proof_freshness() -> RerunCancelProofFreshness {
    RerunCancelProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<RerunCancelDowngradeTrigger> {
    vec![
        RerunCancelDowngradeTrigger::ProofStale,
        RerunCancelDowngradeTrigger::ActionAttributionMissing,
        RerunCancelDowngradeTrigger::ContextReuseStale,
        RerunCancelDowngradeTrigger::SideEffectUnreviewed,
        RerunCancelDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<RerunCancelConsumerSurface> {
    vec![
        RerunCancelConsumerSurface::RunsPanel,
        RerunCancelConsumerSurface::RunControlMenu,
        RerunCancelConsumerSurface::SideEffectReviewSheet,
        RerunCancelConsumerSurface::CliHeadless,
        RerunCancelConsumerSurface::SupportExport,
        RerunCancelConsumerSurface::Diagnostics,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        RERUN_CANCEL_SCHEMA_REF.to_owned(),
        RERUN_CANCEL_DOC_REF.to_owned(),
        RERUN_CANCEL_RUN_CONTROL_CONTRACT_REF.to_owned(),
        RERUN_CANCEL_PIPELINE_RUN_CONTRACT_REF.to_owned(),
        RERUN_CANCEL_EXECUTION_CONTEXT_CONTRACT_REF.to_owned(),
        RERUN_CANCEL_TRUST_CLASS_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> RerunCancelReviewPacket {
    RerunCancelReviewPacket::new(RerunCancelReviewPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Attributable rerun / cancel actions with execution-context reuse"
            .to_owned(),
        action_rows: action_rows(),
        side_effect_rows: side_effect_rows(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

#[test]
fn rerun_cancel_review_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_action_rows_fails() {
    let mut packet = packet();
    packet.action_rows.clear();
    assert!(packet
        .validate()
        .contains(&RerunCancelViolation::ActionRowsMissing));
}

#[test]
fn control_scope_mismatch_fails() {
    let mut packet = packet();
    packet.action_rows[0].target_scope = ActionTargetScope::EntireWorkflowRun;
    assert!(packet
        .validate()
        .contains(&RerunCancelViolation::ControlScopeMismatch));
}

#[test]
fn missing_attribution_fails() {
    let mut packet = packet();
    packet.action_rows[0].actor_attribution_label = String::new();
    assert!(packet
        .validate()
        .contains(&RerunCancelViolation::AttributionMissing));
}

#[test]
fn missing_audit_row_ref_fails() {
    let mut packet = packet();
    packet.action_rows[1].audit_row_ref = String::new();
    assert!(packet
        .validate()
        .contains(&RerunCancelViolation::AttributionMissing));
}

#[test]
fn publish_now_without_approval_ref_fails() {
    let mut packet = packet();
    packet.action_rows[0].approval_ticket_ref = None;
    assert!(packet
        .validate()
        .contains(&RerunCancelViolation::MutationGrantRefMissing));
}

#[test]
fn open_in_provider_without_handoff_ref_fails() {
    let mut packet = packet();
    packet.action_rows[1].browser_handoff_ref = None;
    assert!(packet
        .validate()
        .contains(&RerunCancelViolation::MutationGrantRefMissing));
}

#[test]
fn deferred_publish_without_queue_ref_fails() {
    let mut packet = packet();
    packet.action_rows[2].deferred_queue_ref = None;
    assert!(packet
        .validate()
        .contains(&RerunCancelViolation::MutationGrantRefMissing));
}

#[test]
fn stale_context_reuse_without_label_fails() {
    let mut packet = packet();
    packet.action_rows[2].context_staleness_label = String::new();
    assert!(packet
        .validate()
        .contains(&RerunCancelViolation::ContextReuseStaleUnflagged));
}

#[test]
fn blocked_action_without_attention_reason_fails() {
    let mut packet = packet();
    packet.action_rows[2].attention_reasons.clear();
    assert!(packet
        .validate()
        .contains(&RerunCancelViolation::AttentionReasonMissing));
}

#[test]
fn action_without_side_effect_review_fails() {
    let mut packet = packet();
    packet
        .side_effect_rows
        .retain(|row| row.action_id != "action:rerun-failed-login");
    assert!(packet
        .validate()
        .contains(&RerunCancelViolation::ActionMissingSideEffectReview));
}

#[test]
fn orphan_side_effect_reference_fails() {
    let mut packet = packet();
    packet.side_effect_rows[0].action_id = "action:does-not-exist".to_owned();
    assert!(packet
        .validate()
        .contains(&RerunCancelViolation::OrphanRowReference));
}

#[test]
fn non_inert_side_effect_without_acknowledgment_fails() {
    let mut packet = packet();
    packet.side_effect_rows[3].acknowledgment_requirement = SideEffectAckRequirement::NoAckRequired;
    assert!(packet
        .validate()
        .contains(&RerunCancelViolation::SideEffectUnacknowledged));
}

#[test]
fn incomplete_effect_summary_fails() {
    let mut packet = packet();
    packet.action_rows[0].effect_summary.audit_row_label = String::new();
    assert!(packet
        .validate()
        .contains(&RerunCancelViolation::EffectSummaryIncomplete));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&RerunCancelViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_hidden_write_scope = false;
    assert!(packet
        .validate()
        .contains(&RerunCancelViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .side_effect_sheet_shows_acknowledgment = false;
    assert!(packet
        .validate()
        .contains(&RerunCancelViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&RerunCancelViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_section() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Actions"));
    assert!(summary.contains("## Side-effect review"));
    assert!(summary.contains("anchor:review:0001"));
}

#[test]
fn checked_support_export_validates() {
    let packet =
        current_rerun_cancel_review_export().expect("checked rerun/cancel review export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review/context_reuse_stale_blocked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review/unknown_control_provider_owned.json"
        )),
    ] {
        let packet: RerunCancelReviewPacket =
            serde_json::from_str(raw).expect("fixture parses as rerun/cancel review packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
