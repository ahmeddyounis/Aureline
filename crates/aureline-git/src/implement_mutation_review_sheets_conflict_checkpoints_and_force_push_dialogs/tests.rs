use super::*;

const CANONICAL_PACKET_ID: &str = "m5-git-mutation-review-recovery-component:stable:0001";

const CANONICAL_EXPORT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5-git-mutation-review-recovery-components-proof/support_export.json"
));

const CHERRY_PICK_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-git-mutation-review-recovery-components/cherry_pick_conflict_checkpoint.json"
));

const FORCE_PUSH_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-git-mutation-review-recovery-components/force_push_with_lease_recovery.json"
));

fn cherry_pick_revert_sheets() -> Vec<CherryPickRevertReviewSheet> {
    use GitHistoryDowngradeState::*;
    use MutationBlockerState::*;
    use MutationCheckpointState::*;

    vec![
        // A cherry-pick that lands a new commit on main and updates the open
        // hosted review; its distinct verb, target, and rollback are explicit.
        CherryPickRevertReviewSheet {
            row_id: "cherry:pick".to_owned(),
            component: M5GitHistoryComponent::CherryPickRevertReviewSheet,
            verb: CherryPickRevertVerb::CherryPick,
            source_commit_short_id: "a1b2c3d".to_owned(),
            source_commit_subject: "fix: guard review-queue projection".to_owned(),
            target_ref: "main".to_owned(),
            target_worktree: "/repo (main)".to_owned(),
            review_class: MutationReviewClass::ExplicitVerbConfirm,
            publish_consequence:
                "Creates a new commit on main replaying a1b2c3d; nothing is pushed until you push main"
                    .to_owned(),
            hosted_review_impact: HostedReviewImpact::UpdatesHostedReview,
            approval_consequence:
                "The open review for main gains one new commit; existing approvals are re-requested for it"
                    .to_owned(),
            blocked_state: Ready,
            blocker_disclosure: String::new(),
            rollback_action:
                "Undo with `git cherry-pick --abort` before confirming, or reset main to its prior tip after"
                    .to_owned(),
            checkpoint_state: PreMutationCaptured,
            checkpoint_disclosure:
                "Pre-cherry-pick checkpoint captured at main@{1}; reset returns main to its prior tip"
                    .to_owned(),
            downgrade_vocab: vec![DirtyOrConflictedWorktree, StaleProviderOverlay],
            fields_shown: vec![
                "verb".to_owned(),
                "source_commit_short_id".to_owned(),
                "target_ref".to_owned(),
                "publish_consequence".to_owned(),
            ],
            source_contract_refs: vec![GIT_MUTATION_REVIEW_HISTORY_SURGERY_CONTRACT_REF.to_owned()],
        },
        // A revert that inverts a merged commit, expected to conflict; the blocker
        // is disclosed and recovery stays local through the reflog fallback.
        CherryPickRevertReviewSheet {
            row_id: "cherry:revert".to_owned(),
            component: M5GitHistoryComponent::CherryPickRevertReviewSheet,
            verb: CherryPickRevertVerb::Revert,
            source_commit_short_id: "b2c3d4e".to_owned(),
            source_commit_subject: "feat: risky telemetry probe".to_owned(),
            target_ref: "release/1.4".to_owned(),
            target_worktree: "/repo (release/1.4)".to_owned(),
            review_class: MutationReviewClass::ExplicitVerbConfirm,
            publish_consequence:
                "Creates a new commit on release/1.4 that applies the inverse of b2c3d4e".to_owned(),
            hosted_review_impact: HostedReviewImpact::LocalOnly,
            approval_consequence: String::new(),
            blocked_state: ConflictExpected,
            blocker_disclosure:
                "The revert conflicts with a later change to the same file; resolve at the checkpoint before it applies"
                    .to_owned(),
            rollback_action: "Undo with `git revert --abort`; the checkpoint restores the prior tip"
                .to_owned(),
            checkpoint_state: ReflogFallbackOnly,
            checkpoint_disclosure:
                "No explicit checkpoint; release/1.4's prior tip stays reachable via the reflog until it expires"
                    .to_owned(),
            downgrade_vocab: vec![DirtyOrConflictedWorktree, ReflogOnlyFallback],
            fields_shown: vec![
                "verb".to_owned(),
                "source_commit_short_id".to_owned(),
                "target_ref".to_owned(),
                "blocker_disclosure".to_owned(),
            ],
            source_contract_refs: vec![GIT_MUTATION_REVIEW_HISTORY_SURGERY_CONTRACT_REF.to_owned()],
        },
    ]
}

fn patch_apply_sheets() -> Vec<PatchApplyReviewSheet> {
    use GitHistoryDowngradeState::*;
    use MutationBlockerState::*;
    use MutationCheckpointState::*;

    vec![PatchApplyReviewSheet {
        row_id: "patch:mailbox".to_owned(),
        component: M5GitHistoryComponent::PatchApplyReviewSheet,
        patch_source: PatchSource::MailboxSeries,
        apply_mode: PatchApplyMode::ThreeWayMerge,
        target_ref: "feature/import".to_owned(),
        target_worktree: "/repo (feature/import)".to_owned(),
        commit_count: 2,
        affected_file_count: 3,
        affected_paths: vec![
            "crates/aureline-review/src/queue.rs".to_owned(),
            "crates/aureline-review/src/lib.rs".to_owned(),
            "docs/review/queue.md".to_owned(),
        ],
        review_class: MutationReviewClass::PatchApplyConfirm,
        publish_consequence:
            "Applies 2 commits across 3 files onto feature/import; nothing is pushed until you push"
                .to_owned(),
        hosted_review_impact: HostedReviewImpact::UpdatesHostedReview,
        approval_consequence:
            "The open review for feature/import gains 2 imported commits; reviewers are re-notified"
                .to_owned(),
        blocked_state: Ready,
        blocker_disclosure: String::new(),
        rollback_action:
            "Undo with `git am --abort` before confirming, or reset feature/import to its prior tip after"
                .to_owned(),
        checkpoint_state: PreMutationCaptured,
        checkpoint_disclosure:
            "Pre-apply checkpoint captured at feature/import@{1}; reset restores the prior tip"
                .to_owned(),
        downgrade_vocab: vec![DirtyOrConflictedWorktree, OfflineLocalOnly],
        fields_shown: vec![
            "patch_source".to_owned(),
            "apply_mode".to_owned(),
            "target_ref".to_owned(),
            "affected_paths".to_owned(),
        ],
        source_contract_refs: vec![GIT_MUTATION_REVIEW_HISTORY_SURGERY_CONTRACT_REF.to_owned()],
    }]
}

fn conflict_checkpoint_cards() -> Vec<ConflictCheckpointCard> {
    use ConflictSide::*;
    use GitHistoryDowngradeState::*;
    use MutationCheckpointState::*;

    vec![
        // An unresolved conflict captured during the revert above: base/ours/theirs
        // are preserved and the checkpoint stays reopenable.
        ConflictCheckpointCard {
            row_id: "conflict:revert".to_owned(),
            component: M5GitHistoryComponent::ConflictCheckpointCard,
            checkpoint_label: "Revert of b2c3d4e on release/1.4".to_owned(),
            originating_operation: "revert b2c3d4e".to_owned(),
            target_ref: "release/1.4".to_owned(),
            target_worktree: "/repo (release/1.4)".to_owned(),
            sides_present: vec![Base, Ours, Theirs],
            unresolved_count: 1,
            total_conflict_count: 1,
            reopen_state: CheckpointReopenState::Reopenable,
            reopen_disclosure:
                "Reopen the checkpoint to edit the conflicted file; discard restores release/1.4's prior tip"
                    .to_owned(),
            checkpoint_state: ReflogFallbackOnly,
            checkpoint_disclosure:
                "The prior tip stays reachable via the reflog; recovery remains local".to_owned(),
            review_class: MutationReviewClass::DisplayOnlyNoMutation,
            downgrade_vocab: vec![DirtyOrConflictedWorktree, ReflogOnlyFallback],
            fields_shown: vec![
                "sides_present".to_owned(),
                "unresolved_count".to_owned(),
                "reopen_state".to_owned(),
                "reopen_disclosure".to_owned(),
            ],
            source_contract_refs: vec![GIT_MUTATION_REVIEW_CONFLICT_SESSION_CONTRACT_REF.to_owned()],
        },
        // A fully resolved checkpoint from an earlier merge: the result side is now
        // present and the card records that the resolution was applied.
        ConflictCheckpointCard {
            row_id: "conflict:merge".to_owned(),
            component: M5GitHistoryComponent::ConflictCheckpointCard,
            checkpoint_label: "Merge of feature/review-lane into main".to_owned(),
            originating_operation: "merge feature/review-lane".to_owned(),
            target_ref: "main".to_owned(),
            target_worktree: "/repo (main)".to_owned(),
            sides_present: vec![Base, Ours, Theirs, Result],
            unresolved_count: 0,
            total_conflict_count: 2,
            reopen_state: CheckpointReopenState::ResolvedApplied,
            reopen_disclosure:
                "All conflicts resolved and applied; restore the checkpoint to revisit the recorded resolution"
                    .to_owned(),
            checkpoint_state: PreMutationCaptured,
            checkpoint_disclosure:
                "Pre-merge checkpoint captured at main@{2}; restore returns to the recorded resolution"
                    .to_owned(),
            review_class: MutationReviewClass::DisplayOnlyNoMutation,
            downgrade_vocab: vec![OfflineLocalOnly, StaleProviderOverlay],
            fields_shown: vec![
                "sides_present".to_owned(),
                "unresolved_count".to_owned(),
                "reopen_state".to_owned(),
                "checkpoint_disclosure".to_owned(),
            ],
            source_contract_refs: vec![GIT_MUTATION_REVIEW_CONFLICT_SESSION_CONTRACT_REF.to_owned()],
        },
    ]
}

fn force_push_dialogs() -> Vec<ForcePushReviewDialog> {
    use GitHistoryDowngradeState::*;
    use MutationBlockerState::*;
    use MutationCheckpointState::*;

    vec![ForcePushReviewDialog {
        row_id: "force-push:lease".to_owned(),
        component: M5GitHistoryComponent::ForcePushReviewDialog,
        target_remote: "origin".to_owned(),
        target_ref: "feature/review-lane".to_owned(),
        target_worktree: "/repo (feature/review-lane)".to_owned(),
        local_tip_short_id: "f6a7b8c".to_owned(),
        remote_tip_short_id: "e5f6a7b".to_owned(),
        overwrites_commit_count: 2,
        safety: ForcePushSafety::ForceWithLease,
        lease_disclosure:
            "The lease refuses the push if origin/feature/review-lane is not still e5f6a7b".to_owned(),
        review_class: MutationReviewClass::ForcePushConfirm,
        publish_consequence:
            "Rewrites origin/feature/review-lane from e5f6a7b to f6a7b8c, dropping 2 remote-only commits"
                .to_owned(),
        hosted_review_impact: HostedReviewImpact::InvalidatesApproval,
        approval_consequence:
            "The hosted review's approvals are invalidated because the reviewed commits are rewritten"
                .to_owned(),
        blocked_state: Ready,
        blocker_disclosure: String::new(),
        rollback_action:
            "Restore the remote by pushing e5f6a7b back, recovered from origin/feature/review-lane@{1}"
                .to_owned(),
        recovery_ref: "origin/feature/review-lane@{1}".to_owned(),
        checkpoint_state: PreMutationCaptured,
        checkpoint_disclosure:
            "The overwritten remote tip e5f6a7b stays recoverable from the remote-tracking reflog"
                .to_owned(),
        downgrade_vocab: vec![StaleProviderOverlay, ApprovalInvalidated],
        fields_shown: vec![
            "target_remote".to_owned(),
            "target_ref".to_owned(),
            "remote_tip_short_id".to_owned(),
            "recovery_ref".to_owned(),
        ],
        source_contract_refs: vec![GIT_MUTATION_REVIEW_REF_UPDATE_CONTRACT_REF.to_owned()],
    }]
}

fn trust_review() -> GitMutationReviewTrustReview {
    GitMutationReviewTrustReview {
        distinct_verbs_never_collapsed: true,
        target_ref_worktree_always_named: true,
        affected_scope_always_explicit: true,
        publish_consequence_always_disclosed: true,
        approval_consequence_explicit_when_hosted: true,
        rollback_recovery_always_reachable: true,
        conflict_context_survives_mutation: true,
        local_only_recovery_stays_explicit: true,
        one_component_contract_no_hidden_meaning: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> GitMutationReviewConsumerProjection {
    GitMutationReviewConsumerProjection {
        git_history_reuses_one_contract: true,
        review_reuses_one_contract: true,
        help_support_reuses_one_contract: true,
        support_export_reuses_one_contract: true,
        consequences_explicit_across_surfaces: true,
        cli_headless_shows_truth: true,
        provider_overlay_shows_truth: true,
        ai_context_shows_truth: true,
    }
}

fn proof_freshness() -> GitMutationReviewProofFreshness {
    GitMutationReviewProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-07-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<GitHistoryDowngradeState> {
    vec![
        GitHistoryDowngradeState::StaleProviderOverlay,
        GitHistoryDowngradeState::DetachedOrMissingRef,
        GitHistoryDowngradeState::DirtyOrConflictedWorktree,
        GitHistoryDowngradeState::ReflogOnlyFallback,
        GitHistoryDowngradeState::ApprovalInvalidated,
        GitHistoryDowngradeState::OfflineLocalOnly,
    ]
}

fn consumer_surfaces() -> Vec<ComponentConsumerSurface> {
    ComponentConsumerSurface::ALL.to_vec()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        GIT_MUTATION_REVIEW_SCHEMA_REF.to_owned(),
        GIT_MUTATION_REVIEW_DOC_REF.to_owned(),
        GIT_MUTATION_REVIEW_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        GIT_MUTATION_REVIEW_HISTORY_SURGERY_CONTRACT_REF.to_owned(),
        GIT_MUTATION_REVIEW_CONFLICT_SESSION_CONTRACT_REF.to_owned(),
        GIT_MUTATION_REVIEW_REF_UPDATE_CONTRACT_REF.to_owned(),
    ]
}

fn seed_packet() -> GitMutationReviewPacket {
    GitMutationReviewPacket::new(GitMutationReviewPacketInput {
        packet_id: CANONICAL_PACKET_ID.to_owned(),
        surface_label:
            "Git mutation review sheets, conflict checkpoints, and force-push dialogs: target and recovery truth"
                .to_owned(),
        cherry_pick_revert_sheets: cherry_pick_revert_sheets(),
        patch_apply_sheets: patch_apply_sheets(),
        conflict_checkpoint_cards: conflict_checkpoint_cards(),
        force_push_dialogs: force_push_dialogs(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "aureline.support.redaction.v1".to_owned(),
        minted_at: "2026-07-08T00:00:00Z".to_owned(),
    })
}

fn baseline() -> GitMutationReviewPacket {
    seed_packet()
}

/// Regenerates the checked-in artifacts and fixtures.
///
/// Guarded by `GEN_GIT_MUTATION_REVIEW_ARTIFACTS` so it is inert in CI but can
/// deterministically rewrite the export, summary, and narrowed fixtures.
#[test]
fn generate_artifacts() {
    if std::env::var_os("GEN_GIT_MUTATION_REVIEW_ARTIFACTS").is_none() {
        return;
    }
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

    let canonical = seed_packet();
    assert!(
        canonical.validate().is_empty(),
        "{:?}",
        canonical.validate()
    );
    std::fs::write(
        format!("{root}/{GIT_MUTATION_REVIEW_ARTIFACT_REF}"),
        format!("{}\n", canonical.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        format!("{root}/{GIT_MUTATION_REVIEW_SUMMARY_REF}"),
        canonical.render_markdown_summary(),
    )
    .expect("write summary");

    // Cherry-pick + conflict-checkpoint fixture: a cherry-pick that conflicts and
    // is held at a reopenable checkpoint with base/ours/theirs preserved.
    let mut cherry = seed_packet();
    cherry.packet_id =
        "m5-git-mutation-review-recovery-component:cherry-pick-conflict-checkpoint:0001".to_owned();
    {
        let sheet = cherry
            .cherry_pick_revert_sheets
            .iter_mut()
            .find(|sheet| sheet.verb == CherryPickRevertVerb::CherryPick)
            .expect("cherry-pick sheet present");
        sheet.blocked_state = MutationBlockerState::ConflictExpected;
        sheet.blocker_disclosure =
            "The cherry-pick conflicts with local edits; resolve at the checkpoint before it applies"
                .to_owned();
    }
    assert!(cherry.validate().is_empty(), "{:?}", cherry.validate());
    std::fs::write(
        format!("{root}/{GIT_MUTATION_REVIEW_FIXTURE_DIR}/cherry_pick_conflict_checkpoint.json"),
        format!("{}\n", cherry.export_safe_json()),
    )
    .expect("write cherry-pick fixture");

    // Force-push fixture: the same lease-guarded push, recoverable from the
    // remote-tracking reflog even though it invalidates the hosted approval.
    let mut force = seed_packet();
    force.packet_id =
        "m5-git-mutation-review-recovery-component:force-push-with-lease-recovery:0001".to_owned();
    assert!(force.validate().is_empty(), "{:?}", force.validate());
    std::fs::write(
        format!("{root}/{GIT_MUTATION_REVIEW_FIXTURE_DIR}/force_push_with_lease_recovery.json"),
        format!("{}\n", force.export_safe_json()),
    )
    .expect("write force-push fixture");
}

#[test]
fn seed_packet_validates_clean() {
    assert!(
        baseline().validate().is_empty(),
        "{:?}",
        baseline().validate()
    );
}

#[test]
fn checked_support_export_validates() {
    let packet =
        current_git_mutation_review_export().expect("checked mutation review export validates");
    assert_eq!(packet.packet_id, CANONICAL_PACKET_ID);
}

#[test]
fn checked_export_matches_seed() {
    let checked: GitMutationReviewPacket =
        serde_json::from_str(CANONICAL_EXPORT).expect("canonical export deserializes");
    assert_eq!(checked, seed_packet());
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [CHERRY_PICK_FIXTURE, FORCE_PUSH_FIXTURE] {
        let packet: GitMutationReviewPacket =
            serde_json::from_str(raw).expect("fixture parses as mutation review packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn all_four_components_are_the_frozen_set() {
    assert!(GIT_MUTATION_REVIEW_COMPONENTS
        .contains(&M5GitHistoryComponent::CherryPickRevertReviewSheet));
    assert!(GIT_MUTATION_REVIEW_COMPONENTS.contains(&M5GitHistoryComponent::PatchApplyReviewSheet));
    assert!(GIT_MUTATION_REVIEW_COMPONENTS.contains(&M5GitHistoryComponent::ConflictCheckpointCard));
    assert!(GIT_MUTATION_REVIEW_COMPONENTS.contains(&M5GitHistoryComponent::ForcePushReviewDialog));
    // Three of the four are risky mutation surfaces; the checkpoint card is not.
    assert!(M5GitHistoryComponent::CherryPickRevertReviewSheet.is_risky_mutation_surface());
    assert!(M5GitHistoryComponent::PatchApplyReviewSheet.is_risky_mutation_surface());
    assert!(M5GitHistoryComponent::ForcePushReviewDialog.is_risky_mutation_surface());
    assert!(!M5GitHistoryComponent::ConflictCheckpointCard.is_risky_mutation_surface());
}

#[test]
fn expected_review_class_maps_each_component() {
    assert_eq!(
        expected_review_class(M5GitHistoryComponent::CherryPickRevertReviewSheet),
        MutationReviewClass::ExplicitVerbConfirm
    );
    assert_eq!(
        expected_review_class(M5GitHistoryComponent::PatchApplyReviewSheet),
        MutationReviewClass::PatchApplyConfirm
    );
    assert_eq!(
        expected_review_class(M5GitHistoryComponent::ConflictCheckpointCard),
        MutationReviewClass::DisplayOnlyNoMutation
    );
    assert_eq!(
        expected_review_class(M5GitHistoryComponent::ForcePushReviewDialog),
        MutationReviewClass::ForcePushConfirm
    );
}

#[test]
fn disclosure_resolver_requires_approval_only_when_hosted() {
    let local = resolve_mutation_review_disclosure(
        M5GitHistoryComponent::ForcePushReviewDialog,
        HostedReviewImpact::LocalOnly,
    );
    assert!(local.requires_distinct_confirm);
    assert!(local.must_disclose_publish_consequence);
    assert!(local.must_stay_recoverable);
    assert!(!local.must_disclose_approval_consequence);

    let hosted = resolve_mutation_review_disclosure(
        M5GitHistoryComponent::ForcePushReviewDialog,
        HostedReviewImpact::InvalidatesApproval,
    );
    assert!(hosted.must_disclose_approval_consequence);

    // The read-only card requires no distinct confirm and no recovery.
    let card = resolve_mutation_review_disclosure(
        M5GitHistoryComponent::ConflictCheckpointCard,
        HostedReviewImpact::LocalOnly,
    );
    assert!(!card.requires_distinct_confirm);
    assert!(!card.must_stay_recoverable);
}

#[test]
fn missing_cherry_sheets_fails() {
    let mut packet = baseline();
    packet.cherry_pick_revert_sheets.clear();
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::CherryPickRevertSheetsMissing));
}

#[test]
fn wrong_component_for_cherry_sheet_fails() {
    let mut packet = baseline();
    packet.cherry_pick_revert_sheets[0].component = M5GitHistoryComponent::PatchApplyReviewSheet;
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::WrongComponentForCherrySheet));
}

#[test]
fn cherry_source_commit_identity_missing_fails() {
    let mut packet = baseline();
    packet.cherry_pick_revert_sheets[0].source_commit_subject = String::new();
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::CherrySourceCommitIdentityMissing));
}

#[test]
fn cherry_verb_coverage_missing_fails() {
    let mut packet = baseline();
    // Drop the revert sheet, leaving only cherry-pick represented.
    packet
        .cherry_pick_revert_sheets
        .retain(|sheet| sheet.verb == CherryPickRevertVerb::CherryPick);
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::CherryVerbCoverageMissing));
}

#[test]
fn mutation_verb_confirm_collapsed_fails() {
    let mut packet = baseline();
    // The cherry-pick sheet borrows the force-push confirm, collapsing verbs.
    packet.cherry_pick_revert_sheets[0].review_class = MutationReviewClass::ForcePushConfirm;
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::MutationVerbConfirmCollapsed));
}

#[test]
fn target_ref_worktree_missing_fails() {
    let mut packet = baseline();
    packet.cherry_pick_revert_sheets[0].target_worktree = String::new();
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::TargetRefWorktreeMissing));
}

#[test]
fn publish_consequence_missing_fails() {
    let mut packet = baseline();
    packet.cherry_pick_revert_sheets[0].publish_consequence = String::new();
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::PublishConsequenceMissing));
}

#[test]
fn approval_consequence_missing_when_hosted_fails() {
    let mut packet = baseline();
    // The cherry-pick affects hosted review but drops its approval consequence.
    packet.cherry_pick_revert_sheets[0].approval_consequence = String::new();
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::ApprovalConsequenceMissing));
}

#[test]
fn local_only_surface_needs_no_approval_consequence() {
    // The revert sheet is local-only with an empty approval consequence and stays
    // clean, proving the approval requirement is gated on hosted impact.
    let packet = baseline();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.cherry_pick_revert_sheets[1].hosted_review_impact,
        HostedReviewImpact::LocalOnly
    );
    assert!(packet.cherry_pick_revert_sheets[1]
        .approval_consequence
        .is_empty());
}

#[test]
fn rollback_action_missing_fails() {
    let mut packet = baseline();
    packet.cherry_pick_revert_sheets[0].rollback_action = String::new();
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::RollbackActionMissing));
}

#[test]
fn mutation_blocker_not_disclosed_fails() {
    let mut packet = baseline();
    // The revert sheet keeps its conflict blocker but drops the disclosure.
    packet.cherry_pick_revert_sheets[1].blocker_disclosure = String::new();
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::MutationBlockerNotDisclosed));
}

#[test]
fn mutation_checkpoint_undisclosed_fails() {
    let mut packet = baseline();
    packet.cherry_pick_revert_sheets[0].checkpoint_disclosure = String::new();
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::MutationCheckpointUndisclosed));
}

#[test]
fn mutation_recovery_unreachable_fails() {
    let mut packet = baseline();
    packet.cherry_pick_revert_sheets[0].checkpoint_state = MutationCheckpointState::Unavailable;
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::MutationRecoveryUnreachable));
}

#[test]
fn missing_patch_sheets_fails() {
    let mut packet = baseline();
    packet.patch_apply_sheets.clear();
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::PatchApplySheetsMissing));
}

#[test]
fn wrong_component_for_patch_sheet_fails() {
    let mut packet = baseline();
    packet.patch_apply_sheets[0].component = M5GitHistoryComponent::CherryPickRevertReviewSheet;
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::WrongComponentForPatchSheet));
}

#[test]
fn patch_file_count_inconsistent_fails() {
    let mut packet = baseline();
    packet.patch_apply_sheets[0].affected_file_count = 9;
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::PatchFileCountInconsistent));
}

#[test]
fn patch_affected_paths_missing_fails() {
    let mut packet = baseline();
    packet.patch_apply_sheets[0].affected_paths.clear();
    packet.patch_apply_sheets[0].affected_file_count = 0;
    let violations = packet.validate();
    assert!(violations.contains(&GitMutationReviewViolation::PatchAffectedPathsMissing));
}

#[test]
fn missing_conflict_cards_fails() {
    let mut packet = baseline();
    packet.conflict_checkpoint_cards.clear();
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::ConflictCheckpointCardsMissing));
}

#[test]
fn conflict_card_incomplete_fails() {
    let mut packet = baseline();
    packet.conflict_checkpoint_cards[0].checkpoint_label = String::new();
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::ConflictCardIncomplete));
}

#[test]
fn wrong_component_for_conflict_card_fails() {
    let mut packet = baseline();
    packet.conflict_checkpoint_cards[0].component = M5GitHistoryComponent::ForcePushReviewDialog;
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::WrongComponentForConflictCard));
}

#[test]
fn conflict_context_incomplete_fails() {
    let mut packet = baseline();
    // Dropping the base side hides the common ancestor context.
    packet.conflict_checkpoint_cards[0]
        .sides_present
        .retain(|side| *side != ConflictSide::Base);
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::ConflictContextIncomplete));
}

#[test]
fn conflict_counts_inconsistent_fails() {
    let mut packet = baseline();
    packet.conflict_checkpoint_cards[0].unresolved_count = 9;
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::ConflictCountsInconsistent));
}

#[test]
fn unresolved_conflict_not_reopenable_fails() {
    let mut packet = baseline();
    // The unresolved card claims it is resolved-and-applied, hiding the reopen path.
    packet.conflict_checkpoint_cards[0].reopen_state = CheckpointReopenState::ResolvedApplied;
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::UnresolvedConflictNotReopenable));
}

#[test]
fn conflict_reopen_behavior_missing_fails() {
    let mut packet = baseline();
    packet.conflict_checkpoint_cards[0].reopen_disclosure = String::new();
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::ConflictReopenBehaviorMissing));
}

#[test]
fn conflict_card_claims_mutating_class_fails() {
    let mut packet = baseline();
    packet.conflict_checkpoint_cards[0].review_class = MutationReviewClass::ForcePushConfirm;
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::ConflictCardClaimsMutatingClass));
}

#[test]
fn missing_force_push_dialogs_fails() {
    let mut packet = baseline();
    packet.force_push_dialogs.clear();
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::ForcePushDialogsMissing));
}

#[test]
fn wrong_component_for_force_push_dialog_fails() {
    let mut packet = baseline();
    packet.force_push_dialogs[0].component = M5GitHistoryComponent::ConflictCheckpointCard;
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::WrongComponentForForcePushDialog));
}

#[test]
fn force_push_target_missing_fails() {
    let mut packet = baseline();
    packet.force_push_dialogs[0].target_remote = String::new();
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::ForcePushTargetMissing));
}

#[test]
fn force_push_tips_missing_fails() {
    let mut packet = baseline();
    packet.force_push_dialogs[0].remote_tip_short_id = String::new();
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::ForcePushTipsMissing));
}

#[test]
fn force_push_recovery_ref_missing_fails() {
    let mut packet = baseline();
    packet.force_push_dialogs[0].recovery_ref = String::new();
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::ForcePushRecoveryRefMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = baseline();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::MissingSourceContracts));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = baseline();
    packet.downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = baseline();
    packet.consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::ConsumerSurfacesMissing));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = baseline();
    packet.trust_review.distinct_verbs_never_collapsed = false;
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = baseline();
    packet
        .consumer_projection
        .consequences_explicit_across_surfaces = false;
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = baseline();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::ProofFreshnessIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = baseline();
    packet.record_kind = "something_else".to_owned();
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::WrongRecordKind));
}

#[test]
fn raw_boundary_material_in_export_fails() {
    let mut packet = baseline();
    packet.surface_label = "leak: bearer abc123".to_owned();
    assert!(packet
        .validate()
        .contains(&GitMutationReviewViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_all_four_sections() {
    let summary = baseline().render_markdown_summary();
    assert!(summary.contains("## Cherry-pick / revert review sheets"));
    assert!(summary.contains("## Patch-apply review sheets"));
    assert!(summary.contains("## Conflict-checkpoint cards"));
    assert!(summary.contains("## Force-push review dialogs"));
}

#[test]
fn force_push_safety_and_hosted_impact_semantics() {
    assert!(ForcePushSafety::ForceWithLease.uses_lease());
    assert!(ForcePushSafety::ForceWithLeaseExpecting.uses_lease());
    assert!(!ForcePushSafety::PlainForce.uses_lease());
    assert!(HostedReviewImpact::InvalidatesApproval.affects_hosted_review());
    assert!(!HostedReviewImpact::LocalOnly.affects_hosted_review());
}

#[test]
fn cherry_pick_fixture_holds_at_reopenable_checkpoint() {
    let packet: GitMutationReviewPacket =
        serde_json::from_str(CHERRY_PICK_FIXTURE).expect("cherry-pick fixture parses");
    let sheet = packet
        .cherry_pick_revert_sheets
        .iter()
        .find(|sheet| sheet.verb == CherryPickRevertVerb::CherryPick)
        .expect("cherry-pick sheet present");
    assert_eq!(sheet.blocked_state, MutationBlockerState::ConflictExpected);
    assert!(!sheet.blocker_disclosure.trim().is_empty());
    // The card that captures the conflict stays reopenable with full context.
    let card = packet
        .conflict_checkpoint_cards
        .iter()
        .find(|card| card.unresolved_count > 0)
        .expect("unresolved card present");
    assert!(card.reopen_state.offers_reopen_path());
    assert!(card.preserves_required_context());
}

#[test]
fn force_push_fixture_stays_recoverable() {
    let packet: GitMutationReviewPacket =
        serde_json::from_str(FORCE_PUSH_FIXTURE).expect("force-push fixture parses");
    let dialog = &packet.force_push_dialogs[0];
    assert!(dialog.safety.uses_lease());
    assert!(!dialog.recovery_ref.trim().is_empty());
    assert!(dialog.checkpoint_state.is_recoverable());
    assert_eq!(
        dialog.hosted_review_impact,
        HostedReviewImpact::InvalidatesApproval
    );
}
