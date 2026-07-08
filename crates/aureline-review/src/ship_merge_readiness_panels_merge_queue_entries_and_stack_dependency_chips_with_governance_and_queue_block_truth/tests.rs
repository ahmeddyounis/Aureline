use super::*;

const PACKET_ID: &str = "merge-readiness-panel:stable:0001";

fn trust_review() -> MergeReadinessTrustReview {
    MergeReadinessTrustReview {
        provider_local_estimate_distinct: true,
        queue_owner_always_explicit: true,
        blocked_reason_never_generic_warning: true,
        stale_base_labeled_not_hidden: true,
        approval_recomputation_explicit: true,
        stack_blocking_explicit: true,
        auto_merge_scope_explicit: true,
        provider_outage_preserves_local_continuation: true,
        stale_sync_never_collapses_review_lane: true,
        no_forced_raw_provider_navigation_for_triage: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> MergeReadinessConsumerProjection {
    MergeReadinessConsumerProjection {
        review_workspace_reuses_one_panel_contract: true,
        review_list_reuses_one_panel_contract: true,
        companion_queue_reuses_one_panel_contract: true,
        panel_distinguishes_all_governance_kinds: true,
        queue_result_authority_readable_without_raw_provider: true,
        cli_headless_shows_truth: true,
        support_export_shows_truth: true,
        diagnostics_shows_truth: true,
        help_about_shows_truth: true,
        export_preserves_queue_and_stack_identity: true,
    }
}

fn proof_freshness() -> MergeReadinessProofFreshness {
    MergeReadinessProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<MergeReadinessDowngradeTrigger> {
    vec![
        MergeReadinessDowngradeTrigger::ProofStale,
        MergeReadinessDowngradeTrigger::ProviderFreshnessStale,
        MergeReadinessDowngradeTrigger::StaleBaseUnlabeled,
        MergeReadinessDowngradeTrigger::ApprovalRecomputePending,
        MergeReadinessDowngradeTrigger::StackParentBlocked,
        MergeReadinessDowngradeTrigger::BrowserHandoffUnavailable,
        MergeReadinessDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<MergeReadinessConsumerSurface> {
    vec![
        MergeReadinessConsumerSurface::ReviewWorkspace,
        MergeReadinessConsumerSurface::ReviewList,
        MergeReadinessConsumerSurface::CompanionQueue,
        MergeReadinessConsumerSurface::CliHeadless,
        MergeReadinessConsumerSurface::SupportExport,
        MergeReadinessConsumerSurface::MergeQueueDrawer,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        MERGE_READINESS_PANEL_SCHEMA_REF.to_owned(),
        MERGE_READINESS_PANEL_DOC_REF.to_owned(),
        MERGE_READINESS_PANEL_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        MERGE_READINESS_PANEL_MERGE_QUEUE_ENTRY_CONTRACT_REF.to_owned(),
        MERGE_READINESS_PANEL_REVIEW_WORKSPACE_CONTRACT_REF.to_owned(),
        MERGE_READINESS_PANEL_LANDING_CANDIDATE_CONTRACT_REF.to_owned(),
        MERGE_READINESS_PANEL_PATCH_STACK_CONTRACT_REF.to_owned(),
        MERGE_READINESS_PANEL_CHANGE_LINEAGE_CONTRACT_REF.to_owned(),
    ]
}

fn panel_refs() -> Vec<String> {
    vec![
        MERGE_READINESS_PANEL_MERGE_QUEUE_ENTRY_CONTRACT_REF.to_owned(),
        MERGE_READINESS_PANEL_LANDING_CANDIDATE_CONTRACT_REF.to_owned(),
    ]
}

fn panels() -> Vec<MergeReadinessPanel> {
    vec![
        // 1. Provider-managed, fresh, ready → Authoritative (the only panel that may claim it).
        MergeReadinessPanel {
            panel_id: "panel:pr-5001".to_owned(),
            review_id_label: "PR #5001".to_owned(),
            queue_owner_label: "provider merge queue org/repo".to_owned(),
            governance: QueueGovernance::ProviderManaged,
            provider_freshness: M5ReviewComponentStaleProviderState::ProviderFresh,
            readiness_state: MergeReadinessState::ReadyToMerge,
            claims_authoritative: true,
            headline_readiness_label: "Ready to merge — 1st in the provider queue".to_owned(),
            blocked_reason_detail: String::new(),
            stale_base_note: String::new(),
            approval_recomputation_note: String::new(),
            auto_merge_scope: AutoMergeScope::ThisEntryOnly,
            queue_entries: vec![
                MergeQueueEntry {
                    entry_id: "entry:pr-5001".to_owned(),
                    entry_label: "PR #5001".to_owned(),
                    queue_position_label: "#1 in queue".to_owned(),
                    entry_state: MergeReadinessState::ReadyToMerge,
                    is_this_change: true,
                    blocked_reason_detail: String::new(),
                },
                MergeQueueEntry {
                    entry_id: "entry:pr-4998".to_owned(),
                    entry_label: "PR #4998".to_owned(),
                    queue_position_label: "#2 in queue".to_owned(),
                    entry_state: MergeReadinessState::QueuedWaiting,
                    is_this_change: false,
                    blocked_reason_detail: String::new(),
                },
            ],
            stack_chips: vec![StackDependencyChip {
                chip_id: "chip:pr-5001".to_owned(),
                stack_id_label: "no stack".to_owned(),
                position_label: "standalone".to_owned(),
                relation: StackDependencyState::Standalone,
                blocking_note: String::new(),
            }],
            actions: vec![
                MergeReadinessAction::MergeNow,
                MergeReadinessAction::ExportReadinessPacket,
            ],
            browser_handoff_boundary: String::new(),
            local_continue_fallback: String::new(),
            source_contract_refs: panel_refs(),
        },
        // 2. Local estimate, fresh, queued → Estimated (must not claim authoritative).
        MergeReadinessPanel {
            panel_id: "panel:pr-5002".to_owned(),
            review_id_label: "PR #5002".to_owned(),
            queue_owner_label: "Aureline local queue estimate".to_owned(),
            governance: QueueGovernance::AurelineLocalEstimate,
            provider_freshness: M5ReviewComponentStaleProviderState::ProviderFresh,
            readiness_state: MergeReadinessState::QueuedWaiting,
            claims_authoritative: false,
            headline_readiness_label: "Estimated queue position; not authoritative".to_owned(),
            blocked_reason_detail: String::new(),
            stale_base_note: String::new(),
            approval_recomputation_note: String::new(),
            auto_merge_scope: AutoMergeScope::NotEnabled,
            queue_entries: vec![MergeQueueEntry {
                entry_id: "entry:pr-5002".to_owned(),
                entry_label: "PR #5002".to_owned(),
                queue_position_label: "~#3 estimated".to_owned(),
                entry_state: MergeReadinessState::QueuedWaiting,
                is_this_change: true,
                blocked_reason_detail: String::new(),
            }],
            stack_chips: vec![StackDependencyChip {
                chip_id: "chip:pr-5002".to_owned(),
                stack_id_label: "stack feature-api".to_owned(),
                position_label: "3 of 4".to_owned(),
                relation: StackDependencyState::StackChildPending,
                blocking_note: String::new(),
            }],
            actions: vec![
                MergeReadinessAction::EnqueueForMerge,
                MergeReadinessAction::ExportReadinessPacket,
            ],
            browser_handoff_boundary: String::new(),
            local_continue_fallback: String::new(),
            source_contract_refs: panel_refs(),
        },
        // 3. Provider-managed but unreachable → Stale, needs handoff + local continue.
        MergeReadinessPanel {
            panel_id: "panel:pr-5003".to_owned(),
            review_id_label: "PR #5003".to_owned(),
            queue_owner_label: "provider merge queue org/repo (unreachable)".to_owned(),
            governance: QueueGovernance::ProviderManaged,
            provider_freshness: M5ReviewComponentStaleProviderState::ProviderUnreachable,
            readiness_state: MergeReadinessState::QueuedWaiting,
            claims_authoritative: false,
            headline_readiness_label: "Provider unreachable; last-known queue position below"
                .to_owned(),
            blocked_reason_detail: String::new(),
            stale_base_note: String::new(),
            approval_recomputation_note: String::new(),
            auto_merge_scope: AutoMergeScope::ThisEntryOnly,
            queue_entries: vec![MergeQueueEntry {
                entry_id: "entry:pr-5003".to_owned(),
                entry_label: "PR #5003".to_owned(),
                queue_position_label: "#4 (last known)".to_owned(),
                entry_state: MergeReadinessState::QueuedWaiting,
                is_this_change: true,
                blocked_reason_detail: String::new(),
            }],
            stack_chips: vec![StackDependencyChip {
                chip_id: "chip:pr-5003".to_owned(),
                stack_id_label: "stack feature-api".to_owned(),
                position_label: "root".to_owned(),
                relation: StackDependencyState::StackRootReady,
                blocking_note: String::new(),
            }],
            actions: vec![
                MergeReadinessAction::ContinueLocalReview,
                MergeReadinessAction::OpenProviderInBrowser,
                MergeReadinessAction::ExportReadinessPacket,
            ],
            browser_handoff_boundary:
                "Provider is unreachable; open in browser to inspect the hosted merge queue"
                    .to_owned(),
            local_continue_fallback:
                "Keep reviewing from the local queue estimate while the provider is unreachable"
                    .to_owned(),
            source_contract_refs: panel_refs(),
        },
        // 4. Repo-policy-managed, blocked on a stale base → Blocked, stale-base note + stack parent.
        MergeReadinessPanel {
            panel_id: "panel:pr-5004".to_owned(),
            review_id_label: "PR #5004".to_owned(),
            queue_owner_label: "repo merge policy main".to_owned(),
            governance: QueueGovernance::RepoPolicyManaged,
            provider_freshness: M5ReviewComponentStaleProviderState::ProviderFresh,
            readiness_state: MergeReadinessState::BlockedOnStaleBase,
            claims_authoritative: false,
            headline_readiness_label: "Blocked: base advanced, this change is stale".to_owned(),
            blocked_reason_detail: "Base advanced by 12 commits after this change was approved"
                .to_owned(),
            stale_base_note: "Restack onto main@a1b2c3 and rerun required checks".to_owned(),
            approval_recomputation_note: String::new(),
            auto_merge_scope: AutoMergeScope::QueueBatch,
            queue_entries: vec![MergeQueueEntry {
                entry_id: "entry:pr-5004".to_owned(),
                entry_label: "PR #5004".to_owned(),
                queue_position_label: "ejected".to_owned(),
                entry_state: MergeReadinessState::BlockedOnStaleBase,
                is_this_change: true,
                blocked_reason_detail: "Ejected from the queue when the base advanced".to_owned(),
            }],
            stack_chips: vec![StackDependencyChip {
                chip_id: "chip:pr-5004".to_owned(),
                stack_id_label: "stack payments".to_owned(),
                position_label: "2 of 3".to_owned(),
                relation: StackDependencyState::StackParentBlocked,
                blocking_note: "Parent PR #5000 is blocked on a stale base; resolve it first"
                    .to_owned(),
            }],
            actions: vec![
                MergeReadinessAction::RestackOntoBase,
                MergeReadinessAction::RequeueAfterRerun,
                MergeReadinessAction::ExportReadinessPacket,
            ],
            browser_handoff_boundary: String::new(),
            local_continue_fallback: String::new(),
            source_contract_refs: panel_refs(),
        },
        // 5. Repo-policy-managed, blocked on approval recomputation → Blocked, recomputation note.
        MergeReadinessPanel {
            panel_id: "panel:pr-5005".to_owned(),
            review_id_label: "PR #5005".to_owned(),
            queue_owner_label: "repo merge policy main".to_owned(),
            governance: QueueGovernance::RepoPolicyManaged,
            provider_freshness: M5ReviewComponentStaleProviderState::ProviderFresh,
            readiness_state: MergeReadinessState::BlockedOnApprovalRecomputation,
            claims_authoritative: false,
            headline_readiness_label: "Blocked: approvals invalidated, recomputation pending"
                .to_owned(),
            blocked_reason_detail: "Diff changed under existing approvals".to_owned(),
            stale_base_note: String::new(),
            approval_recomputation_note:
                "2 of 3 approvals invalidated by the new diff; re-request required".to_owned(),
            auto_merge_scope: AutoMergeScope::NotEnabled,
            queue_entries: vec![MergeQueueEntry {
                entry_id: "entry:pr-5005".to_owned(),
                entry_label: "PR #5005".to_owned(),
                queue_position_label: "held".to_owned(),
                entry_state: MergeReadinessState::BlockedOnApprovalRecomputation,
                is_this_change: true,
                blocked_reason_detail: "Held until approvals are recomputed".to_owned(),
            }],
            stack_chips: vec![StackDependencyChip {
                chip_id: "chip:pr-5005".to_owned(),
                stack_id_label: "no stack".to_owned(),
                position_label: "standalone".to_owned(),
                relation: StackDependencyState::Standalone,
                blocking_note: String::new(),
            }],
            actions: vec![
                MergeReadinessAction::RecomputeApprovals,
                MergeReadinessAction::ExportReadinessPacket,
            ],
            browser_handoff_boundary: String::new(),
            local_continue_fallback: String::new(),
            source_contract_refs: panel_refs(),
        },
    ]
}

fn packet() -> MergeReadinessPanelPacket {
    MergeReadinessPanelPacket::new(MergeReadinessPanelPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Merge-readiness panels: governance and queue-block truth".to_owned(),
        panels: panels(),
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
fn merge_readiness_panel_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn resolver_derives_authority_across_governance_and_state() {
    // A fresh, provider-managed, ready panel is authoritative and may claim it.
    let authoritative = resolve_merge_readiness_disclosure(
        QueueGovernance::ProviderManaged,
        M5ReviewComponentStaleProviderState::ProviderFresh,
        MergeReadinessState::ReadyToMerge,
    );
    assert_eq!(authoritative.authority, QueueResultAuthority::Authoritative);
    assert!(authoritative.may_claim_authoritative);

    // A fresh, non-blocked local estimate is estimated and must not claim authoritative.
    let estimated = resolve_merge_readiness_disclosure(
        QueueGovernance::AurelineLocalEstimate,
        M5ReviewComponentStaleProviderState::ProviderFresh,
        MergeReadinessState::QueuedWaiting,
    );
    assert_eq!(estimated.authority, QueueResultAuthority::Estimated);
    assert!(!estimated.may_claim_authoritative);

    // A degraded provider is stale and forces a local-continue path.
    let stale = resolve_merge_readiness_disclosure(
        QueueGovernance::ProviderManaged,
        M5ReviewComponentStaleProviderState::ProviderUnreachable,
        MergeReadinessState::QueuedWaiting,
    );
    assert_eq!(stale.authority, QueueResultAuthority::Stale);
    assert!(stale.needs_browser_handoff_boundary);
    assert!(stale.needs_local_continue_fallback);
    assert!(!stale.may_claim_authoritative);

    // A blocked state reads as blocked even when the source would otherwise be authoritative.
    let blocked = resolve_merge_readiness_disclosure(
        QueueGovernance::ProviderManaged,
        M5ReviewComponentStaleProviderState::ProviderFresh,
        MergeReadinessState::BlockedOnStaleBase,
    );
    assert_eq!(blocked.authority, QueueResultAuthority::Blocked);
    assert!(blocked.needs_blocked_reason);
    assert!(!blocked.may_claim_authoritative);
}

#[test]
fn local_estimate_claiming_authoritative_fails() {
    let mut packet = packet();
    // Panel 2 is a local estimate; it must not claim authoritative.
    packet.panels[1].claims_authoritative = true;
    assert!(packet
        .validate()
        .contains(&MergeReadinessPanelViolation::AuthorityMisrepresented));
}

#[test]
fn authoritative_result_understated_as_estimate_fails() {
    let mut packet = packet();
    // Panel 1 is authoritative; dropping the claim understates provider truth.
    packet.panels[0].claims_authoritative = false;
    assert!(packet
        .validate()
        .contains(&MergeReadinessPanelViolation::AuthorityMisrepresented));
}

#[test]
fn blocked_panel_without_reason_fails() {
    let mut packet = packet();
    packet.panels[3].blocked_reason_detail = String::new();
    assert!(packet
        .validate()
        .contains(&MergeReadinessPanelViolation::BlockedReasonMissing));
}

#[test]
fn stale_base_without_note_fails() {
    let mut packet = packet();
    packet.panels[3].stale_base_note = String::new();
    assert!(packet
        .validate()
        .contains(&MergeReadinessPanelViolation::StaleBaseNoteMissing));
}

#[test]
fn approval_recomputation_without_note_fails() {
    let mut packet = packet();
    packet.panels[4].approval_recomputation_note = String::new();
    assert!(packet
        .validate()
        .contains(&MergeReadinessPanelViolation::ApprovalRecomputationNoteMissing));
}

#[test]
fn blocked_queue_entry_without_reason_fails() {
    let mut packet = packet();
    packet.panels[3].queue_entries[0].blocked_reason_detail = String::new();
    assert!(packet
        .validate()
        .contains(&MergeReadinessPanelViolation::QueueEntryBlockedReasonMissing));
}

#[test]
fn stack_parent_blocked_without_note_fails() {
    let mut packet = packet();
    packet.panels[3].stack_chips[0].blocking_note = String::new();
    assert!(packet
        .validate()
        .contains(&MergeReadinessPanelViolation::StackBlockingNoteMissing));
}

#[test]
fn missing_browser_handoff_boundary_fails() {
    let mut packet = packet();
    packet.panels[2].browser_handoff_boundary = String::new();
    assert!(packet
        .validate()
        .contains(&MergeReadinessPanelViolation::BrowserHandoffBoundaryMissing));
}

#[test]
fn missing_local_continue_fallback_fails() {
    let mut packet = packet();
    packet.panels[2].local_continue_fallback = String::new();
    assert!(packet
        .validate()
        .contains(&MergeReadinessPanelViolation::LocalContinueFallbackMissing));
}

#[test]
fn forced_raw_provider_navigation_fails() {
    let mut packet = packet();
    packet.panels[2].actions = vec![MergeReadinessAction::OpenProviderInBrowser];
    assert!(packet
        .validate()
        .contains(&MergeReadinessPanelViolation::ForcedRawProviderNavigation));
}

#[test]
fn missing_governance_coverage_fails() {
    let mut packet = packet();
    // Force every panel to provider-managed so repo-policy and local-estimate vanish.
    for panel in &mut packet.panels {
        panel.governance = QueueGovernance::ProviderManaged;
        // Keep authority claims consistent so only the coverage violation is asserted.
        panel.claims_authoritative = panel.disclosure().may_claim_authoritative;
    }
    assert!(packet
        .validate()
        .contains(&MergeReadinessPanelViolation::GovernanceCoverageMissing));
}

#[test]
fn missing_panels_fails() {
    let mut packet = packet();
    packet.panels.clear();
    assert!(packet
        .validate()
        .contains(&MergeReadinessPanelViolation::PanelsMissing));
}

#[test]
fn incomplete_panel_fails() {
    let mut packet = packet();
    packet.panels[0].queue_owner_label = String::new();
    assert!(packet
        .validate()
        .contains(&MergeReadinessPanelViolation::PanelIncomplete));
}

#[test]
fn incomplete_queue_entry_fails() {
    let mut packet = packet();
    packet.panels[0].queue_entries[0].queue_position_label = String::new();
    assert!(packet
        .validate()
        .contains(&MergeReadinessPanelViolation::QueueEntryIncomplete));
}

#[test]
fn incomplete_stack_chip_fails() {
    let mut packet = packet();
    packet.panels[0].stack_chips[0].stack_id_label = String::new();
    assert!(packet
        .validate()
        .contains(&MergeReadinessPanelViolation::StackChipIncomplete));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&MergeReadinessPanelViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.stale_base_labeled_not_hidden = false;
    assert!(packet
        .validate()
        .contains(&MergeReadinessPanelViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .queue_result_authority_readable_without_raw_provider = false;
    assert!(packet
        .validate()
        .contains(&MergeReadinessPanelViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&MergeReadinessPanelViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_panels() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Panels"));
    assert!(summary.contains("PR #5001"));
    assert!(summary.contains("aureline_local_estimate"));
    assert!(summary.contains("blocked_on_stale_base"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_merge_readiness_panel_export()
        .expect("checked merge-readiness panel export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-merge-readiness-panels/provider_stale_local_continue.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-merge-readiness-panels/stack_blocked_and_approval_recompute.json"
        )),
    ] {
        let packet: MergeReadinessPanelPacket =
            serde_json::from_str(raw).expect("fixture parses as merge-readiness panel packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

// --- Fixture builders ---------------------------------------------------------

fn fixture_provider_stale_local_continue() -> MergeReadinessPanelPacket {
    let panels = vec![
        MergeReadinessPanel {
            panel_id: "panel:pr-6101".to_owned(),
            review_id_label: "PR #6101".to_owned(),
            queue_owner_label: "provider merge queue org/repo (stale)".to_owned(),
            governance: QueueGovernance::ProviderManaged,
            provider_freshness: M5ReviewComponentStaleProviderState::ProviderStale,
            readiness_state: MergeReadinessState::QueuedWaiting,
            claims_authoritative: false,
            headline_readiness_label: "Provider queue truth is stale; local estimate shown"
                .to_owned(),
            blocked_reason_detail: String::new(),
            stale_base_note: String::new(),
            approval_recomputation_note: String::new(),
            auto_merge_scope: AutoMergeScope::ThisEntryOnly,
            queue_entries: vec![MergeQueueEntry {
                entry_id: "entry:pr-6101".to_owned(),
                entry_label: "PR #6101".to_owned(),
                queue_position_label: "#2 (last synced)".to_owned(),
                entry_state: MergeReadinessState::QueuedWaiting,
                is_this_change: true,
                blocked_reason_detail: String::new(),
            }],
            stack_chips: vec![StackDependencyChip {
                chip_id: "chip:pr-6101".to_owned(),
                stack_id_label: "no stack".to_owned(),
                position_label: "standalone".to_owned(),
                relation: StackDependencyState::Standalone,
                blocking_note: String::new(),
            }],
            actions: vec![
                MergeReadinessAction::ContinueLocalReview,
                MergeReadinessAction::ExportReadinessPacket,
            ],
            browser_handoff_boundary: String::new(),
            local_continue_fallback:
                "Keep reviewing from the local queue estimate while provider truth refreshes"
                    .to_owned(),
            source_contract_refs: panel_refs(),
        },
        MergeReadinessPanel {
            panel_id: "panel:pr-6102".to_owned(),
            review_id_label: "PR #6102".to_owned(),
            queue_owner_label: "repo merge policy main".to_owned(),
            governance: QueueGovernance::RepoPolicyManaged,
            provider_freshness: M5ReviewComponentStaleProviderState::ProviderFresh,
            readiness_state: MergeReadinessState::ReadyToMerge,
            claims_authoritative: true,
            headline_readiness_label: "Ready to merge under repo policy".to_owned(),
            blocked_reason_detail: String::new(),
            stale_base_note: String::new(),
            approval_recomputation_note: String::new(),
            auto_merge_scope: AutoMergeScope::QueueBatch,
            queue_entries: vec![MergeQueueEntry {
                entry_id: "entry:pr-6102".to_owned(),
                entry_label: "PR #6102".to_owned(),
                queue_position_label: "#1 in queue".to_owned(),
                entry_state: MergeReadinessState::ReadyToMerge,
                is_this_change: true,
                blocked_reason_detail: String::new(),
            }],
            stack_chips: vec![],
            actions: vec![
                MergeReadinessAction::MergeNow,
                MergeReadinessAction::ExportReadinessPacket,
            ],
            browser_handoff_boundary: String::new(),
            local_continue_fallback: String::new(),
            source_contract_refs: panel_refs(),
        },
        MergeReadinessPanel {
            panel_id: "panel:pr-6103".to_owned(),
            review_id_label: "PR #6103".to_owned(),
            queue_owner_label: "Aureline local queue estimate".to_owned(),
            governance: QueueGovernance::AurelineLocalEstimate,
            provider_freshness: M5ReviewComponentStaleProviderState::LocalOnlyContinuation,
            readiness_state: MergeReadinessState::QueuedWaiting,
            claims_authoritative: false,
            headline_readiness_label: "Local-only estimate while provider sync is degraded"
                .to_owned(),
            blocked_reason_detail: String::new(),
            stale_base_note: String::new(),
            approval_recomputation_note: String::new(),
            auto_merge_scope: AutoMergeScope::NotEnabled,
            queue_entries: vec![MergeQueueEntry {
                entry_id: "entry:pr-6103".to_owned(),
                entry_label: "PR #6103".to_owned(),
                queue_position_label: "~#5 estimated".to_owned(),
                entry_state: MergeReadinessState::QueuedWaiting,
                is_this_change: true,
                blocked_reason_detail: String::new(),
            }],
            stack_chips: vec![StackDependencyChip {
                chip_id: "chip:pr-6103".to_owned(),
                stack_id_label: "stack tooling".to_owned(),
                position_label: "1 of 2".to_owned(),
                relation: StackDependencyState::StackRootReady,
                blocking_note: String::new(),
            }],
            actions: vec![
                MergeReadinessAction::ContinueLocalReview,
                MergeReadinessAction::ExportReadinessPacket,
            ],
            browser_handoff_boundary: String::new(),
            local_continue_fallback:
                "Continue from the local estimate; provider queue truth is unavailable".to_owned(),
            source_contract_refs: panel_refs(),
        },
    ];

    MergeReadinessPanelPacket::new(MergeReadinessPanelPacketInput {
        packet_id: "merge-readiness-panel:fixture:provider-stale-local-continue".to_owned(),
        surface_label: "Merge-readiness panels: provider stale, local continuation".to_owned(),
        panels,
        downgrade_triggers: vec![
            MergeReadinessDowngradeTrigger::ProviderFreshnessStale,
            MergeReadinessDowngradeTrigger::UpstreamDependencyNarrowed,
        ],
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

fn fixture_stack_blocked_and_approval_recompute() -> MergeReadinessPanelPacket {
    let panels = vec![
        MergeReadinessPanel {
            panel_id: "panel:pr-6201".to_owned(),
            review_id_label: "PR #6201".to_owned(),
            queue_owner_label: "repo merge policy main".to_owned(),
            governance: QueueGovernance::RepoPolicyManaged,
            provider_freshness: M5ReviewComponentStaleProviderState::ProviderFresh,
            readiness_state: MergeReadinessState::BlockedOnStackParent,
            claims_authoritative: false,
            headline_readiness_label: "Blocked: stack parent is not landing".to_owned(),
            blocked_reason_detail: "Parent change is blocked on failing required checks".to_owned(),
            stale_base_note: String::new(),
            approval_recomputation_note: String::new(),
            auto_merge_scope: AutoMergeScope::WholeStack,
            queue_entries: vec![MergeQueueEntry {
                entry_id: "entry:pr-6201".to_owned(),
                entry_label: "PR #6201".to_owned(),
                queue_position_label: "waiting on parent".to_owned(),
                entry_state: MergeReadinessState::BlockedOnStackParent,
                is_this_change: true,
                blocked_reason_detail: "Cannot land ahead of its blocked parent".to_owned(),
            }],
            stack_chips: vec![StackDependencyChip {
                chip_id: "chip:pr-6201".to_owned(),
                stack_id_label: "stack checkout".to_owned(),
                position_label: "3 of 3".to_owned(),
                relation: StackDependencyState::StackParentBlocked,
                blocking_note: "Parent PR #6200 is blocked on failing checks; resolve it first"
                    .to_owned(),
            }],
            actions: vec![
                MergeReadinessAction::RestackOntoBase,
                MergeReadinessAction::ExportReadinessPacket,
            ],
            browser_handoff_boundary: String::new(),
            local_continue_fallback: String::new(),
            source_contract_refs: panel_refs(),
        },
        MergeReadinessPanel {
            panel_id: "panel:pr-6202".to_owned(),
            review_id_label: "PR #6202".to_owned(),
            queue_owner_label: "provider merge queue org/repo".to_owned(),
            governance: QueueGovernance::ProviderManaged,
            provider_freshness: M5ReviewComponentStaleProviderState::ProviderFresh,
            readiness_state: MergeReadinessState::BlockedOnApprovalRecomputation,
            claims_authoritative: false,
            headline_readiness_label: "Blocked: approvals recomputed after a force-push".to_owned(),
            blocked_reason_detail: "Head was force-pushed under existing approvals".to_owned(),
            stale_base_note: String::new(),
            approval_recomputation_note:
                "All 2 approvals reset by the rewritten head; re-request required".to_owned(),
            auto_merge_scope: AutoMergeScope::NotEnabled,
            queue_entries: vec![MergeQueueEntry {
                entry_id: "entry:pr-6202".to_owned(),
                entry_label: "PR #6202".to_owned(),
                queue_position_label: "held".to_owned(),
                entry_state: MergeReadinessState::BlockedOnApprovalRecomputation,
                is_this_change: true,
                blocked_reason_detail: "Held until approvals are re-collected".to_owned(),
            }],
            stack_chips: vec![],
            actions: vec![
                MergeReadinessAction::RecomputeApprovals,
                MergeReadinessAction::OpenProviderInBrowser,
                MergeReadinessAction::ExportReadinessPacket,
            ],
            browser_handoff_boundary: String::new(),
            local_continue_fallback: String::new(),
            source_contract_refs: panel_refs(),
        },
        MergeReadinessPanel {
            panel_id: "panel:pr-6203".to_owned(),
            review_id_label: "PR #6203".to_owned(),
            queue_owner_label: "Aureline local queue estimate (provider unreachable)".to_owned(),
            governance: QueueGovernance::AurelineLocalEstimate,
            provider_freshness: M5ReviewComponentStaleProviderState::ProviderUnreachable,
            readiness_state: MergeReadinessState::QueuedWaiting,
            claims_authoritative: false,
            headline_readiness_label: "Provider unreachable; local estimate only".to_owned(),
            blocked_reason_detail: String::new(),
            stale_base_note: String::new(),
            approval_recomputation_note: String::new(),
            auto_merge_scope: AutoMergeScope::NotEnabled,
            queue_entries: vec![MergeQueueEntry {
                entry_id: "entry:pr-6203".to_owned(),
                entry_label: "PR #6203".to_owned(),
                queue_position_label: "~#6 estimated".to_owned(),
                entry_state: MergeReadinessState::QueuedWaiting,
                is_this_change: true,
                blocked_reason_detail: String::new(),
            }],
            stack_chips: vec![StackDependencyChip {
                chip_id: "chip:pr-6203".to_owned(),
                stack_id_label: "no stack".to_owned(),
                position_label: "standalone".to_owned(),
                relation: StackDependencyState::Standalone,
                blocking_note: String::new(),
            }],
            actions: vec![
                MergeReadinessAction::ContinueLocalReview,
                MergeReadinessAction::OpenProviderInBrowser,
                MergeReadinessAction::ExportReadinessPacket,
            ],
            browser_handoff_boundary:
                "Provider is unreachable; open in browser to inspect the hosted merge queue"
                    .to_owned(),
            local_continue_fallback:
                "Keep reviewing from the local estimate while the provider is unreachable"
                    .to_owned(),
            source_contract_refs: panel_refs(),
        },
    ];

    MergeReadinessPanelPacket::new(MergeReadinessPanelPacketInput {
        packet_id: "merge-readiness-panel:fixture:stack-blocked-approval-recompute".to_owned(),
        surface_label: "Merge-readiness panels: stack blocked and approval recomputation"
            .to_owned(),
        panels,
        downgrade_triggers: vec![
            MergeReadinessDowngradeTrigger::StackParentBlocked,
            MergeReadinessDowngradeTrigger::ApprovalRecomputePending,
            MergeReadinessDowngradeTrigger::BrowserHandoffUnavailable,
        ],
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

/// Regenerates the checked-in support export, summary, and fixtures.
///
/// Gated behind `GEN_MERGE_READINESS_ARTIFACTS` so it never writes during a normal
/// test run. Run with the env var set to refresh the artifacts after a contract
/// change, then review the diff.
#[test]
fn regenerate_merge_readiness_artifacts() {
    if std::env::var("GEN_MERGE_READINESS_ARTIFACTS").is_err() {
        return;
    }

    let manifest = env!("CARGO_MANIFEST_DIR");
    let root = format!("{manifest}/../..");

    let canonical = packet();
    assert!(
        canonical.validate().is_empty(),
        "{:?}",
        canonical.validate()
    );

    let artifact_dir = format!(
        "{root}/artifacts/review/m5/ship_merge_readiness_panels_merge_queue_entries_and_stack_dependency_chips_with_governance_and_queue_block_truth"
    );
    std::fs::create_dir_all(&artifact_dir).expect("create artifact dir");
    std::fs::write(
        format!("{artifact_dir}/support_export.json"),
        format!("{}\n", canonical.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        format!("{root}/artifacts/review/m5/ship_merge_readiness_panels_merge_queue_entries_and_stack_dependency_chips_with_governance_and_queue_block_truth.md"),
        canonical.render_markdown_summary(),
    )
    .expect("write summary");

    let fixture_dir = format!("{root}/fixtures/ui/m5-merge-readiness-panels");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    for (name, fixture) in [
        (
            "provider_stale_local_continue.json",
            fixture_provider_stale_local_continue(),
        ),
        (
            "stack_blocked_and_approval_recompute.json",
            fixture_stack_blocked_and_approval_recompute(),
        ),
    ] {
        assert!(
            fixture.validate().is_empty(),
            "{name}: {:?}",
            fixture.validate()
        );
        std::fs::write(
            format!("{fixture_dir}/{name}"),
            format!("{}\n", fixture.export_safe_json()),
        )
        .expect("write fixture");
    }
}
