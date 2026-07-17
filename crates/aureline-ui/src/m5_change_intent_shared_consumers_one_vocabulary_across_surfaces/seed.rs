//! Canonical seed for the change-intent shared-consumer parity packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix CSV,
//! Markdown summary, and narrowed fixtures. Every binding is derived from one per-subject
//! [`ChangeIntentSharedStateFacetValues`] so the same seeded change-intent subject always carries the same
//! vocabulary across surfaces, and every narrowed representation derives its disclosure from
//! [`resolve_change_intent_shared_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-16T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

fn facets(
    change_intent_role: &str,
    object: &str,
    registry_reference: &str,
    commit_state: &str,
    surface_context: &str,
    relation_source: &str,
) -> ChangeIntentSharedStateFacetValues {
    ChangeIntentSharedStateFacetValues {
        change_intent_role_word: change_intent_role.to_owned(),
        object_word: object.to_owned(),
        registry_reference_word: registry_reference.to_owned(),
        commit_state_word: commit_state.to_owned(),
        surface_context_word: surface_context.to_owned(),
        relation_source_word: relation_source.to_owned(),
    }
}

fn preserved_note_for(reason: ChangeIntentSharedNarrowReason) -> String {
    match reason {
        ChangeIntentSharedNarrowReason::CompactionNarrowed => {
            "change-intent-role, object, registry-reference, commit-state, surface-context, and relation-source words preserved; only disclosure depth compacted"
        }
        ChangeIntentSharedNarrowReason::RemoteProjectionNarrowed => {
            "all change-intent vocabulary preserved; the object is projected from the remote source of truth"
        }
        ChangeIntentSharedNarrowReason::ExportRedactionNarrowed => {
            "all change-intent vocabulary preserved; only surrounding detail is redacted export-safe"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: ChangeIntentSharedNarrowNextAction) -> String {
    match action {
        ChangeIntentSharedNarrowNextAction::ExpandInDesktop => "Expand in the desktop surface",
        ChangeIntentSharedNarrowNextAction::OpenRemoteSource => "Open the remote source",
        ChangeIntentSharedNarrowNextAction::OpenFullDetail => "Open the full detail",
    }
    .to_owned()
}

fn binding_refs(object: M5ChangeIntentObject) -> Vec<String> {
    vec![
        M5_CHANGE_INTENT_MATRIX_SCHEMA_REF.to_owned(),
        object.canonical_domain_schema_ref().to_owned(),
    ]
}

fn make_binding(
    binding_id: &str,
    subject_id: &str,
    subject_label: &str,
    object: M5ChangeIntentObject,
    consumer: M5ChangeIntentConsumerSurface,
    representation: ChangeIntentSharedRepresentation,
    state_facets: ChangeIntentSharedStateFacetValues,
) -> ChangeIntentSharedConsumerBinding {
    let disclosure = resolve_change_intent_shared_render_disclosure(representation);
    let narrow_note = disclosure.narrow_reason.map(|reason| {
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrow reason always carries a next action");
        ChangeIntentSharedNarrowNote {
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

    ChangeIntentSharedConsumerBinding {
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
        lets_a_local_handoff_or_queued_publish_masquerade_as_a_provider_committed_update: false,
        silently_creates_a_branch_worktree_review_draft_or_provider_link_without_disclosure: false,
        flattens_linked_by_provider_linked_locally_suggested_and_stale_or_broken_into_one_relation_badge: false,
        auto_resolves_tracked_work_while_engineering_blockers_remain_unresolved: false,
        drops_local_notes_handoff_packets_or_linked_evidence_when_provider_write_fails:
            false,
        source_contract_refs: binding_refs(object),
    }
}

/// One consumer-surface adoption of a seeded subject, before any representation override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5ChangeIntentConsumerSurface,
    representation: ChangeIntentSharedRepresentation,
}

/// One seeded change-intent subject rendered across several consumer surfaces at one vocabulary.
struct SubjectSpec {
    subject_id: &'static str,
    subject_label: &'static str,
    object: M5ChangeIntentObject,
    facets: ChangeIntentSharedStateFacetValues,
    bindings: Vec<BindingSpec>,
}

fn spec(
    subject_id: &'static str,
    subject_label: &'static str,
    object: M5ChangeIntentObject,
    facets: ChangeIntentSharedStateFacetValues,
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
    consumer: M5ChangeIntentConsumerSurface,
    representation: ChangeIntentSharedRepresentation,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        representation,
    }
}

/// The six seeded change-intent subjects — one per B153 change-intent lifecycle object (the change-intent
/// record, the start-work sheet, the linked-change panel, the ready-for-review handoff sheet, the
/// resolve-or-close sheet, and the blocked-or-escalate card) — and the surfaces that adopt each, drawn from the
/// work-item-detail, start-work-sheet, linked-change-panel, review-detail, ready-for-review-handoff,
/// resolve-close-sheet, blocked-escalate-card, support-export, and help / docs consumers that back the
/// work-item, review, Git / worktree, AI-evidence, support / export, and provider-handoff surfaces.
fn subject_specs() -> Vec<SubjectSpec> {
    use ChangeIntentSharedRepresentation::*;
    use M5ChangeIntentConsumerSurface::*;
    use M5ChangeIntentObject as Object;

    // Gate-role subjects keep a real relation-source / commit-state continuity; non-gate subjects carry a
    // scoped descriptor.
    let relation_bound = "relation_source_disclosed_and_commit_state_bound";
    let change_intent_scoped_descriptor = "change_intent_scoped_descriptor";

    vec![
        spec(
            "change-intent-record/one-tracked-work-item",
            "Change-intent record (one tracked work item's intent bound to provider ownership, local-versus-provider state, and linked branch / worktree / review identity)",
            Object::ChangeIntentRecord,
            facets(
                "provider_ownership_disclosure",
                "change_intent_record",
                "change_intent_registry",
                "provider_committed",
                "work_item_detail_and_start_work_sheet",
                relation_bound,
            ),
            vec![
                bs("cisc-record-work-item", WorkItemDetail, DesktopFull),
                bs("cisc-record-start-work", StartWorkSheet, DesktopFull),
                bs("cisc-record-support", SupportExportPacket, ExportedRedacted),
            ],
        ),
        spec(
            "start-work-sheet/disclosed-side-effects",
            "Start-work sheet (launching work from a tracked item and separately disclosing each side effect it creates: branch, worktree, review draft, provider link)",
            Object::StartWorkSheet,
            facets(
                "side_effect_disclosure",
                "start_work_sheet",
                "start_work_sheet_registry",
                "local_only_draft",
                "start_work_sheet_and_work_item_detail",
                relation_bound,
            ),
            vec![
                bs("cisc-startwork-start-work", StartWorkSheet, DesktopFull),
                bs("cisc-startwork-work-item", WorkItemDetail, CompactNarrowed),
                bs("cisc-startwork-support", SupportExportPacket, ExportedRedacted),
            ],
        ),
        spec(
            "linked-change-panel/four-distinct-relations",
            "Linked-change panel (the relation strip keeping linked-by-provider, linked-locally, suggested-by-Aureline, and stale-or-broken relations distinct)",
            Object::LinkedChangePanel,
            facets(
                "linked_engineering_identity_disclosure",
                "linked_change_panel",
                "linked_change_panel_registry",
                "queued_for_publish",
                "linked_change_panel_and_review_detail",
                relation_bound,
            ),
            vec![
                bs("cisc-linked-panel", LinkedChangePanel, DesktopFull),
                bs("cisc-linked-review-detail", ReviewDetail, DesktopFull),
                bs("cisc-linked-support", SupportExportPacket, ExportedRedacted),
            ],
        ),
        spec(
            "ready-for-review-handoff/validation-evidence-and-publish-later",
            "Ready-for-review handoff sheet (packaging a review handoff with its validation evidence and a publish-later fallback)",
            Object::ReadyForReviewHandoffSheet,
            facets(
                "validation_evidence_disclosure",
                "ready_for_review_handoff_sheet",
                "ready_for_review_handoff_sheet_registry",
                "queued_for_publish",
                "ready_for_review_handoff_and_review_detail",
                change_intent_scoped_descriptor,
            ),
            vec![
                bs("cisc-handoff-ready", ReadyForReviewHandoff, DesktopFull),
                bs("cisc-handoff-review-detail", ReviewDetail, RemoteProjected),
                bs("cisc-handoff-linked-panel", LinkedChangePanel, DesktopFull),
            ],
        ),
        spec(
            "resolve-close-sheet/final-resolution-authority",
            "Resolve-or-close sheet (recording final-resolution authority and refusing to auto-resolve tracked work while engineering blockers remain)",
            Object::ResolveCloseSheet,
            facets(
                "final_resolution_authority_disclosure",
                "resolve_close_sheet",
                "resolve_close_sheet_registry",
                "provider_committed",
                "resolve_close_sheet_and_blocked_escalate_card",
                change_intent_scoped_descriptor,
            ),
            vec![
                bs("cisc-resolve-sheet", ResolveCloseSheet, DesktopFull),
                bs("cisc-resolve-blocked-card", BlockedEscalateCard, DesktopFull),
                bs("cisc-resolve-help-docs", HelpDocs, CompactNarrowed),
            ],
        ),
        spec(
            "blocked-escalate-card/publish-later-fallback",
            "Blocked-or-escalate card (surfacing an unresolved engineering blocker and its escalation path without dropping local notes or linked evidence)",
            Object::BlockedEscalateCard,
            facets(
                "publish_later_fallback_disclosure",
                "blocked_escalate_card",
                "blocked_escalate_card_registry",
                "publish_failed_retained",
                "blocked_escalate_card_and_help_docs",
                change_intent_scoped_descriptor,
            ),
            vec![
                bs("cisc-blocked-card", BlockedEscalateCard, DesktopFull),
                bs("cisc-blocked-help-docs", HelpDocs, DesktopFull),
                bs("cisc-blocked-support", SupportExportPacket, ExportedRedacted),
            ],
        ),
    ]
}

/// Builds all consumer bindings, applying `rep` to override a binding's representation.
fn build_bindings<F>(rep: F) -> Vec<ChangeIntentSharedConsumerBinding>
where
    F: Fn(&str, ChangeIntentSharedRepresentation) -> ChangeIntentSharedRepresentation,
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

fn trust_review() -> M5ChangeIntentSharedConsumersTrustReview {
    M5ChangeIntentSharedConsumersTrustReview {
        object_reuse_proven_by_fixtures: true,
        same_subject_same_change_intent_vocabulary_across_surfaces: true,
        change_intent_role_words_stay_in_frozen_vocabulary: true,
        gate_roles_never_let_local_draft_read_as_provider_committed: true,
        side_effects_never_created_without_separate_disclosure: true,
        relation_sources_never_flattened_into_one_badge: true,
        tracked_work_never_auto_resolved_while_blockers_remain: true,
        local_notes_handoff_and_linked_evidence_never_dropped_on_provider_write_failure: true,
        narrowing_disclosed_across_representations: true,
        support_export_point_canonical_contracts: true,
        copy_export_open_provider_preserve_one_payload: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> M5ChangeIntentSharedConsumersProjection {
    M5ChangeIntentSharedConsumersProjection {
        review_detail_consumes_shared_change_intent_vocabulary: true,
        start_work_sheet_consumes_shared_change_intent_vocabulary: true,
        linked_change_panel_consumes_shared_change_intent_vocabulary: true,
        ready_for_review_handoff_consumes_shared_change_intent_vocabulary: true,
        work_item_detail_consumes_shared_change_intent_vocabulary: true,
        resolve_close_sheet_consumes_shared_change_intent_vocabulary: true,
        blocked_escalate_card_consumes_shared_change_intent_vocabulary: true,
        support_export_packet_consumes_shared_change_intent_vocabulary: true,
        help_docs_consumes_shared_change_intent_vocabulary: true,
        every_object_adopted_by_two_or_more_consumers: true,
        change_intent_vocabulary_identical_for_same_subject: true,
        narrowing_disclosed_not_hidden: true,
        export_maps_back_to_one_change_intent_object: true,
    }
}

fn proof_freshness() -> M5ChangeIntentSharedConsumersProofFreshness {
    M5ChangeIntentSharedConsumersProofFreshness {
        proof_freshness_slo_hours: M5_CHANGE_INTENT_SHARED_CONSUMERS_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_CHANGE_INTENT_SHARED_CONSUMERS_SCHEMA_REF.to_owned(),
        M5_CHANGE_INTENT_SHARED_CONSUMERS_DOC_REF.to_owned(),
        M5_CHANGE_INTENT_MATRIX_SCHEMA_REF.to_owned(),
        M5_CHANGE_INTENT_MATRIX_DOC_REF.to_owned(),
    ];
    // The six objects each map to their own canonical domain schema; include each distinct one once.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object in M5ChangeIntentObject::ALL {
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
    consumer_bindings: Vec<ChangeIntentSharedConsumerBinding>,
) -> M5ChangeIntentSharedConsumersPacket {
    M5ChangeIntentSharedConsumersPacket::new(M5ChangeIntentSharedConsumersPacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        consumer_bindings,
        downgrade_triggers: M5ChangeIntentSharedConsumersDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5ChangeIntentConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in change-intent shared-consumer parity packet.
pub fn seeded_m5_change_intent_shared_consumers() -> M5ChangeIntentSharedConsumersPacket {
    packet_from_bindings(
        M5_CHANGE_INTENT_SHARED_CONSUMERS_PACKET_ID,
        "M5 change-intent shared consumers (one vocabulary across surfaces)",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same subjects with two more desktop surfaces narrowed to compact and remote
/// representations, proving vocabulary survives compact and remote forms.
pub fn seeded_m5_change_intent_shared_consumers_compact_remote_narrowed(
) -> M5ChangeIntentSharedConsumersPacket {
    packet_from_bindings(
        "m5-change-intent-shared-consumers:compact-remote:0001",
        "M5 change-intent shared consumers (compact / remote narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "cisc-record-start-work" => ChangeIntentSharedRepresentation::CompactNarrowed,
            "cisc-handoff-linked-panel" => ChangeIntentSharedRepresentation::RemoteProjected,
            _ => default,
        }),
    )
}

/// Fixture: the same subjects with two more surfaces narrowed to exported, export-safe
/// representations, proving vocabulary survives into exported forms.
pub fn seeded_m5_change_intent_shared_consumers_exported_redaction_narrowed(
) -> M5ChangeIntentSharedConsumersPacket {
    packet_from_bindings(
        "m5-change-intent-shared-consumers:exported-redaction:0001",
        "M5 change-intent shared consumers (exported redaction narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "cisc-record-work-item" => ChangeIntentSharedRepresentation::ExportedRedacted,
            "cisc-resolve-sheet" => ChangeIntentSharedRepresentation::ExportedRedacted,
            _ => default,
        }),
    )
}
