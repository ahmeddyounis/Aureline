//! Canonical seed for the change-orchestration shared-consumer parity packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix CSV,
//! Markdown summary, and narrowed fixtures. Every binding is derived from one per-subject
//! [`ChangeOrchestrationSharedStateFacetValues`] so the same seeded change-orchestration subject always carries the same
//! vocabulary across surfaces, and every narrowed representation derives its disclosure from
//! [`resolve_change_orchestration_shared_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-16T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

fn facets(
    change_orchestration_role: &str,
    object: &str,
    registry_reference: &str,
    commit_state: &str,
    surface_context: &str,
    relation_source: &str,
) -> ChangeOrchestrationSharedStateFacetValues {
    ChangeOrchestrationSharedStateFacetValues {
        change_orchestration_role_word: change_orchestration_role.to_owned(),
        object_word: object.to_owned(),
        registry_reference_word: registry_reference.to_owned(),
        landing_state_word: commit_state.to_owned(),
        surface_context_word: surface_context.to_owned(),
        membership_source_word: relation_source.to_owned(),
    }
}

fn preserved_note_for(reason: ChangeOrchestrationSharedNarrowReason) -> String {
    match reason {
        ChangeOrchestrationSharedNarrowReason::CompactionNarrowed => {
            "change-orchestration-role, object, registry-reference, landing-state, surface-context, and membership-source words preserved; only disclosure depth compacted"
        }
        ChangeOrchestrationSharedNarrowReason::RemoteProjectionNarrowed => {
            "all change-orchestration vocabulary preserved; the object is projected from the remote source of truth"
        }
        ChangeOrchestrationSharedNarrowReason::ExportRedactionNarrowed => {
            "all change-orchestration vocabulary preserved; only surrounding detail is redacted export-safe"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: ChangeOrchestrationSharedNarrowNextAction) -> String {
    match action {
        ChangeOrchestrationSharedNarrowNextAction::ExpandInDesktop => {
            "Expand in the desktop surface"
        }
        ChangeOrchestrationSharedNarrowNextAction::OpenRemoteSource => "Open the remote source",
        ChangeOrchestrationSharedNarrowNextAction::OpenFullDetail => "Open the full detail",
    }
    .to_owned()
}

fn binding_refs(object: M5ChangeOrchestrationObject) -> Vec<String> {
    vec![
        M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_REF.to_owned(),
        object.canonical_domain_schema_ref().to_owned(),
    ]
}

fn make_binding(
    binding_id: &str,
    subject_id: &str,
    subject_label: &str,
    object: M5ChangeOrchestrationObject,
    consumer: M5ChangeOrchestrationConsumerSurface,
    representation: ChangeOrchestrationSharedRepresentation,
    state_facets: ChangeOrchestrationSharedStateFacetValues,
) -> ChangeOrchestrationSharedConsumerBinding {
    let disclosure = resolve_change_orchestration_shared_render_disclosure(representation);
    let narrow_note = disclosure.narrow_reason.map(|reason| {
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrow reason always carries a next action");
        ChangeOrchestrationSharedNarrowNote {
            reason,
            preserved_vocabulary_note: preserved_note_for(reason),
            next_action,
            next_action_label: next_action_label_for(next_action),
        }
    });
    let remote_source_note = if disclosure.needs_remote_source_note {
        "projected from the remote source of truth; the source stays remote".to_owned()
    } else {
        String::new()
    };
    let export_detail_note = if disclosure.needs_export_detail_note {
        "surrounding detail redacted export-safe in this packet; full detail available on request"
            .to_owned()
    } else {
        String::new()
    };

    ChangeOrchestrationSharedConsumerBinding {
        binding_id: binding_id.to_owned(),
        subject_id: subject_id.to_owned(),
        subject_label: subject_label.to_owned(),
        object,
        consumer,
        representation,
        state_facets,
        vocabulary_state: disclosure.vocabulary_state,
        narrow_note,
        remote_source_note,
        export_detail_note,
        treats_ambient_branch_state_as_a_reviewed_landing_candidate: false,
        mutates_another_worktree_without_a_selected_change_object_and_worktree_binding: false,
        infers_stack_membership_from_branch_names_alone: false,
        silently_reorders_collapses_or_retargets_stack_members: false,
        deletes_orphaned_worktrees_or_stale_members_without_previewing_running_work_and_recovery:
            false,
        source_contract_refs: binding_refs(object),
    }
}

/// One consumer-surface adoption of a seeded subject, before any representation override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5ChangeOrchestrationConsumerSurface,
    representation: ChangeOrchestrationSharedRepresentation,
}

/// One seeded change-orchestration subject rendered across several consumer surfaces at one vocabulary.
struct SubjectSpec {
    subject_id: &'static str,
    subject_label: &'static str,
    object: M5ChangeOrchestrationObject,
    facets: ChangeOrchestrationSharedStateFacetValues,
    bindings: Vec<BindingSpec>,
}

fn spec(
    subject_id: &'static str,
    subject_label: &'static str,
    object: M5ChangeOrchestrationObject,
    facets: ChangeOrchestrationSharedStateFacetValues,
    bindings: Vec<BindingSpec>,
) -> SubjectSpec {
    SubjectSpec {
        subject_id,
        subject_label,
        object,
        facets,
        bindings,
    }
}

fn bs(
    binding_id: &'static str,
    consumer: M5ChangeOrchestrationConsumerSurface,
    representation: ChangeOrchestrationSharedRepresentation,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        representation,
    }
}

/// The six seeded change-orchestration subjects — one per B154 change-orchestration object (the change object,
/// the patch stack / queue, the stack-edit / review sheet, the landing-candidate sheet, the portable shelf /
/// bundle, and the worktree cleanup preview) — and the surfaces that adopt each, drawn from the
/// change-object-detail, patch-stack-queue, stack-edit-review-sheet, review-detail, provider-merge-queue,
/// portable-shelf, worktree-cleanup-preview, support-export, and help / docs consumers that back the Git /
/// worktree, review, work-item, AI-branch-agent, support / export, and portable-handoff surfaces.
fn subject_specs() -> Vec<SubjectSpec> {
    use ChangeOrchestrationSharedRepresentation::*;
    use M5ChangeOrchestrationConsumerSurface::*;
    use M5ChangeOrchestrationObject as Object;

    // Gate-role subjects keep a real membership-source / worktree-binding continuity; non-gate subjects carry a
    // scoped descriptor.
    let membership_bound = "membership_source_disclosed_and_worktree_binding_bound";
    let change_orchestration_scoped_descriptor = "change_orchestration_scoped_descriptor";

    vec![
        spec(
            "change-object/one-selected-change",
            "Change object (one non-trivial multi-file change bound to its selected worktree / base identity, stack membership, landing state, and validation freshness)",
            Object::ChangeObject,
            facets(
                "selected_change_object_disclosure",
                "change_object",
                "change_object_registry",
                "selected_change",
                "change_object_detail_and_patch_stack_queue",
                membership_bound,
            ),
            vec![
                bs("cosc-changeobject-detail", ChangeObjectDetail, DesktopFull),
                bs("cosc-changeobject-stack", PatchStackQueue, DesktopFull),
                bs("cosc-changeobject-support", SupportExportPacket, ExportedRedacted),
            ],
        ),
        spec(
            "patch-stack-queue/declared-membership",
            "Patch stack / queue (ordered member IDs and landing order declared in the change object, never inferred from branch names)",
            Object::PatchStackQueue,
            facets(
                "stack_membership_disclosure",
                "patch_stack_queue",
                "patch_stack_queue_registry",
                "queue_eligible",
                "patch_stack_queue_and_change_object_detail",
                membership_bound,
            ),
            vec![
                bs("cosc-stack-queue", PatchStackQueue, DesktopFull),
                bs("cosc-stack-detail", ChangeObjectDetail, CompactNarrowed),
                bs("cosc-stack-support", SupportExportPacket, ExportedRedacted),
            ],
        ),
        spec(
            "stack-edit-review-sheet/worktree-bound-edit",
            "Stack-edit / review sheet (reorder / split / squash / restack edits reviewed against an explicit selected change object and worktree binding)",
            Object::StackEditReviewSheet,
            facets(
                "worktree_binding_disclosure",
                "stack_edit_review_sheet",
                "stack_edit_review_sheet_registry",
                "restack_required",
                "stack_edit_review_sheet_and_review_detail",
                membership_bound,
            ),
            vec![
                bs("cosc-edit-sheet", StackEditReviewSheet, DesktopFull),
                bs("cosc-edit-review", ReviewDetail, DesktopFull),
                bs("cosc-edit-support", SupportExportPacket, ExportedRedacted),
            ],
        ),
        spec(
            "landing-candidate-sheet/validation-freshness",
            "Landing-candidate sheet (a reviewed candidate packaging validation freshness, the protected-branch gate, and a rollback / export fallback rather than landing from ambient branch state)",
            Object::LandingCandidateSheet,
            facets(
                "validation_freshness_disclosure",
                "landing_candidate_sheet",
                "landing_candidate_sheet_registry",
                "queue_eligible",
                "provider_merge_queue_and_review_detail",
                change_orchestration_scoped_descriptor,
            ),
            vec![
                bs("cosc-landing-mergequeue", ProviderMergeQueue, DesktopFull),
                bs("cosc-landing-review", ReviewDetail, RemoteProjected),
                bs("cosc-landing-editsheet", StackEditReviewSheet, DesktopFull),
            ],
        ),
        spec(
            "portable-shelf/export-and-reopen",
            "Portable shelf / bundle (exporting, importing, and reopening a change object with its bundle contents, lineage, and recovery checkpoint via a rollback / export fallback)",
            Object::PortableShelf,
            facets(
                "rollback_export_fallback_disclosure",
                "portable_shelf",
                "portable_shelf_registry",
                "exported",
                "portable_shelf_and_worktree_cleanup_preview",
                change_orchestration_scoped_descriptor,
            ),
            vec![
                bs("cosc-shelf-portable", PortableShelf, DesktopFull),
                bs("cosc-shelf-cleanup", WorktreeCleanupPreview, DesktopFull),
                bs("cosc-shelf-help", HelpDocs, CompactNarrowed),
            ],
        ),
        spec(
            "worktree-cleanup-preview/preview-before-delete",
            "Worktree cleanup preview (naming the cleanup target and previewing running tasks, open editors, uncommitted changes, and recovery checkpoints before any deletion)",
            Object::WorktreeCleanupPreview,
            facets(
                "cleanup_safety_disclosure",
                "worktree_cleanup_preview",
                "worktree_cleanup_preview_registry",
                "orphaned",
                "worktree_cleanup_preview_and_help_docs",
                change_orchestration_scoped_descriptor,
            ),
            vec![
                bs("cosc-cleanup-preview", WorktreeCleanupPreview, DesktopFull),
                bs("cosc-cleanup-help", HelpDocs, DesktopFull),
                bs("cosc-cleanup-support", SupportExportPacket, ExportedRedacted),
            ],
        ),
    ]
}

/// Builds all consumer bindings, applying `rep` to override a binding's representation.
fn build_bindings<F>(rep: F) -> Vec<ChangeOrchestrationSharedConsumerBinding>
where
    F: Fn(&str, ChangeOrchestrationSharedRepresentation) -> ChangeOrchestrationSharedRepresentation,
{
    let mut bindings = Vec::new();
    for subject in subject_specs() {
        for spec in &subject.bindings {
            let representation = rep(spec.binding_id, spec.representation);
            bindings.push(make_binding(
                spec.binding_id,
                subject.subject_id,
                subject.subject_label,
                subject.object,
                spec.consumer,
                representation,
                subject.facets.clone(),
            ));
        }
    }
    bindings
}

fn trust_review() -> M5ChangeOrchestrationSharedConsumersTrustReview {
    M5ChangeOrchestrationSharedConsumersTrustReview {
        object_reuse_proven_by_fixtures: true,
        same_subject_same_change_orchestration_vocabulary_across_surfaces: true,
        change_orchestration_role_words_stay_in_frozen_vocabulary: true,
        gate_roles_never_let_ambient_branch_read_as_landing_candidate: true,
        cross_worktree_writes_never_made_without_selected_change_binding: true,
        stack_membership_never_inferred_from_branch_names: true,
        stack_members_never_silently_reordered_collapsed_or_retargeted: true,
        orphaned_worktrees_never_deleted_without_previewing_running_work_and_recovery: true,
        narrowing_disclosed_across_representations: true,
        support_export_point_canonical_contracts: true,
        copy_export_open_provider_preserve_one_payload: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> M5ChangeOrchestrationSharedConsumersProjection {
    M5ChangeOrchestrationSharedConsumersProjection {
        review_detail_consumes_shared_change_orchestration_vocabulary: true,
        patch_stack_queue_consumes_shared_change_orchestration_vocabulary: true,
        stack_edit_review_sheet_consumes_shared_change_orchestration_vocabulary: true,
        provider_merge_queue_consumes_shared_change_orchestration_vocabulary: true,
        change_object_detail_consumes_shared_change_orchestration_vocabulary: true,
        portable_shelf_consumes_shared_change_orchestration_vocabulary: true,
        worktree_cleanup_preview_consumes_shared_change_orchestration_vocabulary: true,
        support_export_packet_consumes_shared_change_orchestration_vocabulary: true,
        help_docs_consumes_shared_change_orchestration_vocabulary: true,
        every_object_adopted_by_two_or_more_consumers: true,
        change_orchestration_vocabulary_identical_for_same_subject: true,
        narrowing_disclosed_not_hidden: true,
        export_maps_back_to_one_change_orchestration_object: true,
    }
}

fn proof_freshness() -> M5ChangeOrchestrationSharedConsumersProofFreshness {
    M5ChangeOrchestrationSharedConsumersProofFreshness {
        proof_freshness_slo_hours: M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_SCHEMA_REF.to_owned(),
        M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_DOC_REF.to_owned(),
        M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_REF.to_owned(),
        M5_CHANGE_ORCHESTRATION_MATRIX_DOC_REF.to_owned(),
    ];
    // The six objects each map to their own canonical domain schema; include each distinct one once.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object in M5ChangeOrchestrationObject::ALL {
        domains.insert(object.canonical_domain_schema_ref());
    }
    for domain in domains {
        refs.push(domain.to_owned());
    }
    refs
}

fn packet_from_bindings(
    packet_id: &str,
    surface_label: &str,
    consumer_bindings: Vec<ChangeOrchestrationSharedConsumerBinding>,
) -> M5ChangeOrchestrationSharedConsumersPacket {
    M5ChangeOrchestrationSharedConsumersPacket::new(
        M5ChangeOrchestrationSharedConsumersPacketInput {
            packet_id: packet_id.to_owned(),
            surface_label: surface_label.to_owned(),
            consumer_bindings,
            downgrade_triggers: M5ChangeOrchestrationSharedConsumersDowngradeTrigger::ALL.to_vec(),
            consumer_surfaces: M5ChangeOrchestrationConsumerSurface::ALL.to_vec(),
            trust_review: trust_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// The canonical, checked-in change-orchestration shared-consumer parity packet.
pub fn seeded_m5_change_orchestration_shared_consumers(
) -> M5ChangeOrchestrationSharedConsumersPacket {
    packet_from_bindings(
        M5_CHANGE_ORCHESTRATION_SHARED_CONSUMERS_PACKET_ID,
        "M5 change-orchestration shared consumers (one vocabulary across surfaces)",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same subjects with two more desktop surfaces narrowed to compact and remote
/// representations, proving vocabulary survives compact and remote forms.
pub fn seeded_m5_change_orchestration_shared_consumers_compact_remote_narrowed(
) -> M5ChangeOrchestrationSharedConsumersPacket {
    packet_from_bindings(
        "m5-change-orchestration-shared-consumers:compact-remote:0001",
        "M5 change-orchestration shared consumers (compact / remote narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "cosc-changeobject-stack" => ChangeOrchestrationSharedRepresentation::CompactNarrowed,
            "cosc-shelf-cleanup" => ChangeOrchestrationSharedRepresentation::RemoteProjected,
            _ => default,
        }),
    )
}

/// Fixture: the same subjects with two more surfaces narrowed to exported, export-safe
/// representations, proving vocabulary survives into exported forms.
pub fn seeded_m5_change_orchestration_shared_consumers_exported_redaction_narrowed(
) -> M5ChangeOrchestrationSharedConsumersPacket {
    packet_from_bindings(
        "m5-change-orchestration-shared-consumers:exported-redaction:0001",
        "M5 change-orchestration shared consumers (exported redaction narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "cosc-changeobject-detail" => ChangeOrchestrationSharedRepresentation::ExportedRedacted,
            "cosc-edit-sheet" => ChangeOrchestrationSharedRepresentation::ExportedRedacted,
            _ => default,
        }),
    )
}
