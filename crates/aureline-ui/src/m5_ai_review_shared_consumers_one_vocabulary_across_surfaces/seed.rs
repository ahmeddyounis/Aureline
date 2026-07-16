//! Canonical seed for the AI-review shared-consumer parity packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix CSV,
//! Markdown summary, and narrowed fixtures. Every binding is derived from one per-finding
//! [`AiReviewStateFacetValues`] so the same seeded finding always carries the same vocabulary across surfaces,
//! and every narrowed representation derives its disclosure from [`resolve_ai_review_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-16T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

fn facets(
    ai_review_role: &str,
    object: &str,
    registry_reference: &str,
    publish_state: &str,
    surface_context: &str,
    finding_lifecycle: &str,
) -> AiReviewStateFacetValues {
    AiReviewStateFacetValues {
        ai_review_role_word: ai_review_role.to_owned(),
        object_word: object.to_owned(),
        registry_reference_word: registry_reference.to_owned(),
        publish_state_word: publish_state.to_owned(),
        surface_context_word: surface_context.to_owned(),
        finding_lifecycle_word: finding_lifecycle.to_owned(),
    }
}

fn preserved_note_for(reason: AiReviewNarrowReason) -> String {
    match reason {
        AiReviewNarrowReason::CompactionNarrowed => {
            "ai-review-role, object, registry-reference, publish-state, surface-context, and finding-lifecycle words preserved; only disclosure depth compacted"
        }
        AiReviewNarrowReason::RemoteProjectionNarrowed => {
            "all ai-review vocabulary preserved; the object is projected from the remote source of truth"
        }
        AiReviewNarrowReason::ExportRedactionNarrowed => {
            "all ai-review vocabulary preserved; only surrounding detail is redacted export-safe"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: AiReviewNarrowNextAction) -> String {
    match action {
        AiReviewNarrowNextAction::ExpandInDesktop => "Expand in the desktop surface",
        AiReviewNarrowNextAction::OpenRemoteSource => "Open the remote source",
        AiReviewNarrowNextAction::OpenFullDetail => "Open the full detail",
    }
    .to_owned()
}

fn binding_refs(object: M5AiReviewAssistObject) -> Vec<String> {
    vec![
        M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_REF.to_owned(),
        object.canonical_domain_schema_ref().to_owned(),
    ]
}

fn make_binding(
    binding_id: &str,
    finding_id: &str,
    finding_label: &str,
    object: M5AiReviewAssistObject,
    consumer: M5AiReviewAssistConsumerSurface,
    representation: AiReviewRepresentation,
    state_facets: AiReviewStateFacetValues,
) -> AiReviewConsumerBinding {
    let disclosure = resolve_ai_review_render_disclosure(representation);
    let narrow_note = disclosure.narrow_reason.map(|reason| {
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrow reason always carries a next action");
        AiReviewNarrowNote {
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

    AiReviewConsumerBinding {
        binding_id: binding_id.to_owned(),
        finding_id: finding_id.to_owned(),
        finding_label: finding_label.to_owned(),
        object,
        consumer,
        representation,
        state_facets,
        parity_state: disclosure.parity_state,
        narrow_note,
        remote_source_note,
        export_detail_note,
        lets_ai_review_results_publish_or_merge_implicitly: false,
        hides_whether_output_stays_local_or_becomes_a_provider_comment_suggested_patch_or_check_annotation:
            false,
        keeps_stale_findings_looking_current_after_diff_or_instruction_drift: false,
        loses_local_drafts_or_evidence_when_provider_write_scope_is_missing_or_publish_fails: false,
        presents_an_ai_review_finding_without_its_analyzed_scope_publish_destination_or_lifecycle_state:
            false,
        source_contract_refs: binding_refs(object),
    }
}

/// One consumer-surface adoption of a seeded finding, before any representation override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5AiReviewAssistConsumerSurface,
    representation: AiReviewRepresentation,
}

/// One seeded finding rendered across several consumer surfaces at one vocabulary.
struct FindingSpec {
    finding_id: &'static str,
    finding_label: &'static str,
    object: M5AiReviewAssistObject,
    facets: AiReviewStateFacetValues,
    bindings: Vec<BindingSpec>,
}

fn spec(
    finding_id: &'static str,
    finding_label: &'static str,
    object: M5AiReviewAssistObject,
    facets: AiReviewStateFacetValues,
    bindings: Vec<BindingSpec>,
) -> FindingSpec {
    FindingSpec {
        finding_id,
        finding_label,
        object,
        facets,
        bindings,
    }
}

fn bs(
    binding_id: &'static str,
    consumer: M5AiReviewAssistConsumerSurface,
    representation: AiReviewRepresentation,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        representation,
    }
}

/// The four seeded findings — one per B151 AI-review-assist object — and the surfaces that adopt each, drawn
/// from the review-detail, AI-review-panel, finding-row, review-scope-selector, publish-to-review-sheet,
/// pending-review-tray, provider-publish-review, resolution-memory-ledger, and support-export consumers that
/// back the review-list / detail, pending-review-tray, AI-evidence / history, support / export, and help /
/// docs surfaces.
fn finding_specs() -> Vec<FindingSpec> {
    use AiReviewRepresentation::*;
    use M5AiReviewAssistConsumerSurface::*;
    use M5AiReviewAssistObject as Object;

    let finding_lifecycle = "finding_current_scope_bound_and_destination_disclosed";
    let ai_review_scoped_descriptor = "ai_review_scoped_descriptor";

    vec![
        spec(
            "ai-review-finding-row/one-inspectable-finding",
            "AI review finding row (one inspectable finding: class, severity / confidence, analyzed scope, lifecycle)",
            Object::AiReviewFindingRow,
            facets(
                "finding_classification",
                "ai_review_finding_row",
                "ai_review_finding_registry",
                "local_draft",
                "review_detail_and_finding_row",
                finding_lifecycle,
            ),
            vec![
                bs("arsc-finding-review-detail", ReviewDetail, DesktopFull),
                bs("arsc-finding-finding-row", FindingRow, DesktopFull),
                bs("arsc-finding-support", SupportExportPacket, ExportedRedacted),
            ],
        ),
        spec(
            "review-scope-selector/analyzed-diff-scope",
            "Review scope selector (analyzed diff scope and repo-instruction / check source)",
            Object::ReviewScopeSelector,
            facets(
                "analyzed_scope_disclosure",
                "review_scope_selector",
                "review_scope_selector_registry",
                "local_draft",
                "scope_selector_and_ai_panel",
                finding_lifecycle,
            ),
            vec![
                bs("arsc-scope-selector", ReviewScopeSelector, DesktopFull),
                bs("arsc-scope-ai-panel", AiReviewPanel, DesktopFull),
                bs("arsc-scope-pending-tray", PendingReviewTray, RemoteProjected),
            ],
        ),
        spec(
            "publish-to-review-sheet/outbound-publish-mode-and-destination",
            "Publish-to-review sheet (outbound publish mode and provider destination, never implicit)",
            Object::PublishToReviewSheet,
            facets(
                "publish_destination_disclosure",
                "publish_to_review_sheet",
                "publish_to_review_sheet_registry",
                "publish_now_provider_comment",
                "publish_sheet_and_provider_review",
                finding_lifecycle,
            ),
            vec![
                bs("arsc-publish-sheet", PublishToReviewSheet, DesktopFull),
                bs("arsc-publish-provider-review", ProviderPublishReview, DesktopFull),
                bs("arsc-publish-support", SupportExportPacket, ExportedRedacted),
            ],
        ),
        spec(
            "resolution-memory-row/durable-finding-history",
            "Resolution memory row (durable dismissed / published / outdated / suppressed history)",
            Object::ResolutionMemoryRow,
            facets(
                "resolution_memory_disclosure",
                "resolution_memory_row",
                "resolution_memory_registry",
                "export_fallback_offline",
                "resolution_ledger_and_review_detail",
                ai_review_scoped_descriptor,
            ),
            vec![
                bs("arsc-resolution-ledger", ResolutionMemoryLedger, DesktopFull),
                bs("arsc-resolution-review-detail", ReviewDetail, CompactNarrowed),
                bs("arsc-resolution-support", SupportExportPacket, ExportedRedacted),
            ],
        ),
    ]
}

/// Builds all consumer bindings, applying `rep` to override a binding's representation.
fn build_bindings<F>(rep: F) -> Vec<AiReviewConsumerBinding>
where
    F: Fn(&str, AiReviewRepresentation) -> AiReviewRepresentation,
{
    let mut bindings = Vec::new();
    for finding in finding_specs() {
        for spec in &finding.bindings {
            let representation = rep(spec.binding_id, spec.representation);
            bindings.push(make_binding(
                spec.binding_id,
                finding.finding_id,
                finding.finding_label,
                finding.object,
                spec.consumer,
                representation,
                finding.facets.clone(),
            ));
        }
    }
    bindings
}

fn trust_review() -> M5AiReviewSharedConsumersTrustReview {
    M5AiReviewSharedConsumersTrustReview {
        object_reuse_proven_by_fixtures: true,
        same_finding_same_ai_review_vocabulary_across_surfaces: true,
        ai_review_role_words_stay_in_frozen_vocabulary: true,
        gate_roles_never_publish_or_merge_implicitly: true,
        output_destination_class_never_hidden: true,
        stale_findings_never_shown_as_current: true,
        local_drafts_and_evidence_never_lost: true,
        finding_never_shown_without_scope_destination_and_lifecycle: true,
        narrowing_disclosed_across_representations: true,
        support_export_point_canonical_contracts: true,
        copy_export_open_provider_preserve_one_payload: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> M5AiReviewSharedConsumersProjection {
    M5AiReviewSharedConsumersProjection {
        review_detail_consumes_shared_ai_review_vocabulary: true,
        ai_review_panel_consumes_shared_ai_review_vocabulary: true,
        finding_row_consumes_shared_ai_review_vocabulary: true,
        review_scope_selector_consumes_shared_ai_review_vocabulary: true,
        publish_to_review_sheet_consumes_shared_ai_review_vocabulary: true,
        pending_review_tray_consumes_shared_ai_review_vocabulary: true,
        provider_publish_review_consumes_shared_ai_review_vocabulary: true,
        resolution_memory_ledger_consumes_shared_ai_review_vocabulary: true,
        support_export_packet_consumes_shared_ai_review_vocabulary: true,
        every_object_adopted_by_two_or_more_consumers: true,
        ai_review_vocabulary_identical_for_same_finding: true,
        narrowing_disclosed_not_hidden: true,
        export_maps_back_to_one_ai_review_object: true,
    }
}

fn proof_freshness() -> M5AiReviewSharedConsumersProofFreshness {
    M5AiReviewSharedConsumersProofFreshness {
        proof_freshness_slo_hours: M5_AI_REVIEW_SHARED_CONSUMERS_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_AI_REVIEW_SHARED_CONSUMERS_SCHEMA_REF.to_owned(),
        M5_AI_REVIEW_SHARED_CONSUMERS_DOC_REF.to_owned(),
        M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_REF.to_owned(),
        M5_AI_REVIEW_ASSIST_MATRIX_DOC_REF.to_owned(),
    ];
    // The four objects each map to their own canonical domain schema; include each distinct one once.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object in M5AiReviewAssistObject::ALL {
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
    consumer_bindings: Vec<AiReviewConsumerBinding>,
) -> M5AiReviewSharedConsumersPacket {
    M5AiReviewSharedConsumersPacket::new(M5AiReviewSharedConsumersPacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        consumer_bindings,
        downgrade_triggers: AiReviewSharedConsumersDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5AiReviewAssistConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in AI-review shared-consumer parity packet.
pub fn seeded_m5_ai_review_shared_consumers() -> M5AiReviewSharedConsumersPacket {
    packet_from_bindings(
        M5_AI_REVIEW_SHARED_CONSUMERS_PACKET_ID,
        "M5 AI-review shared consumers (one vocabulary across surfaces)",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same findings with two more desktop surfaces narrowed to compact and remote
/// representations, proving vocabulary survives compact and remote forms.
pub fn seeded_m5_ai_review_shared_consumers_compact_remote_narrowed(
) -> M5AiReviewSharedConsumersPacket {
    packet_from_bindings(
        "m5-ai-review-shared-consumers:compact-remote:0001",
        "M5 AI-review shared consumers (compact / remote narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "arsc-finding-finding-row" => AiReviewRepresentation::CompactNarrowed,
            "arsc-publish-provider-review" => AiReviewRepresentation::RemoteProjected,
            _ => default,
        }),
    )
}

/// Fixture: the same findings with two more surfaces narrowed to exported, export-safe
/// representations, proving vocabulary survives into exported forms.
pub fn seeded_m5_ai_review_shared_consumers_exported_redaction_narrowed(
) -> M5AiReviewSharedConsumersPacket {
    packet_from_bindings(
        "m5-ai-review-shared-consumers:exported-redaction:0001",
        "M5 AI-review shared consumers (exported redaction narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "arsc-scope-ai-panel" => AiReviewRepresentation::ExportedRedacted,
            "arsc-resolution-ledger" => AiReviewRepresentation::ExportedRedacted,
            _ => default,
        }),
    )
}
