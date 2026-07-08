use super::*;

const PACKET_ID: &str = "pending-review-approval:stable:0001";

fn trust_review() -> PendingReviewApprovalTrustReview {
    PendingReviewApprovalTrustReview {
        approval_invalidation_kept_separate: true,
        generic_warning_never_masks_invalidation: true,
        queue_block_never_masks_invalidation: true,
        reviewer_scope_always_explicit: true,
        next_action_verb_always_explicit: true,
        local_drafts_visible_under_degraded_provider: true,
        publish_later_packets_visible_under_degraded_provider: true,
        prior_approval_state_preserved_on_invalidation: true,
        reopen_safe_follow_up_preserved: true,
        no_forced_raw_provider_navigation_for_triage: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> PendingReviewApprovalConsumerProjection {
    PendingReviewApprovalConsumerProjection {
        review_workspace_reuses_one_contract: true,
        review_list_reuses_one_contract: true,
        companion_queue_reuses_one_contract: true,
        tray_distinguishes_reviewer_scope: true,
        banner_distinguishes_invalidation_cause: true,
        cli_headless_shows_truth: true,
        support_export_shows_truth: true,
        diagnostics_shows_truth: true,
        help_about_shows_truth: true,
        export_preserves_reviewer_and_invalidation_identity: true,
    }
}

fn proof_freshness() -> PendingReviewApprovalProofFreshness {
    PendingReviewApprovalProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<PendingReviewApprovalDowngradeTrigger> {
    vec![
        PendingReviewApprovalDowngradeTrigger::ProofStale,
        PendingReviewApprovalDowngradeTrigger::ProviderFreshnessStale,
        PendingReviewApprovalDowngradeTrigger::StaleBaseUnlabeled,
        PendingReviewApprovalDowngradeTrigger::ApprovalInvalidationPending,
        PendingReviewApprovalDowngradeTrigger::RewrittenSeriesUnlabeled,
        PendingReviewApprovalDowngradeTrigger::ReviewerScopeUnresolved,
        PendingReviewApprovalDowngradeTrigger::BrowserHandoffUnavailable,
        PendingReviewApprovalDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<PendingReviewApprovalConsumerSurface> {
    vec![
        PendingReviewApprovalConsumerSurface::ReviewWorkspace,
        PendingReviewApprovalConsumerSurface::ReviewList,
        PendingReviewApprovalConsumerSurface::CompanionQueue,
        PendingReviewApprovalConsumerSurface::CliHeadless,
        PendingReviewApprovalConsumerSurface::SupportExport,
        PendingReviewApprovalConsumerSurface::NotificationsInbox,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        PENDING_REVIEW_APPROVAL_SCHEMA_REF.to_owned(),
        PENDING_REVIEW_APPROVAL_DOC_REF.to_owned(),
        PENDING_REVIEW_APPROVAL_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        PENDING_REVIEW_APPROVAL_REVIEW_WORKSPACE_CONTRACT_REF.to_owned(),
        PENDING_REVIEW_APPROVAL_REVIEW_PACK_CONTRACT_REF.to_owned(),
        PENDING_REVIEW_APPROVAL_PUBLISH_LATER_CONTRACT_REF.to_owned(),
        PENDING_REVIEW_APPROVAL_INVALIDATION_CONTRACT_REF.to_owned(),
        PENDING_REVIEW_APPROVAL_LANDING_CANDIDATE_CONTRACT_REF.to_owned(),
    ]
}

fn component_refs() -> Vec<String> {
    vec![
        PENDING_REVIEW_APPROVAL_REVIEW_WORKSPACE_CONTRACT_REF.to_owned(),
        PENDING_REVIEW_APPROVAL_REVIEW_PACK_CONTRACT_REF.to_owned(),
    ]
}

// --- Tray + banner builders ---------------------------------------------------

fn tray_awaiting_my_review_with_drafts() -> PendingReviewTray {
    PendingReviewTray {
        tray_id: "tray:pr-7001".to_owned(),
        review_id_label: "PR #7001".to_owned(),
        provider_freshness: M5ReviewComponentStaleProviderState::ProviderFresh,
        reviewer_scope: ReviewerScopeState::AwaitingMyReview,
        scope_summary_label: "You still owe a review".to_owned(),
        requested_reviewers: vec![RequestedReviewer {
            reviewer_id: "rev:you".to_owned(),
            reviewer_label: "you".to_owned(),
            review_state: ReviewerReviewState::Requested,
            is_required: true,
        }],
        unresolved_thread_count: 2,
        local_draft_comments: vec![LocalDraftComment {
            draft_id: "draft:7001-a".to_owned(),
            thread_label: "src/lib.rs:42".to_owned(),
            preview_label: "Consider a named constant here".to_owned(),
            is_local_only: true,
        }],
        publish_later_packets: vec![],
        local_evidence_visible: true,
        next_action: PendingNextAction::SubmitYourReview,
        next_action_label: "Submit your review".to_owned(),
        local_continue_note: String::new(),
        browser_handoff_boundary: String::new(),
        source_contract_refs: component_refs(),
    }
}

fn tray_awaiting_other_reviewers() -> PendingReviewTray {
    PendingReviewTray {
        tray_id: "tray:pr-7002".to_owned(),
        review_id_label: "PR #7002".to_owned(),
        provider_freshness: M5ReviewComponentStaleProviderState::ProviderFresh,
        reviewer_scope: ReviewerScopeState::AwaitingOtherReviewers,
        scope_summary_label: "Waiting on 1 of 2 requested reviewers".to_owned(),
        requested_reviewers: vec![
            RequestedReviewer {
                reviewer_id: "rev:alex".to_owned(),
                reviewer_label: "alex".to_owned(),
                review_state: ReviewerReviewState::Approved,
                is_required: true,
            },
            RequestedReviewer {
                reviewer_id: "rev:sam".to_owned(),
                reviewer_label: "sam".to_owned(),
                review_state: ReviewerReviewState::Requested,
                is_required: true,
            },
        ],
        unresolved_thread_count: 0,
        local_draft_comments: vec![],
        publish_later_packets: vec![],
        local_evidence_visible: true,
        next_action: PendingNextAction::AwaitReviewers,
        next_action_label: "Await the remaining reviewer".to_owned(),
        local_continue_note: String::new(),
        browser_handoff_boundary: String::new(),
        source_contract_refs: component_refs(),
    }
}

fn tray_changes_requested() -> PendingReviewTray {
    PendingReviewTray {
        tray_id: "tray:pr-7003".to_owned(),
        review_id_label: "PR #7003".to_owned(),
        provider_freshness: M5ReviewComponentStaleProviderState::ProviderFresh,
        reviewer_scope: ReviewerScopeState::ChangesRequested,
        scope_summary_label: "Changes requested by a required reviewer".to_owned(),
        requested_reviewers: vec![RequestedReviewer {
            reviewer_id: "rev:jordan".to_owned(),
            reviewer_label: "jordan".to_owned(),
            review_state: ReviewerReviewState::ChangesRequested,
            is_required: true,
        }],
        unresolved_thread_count: 3,
        local_draft_comments: vec![],
        publish_later_packets: vec![],
        local_evidence_visible: true,
        next_action: PendingNextAction::AddressChangesRequested,
        next_action_label: "Address the requested changes".to_owned(),
        local_continue_note: String::new(),
        browser_handoff_boundary: String::new(),
        source_contract_refs: component_refs(),
    }
}

fn tray_degraded_provider_local_evidence() -> PendingReviewTray {
    PendingReviewTray {
        tray_id: "tray:pr-7004".to_owned(),
        review_id_label: "PR #7004".to_owned(),
        provider_freshness: M5ReviewComponentStaleProviderState::ProviderUnreachable,
        reviewer_scope: ReviewerScopeState::AwaitingMyReview,
        scope_summary_label: "You still owe a review; provider is unreachable".to_owned(),
        requested_reviewers: vec![RequestedReviewer {
            reviewer_id: "rev:you".to_owned(),
            reviewer_label: "you".to_owned(),
            review_state: ReviewerReviewState::Requested,
            is_required: true,
        }],
        unresolved_thread_count: 1,
        local_draft_comments: vec![
            LocalDraftComment {
                draft_id: "draft:7004-a".to_owned(),
                thread_label: "src/queue.rs:88".to_owned(),
                preview_label: "This early-return hides the retry path".to_owned(),
                is_local_only: true,
            },
            LocalDraftComment {
                draft_id: "draft:7004-b".to_owned(),
                thread_label: "src/queue.rs:120".to_owned(),
                preview_label: "Nit: rename for clarity".to_owned(),
                is_local_only: true,
            },
        ],
        publish_later_packets: vec![PublishLaterPacket {
            packet_ref_id: "publish-later:7004".to_owned(),
            packet_label: "2 draft comments queued to publish later".to_owned(),
            item_count: 2,
            is_offline_captured: true,
        }],
        local_evidence_visible: true,
        next_action: PendingNextAction::PublishFollowUpPacket,
        next_action_label: "Publish your follow-up packet when the provider is reachable"
            .to_owned(),
        local_continue_note:
            "Provider is unreachable; your draft comments and follow-up packet stay visible here"
                .to_owned(),
        browser_handoff_boundary:
            "Provider is unreachable; open in browser to inspect the hosted review".to_owned(),
        source_contract_refs: component_refs(),
    }
}

fn invalidation_banner(
    cause: ApprovalInvalidationCause,
    freshness: M5ReviewComponentStaleProviderState,
    reopen_safe: bool,
) -> ApprovalInvalidationBanner {
    let (id, review, cause_detail, prior) = match cause {
        ApprovalInvalidationCause::StaleBase => (
            "banner:pr-8001",
            "PR #8001",
            "Base advanced by 9 commits after this change was approved",
            "Approved by alex at base main@a1b2c3",
        ),
        ApprovalInvalidationCause::RebasedStack => (
            "banner:pr-8002",
            "PR #8002",
            "Stack was rebased, moving this change onto a new parent",
            "Approved by sam before the rebase",
        ),
        ApprovalInvalidationCause::RewrittenSeries => (
            "banner:pr-8003",
            "PR #8003",
            "Head was force-pushed; the series was rewritten under existing approvals",
            "Approved by jordan on the pre-rewrite head",
        ),
        ApprovalInvalidationCause::ChangedQueueState => (
            "banner:pr-8004",
            "PR #8004",
            "Queue state changed under the existing approvals",
            "Approved by alex before requeue",
        ),
        ApprovalInvalidationCause::PolicyDrift => (
            "banner:pr-8005",
            "PR #8005",
            "Required-review policy drifted; a new required reviewer was added",
            "Approved under the prior policy",
        ),
    };

    let local_continue_note = if matches!(
        freshness,
        M5ReviewComponentStaleProviderState::ProviderStale
            | M5ReviewComponentStaleProviderState::ProviderUnreachable
            | M5ReviewComponentStaleProviderState::ProviderConflict
            | M5ReviewComponentStaleProviderState::LocalOnlyContinuation
    ) {
        "Keep comparing and re-reviewing locally while provider freshness is degraded".to_owned()
    } else {
        String::new()
    };
    let browser_handoff_boundary = if matches!(
        freshness,
        M5ReviewComponentStaleProviderState::ProviderUnreachable
    ) {
        "Provider is unreachable; open in browser to inspect the hosted approvals".to_owned()
    } else {
        String::new()
    };

    ApprovalInvalidationBanner {
        banner_id: id.to_owned(),
        review_id_label: review.to_owned(),
        banner_kind: ReviewBannerKind::ApprovalInvalidation,
        approvals_were_invalidated: true,
        invalidation_cause: cause,
        headline_label: format!("Approvals invalidated: {}", cause.as_str()),
        cause_detail: cause_detail.to_owned(),
        prior_approval_state_label: prior.to_owned(),
        reopen_safe,
        reopen_note: if reopen_safe {
            "Reopen your prior review thread as a follow-up; it is preserved".to_owned()
        } else {
            String::new()
        },
        provider_freshness: freshness,
        actions: vec![
            ApprovalBannerAction::CompareInvalidatingChange,
            ApprovalBannerAction::RequestReReview,
            ApprovalBannerAction::ReopenFollowUp,
            ApprovalBannerAction::ExportInvalidationPacket,
        ],
        local_continue_note,
        browser_handoff_boundary,
        source_contract_refs: component_refs(),
    }
}

fn all_cause_invalidation_banners() -> Vec<ApprovalInvalidationBanner> {
    vec![
        invalidation_banner(
            ApprovalInvalidationCause::StaleBase,
            M5ReviewComponentStaleProviderState::ProviderFresh,
            true,
        ),
        invalidation_banner(
            ApprovalInvalidationCause::RebasedStack,
            M5ReviewComponentStaleProviderState::ProviderStale,
            false,
        ),
        invalidation_banner(
            ApprovalInvalidationCause::RewrittenSeries,
            M5ReviewComponentStaleProviderState::ProviderFresh,
            true,
        ),
        invalidation_banner(
            ApprovalInvalidationCause::ChangedQueueState,
            M5ReviewComponentStaleProviderState::ProviderFresh,
            false,
        ),
        invalidation_banner(
            ApprovalInvalidationCause::PolicyDrift,
            M5ReviewComponentStaleProviderState::ProviderFresh,
            true,
        ),
    ]
}

fn generic_warning_banner() -> ApprovalInvalidationBanner {
    ApprovalInvalidationBanner {
        banner_id: "banner:pr-8090".to_owned(),
        review_id_label: "PR #8090".to_owned(),
        banner_kind: ReviewBannerKind::GenericWarning,
        approvals_were_invalidated: false,
        invalidation_cause: ApprovalInvalidationCause::StaleBase,
        headline_label: "Heads up: this branch has not been updated recently".to_owned(),
        cause_detail: String::new(),
        prior_approval_state_label: String::new(),
        reopen_safe: false,
        reopen_note: String::new(),
        provider_freshness: M5ReviewComponentStaleProviderState::ProviderFresh,
        actions: vec![ApprovalBannerAction::ExportInvalidationPacket],
        local_continue_note: String::new(),
        browser_handoff_boundary: String::new(),
        source_contract_refs: component_refs(),
    }
}

fn queue_block_banner() -> ApprovalInvalidationBanner {
    ApprovalInvalidationBanner {
        banner_id: "banner:pr-8091".to_owned(),
        review_id_label: "PR #8091".to_owned(),
        banner_kind: ReviewBannerKind::QueueBlock,
        approvals_were_invalidated: false,
        invalidation_cause: ApprovalInvalidationCause::ChangedQueueState,
        headline_label: "Queue is paused for a maintenance window".to_owned(),
        cause_detail: String::new(),
        prior_approval_state_label: String::new(),
        reopen_safe: false,
        reopen_note: String::new(),
        provider_freshness: M5ReviewComponentStaleProviderState::ProviderFresh,
        actions: vec![ApprovalBannerAction::ExportInvalidationPacket],
        local_continue_note: String::new(),
        browser_handoff_boundary: String::new(),
        source_contract_refs: component_refs(),
    }
}

fn pending_trays() -> Vec<PendingReviewTray> {
    vec![
        tray_awaiting_my_review_with_drafts(),
        tray_awaiting_other_reviewers(),
        tray_changes_requested(),
        tray_degraded_provider_local_evidence(),
    ]
}

fn approval_banners() -> Vec<ApprovalInvalidationBanner> {
    let mut banners = all_cause_invalidation_banners();
    banners.push(generic_warning_banner());
    banners.push(queue_block_banner());
    banners
}

fn packet() -> PendingReviewApprovalPacket {
    PendingReviewApprovalPacket::new(PendingReviewApprovalPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Pending-review trays and approval-invalidation banners".to_owned(),
        pending_trays: pending_trays(),
        approval_banners: approval_banners(),
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
fn pending_review_approval_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn tray_resolver_degrades_only_under_degraded_provider() {
    let fresh = resolve_pending_tray_disclosure(M5ReviewComponentStaleProviderState::ProviderFresh);
    assert!(!fresh.needs_local_continue_note);
    assert!(!fresh.must_keep_local_evidence_visible);
    assert!(!fresh.needs_browser_handoff_boundary);

    let stale = resolve_pending_tray_disclosure(M5ReviewComponentStaleProviderState::ProviderStale);
    assert!(stale.needs_local_continue_note);
    assert!(stale.must_keep_local_evidence_visible);
    assert!(!stale.needs_browser_handoff_boundary);

    let unreachable =
        resolve_pending_tray_disclosure(M5ReviewComponentStaleProviderState::ProviderUnreachable);
    assert!(unreachable.needs_local_continue_note);
    assert!(unreachable.must_keep_local_evidence_visible);
    assert!(unreachable.needs_browser_handoff_boundary);
}

#[test]
fn banner_resolver_requires_invalidation_shape_when_invalidated() {
    let invalidated = resolve_approval_banner_disclosure(
        true,
        M5ReviewComponentStaleProviderState::ProviderFresh,
    );
    assert!(invalidated.must_be_invalidation_kind);
    assert!(invalidated.needs_cause_and_prior_state);
    assert!(invalidated.needs_required_actions);
    assert!(!invalidated.needs_local_continue_note);

    let not_invalidated = resolve_approval_banner_disclosure(
        false,
        M5ReviewComponentStaleProviderState::ProviderFresh,
    );
    assert!(!not_invalidated.must_be_invalidation_kind);
    assert!(!not_invalidated.needs_cause_and_prior_state);
    assert!(!not_invalidated.needs_required_actions);
}

#[test]
fn invalidation_collapsed_into_generic_warning_fails() {
    let mut packet = packet();
    // Banner 0 is an invalidation; collapsing it into a generic warning must fail.
    packet.approval_banners[0].banner_kind = ReviewBannerKind::GenericWarning;
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::ApprovalInvalidationNotSeparated));
}

#[test]
fn invalidation_collapsed_into_queue_block_fails() {
    let mut packet = packet();
    packet.approval_banners[0].banner_kind = ReviewBannerKind::QueueBlock;
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::ApprovalInvalidationNotSeparated));
}

#[test]
fn generic_warning_masquerading_as_invalidation_fails() {
    let mut packet = packet();
    // The generic-warning banner is not invalidating; labeling it approval-invalidation lies.
    packet.approval_banners[5].banner_kind = ReviewBannerKind::ApprovalInvalidation;
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::ApprovalInvalidationNotSeparated));
}

#[test]
fn invalidation_without_cause_detail_fails() {
    let mut packet = packet();
    packet.approval_banners[0].cause_detail = String::new();
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::InvalidationCauseDetailMissing));
}

#[test]
fn invalidation_without_prior_approval_state_fails() {
    let mut packet = packet();
    packet.approval_banners[0].prior_approval_state_label = String::new();
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::PriorApprovalStateMissing));
}

#[test]
fn invalidation_missing_required_actions_fails() {
    let mut packet = packet();
    packet.approval_banners[0].actions = vec![
        ApprovalBannerAction::CompareInvalidatingChange,
        ApprovalBannerAction::ExportInvalidationPacket,
    ];
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::RequiredInvalidationActionsMissing));
}

#[test]
fn reopen_safe_without_note_fails() {
    let mut packet = packet();
    packet.approval_banners[0].reopen_note = String::new();
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::ReopenNoteMissing));
}

#[test]
fn banner_degraded_without_local_continue_note_fails() {
    let mut packet = packet();
    // Banner 1 (rebased stack) is provider-stale and must carry a local-continue note.
    packet.approval_banners[1].local_continue_note = String::new();
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::BannerLocalContinueNoteMissing));
}

#[test]
fn banner_unreachable_without_handoff_boundary_fails() {
    let mut packet = packet();
    packet.approval_banners[0].provider_freshness =
        M5ReviewComponentStaleProviderState::ProviderUnreachable;
    packet.approval_banners[0].browser_handoff_boundary = String::new();
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::BannerBrowserHandoffBoundaryMissing));
}

#[test]
fn banner_forcing_raw_provider_navigation_fails() {
    let mut packet = packet();
    packet.approval_banners[0].actions = vec![ApprovalBannerAction::OpenProviderInBrowser];
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::ForcedRawProviderNavigation));
}

#[test]
fn missing_invalidation_cause_coverage_fails() {
    let mut packet = packet();
    // Collapse PolicyDrift onto StaleBase so PolicyDrift is no longer covered.
    packet.approval_banners[4].invalidation_cause = ApprovalInvalidationCause::StaleBase;
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::InvalidationCauseCoverageMissing));
}

#[test]
fn hidden_local_evidence_under_degraded_provider_fails() {
    let mut packet = packet();
    // Tray 3 is the unreachable-provider tray carrying local drafts and a follow-up packet.
    packet.pending_trays[3].local_evidence_visible = false;
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::LocalDraftsOrFollowUpHidden));
}

#[test]
fn tray_degraded_without_local_continue_note_fails() {
    let mut packet = packet();
    packet.pending_trays[3].local_continue_note = String::new();
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::TrayLocalContinueNoteMissing));
}

#[test]
fn tray_unreachable_without_handoff_boundary_fails() {
    let mut packet = packet();
    packet.pending_trays[3].browser_handoff_boundary = String::new();
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::TrayBrowserHandoffBoundaryMissing));
}

#[test]
fn reviewer_scope_misrepresented_fails() {
    let mut packet = packet();
    // Tray 1 still has an outstanding reviewer; claiming none-outstanding is a lie.
    packet.pending_trays[1].reviewer_scope = ReviewerScopeState::ReadyNoneOutstanding;
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::ReviewerScopeMisrepresented));
}

#[test]
fn incomplete_tray_fails() {
    let mut packet = packet();
    packet.pending_trays[0].scope_summary_label = String::new();
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::TrayIncomplete));
}

#[test]
fn incomplete_reviewer_fails() {
    let mut packet = packet();
    packet.pending_trays[1].requested_reviewers[0].reviewer_label = String::new();
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::ReviewerIncomplete));
}

#[test]
fn incomplete_local_draft_fails() {
    let mut packet = packet();
    packet.pending_trays[0].local_draft_comments[0].thread_label = String::new();
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::LocalDraftIncomplete));
}

#[test]
fn incomplete_publish_later_packet_fails() {
    let mut packet = packet();
    packet.pending_trays[3].publish_later_packets[0].packet_label = String::new();
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::PublishLaterPacketIncomplete));
}

#[test]
fn incomplete_banner_fails() {
    let mut packet = packet();
    packet.approval_banners[0].headline_label = String::new();
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::BannerIncomplete));
}

#[test]
fn missing_trays_fails() {
    let mut packet = packet();
    packet.pending_trays.clear();
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::PendingTraysMissing));
}

#[test]
fn missing_banners_fails() {
    let mut packet = packet();
    packet.approval_banners.clear();
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::ApprovalBannersMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.approval_invalidation_kept_separate = false;
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .banner_distinguishes_invalidation_cause = false;
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&PendingReviewApprovalViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_trays_and_banners() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Pending trays"));
    assert!(summary.contains("## Approval banners"));
    assert!(summary.contains("PR #7001"));
    assert!(summary.contains("awaiting_my_review"));
    assert!(summary.contains("rewritten_series"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_pending_review_approval_export()
        .expect("checked pending-review / approval export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-pending-review-trays/degraded_provider_local_drafts_visible.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-pending-review-trays/approval_invalidated_reopen_safe.json"
        )),
    ] {
        let packet: PendingReviewApprovalPacket =
            serde_json::from_str(raw).expect("fixture parses as pending-review / approval packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

// --- Fixture builders ---------------------------------------------------------

fn fixture_degraded_provider_local_drafts_visible() -> PendingReviewApprovalPacket {
    let trays = vec![
        tray_degraded_provider_local_evidence(),
        PendingReviewTray {
            tray_id: "tray:pr-7101".to_owned(),
            review_id_label: "PR #7101".to_owned(),
            provider_freshness: M5ReviewComponentStaleProviderState::LocalOnlyContinuation,
            reviewer_scope: ReviewerScopeState::AwaitingMyReview,
            scope_summary_label: "You still owe a review; working from local-only continuation"
                .to_owned(),
            requested_reviewers: vec![RequestedReviewer {
                reviewer_id: "rev:you".to_owned(),
                reviewer_label: "you".to_owned(),
                review_state: ReviewerReviewState::Requested,
                is_required: true,
            }],
            unresolved_thread_count: 0,
            local_draft_comments: vec![LocalDraftComment {
                draft_id: "draft:7101-a".to_owned(),
                thread_label: "src/net.rs:12".to_owned(),
                preview_label: "Draft kept locally while sync is degraded".to_owned(),
                is_local_only: true,
            }],
            publish_later_packets: vec![PublishLaterPacket {
                packet_ref_id: "publish-later:7101".to_owned(),
                packet_label: "1 draft queued to publish later".to_owned(),
                item_count: 1,
                is_offline_captured: true,
            }],
            local_evidence_visible: true,
            next_action: PendingNextAction::PublishDraftComments,
            next_action_label: "Publish your draft comments when sync recovers".to_owned(),
            local_continue_note: "Provider sync is degraded; your drafts stay visible here"
                .to_owned(),
            browser_handoff_boundary: String::new(),
            source_contract_refs: component_refs(),
        },
    ];

    PendingReviewApprovalPacket::new(PendingReviewApprovalPacketInput {
        packet_id: "pending-review-approval:fixture:degraded-provider-local-drafts".to_owned(),
        surface_label: "Pending-review trays: degraded provider, local drafts visible".to_owned(),
        pending_trays: trays,
        approval_banners: all_cause_invalidation_banners(),
        downgrade_triggers: vec![
            PendingReviewApprovalDowngradeTrigger::ProviderFreshnessStale,
            PendingReviewApprovalDowngradeTrigger::BrowserHandoffUnavailable,
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

fn fixture_approval_invalidated_reopen_safe() -> PendingReviewApprovalPacket {
    let banners = vec![
        invalidation_banner(
            ApprovalInvalidationCause::StaleBase,
            M5ReviewComponentStaleProviderState::ProviderFresh,
            true,
        ),
        invalidation_banner(
            ApprovalInvalidationCause::RebasedStack,
            M5ReviewComponentStaleProviderState::ProviderFresh,
            true,
        ),
        invalidation_banner(
            ApprovalInvalidationCause::RewrittenSeries,
            M5ReviewComponentStaleProviderState::ProviderFresh,
            true,
        ),
        invalidation_banner(
            ApprovalInvalidationCause::ChangedQueueState,
            M5ReviewComponentStaleProviderState::ProviderConflict,
            true,
        ),
        invalidation_banner(
            ApprovalInvalidationCause::PolicyDrift,
            M5ReviewComponentStaleProviderState::ProviderFresh,
            true,
        ),
    ];

    PendingReviewApprovalPacket::new(PendingReviewApprovalPacketInput {
        packet_id: "pending-review-approval:fixture:approval-invalidated-reopen-safe".to_owned(),
        surface_label: "Approval-invalidation banners: reopen-safe follow-up".to_owned(),
        pending_trays: vec![tray_changes_requested()],
        approval_banners: banners,
        downgrade_triggers: vec![
            PendingReviewApprovalDowngradeTrigger::ApprovalInvalidationPending,
            PendingReviewApprovalDowngradeTrigger::RewrittenSeriesUnlabeled,
            PendingReviewApprovalDowngradeTrigger::StaleBaseUnlabeled,
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
/// Gated behind `GEN_PENDING_REVIEW_APPROVAL_ARTIFACTS` so it never writes during a
/// normal test run. Run with the env var set to refresh the artifacts after a
/// contract change, then review the diff.
#[test]
fn regenerate_pending_review_approval_artifacts() {
    if std::env::var("GEN_PENDING_REVIEW_APPROVAL_ARTIFACTS").is_err() {
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
        "{root}/artifacts/review/m5/ship_pending_review_trays_and_approval_invalidation_banners_with_reviewer_scope_restack_drift_and_reopen_safe_truth"
    );
    std::fs::create_dir_all(&artifact_dir).expect("create artifact dir");
    std::fs::write(
        format!("{artifact_dir}/support_export.json"),
        format!("{}\n", canonical.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        format!("{root}/artifacts/review/m5/ship_pending_review_trays_and_approval_invalidation_banners_with_reviewer_scope_restack_drift_and_reopen_safe_truth.md"),
        canonical.render_markdown_summary(),
    )
    .expect("write summary");

    let fixture_dir = format!("{root}/fixtures/ui/m5-pending-review-trays");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    for (name, fixture) in [
        (
            "degraded_provider_local_drafts_visible.json",
            fixture_degraded_provider_local_drafts_visible(),
        ),
        (
            "approval_invalidated_reopen_safe.json",
            fixture_approval_invalidated_reopen_safe(),
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
