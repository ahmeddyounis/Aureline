use super::*;

const PACKET_ID: &str = "merge-queue-readiness:stable:0001";

fn readiness_entries() -> Vec<MergeQueueReadinessRow> {
    vec![
        MergeQueueReadinessRow {
            entry_id: "entry:feature-login".to_owned(),
            target_identity_label: "Local branch feature/login vs base main".to_owned(),
            durable_anchor_id: "anchor:review:0001".to_owned(),
            queue_position: 1,
            base_freshness: QueueBaseFreshness::Current,
            readiness_verdict: MergeQueueReadinessVerdict::Ready,
            mutation_authority: MutationAuthorityClass::ReadOnlyNoMutation,
            required_checks_summary: "fmt, clippy, tests all green".to_owned(),
            required_approvals_summary: "2 of 2 approvals current".to_owned(),
            blocking_reasons: Vec::new(),
            source_contract_refs: vec![
                MERGE_QUEUE_READINESS_MERGE_QUEUE_CONTRACT_REF.to_owned(),
                MERGE_QUEUE_READINESS_LANDING_CONTRACT_REF.to_owned(),
            ],
        },
        MergeQueueReadinessRow {
            entry_id: "entry:hotfix-crash".to_owned(),
            target_identity_label: "Local branch hotfix/crash vs base release".to_owned(),
            durable_anchor_id: "anchor:review:0002".to_owned(),
            queue_position: 2,
            base_freshness: QueueBaseFreshness::StaleBase,
            readiness_verdict: MergeQueueReadinessVerdict::BlockedOnStaleBase,
            mutation_authority: MutationAuthorityClass::ReadOnlyNoMutation,
            required_checks_summary: "tests must rerun against the advanced base".to_owned(),
            required_approvals_summary: "1 of 2 approvals remain after recomputation".to_owned(),
            blocking_reasons: vec![
                "Base advanced; checks must rerun".to_owned(),
                "One approval invalidated by the new diff".to_owned(),
            ],
            source_contract_refs: vec![
                MERGE_QUEUE_READINESS_MERGE_QUEUE_CONTRACT_REF.to_owned(),
                MERGE_QUEUE_READINESS_ANCHOR_STABILITY_CONTRACT_REF.to_owned(),
            ],
        },
    ]
}

fn stale_base_invalidations() -> Vec<StaleBaseInvalidationRow> {
    vec![StaleBaseInvalidationRow {
        entry_id: "entry:hotfix-crash".to_owned(),
        base_advance_label: "Base release advanced by 3 commits".to_owned(),
        invalidation_action: StaleBaseInvalidationAction::RequeueAfterRerun,
        recompute_required: true,
        invalidation_label: "Entry requeued; checks rerun against the new base".to_owned(),
    }]
}

fn approval_recomputations() -> Vec<ApprovalRecomputationRow> {
    vec![
        ApprovalRecomputationRow {
            entry_id: "entry:feature-login".to_owned(),
            trigger: RecomputationTrigger::DiffChanged,
            outcome: ApprovalRecomputationOutcome::Retained,
            approvals_before: 2,
            approvals_after: 2,
            recomputation_label: String::new(),
        },
        ApprovalRecomputationRow {
            entry_id: "entry:hotfix-crash".to_owned(),
            trigger: RecomputationTrigger::BaseAdvanced,
            outcome: ApprovalRecomputationOutcome::InvalidatedPartial,
            approvals_before: 2,
            approvals_after: 1,
            recomputation_label: "One approval invalidated because the rebased diff changed"
                .to_owned(),
        },
    ]
}

fn trust_review() -> MergeQueueReadinessTrustReview {
    MergeQueueReadinessTrustReview {
        readiness_verdict_explicit: true,
        readiness_never_overstated: true,
        stale_base_invalidation_labeled_not_hidden: true,
        approval_recompute_labeled_not_hidden: true,
        approval_recomputes_on_base_or_diff_change: true,
        base_freshness_explicit: true,
        target_identity_explicit: true,
        no_hidden_write_scope: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> MergeQueueReadinessConsumerProjection {
    MergeQueueReadinessConsumerProjection {
        merge_queue_shows_readiness_verdict: true,
        merge_queue_shows_base_freshness: true,
        merge_queue_shows_blocking_reasons: true,
        stale_base_shows_invalidation_action: true,
        approval_surface_shows_recomputation: true,
        cli_headless_shows_truth: true,
        support_export_shows_truth: true,
        diagnostics_shows_truth: true,
        help_about_shows_truth: true,
        preview_labs_label_for_unqualified: true,
    }
}

fn proof_freshness() -> MergeQueueReadinessProofFreshness {
    MergeQueueReadinessProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<MergeQueueReadinessDowngradeTrigger> {
    vec![
        MergeQueueReadinessDowngradeTrigger::ProofStale,
        MergeQueueReadinessDowngradeTrigger::StaleBaseUnlabeled,
        MergeQueueReadinessDowngradeTrigger::ApprovalRecomputeUnlabeled,
        MergeQueueReadinessDowngradeTrigger::ReadinessOverstated,
        MergeQueueReadinessDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<MergeQueueReadinessConsumerSurface> {
    vec![
        MergeQueueReadinessConsumerSurface::MergeQueuePanel,
        MergeQueueReadinessConsumerSurface::CliHeadless,
        MergeQueueReadinessConsumerSurface::SupportExport,
        MergeQueueReadinessConsumerSurface::Diagnostics,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        MERGE_QUEUE_READINESS_SCHEMA_REF.to_owned(),
        MERGE_QUEUE_READINESS_DOC_REF.to_owned(),
        MERGE_QUEUE_READINESS_MERGE_QUEUE_CONTRACT_REF.to_owned(),
        MERGE_QUEUE_READINESS_LANDING_CONTRACT_REF.to_owned(),
        MERGE_QUEUE_READINESS_ANCHOR_STABILITY_CONTRACT_REF.to_owned(),
        MERGE_QUEUE_READINESS_PIPELINE_RUN_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> MergeQueueReadinessPacket {
    MergeQueueReadinessPacket::new(MergeQueueReadinessPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Merge-queue readiness, stale-base invalidation, and approval recomputation"
            .to_owned(),
        readiness_entries: readiness_entries(),
        stale_base_invalidations: stale_base_invalidations(),
        approval_recomputations: approval_recomputations(),
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
fn merge_queue_readiness_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_readiness_entries_fails() {
    let mut packet = packet();
    packet.readiness_entries.clear();
    assert!(packet
        .validate()
        .contains(&MergeQueueReadinessViolation::ReadinessEntriesMissing));
}

#[test]
fn non_ready_entry_without_blocking_reason_fails() {
    let mut packet = packet();
    packet.readiness_entries[1].blocking_reasons.clear();
    assert!(packet
        .validate()
        .contains(&MergeQueueReadinessViolation::BlockingReasonMissing));
}

#[test]
fn stale_base_entry_without_invalidation_fails() {
    let mut packet = packet();
    packet
        .stale_base_invalidations
        .retain(|row| row.entry_id != "entry:hotfix-crash");
    // Keep at least one stale-base invalidation row so the missing-rows check
    // does not mask the per-entry invariant.
    packet
        .stale_base_invalidations
        .push(StaleBaseInvalidationRow {
            entry_id: "entry:feature-login".to_owned(),
            base_advance_label: "Base main advanced by 1 commit".to_owned(),
            invalidation_action: StaleBaseInvalidationAction::NoActionNeeded,
            recompute_required: false,
            invalidation_label: String::new(),
        });
    assert!(packet
        .validate()
        .contains(&MergeQueueReadinessViolation::StaleBaseEntryMissingInvalidation));
}

#[test]
fn entry_without_approval_recomputation_fails() {
    let mut packet = packet();
    packet
        .approval_recomputations
        .retain(|row| row.entry_id != "entry:feature-login");
    assert!(packet
        .validate()
        .contains(&MergeQueueReadinessViolation::EntryMissingApprovalRecomputation));
}

#[test]
fn invalidating_action_without_label_fails() {
    let mut packet = packet();
    packet.stale_base_invalidations[0].invalidation_label = String::new();
    assert!(packet
        .validate()
        .contains(&MergeQueueReadinessViolation::InvalidationLabelMissing));
}

#[test]
fn label_required_recomputation_without_label_fails() {
    let mut packet = packet();
    packet.approval_recomputations[1].recomputation_label = String::new();
    assert!(packet
        .validate()
        .contains(&MergeQueueReadinessViolation::RecomputationLabelMissing));
}

#[test]
fn invalidating_recomputation_that_increases_count_fails() {
    let mut packet = packet();
    packet.approval_recomputations[1].approvals_before = 1;
    packet.approval_recomputations[1].approvals_after = 2;
    assert!(packet
        .validate()
        .contains(&MergeQueueReadinessViolation::ApprovalCountInconsistent));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&MergeQueueReadinessViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_hidden_write_scope = false;
    assert!(packet
        .validate()
        .contains(&MergeQueueReadinessViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .stale_base_shows_invalidation_action = false;
    assert!(packet
        .validate()
        .contains(&MergeQueueReadinessViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&MergeQueueReadinessViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_section() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Readiness"));
    assert!(summary.contains("## Stale-base invalidation"));
    assert!(summary.contains("## Approval recomputation"));
    assert!(summary.contains("anchor:review:0001"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_merge_queue_readiness_export()
        .expect("checked merge-queue readiness export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows/stale_base_eject_to_author.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows/approval_reset_offline.json"
        )),
    ] {
        let packet: MergeQueueReadinessPacket =
            serde_json::from_str(raw).expect("fixture parses as merge-queue readiness packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
