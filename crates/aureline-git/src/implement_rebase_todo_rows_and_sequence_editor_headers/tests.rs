use super::*;

const CANONICAL_PACKET_ID: &str = "m5-rebase-todo-sequence-editor-component:stable:0001";

const CANONICAL_EXPORT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5-rebase-todo-sequence-editor-components-proof/support_export.json"
));

const REORDERED_PLAN_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-rebase-todo-sequence-editor-components/reordered_plan.json"
));

const DROPPED_STEP_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-rebase-todo-sequence-editor-components/dropped_step_recovery.json"
));

fn todo_rows() -> Vec<RebaseTodoRow> {
    use GitHistoryDowngradeState::*;
    use SequenceBlockerKind::*;
    use SequenceCheckpointState::*;
    use SequenceOperation::*;

    vec![
        // A picked commit that keeps its original position: the anchor that proves
        // original order survives editing.
        RebaseTodoRow {
            row_id: "todo:0".to_owned(),
            component: M5GitHistoryComponent::RebaseTodoRow,
            original_index: 0,
            display_index: 0,
            commit_short_id: "a1b2c3d".to_owned(),
            commit_subject: "add review-queue projection scaffold".to_owned(),
            commit_author: "Dev A <dev.a@example.com>".to_owned(),
            operation: Pick,
            plan_state: TodoPlanState::Unchanged,
            unresolved_blockers: vec![],
            blocker_disclosure: String::new(),
            checkpoint_state: Captured,
            checkpoint_disclosure:
                "Pre-rebase checkpoint captured at feature/review-lane@{1}; restore returns the original tip"
                    .to_owned(),
            raw_todo_line: "pick a1b2c3d add review-queue projection scaffold".to_owned(),
            downgrade_vocab: vec![DirtyOrConflictedWorktree, OfflineLocalOnly],
            fields_shown: vec![
                "commit_short_id".to_owned(),
                "commit_subject".to_owned(),
                "operation".to_owned(),
                "plan_state".to_owned(),
            ],
            source_contract_refs: vec![REBASE_SEQUENCE_SEQUENCE_EDIT_CONTRACT_REF.to_owned()],
        },
        // A squash folded into the previous commit; a missing combined message is
        // an unresolved blocker that is disclosed, not hidden.
        RebaseTodoRow {
            row_id: "todo:1".to_owned(),
            component: M5GitHistoryComponent::RebaseTodoRow,
            original_index: 1,
            display_index: 1,
            commit_short_id: "b2c3d4e".to_owned(),
            commit_subject: "fold: review-queue projection tests".to_owned(),
            commit_author: "Dev A <dev.a@example.com>".to_owned(),
            operation: Squash,
            plan_state: TodoPlanState::SquashedIntoPrevious,
            unresolved_blockers: vec![MissingSquashMessage],
            blocker_disclosure:
                "Squash needs a combined message before this step can be written".to_owned(),
            checkpoint_state: PerStepCaptured,
            checkpoint_disclosure:
                "A checkpoint is captured before each applied step; undo returns to the prior step"
                    .to_owned(),
            raw_todo_line: "squash b2c3d4e fold: review-queue projection tests".to_owned(),
            downgrade_vocab: vec![DirtyOrConflictedWorktree, ReflogOnlyFallback],
            fields_shown: vec![
                "commit_short_id".to_owned(),
                "operation".to_owned(),
                "plan_state".to_owned(),
                "unresolved_blockers".to_owned(),
            ],
            source_contract_refs: vec![
                REBASE_SEQUENCE_SEQUENCE_EDIT_CONTRACT_REF.to_owned(),
                REBASE_SEQUENCE_CHECKPOINT_CONTRACT_REF.to_owned(),
            ],
        },
        // A reworded commit moved later in the plan: reordered because its display
        // position differs from the original.
        RebaseTodoRow {
            row_id: "todo:2".to_owned(),
            component: M5GitHistoryComponent::RebaseTodoRow,
            original_index: 2,
            display_index: 3,
            commit_short_id: "c3d4e5f".to_owned(),
            commit_subject: "rename queue projection to review lane".to_owned(),
            commit_author: "Dev B <dev.b@example.com>".to_owned(),
            operation: Reword,
            plan_state: TodoPlanState::Reordered,
            unresolved_blockers: vec![],
            blocker_disclosure: String::new(),
            checkpoint_state: Captured,
            checkpoint_disclosure:
                "Pre-rebase checkpoint captured; the reworded message can be recovered from the reflog"
                    .to_owned(),
            raw_todo_line: "reword c3d4e5f rename queue projection to review lane".to_owned(),
            downgrade_vocab: vec![DetachedOrMissingRef, StaleProviderOverlay],
            fields_shown: vec![
                "commit_short_id".to_owned(),
                "commit_subject".to_owned(),
                "operation".to_owned(),
                "plan_state".to_owned(),
            ],
            source_contract_refs: vec![REBASE_SEQUENCE_SEQUENCE_EDIT_CONTRACT_REF.to_owned()],
        },
        // A picked commit moved earlier in the plan: also reordered.
        RebaseTodoRow {
            row_id: "todo:3".to_owned(),
            component: M5GitHistoryComponent::RebaseTodoRow,
            original_index: 3,
            display_index: 2,
            commit_short_id: "d4e5f6a".to_owned(),
            commit_subject: "wire review lane into shell".to_owned(),
            commit_author: "Dev B <dev.b@example.com>".to_owned(),
            operation: Pick,
            plan_state: TodoPlanState::Reordered,
            unresolved_blockers: vec![],
            blocker_disclosure: String::new(),
            checkpoint_state: Captured,
            checkpoint_disclosure:
                "Pre-rebase checkpoint captured; the original ordering is restorable from the reflog"
                    .to_owned(),
            raw_todo_line: "pick d4e5f6a wire review lane into shell".to_owned(),
            downgrade_vocab: vec![DirtyOrConflictedWorktree, OfflineLocalOnly],
            fields_shown: vec![
                "commit_short_id".to_owned(),
                "operation".to_owned(),
                "plan_state".to_owned(),
            ],
            source_contract_refs: vec![REBASE_SEQUENCE_SEQUENCE_EDIT_CONTRACT_REF.to_owned()],
        },
        // A dropped commit: removed from the rewritten history but still recoverable
        // through the reflog fallback.
        RebaseTodoRow {
            row_id: "todo:4".to_owned(),
            component: M5GitHistoryComponent::RebaseTodoRow,
            original_index: 4,
            display_index: 4,
            commit_short_id: "e5f6a7b".to_owned(),
            commit_subject: "spike: throwaway telemetry probe".to_owned(),
            commit_author: "Dev A <dev.a@example.com>".to_owned(),
            operation: Drop,
            plan_state: TodoPlanState::Dropped,
            unresolved_blockers: vec![],
            blocker_disclosure: String::new(),
            checkpoint_state: ReflogFallbackOnly,
            checkpoint_disclosure:
                "No explicit checkpoint for this drop; the dropped commit stays reachable via the reflog until it expires"
                    .to_owned(),
            raw_todo_line: "drop e5f6a7b spike: throwaway telemetry probe".to_owned(),
            downgrade_vocab: vec![ReflogOnlyFallback, OfflineLocalOnly],
            fields_shown: vec![
                "commit_short_id".to_owned(),
                "operation".to_owned(),
                "plan_state".to_owned(),
            ],
            source_contract_refs: vec![
                REBASE_SEQUENCE_SEQUENCE_EDIT_CONTRACT_REF.to_owned(),
                REBASE_SEQUENCE_CHECKPOINT_CONTRACT_REF.to_owned(),
            ],
        },
    ]
}

fn sequence_headers() -> Vec<SequenceEditorHeaderRow> {
    use GitHistoryDowngradeState::*;
    use SequenceCheckpointState::*;

    vec![
        // The active rewrite session: reorders, squashes, and drops, so recovery
        // must stay reachable and it confirms as a full sequence rewrite.
        SequenceEditorHeaderRow {
            row_id: "header:rewrite".to_owned(),
            component: M5GitHistoryComponent::SequenceEditorHeader,
            session_label: "Interactive rebase: tidy review-lane history".to_owned(),
            onto_ref: "main".to_owned(),
            original_tip_ref: "feature/review-lane@{1}".to_owned(),
            total_commits: 5,
            reordered_count: 2,
            squashed_count: 1,
            dropped_count: 1,
            unresolved_blocker_count: 1,
            checkpoint_state: Captured,
            checkpoint_disclosure:
                "Pre-rebase checkpoint captured at feature/review-lane@{1}; abort restores the original tip"
                    .to_owned(),
            review_class: MutationReviewClass::SequenceRewriteConfirm,
            original_order_note:
                "The original 5-commit order is shown alongside each row's new position; nothing is applied until the whole plan is confirmed"
                    .to_owned(),
            downgrade_vocab: vec![DirtyOrConflictedWorktree, StaleProviderOverlay],
            fields_shown: vec![
                "onto_ref".to_owned(),
                "original_tip_ref".to_owned(),
                "total_commits".to_owned(),
                "original_order_note".to_owned(),
            ],
            source_contract_refs: vec![
                REBASE_SEQUENCE_SEQUENCE_EDIT_CONTRACT_REF.to_owned(),
                REBASE_SEQUENCE_REVIEW_CONTRACT_REF.to_owned(),
            ],
        },
        // A review-only session: no commit changes position or shape, but the
        // checkpoint state and original order are still disclosed.
        SequenceEditorHeaderRow {
            row_id: "header:review-only".to_owned(),
            component: M5GitHistoryComponent::SequenceEditorHeader,
            session_label: "Interactive rebase: review only, no edits".to_owned(),
            onto_ref: "main".to_owned(),
            original_tip_ref: "feature/docs@{2}".to_owned(),
            total_commits: 3,
            reordered_count: 0,
            squashed_count: 0,
            dropped_count: 0,
            unresolved_blocker_count: 0,
            checkpoint_state: PerStepCaptured,
            checkpoint_disclosure:
                "A checkpoint is captured before each step even though no edits are planned"
                    .to_owned(),
            review_class: MutationReviewClass::SequenceRewriteConfirm,
            original_order_note:
                "All 3 commits keep their original order; confirming leaves history unchanged"
                    .to_owned(),
            downgrade_vocab: vec![OfflineLocalOnly, ReflogOnlyFallback],
            fields_shown: vec![
                "onto_ref".to_owned(),
                "original_tip_ref".to_owned(),
                "total_commits".to_owned(),
                "original_order_note".to_owned(),
            ],
            source_contract_refs: vec![REBASE_SEQUENCE_SEQUENCE_EDIT_CONTRACT_REF.to_owned()],
        },
    ]
}

fn trust_review() -> RebaseSequenceTrustReview {
    RebaseSequenceTrustReview {
        commit_identity_always_explicit: true,
        original_order_always_preserved: true,
        plan_state_never_misrepresented: true,
        operations_stay_distinct: true,
        unresolved_blockers_always_disclosed: true,
        checkpoint_reachable_after_rewrite: true,
        raw_and_structured_meaning_equivalent: true,
        sequence_rewrite_confirm_never_collapsed: true,
        local_only_recovery_stays_explicit: true,
        one_component_contract_no_hidden_meaning: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> RebaseSequenceConsumerProjection {
    RebaseSequenceConsumerProjection {
        git_history_reuses_one_contract: true,
        review_reuses_one_contract: true,
        help_support_reuses_one_contract: true,
        support_export_reuses_one_contract: true,
        raw_fallback_equivalent_across_surfaces: true,
        cli_headless_shows_truth: true,
        provider_overlay_shows_truth: true,
        ai_context_shows_truth: true,
    }
}

fn proof_freshness() -> RebaseSequenceProofFreshness {
    RebaseSequenceProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-07-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<GitHistoryDowngradeState> {
    vec![
        GitHistoryDowngradeState::DirtyOrConflictedWorktree,
        GitHistoryDowngradeState::DetachedOrMissingRef,
        GitHistoryDowngradeState::ReflogOnlyFallback,
        GitHistoryDowngradeState::OfflineLocalOnly,
        GitHistoryDowngradeState::StaleProviderOverlay,
    ]
}

fn consumer_surfaces() -> Vec<ComponentConsumerSurface> {
    ComponentConsumerSurface::ALL.to_vec()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        REBASE_SEQUENCE_SCHEMA_REF.to_owned(),
        REBASE_SEQUENCE_DOC_REF.to_owned(),
        REBASE_SEQUENCE_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        REBASE_SEQUENCE_SEQUENCE_EDIT_CONTRACT_REF.to_owned(),
        REBASE_SEQUENCE_CHECKPOINT_CONTRACT_REF.to_owned(),
        REBASE_SEQUENCE_REVIEW_CONTRACT_REF.to_owned(),
    ]
}

fn seed_packet() -> RebaseSequenceEditPacket {
    RebaseSequenceEditPacket::new(RebaseSequenceEditPacketInput {
        packet_id: CANONICAL_PACKET_ID.to_owned(),
        surface_label:
            "Rebase todo rows and sequence-editor headers: ordered-plan and checkpoint truth"
                .to_owned(),
        todo_rows: todo_rows(),
        sequence_headers: sequence_headers(),
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

fn baseline() -> RebaseSequenceEditPacket {
    seed_packet()
}

/// Regenerates the checked-in artifacts and fixtures.
///
/// Guarded by `GEN_REBASE_SEQUENCE_ARTIFACTS` so it is inert in CI but can
/// deterministically rewrite the export, summary, and narrowed fixtures.
#[test]
fn generate_artifacts() {
    if std::env::var_os("GEN_REBASE_SEQUENCE_ARTIFACTS").is_none() {
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
        format!("{root}/{REBASE_SEQUENCE_ARTIFACT_REF}"),
        format!("{}\n", canonical.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        format!("{root}/{REBASE_SEQUENCE_SUMMARY_REF}"),
        canonical.render_markdown_summary(),
    )
    .expect("write summary");

    // Reordered-plan fixture: the two reordered rows keep their original indices
    // explicit alongside the new positions.
    let mut reordered = seed_packet();
    reordered.packet_id = "m5-rebase-todo-sequence-editor-component:reordered-plan:0001".to_owned();
    assert!(
        reordered.validate().is_empty(),
        "{:?}",
        reordered.validate()
    );
    std::fs::write(
        format!("{root}/{REBASE_SEQUENCE_FIXTURE_DIR}/reordered_plan.json"),
        format!("{}\n", reordered.export_safe_json()),
    )
    .expect("write reordered-plan fixture");

    // Dropped-step fixture: the dropped commit gains a conflict blocker but stays
    // recoverable through the reflog fallback.
    let mut dropped = seed_packet();
    {
        let row = dropped
            .todo_rows
            .iter_mut()
            .find(|row| row.plan_state == TodoPlanState::Dropped)
            .expect("dropped row present");
        row.unresolved_blockers = vec![SequenceBlockerKind::ConflictAtStep];
        row.blocker_disclosure =
            "Dropping this commit conflicts with a later step; resolve before continuing"
                .to_owned();
        row.checkpoint_disclosure =
            "The dropped commit stays reachable via the reflog until it expires; recovery remains local"
                .to_owned();
    }
    dropped.packet_id =
        "m5-rebase-todo-sequence-editor-component:dropped-step-recovery:0001".to_owned();
    assert!(dropped.validate().is_empty(), "{:?}", dropped.validate());
    std::fs::write(
        format!("{root}/{REBASE_SEQUENCE_FIXTURE_DIR}/dropped_step_recovery.json"),
        format!("{}\n", dropped.export_safe_json()),
    )
    .expect("write dropped-step fixture");
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
        current_rebase_sequence_edit_export().expect("checked rebase sequence export validates");
    assert_eq!(packet.packet_id, CANONICAL_PACKET_ID);
}

#[test]
fn checked_export_matches_seed() {
    let checked: RebaseSequenceEditPacket =
        serde_json::from_str(CANONICAL_EXPORT).expect("canonical export deserializes");
    assert_eq!(checked, seed_packet());
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [REORDERED_PLAN_FIXTURE, DROPPED_STEP_FIXTURE] {
        let packet: RebaseSequenceEditPacket =
            serde_json::from_str(raw).expect("fixture parses as rebase sequence packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn plan_state_resolver_derives_precedence() {
    use SequenceOperation::*;
    // Drop wins over position.
    assert_eq!(resolve_todo_plan_state(Drop, 2, 5), TodoPlanState::Dropped);
    // Squash/fixup fold into previous regardless of position.
    assert_eq!(
        resolve_todo_plan_state(Squash, 1, 1),
        TodoPlanState::SquashedIntoPrevious
    );
    assert_eq!(
        resolve_todo_plan_state(Fixup, 3, 4),
        TodoPlanState::SquashedIntoPrevious
    );
    // A moved pick is reordered; a stationary pick is unchanged.
    assert_eq!(
        resolve_todo_plan_state(Pick, 3, 2),
        TodoPlanState::Reordered
    );
    assert_eq!(
        resolve_todo_plan_state(Pick, 0, 0),
        TodoPlanState::Unchanged
    );
    // A stationary reword keeps its order-neutral position.
    assert_eq!(
        resolve_todo_plan_state(Reword, 1, 1),
        TodoPlanState::Unchanged
    );
}

#[test]
fn step_recovery_resolver_requires_recovery_for_rewrites() {
    use SequenceOperation::*;
    // A plain pick that stays put does not force a checkpoint.
    let pick = resolve_sequence_step_recovery(Pick, SequenceCheckpointState::Unavailable);
    assert!(!pick.must_be_recoverable);
    assert!(pick.must_disclose_checkpoint);
    // Any rewriting op must be recoverable.
    let drop = resolve_sequence_step_recovery(Drop, SequenceCheckpointState::Captured);
    assert!(drop.must_be_recoverable);
    assert!(drop.is_recoverable);
    let squash = resolve_sequence_step_recovery(Squash, SequenceCheckpointState::Unavailable);
    assert!(squash.must_be_recoverable);
    assert!(!squash.is_recoverable);
}

#[test]
fn missing_todo_rows_fails() {
    let mut packet = baseline();
    packet.todo_rows.clear();
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::RebaseTodoRowsMissing));
}

#[test]
fn incomplete_todo_row_fails() {
    let mut packet = baseline();
    packet.todo_rows[0].raw_todo_line = String::new();
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::TodoRowIncomplete));
}

#[test]
fn wrong_component_for_todo_row_fails() {
    let mut packet = baseline();
    packet.todo_rows[0].component = M5GitHistoryComponent::SequenceEditorHeader;
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::WrongComponentForTodoRow));
}

#[test]
fn commit_identity_missing_fails() {
    let mut packet = baseline();
    packet.todo_rows[0].commit_author = String::new();
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::CommitIdentityMissing));
}

#[test]
fn plan_state_misrepresented_fails() {
    let mut packet = baseline();
    // The squash row claims to be merely reordered, hiding that it folds away.
    packet.todo_rows[1].plan_state = TodoPlanState::Reordered;
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::PlanStateMisrepresented));
}

#[test]
fn reordered_row_claiming_unchanged_fails() {
    let mut packet = baseline();
    // The reordered reword row claims its position never moved.
    packet.todo_rows[2].plan_state = TodoPlanState::Unchanged;
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::PlanStateMisrepresented));
}

#[test]
fn raw_todo_line_misaligned_fails() {
    let mut packet = baseline();
    // The raw line claims a drop while the structured op is a pick.
    packet.todo_rows[0].raw_todo_line =
        "drop a1b2c3d add review-queue projection scaffold".to_owned();
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::RawTodoLineMisaligned));
}

#[test]
fn raw_todo_line_wrong_commit_fails() {
    let mut packet = baseline();
    // The raw line keeps the verb but names a different commit.
    packet.todo_rows[0].raw_todo_line = "pick 9999999 unrelated commit".to_owned();
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::RawTodoLineMisaligned));
}

#[test]
fn unresolved_blocker_not_disclosed_fails() {
    let mut packet = baseline();
    // The squash row keeps its blocker but drops the disclosure.
    packet.todo_rows[1].blocker_disclosure = String::new();
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::UnresolvedBlockerNotDisclosed));
}

#[test]
fn checkpoint_state_undisclosed_fails() {
    let mut packet = baseline();
    packet.todo_rows[0].checkpoint_disclosure = String::new();
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::CheckpointStateUndisclosed));
}

#[test]
fn recovery_checkpoint_missing_fails() {
    let mut packet = baseline();
    // The squash rewrites history but loses its recovery checkpoint.
    packet.todo_rows[1].checkpoint_state = SequenceCheckpointState::Unavailable;
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::RecoveryCheckpointMissing));
}

#[test]
fn original_order_preservation_coverage_missing_fails() {
    let mut packet = baseline();
    // With every remaining row reordered/squashed/dropped, nothing proves the
    // original order survives editing.
    packet
        .todo_rows
        .retain(|row| row.plan_state != TodoPlanState::Unchanged);
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::OriginalOrderPreservationCoverageMissing));
}

#[test]
fn missing_sequence_headers_fails() {
    let mut packet = baseline();
    packet.sequence_headers.clear();
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::SequenceEditorHeadersMissing));
}

#[test]
fn incomplete_header_row_fails() {
    let mut packet = baseline();
    packet.sequence_headers[0].session_label = String::new();
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::HeaderRowIncomplete));
}

#[test]
fn wrong_component_for_header_row_fails() {
    let mut packet = baseline();
    packet.sequence_headers[0].component = M5GitHistoryComponent::RebaseTodoRow;
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::WrongComponentForHeaderRow));
}

#[test]
fn onto_ref_missing_fails() {
    let mut packet = baseline();
    packet.sequence_headers[0].onto_ref = String::new();
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::OntoRefMissing));
}

#[test]
fn original_tip_recovery_anchor_missing_fails() {
    let mut packet = baseline();
    packet.sequence_headers[0].original_tip_ref = String::new();
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::OriginalTipRecoveryAnchorMissing));
}

#[test]
fn original_order_context_missing_fails() {
    let mut packet = baseline();
    packet.sequence_headers[0].original_order_note = String::new();
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::OriginalOrderContextMissing));
}

#[test]
fn header_counts_inconsistent_fails() {
    let mut packet = baseline();
    // More dropped than exist in the whole sequence.
    packet.sequence_headers[0].dropped_count = 9;
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::HeaderCountsInconsistent));
}

#[test]
fn sequence_confirm_collapsed_fails() {
    let mut packet = baseline();
    // Collapsing the sequence rewrite into a single-verb confirm.
    packet.sequence_headers[0].review_class = MutationReviewClass::ExplicitVerbConfirm;
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::SequenceConfirmCollapsed));
}

#[test]
fn header_recovery_checkpoint_missing_fails() {
    let mut packet = baseline();
    // The rewrite session loses its recovery checkpoint.
    packet.sequence_headers[0].checkpoint_state = SequenceCheckpointState::Unavailable;
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::HeaderRecoveryCheckpointMissing));
}

#[test]
fn header_checkpoint_undisclosed_fails() {
    let mut packet = baseline();
    packet.sequence_headers[0].checkpoint_disclosure = String::new();
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::HeaderCheckpointUndisclosed));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = baseline();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::MissingSourceContracts));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = baseline();
    packet.downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = baseline();
    packet.consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::ConsumerSurfacesMissing));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = baseline();
    packet.trust_review.plan_state_never_misrepresented = false;
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = baseline();
    packet
        .consumer_projection
        .raw_fallback_equivalent_across_surfaces = false;
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = baseline();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::ProofFreshnessIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = baseline();
    packet.record_kind = "something_else".to_owned();
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::WrongRecordKind));
}

#[test]
fn raw_boundary_material_in_export_fails() {
    let mut packet = baseline();
    packet.surface_label = "leak: bearer abc123".to_owned();
    assert!(packet
        .validate()
        .contains(&RebaseSequenceViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_todo_and_header_sections() {
    let summary = baseline().render_markdown_summary();
    assert!(summary.contains("## Rebase todo rows"));
    assert!(summary.contains("## Sequence-editor headers"));
    for row in todo_rows() {
        assert!(
            summary.contains(&row.commit_short_id),
            "summary missing commit {}",
            row.commit_short_id
        );
    }
}

#[test]
fn both_sequence_components_are_the_frozen_pair() {
    assert!(REBASE_SEQUENCE_COMPONENTS.contains(&M5GitHistoryComponent::RebaseTodoRow));
    assert!(REBASE_SEQUENCE_COMPONENTS.contains(&M5GitHistoryComponent::SequenceEditorHeader));
    // Both sequence-edit components are risky-mutation surfaces in the matrix.
    assert!(M5GitHistoryComponent::RebaseTodoRow.is_risky_mutation_surface());
    assert!(M5GitHistoryComponent::SequenceEditorHeader.is_risky_mutation_surface());
}

#[test]
fn operation_semantics_stay_distinct() {
    assert!(SequenceOperation::Squash.combines_with_previous());
    assert!(SequenceOperation::Fixup.combines_with_previous());
    assert!(!SequenceOperation::Pick.combines_with_previous());
    assert!(SequenceOperation::Drop.removes_commit());
    assert!(!SequenceOperation::Reword.removes_commit());
    // Only a plain pick leaves history untouched.
    assert!(!SequenceOperation::Pick.rewrites_or_removes());
    for op in [
        SequenceOperation::Reword,
        SequenceOperation::Edit,
        SequenceOperation::Squash,
        SequenceOperation::Fixup,
        SequenceOperation::Drop,
    ] {
        assert!(op.rewrites_or_removes(), "{} should rewrite", op.as_str());
    }
}

#[test]
fn reordered_fixture_keeps_original_index_explicit() {
    let packet: RebaseSequenceEditPacket =
        serde_json::from_str(REORDERED_PLAN_FIXTURE).expect("reordered-plan fixture parses");
    let row = packet
        .todo_rows
        .iter()
        .find(|row| row.plan_state == TodoPlanState::Reordered)
        .expect("reordered row present");
    assert_ne!(row.original_index, row.display_index);
    assert!(row.raw_and_structured_agree());
}

#[test]
fn dropped_fixture_stays_recoverable() {
    let packet: RebaseSequenceEditPacket =
        serde_json::from_str(DROPPED_STEP_FIXTURE).expect("dropped-step fixture parses");
    let row = packet
        .todo_rows
        .iter()
        .find(|row| row.plan_state == TodoPlanState::Dropped)
        .expect("dropped row present");
    assert!(row.checkpoint_state.is_recoverable());
    assert!(!row.blocker_disclosure.trim().is_empty());
}
