//! Canonical seed builders for the frozen M5 change-orchestration matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code matrix, the artifact, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical change-orchestration matrix.
pub const M5_CHANGE_ORCHESTRATION_MATRIX_PACKET_ID: &str = "m5-change-orchestration:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-17T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn mandatory_labels() -> Vec<M5ChangeOrchestrationRequiredLabel> {
    M5ChangeOrchestrationRequiredLabel::MANDATORY.to_vec()
}

fn labels_with(
    extra: &[M5ChangeOrchestrationRequiredLabel],
) -> Vec<M5ChangeOrchestrationRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

#[allow(clippy::too_many_arguments)]
fn base_row(
    object_class: M5ChangeOrchestrationObject,
    qualification: M5ChangeOrchestrationQualificationClass,
    owner_role: &str,
    backup_owner_role: &str,
    scope_summary: &str,
    closure_ref: &str,
    source_refs: &[&str],
    required_visible_state: M5ChangeOrchestrationVisibleState,
) -> M5ChangeOrchestrationRow {
    M5ChangeOrchestrationRow {
        object_class,
        qualification,
        landing_state: M5ChangeOrchestrationState::SelectedChange,
        owner_role: owner_role.to_owned(),
        backup_owner_role: backup_owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        required_visible_state,
        surface_families: M5ChangeOrchestrationSurfaceFamily::ALL.to_vec(),
        classification_stages: M5ChangeOrchestrationClassificationStage::ALL.to_vec(),
        required_labels: mandatory_labels(),
        semantic_roles: vec![],
        change_object_roles: vec![],
        patch_stack_queue_roles: vec![],
        stack_edit_review_roles: vec![],
        landing_candidate_roles: vec![],
        portable_shelf_roles: vec![],
        worktree_cleanup_roles: vec![],
        degraded_reasons: M5ChangeOrchestrationDegradedReason::ALL.to_vec(),
        accessibility_routes: M5ChangeOrchestrationAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ChangeOrchestrationConsumerSurface::ChangeObjectDetail,
            M5ChangeOrchestrationConsumerSurface::SupportExportPacket,
        ],
        downgrade_triggers: vec![M5ChangeOrchestrationDowngradeTrigger::ChangeOrchestrationMatrixStale],
        required_closure_artifact_refs: strings(&[closure_ref]),
        source_contract_refs: strings(source_refs),
        infers_stack_membership_from_branch_names_alone: false,
        mutates_files_in_another_worktree_without_an_explicit_selected_change_object_and_worktree_binding:
            false,
        silently_reorders_collapses_or_retargets_stack_members: false,
        lands_from_ambient_branch_state_without_a_reviewed_landing_candidate: false,
        deletes_orphaned_worktrees_or_stale_stack_members_without_previewing_running_work_and_export_safe_evidence:
            false,
    }
}

fn txn(f: [&str; 7]) -> M5ChangeOrchestrationVisibleState {
    M5ChangeOrchestrationVisibleState {
        surface_label: f[0].to_owned(),
        selected_change_object: f[1].to_owned(),
        worktree_base_identity: f[2].to_owned(),
        stack_membership_and_order: f[3].to_owned(),
        landing_state_summary: f[4].to_owned(),
        cleanup_safety: f[5].to_owned(),
        validation_evidence: f[6].to_owned(),
    }
}

fn change_orchestration_rows() -> Vec<M5ChangeOrchestrationRow> {
    use M5ChangeOrchestrationConsumerSurface as C;
    use M5ChangeOrchestrationDowngradeTrigger as D;
    use M5ChangeOrchestrationObject as O;
    use M5ChangeOrchestrationQualificationClass as Q;
    use M5ChangeOrchestrationRequiredLabel as L;
    use M5ChangeOrchestrationRole as R;
    use M5ChangeOrchestrationState as S;

    let mut rows = Vec::new();

    // 1. ChangeObject.
    let mut row = base_row(
        O::ChangeObject,
        Q::Stable,
        "Change-object owner",
        "Git-orchestration backup owner",
        "One explicit change object binds a non-trivial multi-file change to its selected worktree / base identity, names whether it is a working-set patch or a side-branch work unit, shows its validation freshness, and never lets a command, AI tool, refactor, formatter, or provider action mutate files in another worktree without an explicit selected change object and worktree binding",
        "evidence:m5-change-object-closure:001",
        &[
            M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_REF,
            M5_CHANGE_OBJECT_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "change object",
            "explicit change object selected with its worktree and base identity bound",
            "selected worktree and base commit named, no ambient branch is treated as the change object",
            "working-set patch or side-branch work unit scope named with its stack membership",
            "selected change: the change object is chosen and its worktree binding is live",
            "clear to land once validation is fresh and no blocker remains",
            "validation freshness captured for the change object's current contents",
        ]),
    );
    row.change_object_roles = M5ChangeObjectRole::ALL.to_vec();
    row.semantic_roles = vec![
        R::SelectedChangeObjectDisclosure,
        R::WorktreeBindingDisclosure,
        R::StackMembershipDisclosure,
    ];
    row.required_labels = labels_with(&[L::LandingState]);
    row.consumer_surfaces = vec![
        C::ChangeObjectDetail,
        C::PatchStackQueue,
        C::StackEditReviewSheet,
        C::SupportExportPacket,
    ];
    row.landing_state = S::SelectedChange;
    row.downgrade_triggers = vec![
        D::StackMembershipInferredFromBranchNameAlone,
        D::CrossWorktreeWriteWithoutSelectedChangeObject,
        D::SelectedChangeObjectUnstated,
        D::ChangeOrchestrationMatrixStale,
    ];
    rows.push(row);

    // 2. PatchStackQueue.
    let mut row = base_row(
        O::PatchStackQueue,
        Q::Stable,
        "Patch-stack-queue owner",
        "Git-orchestration backup owner",
        "One patch stack / queue shows its member order, shows queue eligibility and any queue-blocked reason, shows the stack dependency edges, and never silently reorders, collapses, or retargets stack members or infers stack membership from branch names alone",
        "evidence:m5-patch-stack-queue-closure:001",
        &[
            M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_REF,
            M5_PATCH_STACK_QUEUE_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "patch stack queue",
            "the change objects that make up the stack, in explicit order",
            "each member's worktree / base identity, never inferred from a branch name",
            "ordered stack membership with its dependency edges declared, not guessed",
            "queue eligible once each member passes its gate",
            "clear to land per member; a queue-blocked member is named, not hidden",
            "per-member validation freshness rolled up for the queue",
        ]),
    );
    row.patch_stack_queue_roles = M5ChangeOrchestrationPatchStackQueueRole::ALL.to_vec();
    row.semantic_roles = vec![R::LandingStateDisclosure];
    row.required_labels = labels_with(&[L::StackMembershipSource]);
    row.consumer_surfaces = vec![
        C::PatchStackQueue,
        C::ChangeObjectDetail,
        C::SupportExportPacket,
    ];
    row.landing_state = S::QueueEligible;
    row.downgrade_triggers = vec![
        D::StackMembersSilentlyReordered,
        D::StackOrderUnstated,
        D::StackMembershipSourceUnstated,
        D::ChangeOrchestrationMatrixStale,
    ];
    rows.push(row);

    // 3. StackEditReviewSheet.
    let mut row = base_row(
        O::StackEditReviewSheet,
        Q::Stable,
        "Stack-edit-review-sheet owner",
        "Review-governance backup owner",
        "One stack-edit / review sheet shows each member's stack membership source, flags a restack-required stack, flags stale-or-broken membership, labels an inferred-from-branch-name membership as inferred, and never flattens declared-in-change-object, declared-locally, inferred-from-branch-name, and stale-or-broken membership into one badge",
        "evidence:m5-stack-edit-review-sheet-closure:001",
        &[
            M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_REF,
            M5_STACK_EDIT_REVIEW_SHEET_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "stack edit review sheet",
            "the change object under edit with its selected worktree / base",
            "each member's worktree binding shown while the stack is edited",
            "membership source per member: declared-in-change-object, declared-locally, inferred, or stale/broken",
            "restack required: the stack drifted and members must be restacked before review",
            "clear to land is withheld until the restack is applied",
            "validation freshness re-checked after a restack, never assumed",
        ]),
    );
    row.stack_edit_review_roles = M5ChangeOrchestrationStackEditReviewRole::ALL.to_vec();
    row.semantic_roles = vec![R::StackMembershipDisclosure];
    row.required_labels = labels_with(&[L::StackMembershipSource]);
    row.consumer_surfaces = vec![
        C::StackEditReviewSheet,
        C::ChangeObjectDetail,
        C::ReviewDetail,
        C::SupportExportPacket,
    ];
    row.landing_state = S::RestackRequired;
    row.downgrade_triggers = vec![
        D::StackMembersSilentlyReordered,
        D::StackMembershipSourceUnstated,
        D::StackMembershipInferredFromBranchNameAlone,
        D::ChangeOrchestrationMatrixStale,
    ];
    rows.push(row);

    // 4. LandingCandidateSheet.
    let mut row = base_row(
        O::LandingCandidateSheet,
        Q::Stable,
        "Landing-candidate-sheet owner",
        "Review-governance backup owner",
        "One landing-candidate sheet shows its validation freshness, shows the protected-branch gate, labels ambient branch state as not a reviewed landing candidate, names the landing target, and never lands from ambient branch state without a reviewed landing candidate",
        "evidence:m5-landing-candidate-sheet-closure:001",
        &[
            M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_REF,
            M5_LANDING_CANDIDATE_SHEET_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "landing candidate sheet",
            "the reviewed change object proposed for landing, not ambient branch state",
            "the landing target and its base, with the selected worktree named",
            "the stack members that must land together, in order",
            "protected-branch blocked: the target is protected and blocks a direct land",
            "clear to land only when validation is fresh and no gate is failing",
            "the validation freshness and rollback / export fallback backing the candidate",
        ]),
    );
    row.landing_candidate_roles = M5ChangeOrchestrationLandingCandidateRole::ALL.to_vec();
    row.semantic_roles = vec![
        R::ValidationFreshnessDisclosure,
        R::RollbackExportFallbackDisclosure,
    ];
    row.required_labels = labels_with(&[L::LandingState]);
    row.consumer_surfaces = vec![
        C::ProviderMergeQueue,
        C::ReviewDetail,
        C::SupportExportPacket,
    ];
    row.landing_state = S::ProtectedBranchBlocked;
    row.downgrade_triggers = vec![
        D::LandedFromAmbientBranchState,
        D::ValidationFreshnessUnstated,
        D::LandingStateUnstated,
        D::ChangeOrchestrationMatrixStale,
    ];
    rows.push(row);

    // 5. PortableShelf.
    let mut row = base_row(
        O::PortableShelf,
        Q::Stable,
        "Portable-shelf owner",
        "Git-orchestration backup owner",
        "One portable shelf / bundle shows its export bundle contents, shows its import / reopen lineage, names the shelf state (exported, imported, reopened), shows the recovery checkpoint, and never drops shelf contents on an export failure",
        "evidence:m5-portable-shelf-closure:001",
        &[
            M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_REF,
            M5_PORTABLE_SHELF_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "portable shelf",
            "the change object bundled into a portable shelf for handoff",
            "the worktree / base the shelf was captured from and reopens against",
            "the stack membership captured in the bundle so it reopens in order",
            "exported: the change object was bundled to a portable shelf",
            "clear to land is re-evaluated after import / reopen, never carried over blindly",
            "the recovery checkpoint and validation freshness retained in the bundle",
        ]),
    );
    row.portable_shelf_roles = M5ChangeOrchestrationPortableShelfRole::ALL.to_vec();
    row.semantic_roles = vec![R::RollbackExportFallbackDisclosure];
    row.required_labels = labels_with(&[L::CleanupSafety]);
    row.consumer_surfaces = vec![
        C::PortableShelf,
        C::ChangeObjectDetail,
        C::SupportExportPacket,
    ];
    row.landing_state = S::Exported;
    row.downgrade_triggers = vec![
        D::LandingStateUnstated,
        D::ValidationFreshnessUnstated,
        D::SelectedChangeObjectUnstated,
        D::ChangeOrchestrationMatrixStale,
    ];
    rows.push(row);

    // 6. WorktreeCleanupPreview.
    let mut row = base_row(
        O::WorktreeCleanupPreview,
        Q::Stable,
        "Worktree-cleanup-preview owner",
        "Incident-governance backup owner",
        "One worktree cleanup preview names the cleanup target, previews running tasks and open editors, previews uncommitted changes and recovery checkpoints, shows the cleanup state (orphaned, abandoned), and never deletes orphaned worktrees or stale stack members without previewing running work and export-safe evidence",
        "evidence:m5-worktree-cleanup-preview-closure:001",
        &[
            M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_REF,
            M5_WORKTREE_MANAGER_ROW_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "worktree cleanup preview",
            "the orphaned or abandoned worktree / stack member proposed for cleanup",
            "the worktree path and base the cleanup would remove, previewed before deletion",
            "the stale stack members that would be removed, with their membership source",
            "orphaned: the worktree no longer maps to a live change object",
            "running tasks, open editors, uncommitted changes, and checkpoints previewed before delete",
            "export-safe evidence and recovery checkpoint retained before any deletion",
        ]),
    );
    row.worktree_cleanup_roles = M5ChangeOrchestrationWorktreeCleanupRole::ALL.to_vec();
    row.semantic_roles = vec![R::CleanupSafetyDisclosure];
    row.required_labels = labels_with(&[L::CleanupSafety]);
    row.consumer_surfaces = vec![
        C::WorktreeCleanupPreview,
        C::ChangeObjectDetail,
        C::HelpDocs,
        C::SupportExportPacket,
    ];
    row.landing_state = S::Orphaned;
    row.downgrade_triggers = vec![
        D::OrphanDeletedWithoutSafetyPreview,
        D::WorktreeBindingUnstated,
        D::CrossWorktreeWriteWithoutSelectedChangeObject,
        D::ChangeOrchestrationMatrixStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5ChangeOrchestrationGovernanceReview {
    M5ChangeOrchestrationGovernanceReview {
        no_local_shelf_or_ambient_branch_reads_as_a_reviewed_landing_candidate: true,
        every_covered_object_class_names_owner_backup_owner_and_first_consumer: true,
        queue_eligible_state_is_mechanically_distinct_from_selected_change: true,
        every_change_orchestration_names_its_selected_change_object: true,
        every_patch_stack_queue_discloses_each_side_effect_separately: true,
        every_linked_change_names_its_stack_membership_source: true,
        no_cross_worktree_write_without_a_selected_change_object_and_binding: true,
        every_landing_candidate_discloses_its_validation_freshness_and_protected_branch_gate: true,
        no_landing_from_ambient_branch_state: true,
        every_object_declares_classification_stages: true,
        every_object_declares_accessibility_route: true,
        support_export_reads_single_change_orchestration_source: true,
        git_surface_start_work_review_provider_and_support_bind_to_single_source: true,
        later_rows_cannot_invent_parallel_change_orchestration_vocabulary: true,
        change_orchestration_truth_survives_zoom_and_high_contrast: true,
        claims_narrow_automatically_when_matrix_row_missing_or_stale: true,
    }
}

fn consumer_projection() -> M5ChangeOrchestrationConsumerProjection {
    M5ChangeOrchestrationConsumerProjection {
        change_object_detail_and_start_work_consume_shared_change_orchestration_truth: true,
        provider_merge_queue_and_provider_handoff_consume_shared_landing_state_truth: true,
        help_and_support_export_consume_shared_membership_and_cleanup_truth: true,
        docs_help_and_screenshots_read_single_change_orchestration_source: true,
        change_objects_bind_to_shared_stack_membership_source: true,
        support_export_reads_single_change_orchestration_source: true,
    }
}

fn proof_freshness() -> M5ChangeOrchestrationProofFreshness {
    M5ChangeOrchestrationProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ChangeOrchestrationReleasePosture {
    M5ChangeOrchestrationReleasePosture {
        proof_packet_ref: M5_CHANGE_ORCHESTRATION_ARTIFACT_REF.to_owned(),
        change_orchestration_audit_ref: M5_CHANGE_ORCHESTRATION_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_REF,
        M5_CHANGE_ORCHESTRATION_MATRIX_DOC_REF,
        M5_CHANGE_OBJECT_DOMAIN_SCHEMA_REF,
        M5_PATCH_STACK_QUEUE_DOMAIN_SCHEMA_REF,
        M5_STACK_EDIT_REVIEW_SHEET_DOMAIN_SCHEMA_REF,
        M5_LANDING_CANDIDATE_SHEET_DOMAIN_SCHEMA_REF,
        M5_PORTABLE_SHELF_DOMAIN_SCHEMA_REF,
        M5_WORKTREE_MANAGER_ROW_DOMAIN_SCHEMA_REF,
        M5_PORTABLE_BUNDLE_LANDED_SCHEMA_REF,
        M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 change-orchestration matrix packet.
pub fn seeded_m5_change_orchestration_matrix() -> M5ChangeOrchestrationMatrixPacket {
    M5ChangeOrchestrationMatrixPacket::new(M5ChangeOrchestrationMatrixPacketInput {
        packet_id: M5_CHANGE_ORCHESTRATION_MATRIX_PACKET_ID.to_owned(),
        matrix_label: "M5 change-object, patch-stack/queue, stack-edit-review, landing-candidate, portable-shelf, and worktree-cleanup-preview matrix"
            .to_owned(),
        change_orchestration_rows: change_orchestration_rows(),
        vocabulary_set: M5ChangeOrchestrationVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the patch-stack queue is held at Beta because its queue-eligibility semantics are not
/// yet fully proven across every provider merge queue; every object class stays visible.
pub fn seeded_m5_change_orchestration_matrix_patch_stack_queue_beta_narrowed(
) -> M5ChangeOrchestrationMatrixPacket {
    let mut packet = seeded_m5_change_orchestration_matrix();
    packet.packet_id = "m5-change-orchestration:patch-stack-queue-beta:0001".to_owned();
    let row = packet
        .change_orchestration_rows
        .iter_mut()
        .find(|row| row.object_class == M5ChangeOrchestrationObject::PatchStackQueue)
        .expect("patch-stack-queue row present");
    row.qualification = M5ChangeOrchestrationQualificationClass::Beta;
    packet
}

/// Narrowed variant: the worktree cleanup preview is narrowed to Preview pending durable running-work and
/// export-safe-evidence preview proof; every object class stays visible.
pub fn seeded_m5_change_orchestration_matrix_worktree_cleanup_preview_preview_narrowed(
) -> M5ChangeOrchestrationMatrixPacket {
    let mut packet = seeded_m5_change_orchestration_matrix();
    packet.packet_id = "m5-change-orchestration:worktree-cleanup-preview-preview:0001".to_owned();
    let row = packet
        .change_orchestration_rows
        .iter_mut()
        .find(|row| row.object_class == M5ChangeOrchestrationObject::WorktreeCleanupPreview)
        .expect("worktree-cleanup-preview row present");
    row.qualification = M5ChangeOrchestrationQualificationClass::Preview;
    packet
}
