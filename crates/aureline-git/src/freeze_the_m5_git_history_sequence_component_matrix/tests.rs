use super::*;

const CANONICAL_PACKET_ID: &str = "m5-git-history-sequence-component-matrix:frozen:0001";

const CANONICAL_EXPORT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5-git-history-sequence-proof/support_export.json"
));

const FORCE_PUSH_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-git-history-sequence-components/force_push_approval_invalidated.json"
));

const STASH_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-git-history-sequence-components/stash_entry_reflog_only_recovery.json"
));

/// Builds one component row from frozen inputs.
#[allow(clippy::too_many_arguments)]
fn row(
    component: M5GitHistoryComponent,
    label: &str,
    maturity: ComponentMaturityPosture,
    identity: &str,
    recovery: &str,
    approval: &str,
    handoff: &str,
    downgrade_vocab: Vec<GitHistoryDowngradeState>,
    mutation_review_class: MutationReviewClass,
) -> ComponentRow {
    ComponentRow {
        component,
        label: label.to_owned(),
        maturity,
        canonical_source_contract_ref: component.canonical_source_contract_ref().to_owned(),
        identity_preservation: identity.to_owned(),
        recovery_checkpoint_rule: recovery.to_owned(),
        approval_invalidation_rule: approval.to_owned(),
        browser_provider_handoff_rule: handoff.to_owned(),
        downgrade_vocab,
        mutation_review_class,
        preserves_distinct_verb: true,
        consumer_surfaces: ComponentConsumerSurface::ALL.to_vec(),
    }
}

/// Canonical, frozen component-matrix packet built in Rust.
fn seed_packet() -> M5GitHistoryComponentMatrixPacket {
    use ComponentMaturityPosture::{Beta, Preview, Stable};
    use GitHistoryDowngradeState::*;
    use M5GitHistoryComponent::*;
    use MutationReviewClass::*;

    let component_rows = vec![
        row(
            CommitGraphHeader,
            "Commit graph header",
            Stable,
            "Names the exact repo root, active branch/ref anchor, and commit range shown; never a bare 'history' label.",
            "Read-only header; links to the reflog recovery banner rather than owning a recovery destination.",
            "Surfaces no approval; annotates when a shown ref underlies an approval that a rewrite would invalidate.",
            "Opens the hosted commit view via explicit browser handoff; never implies local truth is the provider's.",
            vec![StaleProviderOverlay, DetachedOrMissingRef, ShallowOrPartialTopology, OfflineLocalOnly],
            DisplayOnlyNoMutation,
        ),
        row(
            HistoryGraphRow,
            "History graph row",
            Stable,
            "Names the exact commit id, parent lineage, and the worktree/ref it belongs to.",
            "Read-only row; a risky action on it hands off to a review sheet that owns recovery.",
            "Marks a commit whose rewrite would invalidate a linked approval.",
            "Deep-links to the hosted commit via explicit handoff; local graph stays authoritative.",
            vec![ShallowOrPartialTopology, DetachedOrMissingRef, StaleProviderOverlay],
            DisplayOnlyNoMutation,
        ),
        row(
            BranchComparisonChip,
            "Branch comparison chip",
            Stable,
            "Names both exact refs and the merge base; ahead/behind counts never hide which ref is which.",
            "Read-only chip; comparison never mutates, so it references not owns recovery.",
            "Flags when the compared base has moved such that an existing approval is stale.",
            "Links to the hosted compare view via explicit handoff; local comparison stays authoritative.",
            vec![StaleProviderOverlay, DetachedOrMissingRef, OfflineLocalOnly],
            DisplayOnlyNoMutation,
        ),
        row(
            WorktreeRow,
            "Worktree row",
            Stable,
            "Names the exact worktree path, its checked-out branch/ref, and its repo root; never conflates worktrees.",
            "Read-only row; switching worktrees preserves the dirty state and points at stash recovery.",
            "Notes when a worktree holds changes underlying a pending approval.",
            "Provider handoff is per-repo, not per-worktree; the row keeps the distinction explicit.",
            vec![DirtyOrConflictedWorktree, DetachedOrMissingRef, OfflineLocalOnly],
            DisplayOnlyNoMutation,
        ),
        row(
            StashEntry,
            "Stash entry",
            Stable,
            "Names the exact stash ref, its message, and the worktree/branch it was taken from.",
            "Apply keeps the shelf; pop/drop expose a reflog-only recovery destination before running.",
            "Applying a stash that reintroduces reviewed changes marks the approval as needing recomputation.",
            "Stash entries are local-only; the row states there is no provider handoff for them.",
            vec![DirtyOrConflictedWorktree, DetachedOrMissingRef, OfflineLocalOnly],
            StashRestoreConfirm,
        ),
        row(
            ReflogRecoveryBanner,
            "Reflog recovery banner",
            Stable,
            "Names the exact prior ref position (and reflog selector) the banner can restore to.",
            "Is itself the recovery destination surface; it discloses the reflog-only fallback path.",
            "States when restoring will re-open a previously invalidated approval.",
            "Reflog recovery is local-only; the banner never implies the provider can restore it.",
            vec![ReflogOnlyFallback, DetachedOrMissingRef, OfflineLocalOnly],
            DisplayOnlyNoMutation,
        ),
        row(
            RebaseTodoRow,
            "Rebase todo row",
            Stable,
            "Names the exact commit and the pick/reword/edit/squash/drop verb applied to it; verbs never collapse.",
            "The parent sequence captures a recovery checkpoint before the plan runs.",
            "Editing a commit that underlies an approval marks the approval invalidated in the plan.",
            "Sequence edits are local; publishing the rewrite is a separate provider handoff.",
            vec![DirtyOrConflictedWorktree, DetachedOrMissingRef, ApprovalInvalidated, ReflogOnlyFallback],
            SequenceRewriteConfirm,
        ),
        row(
            SequenceEditorHeader,
            "Sequence editor header",
            Stable,
            "Names the exact base ref, target branch, and onto point of the interactive-rebase session.",
            "Owns the pre-run recovery checkpoint and shows the reflog-only fallback if none can be captured.",
            "Summarizes which approvals the planned rewrite will invalidate before it runs.",
            "The header states the rewrite is local until an explicit publish/force-push handoff.",
            vec![DirtyOrConflictedWorktree, ApprovalInvalidated, ReflogOnlyFallback, StaleProviderOverlay],
            SequenceRewriteConfirm,
        ),
        row(
            CherryPickRevertReviewSheet,
            "Cherry-pick / revert review sheet",
            Stable,
            "Names the exact source commit(s), the target branch/worktree, and keeps cherry-pick vs revert distinct.",
            "Captures a recovery checkpoint before applying; conflicts route to a conflict checkpoint card.",
            "Applying onto an approved branch marks the approval as needing recomputation.",
            "The sheet applies locally; pushing the result is a separate explicit handoff.",
            vec![DirtyOrConflictedWorktree, ApprovalInvalidated, DetachedOrMissingRef],
            ExplicitVerbConfirm,
        ),
        row(
            PatchApplyReviewSheet,
            "Patch-apply review sheet",
            Beta,
            "Names the exact patch/mailbox source and the target ref/worktree it will apply onto.",
            "Captures a recovery checkpoint before applying; partial application discloses reflog fallback.",
            "Applying a patch over reviewed content marks the approval invalidated.",
            "Patch source may be a local file or a provider fetch; the sheet keeps the origin explicit.",
            vec![DirtyOrConflictedWorktree, DetachedOrMissingRef, OfflineLocalOnly],
            PatchApplyConfirm,
        ),
        row(
            ConflictCheckpointCard,
            "Conflict checkpoint card",
            Stable,
            "Names the exact operation, the conflicted paths, and the checkpoint ref that preserves pre-conflict state.",
            "Is a recovery surface: it exposes the checkpoint and reflog fallback that survive the mutation.",
            "States that the in-flight operation has invalidated approvals until it resolves.",
            "Conflict state is local; the card never lets a provider overlay erase it after a mutation.",
            vec![DirtyOrConflictedWorktree, ReflogOnlyFallback, ApprovalInvalidated],
            DisplayOnlyNoMutation,
        ),
        row(
            ForcePushReviewDialog,
            "Force-push review dialog",
            Preview,
            "Names the exact local ref, remote ref, and before/after positions; force-push never hides the target.",
            "Discloses the ref-update rollback (prior remote position) and local reflog fallback before confirming.",
            "Lists every approval the ref rewrite will invalidate before the push is confirmed.",
            "The dialog is the explicit provider handoff boundary; local truth is shown alongside the remote effect.",
            vec![ApprovalInvalidated, StaleProviderOverlay, DetachedOrMissingRef, ReflogOnlyFallback],
            ForcePushConfirm,
        ),
    ];

    let downgrade_state_rows = vec![
        DowngradeStateRow {
            state: StaleProviderOverlay,
            meaning: "A hosted provider overlay is older than local Git truth and must be labeled, not trusted.".to_owned(),
            narrows_claim: true,
            must_stay_visible_after_mutation: true,
        },
        DowngradeStateRow {
            state: DetachedOrMissingRef,
            meaning: "The target ref is detached or missing, so exact identity must be spelled out before any action.".to_owned(),
            narrows_claim: true,
            must_stay_visible_after_mutation: true,
        },
        DowngradeStateRow {
            state: DirtyOrConflictedWorktree,
            meaning: "The worktree has uncommitted or conflicted changes at the operation target.".to_owned(),
            narrows_claim: true,
            must_stay_visible_after_mutation: true,
        },
        DowngradeStateRow {
            state: ShallowOrPartialTopology,
            meaning: "History is shallow/partial/sparse here, so the shown graph is incomplete.".to_owned(),
            narrows_claim: true,
            must_stay_visible_after_mutation: false,
        },
        DowngradeStateRow {
            state: ReflogOnlyFallback,
            meaning: "No checkpoint exists; only a reflog-only recovery fallback is offered and must stay visible.".to_owned(),
            narrows_claim: true,
            must_stay_visible_after_mutation: true,
        },
        DowngradeStateRow {
            state: ApprovalInvalidated,
            meaning: "A prior approval was invalidated by this change and must be recomputed, never hidden.".to_owned(),
            narrows_claim: true,
            must_stay_visible_after_mutation: true,
        },
        DowngradeStateRow {
            state: OfflineLocalOnly,
            meaning: "Operating offline / local-only; provider handoff is unavailable and the surface says so.".to_owned(),
            narrows_claim: true,
            must_stay_visible_after_mutation: false,
        },
    ];

    let governance_review = MatrixGovernanceReview {
        every_surface_consumes_one_shared_matrix: true,
        exact_ref_worktree_identity_preserved: true,
        recovery_destination_always_explicit: true,
        approval_invalidation_never_silent: true,
        no_verb_collapsed_into_ambiguous_confirm: true,
        conflict_recovery_state_survives_mutation: true,
        provider_overlay_never_overwrites_local_truth: true,
        local_only_recovery_stays_explicit: true,
        downgrade_vocabulary_shared_across_surfaces: true,
    };

    let consumer_parity = MatrixConsumerParity {
        review_expresses_family: true,
        shell_expresses_family: true,
        help_expresses_family: true,
        support_export_expresses_family: true,
        cli_expresses_family: true,
        provider_overlay_expresses_family: true,
    };

    let freeze_posture = MatrixFreezePosture {
        frozen: true,
        review_slo_hours: 168,
        last_reviewed_at: "2026-07-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    };

    let source_contract_refs = vec![
        M5_GIT_HISTORY_COMPONENT_MATRIX_SCHEMA_REF.to_owned(),
        M5_GIT_HISTORY_COMPONENT_MATRIX_DOC_REF.to_owned(),
        M5_GIT_HISTORY_COMPONENT_MATRIX_COMMIT_HISTORY_CONTRACT_REF.to_owned(),
        M5_GIT_HISTORY_COMPONENT_MATRIX_TOPOLOGY_CONTRACT_REF.to_owned(),
        M5_GIT_HISTORY_COMPONENT_MATRIX_STASH_CONTRACT_REF.to_owned(),
        M5_GIT_HISTORY_COMPONENT_MATRIX_RECOVERY_CHECKPOINT_CONTRACT_REF.to_owned(),
        M5_GIT_HISTORY_COMPONENT_MATRIX_SEQUENCE_EDIT_CONTRACT_REF.to_owned(),
        M5_GIT_HISTORY_COMPONENT_MATRIX_HISTORY_SURGERY_CONTRACT_REF.to_owned(),
        M5_GIT_HISTORY_COMPONENT_MATRIX_CONFLICT_SESSION_CONTRACT_REF.to_owned(),
        M5_GIT_HISTORY_COMPONENT_MATRIX_REF_UPDATE_CONTRACT_REF.to_owned(),
    ];

    M5GitHistoryComponentMatrixPacket::new(M5GitHistoryComponentMatrixPacketInput {
        packet_id: CANONICAL_PACKET_ID.to_owned(),
        matrix_label: "M5 Git-history and risky-mutation component family".to_owned(),
        component_rows,
        downgrade_state_rows,
        governance_review,
        consumer_parity,
        freeze_posture,
        source_contract_refs,
        redaction_class_token: "aureline.support.redaction.v1".to_owned(),
        minted_at: "2026-07-08T00:00:00Z".to_owned(),
    })
}

fn baseline() -> M5GitHistoryComponentMatrixPacket {
    seed_packet()
}

/// Regenerates the checked-in artifacts and fixtures.
///
/// Guarded by `GEN_GIT_HISTORY_COMPONENT_ARTIFACTS` so it is inert in CI but can
/// deterministically rewrite the export, summary, and narrowed fixtures.
#[test]
fn generate_artifacts() {
    if std::env::var_os("GEN_GIT_HISTORY_COMPONENT_ARTIFACTS").is_none() {
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
        format!("{root}/{M5_GIT_HISTORY_COMPONENT_MATRIX_ARTIFACT_REF}"),
        format!("{}\n", canonical.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        format!("{root}/{M5_GIT_HISTORY_COMPONENT_MATRIX_SUMMARY_REF}"),
        canonical.render_markdown_summary(),
    )
    .expect("write summary");

    // Force-push dialog narrowed by an invalidated approval; still valid.
    let mut force_push = seed_packet();
    {
        let dialog = force_push
            .component_rows
            .iter_mut()
            .find(|r| r.component == M5GitHistoryComponent::ForcePushReviewDialog)
            .expect("force-push row present");
        dialog.maturity = ComponentMaturityPosture::Preview;
        if !dialog
            .downgrade_vocab
            .contains(&GitHistoryDowngradeState::ApprovalInvalidated)
        {
            dialog
                .downgrade_vocab
                .push(GitHistoryDowngradeState::ApprovalInvalidated);
        }
        dialog.approval_invalidation_rule =
            "Two approvals are invalidated by this ref rewrite and are listed before confirm."
                .to_owned();
    }
    force_push.packet_id =
        "m5-git-history-sequence-component-matrix:force-push-approval-invalidated:0001".to_owned();
    assert!(
        force_push.validate().is_empty(),
        "{:?}",
        force_push.validate()
    );
    std::fs::write(
        format!(
            "{root}/{M5_GIT_HISTORY_COMPONENT_MATRIX_FIXTURE_DIR}/force_push_approval_invalidated.json"
        ),
        format!("{}\n", force_push.export_safe_json()),
    )
    .expect("write force-push fixture");

    // Stash entry narrowed to a reflog-only recovery destination; still valid.
    let mut stash = seed_packet();
    {
        let entry = stash
            .component_rows
            .iter_mut()
            .find(|r| r.component == M5GitHistoryComponent::StashEntry)
            .expect("stash row present");
        if !entry
            .downgrade_vocab
            .contains(&GitHistoryDowngradeState::ReflogOnlyFallback)
        {
            entry
                .downgrade_vocab
                .push(GitHistoryDowngradeState::ReflogOnlyFallback);
        }
        entry.recovery_checkpoint_rule =
            "No checkpoint captured; pop discloses the reflog-only recovery destination first."
                .to_owned();
    }
    stash.packet_id = "m5-git-history-sequence-component-matrix:stash-reflog-only:0001".to_owned();
    assert!(stash.validate().is_empty(), "{:?}", stash.validate());
    std::fs::write(
        format!(
            "{root}/{M5_GIT_HISTORY_COMPONENT_MATRIX_FIXTURE_DIR}/stash_entry_reflog_only_recovery.json"
        ),
        format!("{}\n", stash.export_safe_json()),
    )
    .expect("write stash fixture");
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
    let packet = current_stable_m5_git_history_component_matrix_export()
        .expect("checked M5 git history component matrix export validates");
    assert_eq!(packet.packet_id, CANONICAL_PACKET_ID);
}

#[test]
fn checked_export_matches_seed() {
    let checked: M5GitHistoryComponentMatrixPacket =
        serde_json::from_str(CANONICAL_EXPORT).expect("canonical export deserializes");
    assert_eq!(checked, seed_packet());
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [FORCE_PUSH_FIXTURE, STASH_FIXTURE] {
        let packet: M5GitHistoryComponentMatrixPacket =
            serde_json::from_str(raw).expect("fixture parses as matrix packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn matrix_covers_every_frozen_component() {
    let packet = baseline();
    for required in M5GitHistoryComponent::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.component == required),
            "missing component {}",
            required.as_str()
        );
    }
}

#[test]
fn component_rows_bind_canonical_source_contracts() {
    let packet = baseline();
    for component in M5GitHistoryComponent::ALL {
        let row = packet
            .component_rows
            .iter()
            .find(|row| row.component == component)
            .expect("component row present");
        assert_eq!(
            row.canonical_source_contract_ref,
            component.canonical_source_contract_ref(),
            "component {} must bind its canonical source contract",
            component.as_str()
        );
    }
}

#[test]
fn every_risky_component_has_a_real_mutation_review() {
    let packet = baseline();
    for row in &packet.component_rows {
        if row.component.is_risky_mutation_surface() {
            assert!(
                row.mutation_review_class.is_risky_mutation(),
                "{} must carry a risky mutation-review class",
                row.component.as_str()
            );
            assert!(
                row.preserves_distinct_verb,
                "{} must keep its verb distinct",
                row.component.as_str()
            );
        }
    }
}

#[test]
fn matrix_covers_every_downgrade_state() {
    let packet = baseline();
    for state in GitHistoryDowngradeState::ALL {
        assert!(
            packet
                .downgrade_state_rows
                .iter()
                .any(|row| row.state == state),
            "missing downgrade state {}",
            state.as_str()
        );
    }
}

#[test]
fn missing_component_fails() {
    let mut packet = baseline();
    packet
        .component_rows
        .retain(|row| row.component != M5GitHistoryComponent::WorktreeRow);
    assert!(packet
        .validate()
        .contains(&M5GitHistoryComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn duplicate_component_fails() {
    let mut packet = baseline();
    let dup = packet.component_rows[0].clone();
    packet.component_rows.push(dup);
    assert!(packet
        .validate()
        .contains(&M5GitHistoryComponentMatrixViolation::DuplicateComponent));
}

#[test]
fn component_source_contract_mismatch_fails() {
    let mut packet = baseline();
    packet.component_rows[0].canonical_source_contract_ref =
        "schemas/git/wrong.schema.json".to_owned();
    assert!(packet
        .validate()
        .contains(&M5GitHistoryComponentMatrixViolation::ComponentSourceContractMismatch));
}

#[test]
fn missing_identity_preservation_fails() {
    let mut packet = baseline();
    packet.component_rows[0].identity_preservation = "   ".to_owned();
    assert!(packet
        .validate()
        .contains(&M5GitHistoryComponentMatrixViolation::IdentityPreservationMissing));
}

#[test]
fn missing_recovery_checkpoint_rule_fails() {
    let mut packet = baseline();
    packet.component_rows[0].recovery_checkpoint_rule = String::new();
    assert!(packet
        .validate()
        .contains(&M5GitHistoryComponentMatrixViolation::RecoveryCheckpointRuleMissing));
}

#[test]
fn missing_approval_invalidation_rule_fails() {
    let mut packet = baseline();
    packet.component_rows[0].approval_invalidation_rule = String::new();
    assert!(packet
        .validate()
        .contains(&M5GitHistoryComponentMatrixViolation::ApprovalInvalidationRuleMissing));
}

#[test]
fn missing_handoff_rule_fails() {
    let mut packet = baseline();
    packet.component_rows[0].browser_provider_handoff_rule = String::new();
    assert!(packet
        .validate()
        .contains(&M5GitHistoryComponentMatrixViolation::BrowserProviderHandoffRuleMissing));
}

#[test]
fn risky_component_without_mutation_review_fails() {
    let mut packet = baseline();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component == M5GitHistoryComponent::ForcePushReviewDialog)
        .expect("force-push row present");
    row.mutation_review_class = MutationReviewClass::DisplayOnlyNoMutation;
    assert!(packet
        .validate()
        .contains(&M5GitHistoryComponentMatrixViolation::RiskyComponentMissingMutationReview));
}

#[test]
fn risky_component_collapsing_verb_fails() {
    let mut packet = baseline();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component == M5GitHistoryComponent::StashEntry)
        .expect("stash row present");
    row.preserves_distinct_verb = false;
    assert!(packet
        .validate()
        .contains(&M5GitHistoryComponentMatrixViolation::RiskyComponentCollapsesVerbs));
}

#[test]
fn missing_downgrade_state_fails() {
    let mut packet = baseline();
    packet
        .downgrade_state_rows
        .retain(|row| row.state != GitHistoryDowngradeState::ReflogOnlyFallback);
    assert!(packet
        .validate()
        .contains(&M5GitHistoryComponentMatrixViolation::RequiredDowngradeStateMissing));
}

#[test]
fn downgrade_state_reduced_to_badge_fails() {
    let mut packet = baseline();
    packet.downgrade_state_rows[0].narrows_claim = false;
    assert!(packet
        .validate()
        .contains(&M5GitHistoryComponentMatrixViolation::DowngradeStateRowIncomplete));
}

#[test]
fn reflog_only_invisible_after_mutation_fails() {
    let mut packet = baseline();
    let row = packet
        .downgrade_state_rows
        .iter_mut()
        .find(|row| row.state == GitHistoryDowngradeState::ReflogOnlyFallback)
        .expect("reflog-only row present");
    row.must_stay_visible_after_mutation = false;
    assert!(packet
        .validate()
        .contains(&M5GitHistoryComponentMatrixViolation::DowngradeStateRowIncomplete));
}

#[test]
fn missing_source_contract_fails() {
    let mut packet = baseline();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5GitHistoryComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = baseline();
    packet
        .governance_review
        .no_verb_collapsed_into_ambiguous_confirm = false;
    assert!(packet
        .validate()
        .contains(&M5GitHistoryComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_parity_incomplete_fails() {
    let mut packet = baseline();
    packet.consumer_parity.support_export_expresses_family = false;
    assert!(packet
        .validate()
        .contains(&M5GitHistoryComponentMatrixViolation::ConsumerParityIncomplete));
}

#[test]
fn freeze_posture_unfrozen_fails() {
    let mut packet = baseline();
    packet.freeze_posture.frozen = false;
    assert!(packet
        .validate()
        .contains(&M5GitHistoryComponentMatrixViolation::FreezePostureIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = baseline();
    packet.record_kind = "something_else".to_owned();
    assert!(packet
        .validate()
        .contains(&M5GitHistoryComponentMatrixViolation::WrongRecordKind));
}

#[test]
fn raw_boundary_material_in_export_fails() {
    let mut packet = baseline();
    packet.matrix_label = "leak: bearer abc123".to_owned();
    assert!(packet
        .validate()
        .contains(&M5GitHistoryComponentMatrixViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_every_component_and_state() {
    let summary = baseline().render_markdown_summary();
    for component in M5GitHistoryComponent::ALL {
        assert!(
            summary.contains(component.as_str()),
            "summary missing component {}",
            component.as_str()
        );
    }
    for state in GitHistoryDowngradeState::ALL {
        assert!(
            summary.contains(state.as_str()),
            "summary missing downgrade state {}",
            state.as_str()
        );
    }
}

#[test]
fn force_push_fixture_narrows_maturity_to_preview() {
    let packet: M5GitHistoryComponentMatrixPacket =
        serde_json::from_str(FORCE_PUSH_FIXTURE).expect("force-push fixture parses");
    let row = packet
        .component_rows
        .iter()
        .find(|row| row.component == M5GitHistoryComponent::ForcePushReviewDialog)
        .expect("force-push row present");
    assert_eq!(row.maturity, ComponentMaturityPosture::Preview);
    assert!(row
        .downgrade_vocab
        .contains(&GitHistoryDowngradeState::ApprovalInvalidated));
}

#[test]
fn stash_fixture_falls_back_to_reflog_only_recovery() {
    let packet: M5GitHistoryComponentMatrixPacket =
        serde_json::from_str(STASH_FIXTURE).expect("stash fixture parses");
    let row = packet
        .component_rows
        .iter()
        .find(|row| row.component == M5GitHistoryComponent::StashEntry)
        .expect("stash row present");
    assert!(row
        .downgrade_vocab
        .contains(&GitHistoryDowngradeState::ReflogOnlyFallback));
    assert!(row.recovery_checkpoint_rule.contains("reflog-only"));
}
