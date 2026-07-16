//! Canonical seed for the write-review-sheet fallback-path packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix CSV, Markdown
//! summary, and narrowed fixtures. Every binding is derived from one per-profile [`WriteReviewSheetContent`] so
//! the same constrained-object profile always carries the same reviewed-transition content across the flows that
//! reach it, and every narrowed posture derives its disclosure and action set from
//! [`resolve_review_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-16T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

/// The full accessibility route set every review sheet offers so the write target, reason, and recovery class
/// are discoverable without pointer-only chrome.
fn all_accessibility_routes() -> Vec<M5ConstrainedFileStateAccessibilityRoute> {
    M5ConstrainedFileStateAccessibilityRoute::ALL.to_vec()
}

#[allow(clippy::too_many_arguments)]
fn content(
    write_target: &str,
    write_disposition: &str,
    side_effects: &[&str],
    retained: &[&str],
    lost: &[&str],
    sync_or_regenerate_path: &str,
    required_approvals: &[&str],
    checkpoint_undo_class: CheckpointUndoClass,
    canonical_source: &str,
    export_safe_explanation: &str,
    co_applicable_state_labels: &[&str],
) -> WriteReviewSheetContent {
    let to_owned = |items: &[&str]| items.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();
    WriteReviewSheetContent {
        write_target_word: write_target.to_owned(),
        write_disposition_word: write_disposition.to_owned(),
        side_effect_words: to_owned(side_effects),
        preserved_versus_lost: PreservedVersusLostSync {
            retained: to_owned(retained),
            lost: to_owned(lost),
            sync_or_regenerate_path: sync_or_regenerate_path.to_owned(),
        },
        required_approval_words: to_owned(required_approvals),
        checkpoint_undo_class,
        canonical_source_word: canonical_source.to_owned(),
        export_safe_explanation: export_safe_explanation.to_owned(),
        co_applicable_state_labels: to_owned(co_applicable_state_labels),
    }
}

fn preserved_note_for(reason: ReviewNarrowReason) -> String {
    match reason {
        ReviewNarrowReason::CompactedToPreconditionNotice => {
            "write target, side effects, preserved-versus-lost sync, required approvals, checkpoint / undo class, and export-safe explanation preserved; the sheet is compacted to a precondition notice"
        }
        ReviewNarrowReason::ExportRedactionNarrowed => {
            "all reviewed-transition content preserved; only surrounding detail is redacted export-safe"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: ReviewNarrowNextAction) -> String {
    match action {
        ReviewNarrowNextAction::OpenFullReviewSheet => "Open the full review sheet",
        ReviewNarrowNextAction::OpenFullDetail => "Open the full detail",
    }
    .to_owned()
}

fn allowed_actions_for(disclosure: ReviewRenderDisclosure) -> Vec<WriteReviewAction> {
    let mut actions = WriteReviewAction::SAFE_BASE.to_vec();
    if disclosure.offers_reviewed_commit {
        actions.push(WriteReviewAction::CommitReviewedTransition);
    }
    actions
}

fn binding_refs(object_class: M5ConstrainedFileStateObject) -> Vec<String> {
    vec![
        M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF.to_owned(),
        object_class.canonical_domain_schema_ref().to_owned(),
    ]
}

#[allow(clippy::too_many_arguments)]
fn make_binding(
    binding_id: &str,
    object_profile_id: &str,
    object_profile_label: &str,
    object_class: M5ConstrainedFileStateObject,
    co_applicable_states: Vec<M5ConstrainedFileStateObject>,
    fallback_action: WriteReviewFallbackAction,
    originating_flow: WriteReviewOriginatingFlow,
    posture: ReviewSheetPosture,
    review_content: WriteReviewSheetContent,
) -> WriteReviewSheetBinding {
    let disclosure = resolve_review_render_disclosure(posture);
    let narrow_note = disclosure.narrow_reason.map(|reason| {
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrow reason always carries a next action");
        ReviewNarrowNote {
            reason,
            preserved_content_note: preserved_note_for(reason),
            next_action,
            next_action_label: next_action_label_for(next_action),
        }
    });
    let export_detail_note = if disclosure.needs_export_detail_note {
        "surrounding detail redacted export-safe in this packet; full detail available on request"
            .to_owned()
    } else {
        String::new()
    };

    WriteReviewSheetBinding {
        binding_id: binding_id.to_owned(),
        object_profile_id: object_profile_id.to_owned(),
        object_profile_label: object_profile_label.to_owned(),
        object_class,
        co_applicable_states,
        fallback_action,
        originating_flow,
        posture,
        review_content,
        parity_state: disclosure.parity_state,
        allowed_actions: allowed_actions_for(disclosure),
        accessibility_routes: all_accessibility_routes(),
        narrow_note,
        export_detail_note,
        reviewed_before_commit: true,
        recovery_visible_before_commit: true,
        silently_mutates_current_object_through_lossy_fallback: false,
        gives_ai_automation_import_or_repair_flows_a_hidden_bypass: false,
        leaves_exact_write_target_or_preserved_versus_lost_sync_unstated: false,
        hides_recovery_or_undo_class_before_commit: false,
        lets_one_state_class_hide_another_when_both_materially_affect_behavior: false,
        source_contract_refs: binding_refs(object_class),
    }
}

/// One originating-flow rendering of a constrained-object profile, before any posture override.
struct BindingSpec {
    binding_id: &'static str,
    originating_flow: WriteReviewOriginatingFlow,
    posture: ReviewSheetPosture,
}

/// One constrained-object profile reviewed across several originating flows at one reviewed-transition content.
struct ProfileSpec {
    profile_id: &'static str,
    profile_label: &'static str,
    object_class: M5ConstrainedFileStateObject,
    co_applicable_states: Vec<M5ConstrainedFileStateObject>,
    fallback_action: WriteReviewFallbackAction,
    content: WriteReviewSheetContent,
    bindings: Vec<BindingSpec>,
}

fn bs(
    binding_id: &'static str,
    originating_flow: WriteReviewOriginatingFlow,
    posture: ReviewSheetPosture,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        originating_flow,
        posture,
    }
}

/// The six constrained-object profiles — one per B150 constrained-file-state object class — each mapped to one of
/// the five reviewed fallback transitions and reviewed across the direct-save, code-action, AI-apply, importer,
/// repair, and batch-edit flows. Two profiles are multi-state (`Generated` plus `Policy locked`, `Managed` plus
/// `Captured snapshot`) so both facts stay visible across flows.
fn profile_specs() -> Vec<ProfileSpec> {
    use M5ConstrainedFileStateObject::*;
    use ReviewSheetPosture::*;
    use WriteReviewFallbackAction::*;
    use WriteReviewOriginatingFlow::*;

    vec![
        ProfileSpec {
            profile_id: "read-only/vendored-source",
            profile_label: "Read-only vendored source (duplicate into an editable copy)",
            object_class: ReadOnly,
            co_applicable_states: vec![],
            fallback_action: DuplicateToEditableCopy,
            content: content(
                "new_editable_copy_beside_the_read_only_source",
                "read_only_blocked",
                &[
                    "creates_a_new_writable_file",
                    "read_only_source_left_untouched_in_place",
                ],
                &["read_only_source_bytes", "source_provenance"],
                &[],
                "no_link_kept_edits_apply_only_to_the_new_copy",
                &[],
                CheckpointUndoClass::NewCopyLeavesOriginalIntact,
                "vendored_source_of_truth",
                "duplicating_writes_a_new_editable_copy_and_never_mutates_the_read_only_source_in_place",
                &[],
            ),
            bindings: vec![
                bs("wrs-readonly-directsave", DirectSave, FullReviewSheet),
                bs("wrs-readonly-codeaction", CodeAction, PreconditionNoticeCompact),
                bs("wrs-readonly-export", Importer, ExportRedacted),
            ],
        },
        ProfileSpec {
            profile_id: "generated/policy-locked-artifact",
            profile_label: "Generated artifact that is also policy locked (regenerate with preview)",
            object_class: Generated,
            co_applicable_states: vec![PolicyLocked],
            fallback_action: RegenerateWithPreview,
            content: content(
                "regenerated_artifact_rendered_from_the_generator_input",
                "regenerate_only",
                &[
                    "overwrites_the_generated_artifact_with_a_fresh_render",
                    "local_hand_edits_to_the_artifact_are_discarded",
                ],
                &[
                    "generator_input_source",
                    "previous_render_snapshot_for_undo",
                ],
                &["unsaved_hand_edits_to_the_generated_artifact"],
                "move_edits_to_the_generator_input_then_regenerate",
                &["policy_owner_sign_off_because_the_artifact_is_also_policy_locked"],
                CheckpointUndoClass::RegeneratePreviewDiscardable,
                "generator_source_of_truth",
                "regenerating_previews_the_fresh_render_before_commit_and_keeps_a_restore_point_for_the_previous_render",
                &["policy_locked"],
            ),
            bindings: vec![
                bs("wrs-generated-aiapply", AiApply, FullReviewSheet),
                bs("wrs-generated-batch", BatchEdit, PreconditionNoticeCompact),
                bs("wrs-generated-export", Repair, ExportRedacted),
            ],
        },
        ProfileSpec {
            profile_id: "policy-locked/protected-config",
            profile_label: "Policy-locked protected config (request approval)",
            object_class: PolicyLocked,
            co_applicable_states: vec![],
            fallback_action: RequestApproval,
            content: content(
                "protected_config_pending_an_approved_change_request",
                "approval_gated",
                &[
                    "opens_an_approval_request_to_the_policy_owner",
                    "no_bytes_change_until_approval_is_granted",
                ],
                &["current_protected_config", "approval_audit_trail"],
                &[],
                "change_applies_only_after_the_policy_owner_approves",
                &["policy_owner_of_record_approval"],
                CheckpointUndoClass::ApprovalRequestWithdrawable,
                "policy_owner_of_record",
                "requesting_approval_opens_a_withdrawable_request_and_changes_nothing_until_the_policy_owner_approves",
                &[],
            ),
            bindings: vec![
                bs("wrs-policy-directsave", DirectSave, FullReviewSheet),
                bs("wrs-policy-repair", Repair, PreconditionNoticeCompact),
            ],
        },
        ProfileSpec {
            profile_id: "managed/captured-snapshot-mirror",
            profile_label: "Managed mirror that is also a captured snapshot (detach from managed source)",
            object_class: Managed,
            co_applicable_states: vec![CapturedSnapshot],
            fallback_action: DetachFromManagedSource,
            content: content(
                "detached_local_copy_forked_from_the_managed_source",
                "detach_required",
                &[
                    "forks_a_local_copy_no_longer_synced_to_upstream",
                    "future_upstream_updates_stop_flowing_in",
                ],
                &[
                    "upstream_managed_source",
                    "captured_snapshot_of_the_pre_detach_state",
                ],
                &["automatic_upstream_sync"],
                "re_link_or_re_import_from_upstream_to_restore_sync",
                &[],
                CheckpointUndoClass::DetachCheckpointRestorable,
                "upstream_managing_owner",
                "detaching_forks_a_local_copy_and_records_a_restorable_checkpoint_so_upstream_sync_can_be_re_linked",
                &["captured_snapshot"],
            ),
            bindings: vec![
                bs("wrs-managed-aiapply", AiApply, FullReviewSheet),
                bs("wrs-managed-importer", Importer, PreconditionNoticeCompact),
                bs("wrs-managed-export", BatchEdit, ExportRedacted),
            ],
        },
        ProfileSpec {
            profile_id: "projection/virtual-view",
            profile_label: "Projection / virtual view (create overlay patch)",
            object_class: Projection,
            co_applicable_states: vec![],
            fallback_action: CreateOverlayPatch,
            content: content(
                "overlay_patch_layered_over_the_backing_source",
                "detach_required",
                &[
                    "records_edits_as_an_overlay_patch",
                    "backing_source_object_is_not_modified",
                ],
                &["backing_source_object", "overlay_patch_history"],
                &[],
                "overlay_reapplies_over_the_backing_source_and_can_be_reverted",
                &[],
                CheckpointUndoClass::OverlayPatchRevertible,
                "backing_source_object",
                "an_overlay_patch_records_edits_over_the_backing_source_without_modifying_it_and_can_be_reverted",
                &[],
            ),
            bindings: vec![
                bs("wrs-projection-codeaction", CodeAction, FullReviewSheet),
                bs("wrs-projection-batch", BatchEdit, PreconditionNoticeCompact),
            ],
        },
        ProfileSpec {
            profile_id: "captured-snapshot/preserved-state",
            profile_label: "Captured snapshot of a preserved past state (duplicate into an editable copy)",
            object_class: CapturedSnapshot,
            co_applicable_states: vec![],
            fallback_action: DuplicateToEditableCopy,
            content: content(
                "new_editable_copy_materialized_from_the_captured_snapshot",
                "read_only_blocked",
                &[
                    "materializes_the_snapshot_into_a_new_editable_file",
                    "captured_snapshot_stays_immutable",
                ],
                &["captured_snapshot_bytes", "live_object_of_record"],
                &[],
                "edits_apply_only_to_the_new_copy_not_the_snapshot_or_live_object",
                &[],
                CheckpointUndoClass::NewCopyLeavesOriginalIntact,
                "live_object_of_record",
                "duplicating_materializes_the_snapshot_into_a_new_editable_copy_and_leaves_the_snapshot_immutable",
                &[],
            ),
            bindings: vec![
                bs("wrs-snapshot-directsave", DirectSave, FullReviewSheet),
                bs("wrs-snapshot-aiapply", AiApply, PreconditionNoticeCompact),
            ],
        },
    ]
}

/// Builds all review bindings, applying `posture_override` to override a binding's posture.
fn build_bindings<F>(posture_override: F) -> Vec<WriteReviewSheetBinding>
where
    F: Fn(&str, ReviewSheetPosture) -> ReviewSheetPosture,
{
    let mut bindings = Vec::new();
    for profile in profile_specs() {
        for spec in &profile.bindings {
            let posture = posture_override(spec.binding_id, spec.posture);
            bindings.push(make_binding(
                spec.binding_id,
                profile.profile_id,
                profile.profile_label,
                profile.object_class,
                profile.co_applicable_states.clone(),
                profile.fallback_action,
                spec.originating_flow,
                posture,
                profile.content.clone(),
            ));
        }
    }
    bindings
}

fn trust_review() -> WriteReviewSheetTrustReview {
    WriteReviewSheetTrustReview {
        fallback_action_reuse_proven_by_fixtures: true,
        same_profile_same_review_content_across_flows: true,
        write_disposition_never_masquerades_as_directly_writable: true,
        no_silent_lossy_direct_write_fallback: true,
        no_hidden_bypass_for_ai_automation_import_repair: true,
        exact_write_target_and_preserved_versus_lost_sync_always_stated: true,
        recovery_or_undo_class_visible_before_commit: true,
        multi_state_objects_keep_every_state_visible: true,
        accessibility_routes_present_for_write_target_reason_and_recovery: true,
        narrowing_disclosed_across_postures: true,
        export_views_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn flow_projection() -> WriteReviewSheetFlowProjection {
    WriteReviewSheetFlowProjection {
        direct_save_reuses_sheet: true,
        code_action_reuses_sheet: true,
        ai_apply_reuses_sheet: true,
        importer_reuses_sheet: true,
        repair_reuses_sheet: true,
        batch_edit_reuses_sheet: true,
        every_fallback_action_reviewed_by_two_or_more_flows: true,
        review_content_identical_for_same_profile: true,
        multi_state_objects_keep_both_facts_visible: true,
        narrowing_disclosed_not_hidden: true,
        export_maps_back_to_one_constrained_file_state_object: true,
        duplicate_detach_overlay_request_approval_and_regenerate_all_reviewable: true,
        recovery_visible_before_commit_on_every_path: true,
        no_constrained_write_silently_mutates_current_object: true,
    }
}

fn proof_freshness() -> WriteReviewSheetProofFreshness {
    WriteReviewSheetProofFreshness {
        proof_freshness_slo_hours: M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_SCHEMA_REF.to_owned(),
        M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_DOC_REF.to_owned(),
        M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF.to_owned(),
        M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF.to_owned(),
    ];
    // The six object classes map to three canonical domain schemas; include each distinct one once.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object_class in M5ConstrainedFileStateObject::ALL {
        domains.insert(object_class.canonical_domain_schema_ref());
    }
    for domain in domains {
        refs.push(domain.to_owned());
    }
    refs
}

fn packet_from_bindings(
    packet_id: &str,
    surface_label: &str,
    review_bindings: Vec<WriteReviewSheetBinding>,
) -> M5WriteReviewSheetFallbackPathsPacket {
    M5WriteReviewSheetFallbackPathsPacket::new(M5WriteReviewSheetFallbackPathsPacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        review_bindings,
        downgrade_triggers: WriteReviewSheetFallbackPathsDowngradeTrigger::ALL.to_vec(),
        originating_flows: WriteReviewOriginatingFlow::ALL.to_vec(),
        trust_review: trust_review(),
        flow_projection: flow_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in write-review-sheet fallback-path packet.
pub fn seeded_m5_write_review_sheet_fallback_paths() -> M5WriteReviewSheetFallbackPathsPacket {
    packet_from_bindings(
        M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_PACKET_ID,
        "M5 write-review sheets (reviewed fallback transitions across flows)",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same profiles with two more full review sheets narrowed to a compact precondition notice,
/// proving the reviewed-transition content survives when the sheet is compacted.
pub fn seeded_m5_write_review_sheet_fallback_paths_precondition_notice_narrowed(
) -> M5WriteReviewSheetFallbackPathsPacket {
    packet_from_bindings(
        "m5-write-review-sheet-fallback-paths:precondition-notice:0001",
        "M5 write-review sheets (precondition notice narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "wrs-policy-directsave" => ReviewSheetPosture::PreconditionNoticeCompact,
            "wrs-projection-codeaction" => ReviewSheetPosture::PreconditionNoticeCompact,
            _ => default,
        }),
    )
}

/// Fixture: the same profiles with two more full review sheets narrowed to export-safe redaction, proving the
/// reviewed-transition content survives into exported forms.
pub fn seeded_m5_write_review_sheet_fallback_paths_export_redacted_narrowed(
) -> M5WriteReviewSheetFallbackPathsPacket {
    packet_from_bindings(
        "m5-write-review-sheet-fallback-paths:export-redacted:0001",
        "M5 write-review sheets (export redacted narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "wrs-readonly-directsave" => ReviewSheetPosture::ExportRedacted,
            "wrs-snapshot-directsave" => ReviewSheetPosture::ExportRedacted,
            _ => default,
        }),
    )
}
