use super::*;

const PACKET_ID: &str = "git-history-component-consumer:stable:0001";

fn trust_review() -> GitHistoryComponentConsumerTrustReview {
    GitHistoryComponentConsumerTrustReview {
        component_reuse_proven_by_fixtures: true,
        same_object_same_language_across_surfaces: true,
        exact_target_ref_and_worktree_never_hidden: true,
        conflict_and_recovery_state_survives_mutation: true,
        primary_verbs_identical_across_surfaces: true,
        ref_worktree_recovery_labels_identical_across_surfaces: true,
        local_only_recovery_stays_explicit_with_provider_state: true,
        recovery_destination_always_reachable_when_risky: true,
        no_git_verb_collapsed_into_ambiguous_confirm: true,
        help_support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> GitHistoryComponentConsumerProjection {
    GitHistoryComponentConsumerProjection {
        history_sidebar_reuses_shared_components: true,
        risky_mutation_sheet_reuses_shared_components: true,
        review_workspace_banner_reuses_shared_components: true,
        command_help_reuses_shared_components: true,
        support_bundle_reuses_shared_components: true,
        exported_recovery_packet_reuses_shared_components: true,
        every_component_adopted_by_two_or_more_consumers: true,
        parity_facets_identical_for_same_object: true,
        narrowing_disclosed_not_hidden: true,
        export_preserves_ref_worktree_recovery_identity: true,
    }
}

fn proof_freshness() -> GitHistoryComponentConsumerProofFreshness {
    GitHistoryComponentConsumerProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-07-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<GitHistoryComponentConsumerDowngradeTrigger> {
    vec![
        GitHistoryComponentConsumerDowngradeTrigger::ProofStale,
        GitHistoryComponentConsumerDowngradeTrigger::ProviderOverlayStale,
        GitHistoryComponentConsumerDowngradeTrigger::ApprovalInvalidationPending,
        GitHistoryComponentConsumerDowngradeTrigger::RecoveryCheckpointUnreachable,
        GitHistoryComponentConsumerDowngradeTrigger::ParityDriftDetected,
        GitHistoryComponentConsumerDowngradeTrigger::UpstreamComponentNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<GitHistoryComponentConsumer> {
    GitHistoryComponentConsumer::ALL.to_vec()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        GIT_HISTORY_COMPONENT_CONSUMER_SCHEMA_REF.to_owned(),
        GIT_HISTORY_COMPONENT_CONSUMER_DOC_REF.to_owned(),
        GIT_HISTORY_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        GIT_HISTORY_COMPONENT_CONSUMER_IDENTITY_CONTRACT_REF.to_owned(),
        GIT_HISTORY_COMPONENT_CONSUMER_STASH_RECOVERY_CONTRACT_REF.to_owned(),
        GIT_HISTORY_COMPONENT_CONSUMER_SEQUENCE_EDIT_CONTRACT_REF.to_owned(),
        GIT_HISTORY_COMPONENT_CONSUMER_MUTATION_REVIEW_CONTRACT_REF.to_owned(),
    ]
}

fn binding_refs(component: M5GitHistoryComponent) -> Vec<String> {
    vec![
        GIT_HISTORY_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        component_canonical_schema_ref(component).to_owned(),
    ]
}

fn facets(
    ref_identity: &str,
    worktree_scope: &str,
    recovery_destination: &str,
    primary_verb: &str,
) -> GitHistoryComponentParityFacetValues {
    GitHistoryComponentParityFacetValues {
        ref_identity_label: ref_identity.to_owned(),
        worktree_scope_label: worktree_scope.to_owned(),
        recovery_destination_label: recovery_destination.to_owned(),
        primary_verb: primary_verb.to_owned(),
    }
}

/// Builds one binding, deriving render mode, parity state, narrow banner, and
/// disclosure notes from the object's render condition so the fixture stays
/// self-consistent by construction.
#[allow(clippy::too_many_arguments)]
fn binding(
    binding_id: &str,
    object_id: &str,
    object_label: &str,
    component: M5GitHistoryComponent,
    consumer: GitHistoryComponentConsumer,
    condition: GitHistoryRenderCondition,
    facets: &GitHistoryComponentParityFacetValues,
) -> GitHistoryComponentConsumerBinding {
    let disclosure = resolve_git_history_component_render_disclosure(condition);

    let narrow_banner = disclosure.narrow_reason.map(|reason| {
        let (next_action, next_action_label) = match reason {
            GitHistoryNarrowReason::RefWorktreeIdentityNarrowed => (
                GitHistoryNarrowNextAction::ReconcileRefWorktreeIdentity,
                "Reconcile the exact ref and worktree to restore full parity".to_owned(),
            ),
            GitHistoryNarrowReason::RecoveryDestinationNarrowed => (
                GitHistoryNarrowNextAction::OpenRecoveryCheckpoint,
                "Open the recovery checkpoint to restore this state".to_owned(),
            ),
            GitHistoryNarrowReason::LocalContinueEngaged => (
                GitHistoryNarrowNextAction::ContinueLocalHistory,
                "Continue the history work locally while offline".to_owned(),
            ),
        };
        GitHistoryNarrowBanner {
            reason,
            preserved_facets_note:
                "Ref identity, worktree scope, recovery destination, and verb are preserved; only rendering narrowed"
                    .to_owned(),
            next_action,
            next_action_label,
        }
    });

    let ref_identity_note = if disclosure.needs_ref_identity_note {
        "Target ref is detached/missing here; the exact commit identity stays spelled out"
            .to_owned()
    } else {
        String::new()
    };
    let recovery_note = if disclosure.needs_recovery_note {
        "Recovery is narrowed but the destination stays named and reachable".to_owned()
    } else {
        String::new()
    };
    let local_continue_note = if disclosure.needs_local_continue_note {
        "Your local history work continues here while provider handoff is unavailable".to_owned()
    } else {
        String::new()
    };

    GitHistoryComponentConsumerBinding {
        binding_id: binding_id.to_owned(),
        history_object_id: object_id.to_owned(),
        history_object_label: object_label.to_owned(),
        component,
        consumer,
        condition,
        render_mode: disclosure.expected_mode,
        parity_facets: facets.clone(),
        parity_state: parity_state_for_mode(disclosure.expected_mode),
        narrow_banner,
        ref_identity_note,
        recovery_note,
        local_continue_note,
        collapses_git_verb_into_ambiguous_confirm: false,
        hides_exact_target_ref_or_worktree: false,
        drops_conflict_or_recovery_state_after_mutation: false,
        rewords_ref_worktree_recovery_labels_per_surface: false,
        hides_local_only_recovery_when_provider_linked: false,
        source_contract_refs: binding_refs(component),
    }
}

/// The canonical binding set: twelve components, each adopted by >= 2 consumers,
/// covering all six consumer surfaces and all eight render conditions. Objects
/// sharing an id share parity facets.
fn consumer_bindings() -> Vec<GitHistoryComponentConsumerBinding> {
    // Object 1: commit-graph header, aligned, on history sidebar + command help.
    let cgh = facets(
        "main @ 4f2a9c1",
        "Primary worktree · repo root",
        "Reflog checkpoint before last fetch",
        "View commit graph",
    );
    // Object 2: history-graph row, aligned, on history sidebar + exported recovery.
    let hgr = facets(
        "Commit 4f2a9c1 · Add retry backoff",
        "Primary worktree · repo root",
        "Reachable from main; reflog available",
        "Open commit",
    );
    // Object 3: branch-comparison chip, stale provider overlay, on sidebar + review banner.
    let bcc = facets(
        "feature/queue ahead 3 · behind 1 of main",
        "Primary worktree · repo root",
        "Base main is recoverable via reflog",
        "Compare branches",
    );
    // Object 4: worktree row, detached/missing ref, on history sidebar + support bundle.
    let wtr = facets(
        "detached HEAD @ 91ba0de",
        "Linked worktree ../hotfix · detached",
        "Reattach via reflog checkpoint",
        "Open worktree",
    );
    // Object 5: stash entry, dirty/conflicted worktree, on risky-mutation sheet + sidebar.
    let se = facets(
        "stash@{0} on feature/queue",
        "Primary worktree · dirty at 2 paths",
        "Stash content restorable via apply",
        "Apply stash",
    );
    // Object 6: reflog-recovery banner, reflog-only fallback, on sheet + support bundle.
    let rrb = facets(
        "HEAD@{2} before rebase",
        "Primary worktree · repo root",
        "Restore from reflog HEAD@{2} (no checkpoint)",
        "Restore from reflog",
    );
    // Object 7: rebase-todo row, shallow/partial topology, on sheet + review banner.
    let rtr = facets(
        "pick 4f2a9c1 · Add retry backoff",
        "Primary worktree · shallow depth 50",
        "Abort restores pre-rebase checkpoint",
        "Reorder rebase step",
    );
    // Object 8: sequence-editor header, aligned, on risky-mutation sheet + command help.
    let seh = facets(
        "Interactive rebase onto main (5 commits)",
        "Primary worktree · repo root",
        "Abort restores pre-rebase ORIG_HEAD",
        "Edit rebase sequence",
    );
    // Object 9: cherry-pick/revert review sheet, approval invalidated, on sheet + review banner.
    let cprs = facets(
        "cherry-pick 91ba0de onto feature/queue",
        "Primary worktree · repo root",
        "Rollback restores pre-cherry-pick checkpoint",
        "Cherry-pick commit",
    );
    // Object 10: patch-apply review sheet, dirty/conflicted worktree, on sheet + support bundle.
    let pars = facets(
        "Apply 0001-add-retry.patch to feature/queue",
        "Primary worktree · dirty at 1 path",
        "Rollback restores pre-apply checkpoint",
        "Apply patch",
    );
    // Object 11: conflict-checkpoint card, reflog-only fallback, on sheet + exported recovery.
    let ccc = facets(
        "Conflict at src/queue.rs (base/ours/theirs)",
        "Primary worktree · repo root",
        "Reopen checkpoint cp-771 to resume",
        "Reopen conflict checkpoint",
    );
    // Object 12: force-push review dialog, offline/local-only, on sheet + exported recovery.
    let fprd = facets(
        "force-push feature/queue → origin/feature/queue",
        "Primary worktree · repo root",
        "Recover prior tip via origin reflog",
        "Force-push with lease",
    );

    vec![
        binding(
            "bind:cgh-1:sidebar",
            "obj:cgh-1",
            "Commit graph: main",
            M5GitHistoryComponent::CommitGraphHeader,
            GitHistoryComponentConsumer::HistorySidebar,
            GitHistoryRenderCondition::AlignedLocalTruth,
            &cgh,
        ),
        binding(
            "bind:cgh-1:help",
            "obj:cgh-1",
            "Commit graph: main",
            M5GitHistoryComponent::CommitGraphHeader,
            GitHistoryComponentConsumer::CommandHelp,
            GitHistoryRenderCondition::AlignedLocalTruth,
            &cgh,
        ),
        binding(
            "bind:hgr-1:sidebar",
            "obj:hgr-1",
            "History row: 4f2a9c1",
            M5GitHistoryComponent::HistoryGraphRow,
            GitHistoryComponentConsumer::HistorySidebar,
            GitHistoryRenderCondition::AlignedLocalTruth,
            &hgr,
        ),
        binding(
            "bind:hgr-1:export",
            "obj:hgr-1",
            "History row: 4f2a9c1",
            M5GitHistoryComponent::HistoryGraphRow,
            GitHistoryComponentConsumer::ExportedRecoveryPacket,
            GitHistoryRenderCondition::AlignedLocalTruth,
            &hgr,
        ),
        binding(
            "bind:bcc-1:sidebar",
            "obj:bcc-1",
            "Compare: feature/queue vs main",
            M5GitHistoryComponent::BranchComparisonChip,
            GitHistoryComponentConsumer::HistorySidebar,
            GitHistoryRenderCondition::StaleProviderOverlay,
            &bcc,
        ),
        binding(
            "bind:bcc-1:review",
            "obj:bcc-1",
            "Compare: feature/queue vs main",
            M5GitHistoryComponent::BranchComparisonChip,
            GitHistoryComponentConsumer::ReviewWorkspaceBanner,
            GitHistoryRenderCondition::StaleProviderOverlay,
            &bcc,
        ),
        binding(
            "bind:wtr-1:sidebar",
            "obj:wtr-1",
            "Worktree: ../hotfix",
            M5GitHistoryComponent::WorktreeRow,
            GitHistoryComponentConsumer::HistorySidebar,
            GitHistoryRenderCondition::DetachedOrMissingRef,
            &wtr,
        ),
        binding(
            "bind:wtr-1:support",
            "obj:wtr-1",
            "Worktree: ../hotfix",
            M5GitHistoryComponent::WorktreeRow,
            GitHistoryComponentConsumer::SupportBundle,
            GitHistoryRenderCondition::DetachedOrMissingRef,
            &wtr,
        ),
        binding(
            "bind:se-1:sheet",
            "obj:se-1",
            "Stash: stash@{0}",
            M5GitHistoryComponent::StashEntry,
            GitHistoryComponentConsumer::RiskyMutationSheet,
            GitHistoryRenderCondition::DirtyOrConflictedWorktree,
            &se,
        ),
        binding(
            "bind:se-1:sidebar",
            "obj:se-1",
            "Stash: stash@{0}",
            M5GitHistoryComponent::StashEntry,
            GitHistoryComponentConsumer::HistorySidebar,
            GitHistoryRenderCondition::DirtyOrConflictedWorktree,
            &se,
        ),
        binding(
            "bind:rrb-1:sheet",
            "obj:rrb-1",
            "Reflog recovery: HEAD@{2}",
            M5GitHistoryComponent::ReflogRecoveryBanner,
            GitHistoryComponentConsumer::RiskyMutationSheet,
            GitHistoryRenderCondition::ReflogOnlyFallback,
            &rrb,
        ),
        binding(
            "bind:rrb-1:support",
            "obj:rrb-1",
            "Reflog recovery: HEAD@{2}",
            M5GitHistoryComponent::ReflogRecoveryBanner,
            GitHistoryComponentConsumer::SupportBundle,
            GitHistoryRenderCondition::ReflogOnlyFallback,
            &rrb,
        ),
        binding(
            "bind:rtr-1:sheet",
            "obj:rtr-1",
            "Rebase step: pick 4f2a9c1",
            M5GitHistoryComponent::RebaseTodoRow,
            GitHistoryComponentConsumer::RiskyMutationSheet,
            GitHistoryRenderCondition::ShallowOrPartialTopology,
            &rtr,
        ),
        binding(
            "bind:rtr-1:review",
            "obj:rtr-1",
            "Rebase step: pick 4f2a9c1",
            M5GitHistoryComponent::RebaseTodoRow,
            GitHistoryComponentConsumer::ReviewWorkspaceBanner,
            GitHistoryRenderCondition::ShallowOrPartialTopology,
            &rtr,
        ),
        binding(
            "bind:seh-1:sheet",
            "obj:seh-1",
            "Rebase sequence onto main",
            M5GitHistoryComponent::SequenceEditorHeader,
            GitHistoryComponentConsumer::RiskyMutationSheet,
            GitHistoryRenderCondition::AlignedLocalTruth,
            &seh,
        ),
        binding(
            "bind:seh-1:help",
            "obj:seh-1",
            "Rebase sequence onto main",
            M5GitHistoryComponent::SequenceEditorHeader,
            GitHistoryComponentConsumer::CommandHelp,
            GitHistoryRenderCondition::AlignedLocalTruth,
            &seh,
        ),
        binding(
            "bind:cprs-1:sheet",
            "obj:cprs-1",
            "Cherry-pick 91ba0de",
            M5GitHistoryComponent::CherryPickRevertReviewSheet,
            GitHistoryComponentConsumer::RiskyMutationSheet,
            GitHistoryRenderCondition::ApprovalInvalidated,
            &cprs,
        ),
        binding(
            "bind:cprs-1:review",
            "obj:cprs-1",
            "Cherry-pick 91ba0de",
            M5GitHistoryComponent::CherryPickRevertReviewSheet,
            GitHistoryComponentConsumer::ReviewWorkspaceBanner,
            GitHistoryRenderCondition::ApprovalInvalidated,
            &cprs,
        ),
        binding(
            "bind:pars-1:sheet",
            "obj:pars-1",
            "Apply 0001-add-retry.patch",
            M5GitHistoryComponent::PatchApplyReviewSheet,
            GitHistoryComponentConsumer::RiskyMutationSheet,
            GitHistoryRenderCondition::DirtyOrConflictedWorktree,
            &pars,
        ),
        binding(
            "bind:pars-1:support",
            "obj:pars-1",
            "Apply 0001-add-retry.patch",
            M5GitHistoryComponent::PatchApplyReviewSheet,
            GitHistoryComponentConsumer::SupportBundle,
            GitHistoryRenderCondition::DirtyOrConflictedWorktree,
            &pars,
        ),
        binding(
            "bind:ccc-1:sheet",
            "obj:ccc-1",
            "Conflict checkpoint cp-771",
            M5GitHistoryComponent::ConflictCheckpointCard,
            GitHistoryComponentConsumer::RiskyMutationSheet,
            GitHistoryRenderCondition::ReflogOnlyFallback,
            &ccc,
        ),
        binding(
            "bind:ccc-1:export",
            "obj:ccc-1",
            "Conflict checkpoint cp-771",
            M5GitHistoryComponent::ConflictCheckpointCard,
            GitHistoryComponentConsumer::ExportedRecoveryPacket,
            GitHistoryRenderCondition::ReflogOnlyFallback,
            &ccc,
        ),
        binding(
            "bind:fprd-1:sheet",
            "obj:fprd-1",
            "Force-push feature/queue",
            M5GitHistoryComponent::ForcePushReviewDialog,
            GitHistoryComponentConsumer::RiskyMutationSheet,
            GitHistoryRenderCondition::OfflineLocalOnly,
            &fprd,
        ),
        binding(
            "bind:fprd-1:export",
            "obj:fprd-1",
            "Force-push feature/queue",
            M5GitHistoryComponent::ForcePushReviewDialog,
            GitHistoryComponentConsumer::ExportedRecoveryPacket,
            GitHistoryRenderCondition::OfflineLocalOnly,
            &fprd,
        ),
    ]
}

fn packet_with(
    bindings: Vec<GitHistoryComponentConsumerBinding>,
) -> GitHistoryComponentConsumerPacket {
    GitHistoryComponentConsumerPacket::new(GitHistoryComponentConsumerPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Shared Git-history component consumers".to_owned(),
        consumer_bindings: bindings,
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-08T00:00:00Z".to_owned(),
    })
}

fn packet() -> GitHistoryComponentConsumerPacket {
    packet_with(consumer_bindings())
}

#[test]
fn consumer_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn render_disclosure_maps_condition_to_mode() {
    let aligned = resolve_git_history_component_render_disclosure(
        GitHistoryRenderCondition::AlignedLocalTruth,
    );
    assert_eq!(aligned.expected_mode, GitHistoryRenderMode::FullParity);
    assert!(!aligned.needs_narrow_banner);
    assert!(!aligned.needs_ref_identity_note);
    assert!(!aligned.needs_recovery_note);
    assert!(!aligned.needs_local_continue_note);

    let detached = resolve_git_history_component_render_disclosure(
        GitHistoryRenderCondition::DetachedOrMissingRef,
    );
    assert_eq!(
        detached.expected_mode,
        GitHistoryRenderMode::IdentityNarrowed
    );
    assert!(detached.needs_narrow_banner);
    assert!(detached.needs_ref_identity_note);
    assert!(!detached.needs_recovery_note);

    let stale = resolve_git_history_component_render_disclosure(
        GitHistoryRenderCondition::StaleProviderOverlay,
    );
    assert_eq!(stale.expected_mode, GitHistoryRenderMode::IdentityNarrowed);
    assert!(stale.needs_narrow_banner);
    // Only a detached/missing ref forces the exact-ref note.
    assert!(!stale.needs_ref_identity_note);

    let reflog = resolve_git_history_component_render_disclosure(
        GitHistoryRenderCondition::ReflogOnlyFallback,
    );
    assert_eq!(reflog.expected_mode, GitHistoryRenderMode::RecoveryNarrowed);
    assert!(reflog.needs_recovery_note);
    assert!(!reflog.needs_local_continue_note);

    let approval = resolve_git_history_component_render_disclosure(
        GitHistoryRenderCondition::ApprovalInvalidated,
    );
    assert_eq!(
        approval.expected_mode,
        GitHistoryRenderMode::RecoveryNarrowed
    );
    assert!(approval.needs_recovery_note);

    let offline = resolve_git_history_component_render_disclosure(
        GitHistoryRenderCondition::OfflineLocalOnly,
    );
    assert_eq!(
        offline.expected_mode,
        GitHistoryRenderMode::LocalContinueFallback
    );
    assert!(offline.needs_local_continue_note);
    assert!(!offline.needs_recovery_note);
}

#[test]
fn every_condition_binds_frozen_downgrade_vocabulary() {
    for condition in GitHistoryRenderCondition::ALL {
        match condition {
            GitHistoryRenderCondition::AlignedLocalTruth => {
                assert!(condition.downgrade_state().is_none());
            }
            other => assert!(
                other.downgrade_state().is_some(),
                "{other:?} must bind a frozen downgrade state"
            ),
        }
    }
}

#[test]
fn parity_drift_across_surfaces_fails() {
    let mut packet = packet();
    // Reword the ref identity on one surface for a shared object; the other disagrees.
    packet.consumer_bindings[1].parity_facets.ref_identity_label =
        "Reworded ref identity for help".to_owned();
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::ParityDriftAcrossSurfaces));
}

#[test]
fn primary_verb_drift_across_surfaces_fails() {
    let mut packet = packet();
    packet.consumer_bindings[3].parity_facets.primary_verb = "Different verb".to_owned();
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::ParityDriftAcrossSurfaces));
}

#[test]
fn recovery_destination_drift_across_surfaces_fails() {
    let mut packet = packet();
    packet.consumer_bindings[1]
        .parity_facets
        .recovery_destination_label = "Different recovery destination".to_owned();
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::ParityDriftAcrossSurfaces));
}

#[test]
fn component_reuse_by_single_consumer_fails() {
    let mut bindings = consumer_bindings();
    // Drop the second commit-graph-header binding so it is adopted by one consumer.
    bindings.retain(|b| b.binding_id != "bind:cgh-1:help");
    let packet = packet_with(bindings);
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::GitHistoryComponentReuseUnproven));
}

#[test]
fn missing_component_coverage_fails() {
    let mut bindings = consumer_bindings();
    bindings.retain(|b| b.component != M5GitHistoryComponent::ForcePushReviewDialog);
    let packet = packet_with(bindings);
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::ComponentCoverageMissing));
}

#[test]
fn missing_consumer_coverage_fails() {
    let mut bindings = consumer_bindings();
    // Remove the only command-help bindings.
    bindings.retain(|b| b.consumer != GitHistoryComponentConsumer::CommandHelp);
    let packet = packet_with(bindings);
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::ConsumerCoverageMissing));
}

#[test]
fn help_support_export_without_canonical_refs_fails() {
    let mut packet = packet();
    // An exported-recovery binding drops its canonical component ref.
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.consumer == GitHistoryComponentConsumer::ExportedRecoveryPacket)
        .expect("exported-recovery binding present");
    packet.consumer_bindings[index].source_contract_refs =
        vec![GIT_HISTORY_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF.to_owned()];
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::HelpSupportExportReferenceMissing));
}

#[test]
fn render_mode_mismatch_fails() {
    let mut packet = packet();
    // Claim full parity on a stale-overlay branch-comparison chip.
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.condition == GitHistoryRenderCondition::StaleProviderOverlay)
        .expect("stale binding present");
    packet.consumer_bindings[index].render_mode = GitHistoryRenderMode::FullParity;
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::RenderModeMismatch));
}

#[test]
fn parity_state_mismatch_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].parity_state =
        GitHistoryComponentParityState::FacetsDisclosedNarrowed;
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::ParityStateMismatch));
}

#[test]
fn narrowed_binding_without_banner_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.is_narrowed())
        .expect("narrowed binding present");
    packet.consumer_bindings[index].narrow_banner = None;
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::NarrowBannerMissing));
}

#[test]
fn full_parity_binding_with_banner_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].narrow_banner = Some(GitHistoryNarrowBanner {
        reason: GitHistoryNarrowReason::RefWorktreeIdentityNarrowed,
        preserved_facets_note: "note".to_owned(),
        next_action: GitHistoryNarrowNextAction::ReconcileRefWorktreeIdentity,
        next_action_label: "Reconcile".to_owned(),
    });
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::NarrowBannerMissing));
}

#[test]
fn narrow_reason_mismatch_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.condition == GitHistoryRenderCondition::ReflogOnlyFallback)
        .expect("reflog binding present");
    if let Some(banner) = packet.consumer_bindings[index].narrow_banner.as_mut() {
        banner.reason = GitHistoryNarrowReason::LocalContinueEngaged;
    }
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::NarrowReasonMismatch));
}

#[test]
fn narrow_banner_missing_preserved_facets_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.is_narrowed())
        .expect("narrowed binding present");
    if let Some(banner) = packet.consumer_bindings[index].narrow_banner.as_mut() {
        banner.preserved_facets_note = String::new();
    }
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::NarrowBannerPreservedFacetsMissing));
}

#[test]
fn ref_identity_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.condition == GitHistoryRenderCondition::DetachedOrMissingRef)
        .expect("detached binding present");
    packet.consumer_bindings[index].ref_identity_note = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::RefIdentityNoteMissing));
}

#[test]
fn recovery_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.render_mode == GitHistoryRenderMode::RecoveryNarrowed)
        .expect("recovery-narrowed binding present");
    packet.consumer_bindings[index].recovery_note = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::RecoveryNoteMissing));
}

#[test]
fn local_continue_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.condition == GitHistoryRenderCondition::OfflineLocalOnly)
        .expect("offline binding present");
    packet.consumer_bindings[index].local_continue_note = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::LocalContinueNoteMissing));
}

#[test]
fn git_verb_collapsed_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].collapses_git_verb_into_ambiguous_confirm = true;
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::GitVerbCollapsedIntoAmbiguousConfirm));
}

#[test]
fn exact_target_ref_hidden_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].hides_exact_target_ref_or_worktree = true;
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::ExactTargetRefOrWorktreeHidden));
}

#[test]
fn conflict_or_recovery_state_dropped_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].drops_conflict_or_recovery_state_after_mutation = true;
    assert!(packet.validate().contains(
        &GitHistoryComponentConsumerViolation::ConflictOrRecoveryStateDroppedAfterMutation
    ));
}

#[test]
fn labels_reworded_per_surface_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].rewords_ref_worktree_recovery_labels_per_surface = true;
    assert!(packet.validate().contains(
        &GitHistoryComponentConsumerViolation::RefWorktreeRecoveryLabelsRewordedPerSurface
    ));
}

#[test]
fn local_only_recovery_hidden_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].hides_local_only_recovery_when_provider_linked = true;
    assert!(packet.validate().contains(
        &GitHistoryComponentConsumerViolation::LocalOnlyRecoveryHiddenWhenProviderLinked
    ));
}

#[test]
fn parity_facet_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].parity_facets.primary_verb = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::ParityFacetIncomplete));
}

#[test]
fn incomplete_binding_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].history_object_label = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::BindingIncomplete));
}

#[test]
fn missing_bindings_fails() {
    let mut packet = packet();
    packet.consumer_bindings.clear();
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::ConsumerBindingsMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .trust_review
        .same_object_same_language_across_surfaces = false;
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .every_component_adopted_by_two_or_more_consumers = false;
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_bindings() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Consumer bindings"));
    assert!(summary.contains("commit_graph_header"));
    assert!(summary.contains("force_push_review_dialog"));
    assert!(summary.contains("local_continue_fallback"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_git_history_component_consumer_export()
        .expect("checked git-history component consumer export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-git-history-component-consumers/detached_ref_and_dirty_worktree_identity_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-git-history-component-consumers/reflog_only_and_offline_recovery_narrowed.json"
        )),
    ] {
        let packet: GitHistoryComponentConsumerPacket = serde_json::from_str(raw)
            .expect("fixture parses as git-history component consumer packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

// --- Fixture builders ---------------------------------------------------------

/// Re-derives the canonical bindings after overriding some objects' render
/// condition, keeping the parity facets identical per object so the packet still
/// validates.
fn bindings_with_condition_overrides(
    overrides: &[(&str, GitHistoryRenderCondition)],
) -> Vec<GitHistoryComponentConsumerBinding> {
    consumer_bindings()
        .into_iter()
        .map(|existing| {
            if let Some((_, condition)) = overrides
                .iter()
                .find(|(object_id, _)| *object_id == existing.history_object_id)
            {
                binding(
                    &existing.binding_id,
                    &existing.history_object_id,
                    &existing.history_object_label,
                    existing.component,
                    existing.consumer,
                    *condition,
                    &existing.parity_facets,
                )
            } else {
                existing
            }
        })
        .collect()
}

fn fixture_detached_ref_and_dirty_worktree_identity_narrowed() -> GitHistoryComponentConsumerPacket
{
    let bindings = bindings_with_condition_overrides(&[
        ("obj:cgh-1", GitHistoryRenderCondition::DetachedOrMissingRef),
        (
            "obj:hgr-1",
            GitHistoryRenderCondition::DirtyOrConflictedWorktree,
        ),
    ]);
    GitHistoryComponentConsumerPacket::new(GitHistoryComponentConsumerPacketInput {
        packet_id: "git-history-component-consumer:fixture:detached-ref-dirty-worktree-narrowed"
            .to_owned(),
        surface_label:
            "Shared Git-history component consumers: detached ref and dirty worktree, identity narrowed"
                .to_owned(),
        consumer_bindings: bindings,
        downgrade_triggers: vec![
            GitHistoryComponentConsumerDowngradeTrigger::ProviderOverlayStale,
            GitHistoryComponentConsumerDowngradeTrigger::UpstreamComponentNarrowed,
        ],
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-08T00:00:00Z".to_owned(),
    })
}

fn fixture_reflog_only_and_offline_recovery_narrowed() -> GitHistoryComponentConsumerPacket {
    let bindings = bindings_with_condition_overrides(&[
        ("obj:bcc-1", GitHistoryRenderCondition::ReflogOnlyFallback),
        ("obj:seh-1", GitHistoryRenderCondition::OfflineLocalOnly),
    ]);
    GitHistoryComponentConsumerPacket::new(GitHistoryComponentConsumerPacketInput {
        packet_id: "git-history-component-consumer:fixture:reflog-only-offline-recovery-narrowed"
            .to_owned(),
        surface_label:
            "Shared Git-history component consumers: reflog-only and offline, recovery narrowed"
                .to_owned(),
        consumer_bindings: bindings,
        downgrade_triggers: vec![
            GitHistoryComponentConsumerDowngradeTrigger::RecoveryCheckpointUnreachable,
            GitHistoryComponentConsumerDowngradeTrigger::LocalContinueUnavailable,
        ],
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-08T00:00:00Z".to_owned(),
    })
}

/// Regenerates the checked-in support export, summary, and fixtures.
///
/// Gated behind `GEN_GIT_HISTORY_COMPONENT_CONSUMER_ARTIFACTS` so it never writes
/// during a normal test run. Run with the env var set to refresh the artifacts after
/// a contract change, then review the diff.
#[test]
fn regenerate_git_history_component_consumer_artifacts() {
    if std::env::var_os("GEN_GIT_HISTORY_COMPONENT_CONSUMER_ARTIFACTS").is_none() {
        return;
    }

    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

    let canonical = packet();
    assert!(
        canonical.validate().is_empty(),
        "{:?}",
        canonical.validate()
    );
    std::fs::write(
        format!("{root}/{GIT_HISTORY_COMPONENT_CONSUMER_ARTIFACT_REF}"),
        format!("{}\n", canonical.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        format!("{root}/{GIT_HISTORY_COMPONENT_CONSUMER_SUMMARY_REF}"),
        canonical.render_markdown_summary(),
    )
    .expect("write summary");

    for (name, fixture) in [
        (
            "detached_ref_and_dirty_worktree_identity_narrowed.json",
            fixture_detached_ref_and_dirty_worktree_identity_narrowed(),
        ),
        (
            "reflog_only_and_offline_recovery_narrowed.json",
            fixture_reflog_only_and_offline_recovery_narrowed(),
        ),
    ] {
        assert!(
            fixture.validate().is_empty(),
            "{name}: {:?}",
            fixture.validate()
        );
        std::fs::write(
            format!("{root}/{GIT_HISTORY_COMPONENT_CONSUMER_FIXTURE_DIR}/{name}"),
            format!("{}\n", fixture.export_safe_json()),
        )
        .expect("write fixture");
    }
}
