//! Canonical seed for the review-pack shared-consumer parity packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix CSV,
//! Markdown summary, and narrowed fixtures. Every binding is derived from one per-subject
//! [`ReviewPackSharedStateFacetValues`] so the same seeded review-pack subject always carries the same
//! vocabulary across surfaces, and every narrowed representation derives its disclosure from
//! [`resolve_review_pack_shared_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-16T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

fn facets(
    review_pack_role: &str,
    object: &str,
    registry_reference: &str,
    parity_state: &str,
    surface_context: &str,
    pack_freshness: &str,
) -> ReviewPackSharedStateFacetValues {
    ReviewPackSharedStateFacetValues {
        review_pack_role_word: review_pack_role.to_owned(),
        object_word: object.to_owned(),
        registry_reference_word: registry_reference.to_owned(),
        parity_state_word: parity_state.to_owned(),
        surface_context_word: surface_context.to_owned(),
        pack_freshness_word: pack_freshness.to_owned(),
    }
}

fn preserved_note_for(reason: ReviewPackSharedNarrowReason) -> String {
    match reason {
        ReviewPackSharedNarrowReason::CompactionNarrowed => {
            "review-pack-role, object, registry-reference, parity-state, surface-context, and pack-freshness words preserved; only disclosure depth compacted"
        }
        ReviewPackSharedNarrowReason::RemoteProjectionNarrowed => {
            "all review-pack vocabulary preserved; the object is projected from the remote source of truth"
        }
        ReviewPackSharedNarrowReason::ExportRedactionNarrowed => {
            "all review-pack vocabulary preserved; only surrounding detail is redacted export-safe"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: ReviewPackSharedNarrowNextAction) -> String {
    match action {
        ReviewPackSharedNarrowNextAction::ExpandInDesktop => "Expand in the desktop surface",
        ReviewPackSharedNarrowNextAction::OpenRemoteSource => "Open the remote source",
        ReviewPackSharedNarrowNextAction::OpenFullDetail => "Open the full detail",
    }
    .to_owned()
}

fn binding_refs(object: M5ReviewPackObject) -> Vec<String> {
    vec![
        M5_REVIEW_PACK_MATRIX_SCHEMA_REF.to_owned(),
        object.canonical_domain_schema_ref().to_owned(),
    ]
}

fn make_binding(
    binding_id: &str,
    subject_id: &str,
    subject_label: &str,
    object: M5ReviewPackObject,
    consumer: M5ReviewPackConsumerSurface,
    representation: ReviewPackSharedRepresentation,
    state_facets: ReviewPackSharedStateFacetValues,
) -> ReviewPackSharedConsumerBinding {
    let disclosure = resolve_review_pack_shared_render_disclosure(representation);
    let narrow_note = disclosure.narrow_reason.map(|reason| {
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrow reason always carries a next action");
        ReviewPackSharedNarrowNote {
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

    ReviewPackSharedConsumerBinding {
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
        lets_a_local_parity_estimate_masquerade_as_provider_authoritative: false,
        hides_ci_only_not_evaluated_here_or_provider_unavailable_behind_a_green_summary: false,
        flattens_advisory_owner_and_enforced_owner_into_one_owner_pill: false,
        lets_ai_review_run_under_a_different_pack_version_without_disclosure: false,
        loses_review_pack_version_digest_or_template_attribution_when_exporting_publishing_or_reopening:
            false,
        source_contract_refs: binding_refs(object),
    }
}

/// One consumer-surface adoption of a seeded subject, before any representation override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5ReviewPackConsumerSurface,
    representation: ReviewPackSharedRepresentation,
}

/// One seeded review-pack subject rendered across several consumer surfaces at one vocabulary.
struct SubjectSpec {
    subject_id: &'static str,
    subject_label: &'static str,
    object: M5ReviewPackObject,
    facets: ReviewPackSharedStateFacetValues,
    bindings: Vec<BindingSpec>,
}

fn spec(
    subject_id: &'static str,
    subject_label: &'static str,
    object: M5ReviewPackObject,
    facets: ReviewPackSharedStateFacetValues,
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
    consumer: M5ReviewPackConsumerSurface,
    representation: ReviewPackSharedRepresentation,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        representation,
    }
}

/// The six seeded review-pack subjects — one per B152 review-pack evaluator object — and the surfaces that
/// adopt each, drawn from the review-detail, merge-readiness, AI-review-panel, provider-handoff,
/// review-pack-summary, ownership-overlay, local-CI-parity-strip, support-export, and help / docs consumers
/// that back the review-list / detail, merge-readiness / merge-queue, AI-review, support / export, and browser
/// / provider-handoff surfaces.
fn subject_specs() -> Vec<SubjectSpec> {
    use M5ReviewPackConsumerSurface::*;
    use M5ReviewPackObject as Object;
    use ReviewPackSharedRepresentation::*;

    let pack_fresh = "pack_fresh_scope_bound_and_parity_disclosed";
    let review_pack_scoped_descriptor = "review_pack_scoped_descriptor";

    vec![
        spec(
            "review-pack-record/one-repo-defined-pack",
            "Review-pack record (one repo-defined pack: version / digest, scope selectors, evaluator identity)",
            Object::ReviewPackRecord,
            facets(
                "pack_version_and_digest_disclosure",
                "review_pack_record",
                "review_pack_registry",
                "local_parity_estimate",
                "review_detail_and_review_pack_summary",
                pack_fresh,
            ),
            vec![
                bs("rpsc-record-review-detail", ReviewDetail, DesktopFull),
                bs("rpsc-record-summary", ReviewPackSummary, DesktopFull),
                bs("rpsc-record-support", SupportExportPacket, ExportedRedacted),
            ],
        ),
        spec(
            "ownership-signal/advisory-versus-enforced-owner",
            "Ownership signal (advisory-owner-versus-enforced-owner provenance for a scope slice)",
            Object::OwnershipSignal,
            facets(
                "owner_provenance_disclosure",
                "ownership_signal",
                "ownership_signal_registry",
                "provider_authoritative",
                "ownership_overlay_and_review_detail",
                pack_fresh,
            ),
            vec![
                bs("rpsc-ownership-overlay", OwnershipOverlay, DesktopFull),
                bs("rpsc-ownership-review-detail", ReviewDetail, CompactNarrowed),
                bs("rpsc-ownership-support", SupportExportPacket, ExportedRedacted),
            ],
        ),
        spec(
            "required-evidence-check-row/one-required-check",
            "Required-evidence / required-check row (one required evidence or check plus its evaluator result class)",
            Object::RequiredEvidenceCheckRow,
            facets(
                "required_evidence_and_check_disclosure",
                "required_evidence_check_row",
                "required_evidence_check_registry",
                "ci_only",
                "merge_readiness_and_review_detail",
                review_pack_scoped_descriptor,
            ),
            vec![
                bs("rpsc-evidence-merge-readiness", MergeReadiness, DesktopFull),
                bs("rpsc-evidence-review-detail", ReviewDetail, DesktopFull),
                bs("rpsc-evidence-support", SupportExportPacket, ExportedRedacted),
            ],
        ),
        spec(
            "local-ci-parity-strip/local-versus-provider-parity",
            "Local-CI parity strip (local-parity-estimate-versus-provider-authoritative state per check)",
            Object::LocalCiParityStrip,
            facets(
                "local_versus_provider_parity_disclosure",
                "local_ci_parity_strip",
                "local_ci_parity_registry",
                "local_parity_estimate",
                "local_ci_parity_strip_and_provider_handoff",
                pack_fresh,
            ),
            vec![
                bs("rpsc-parity-strip", LocalCiParityStrip, DesktopFull),
                bs("rpsc-parity-merge-readiness", MergeReadiness, RemoteProjected),
                bs("rpsc-parity-provider-handoff", ProviderHandoff, DesktopFull),
            ],
        ),
        spec(
            "ai-policy-hook/pack-version-bound-ai-review",
            "AI review policy hook (an AI review run under a disclosed review-pack version / digest and policy)",
            Object::AiPolicyHook,
            facets(
                "evaluator_result_class_disclosure",
                "ai_policy_hook",
                "ai_policy_hook_registry",
                "not_evaluated_here",
                "ai_review_panel_and_provider_handoff",
                pack_fresh,
            ),
            vec![
                bs("rpsc-aihook-ai-panel", AiReviewPanel, DesktopFull),
                bs("rpsc-aihook-review-detail", ReviewDetail, DesktopFull),
                bs("rpsc-aihook-provider-handoff", ProviderHandoff, RemoteProjected),
            ],
        ),
        spec(
            "review-template-packet/comment-summary-template",
            "Review-template packet (comment / summary template and attribution bound to the pack it came from)",
            Object::ReviewTemplatePacket,
            facets(
                "template_attribution_disclosure",
                "review_template_packet",
                "review_template_packet_registry",
                "draft_only_review_state",
                "review_pack_summary_and_help_docs",
                review_pack_scoped_descriptor,
            ),
            vec![
                bs("rpsc-template-summary", ReviewPackSummary, DesktopFull),
                bs("rpsc-template-help-docs", HelpDocs, CompactNarrowed),
                bs("rpsc-template-support", SupportExportPacket, ExportedRedacted),
            ],
        ),
    ]
}

/// Builds all consumer bindings, applying `rep` to override a binding's representation.
fn build_bindings<F>(rep: F) -> Vec<ReviewPackSharedConsumerBinding>
where
    F: Fn(&str, ReviewPackSharedRepresentation) -> ReviewPackSharedRepresentation,
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

fn trust_review() -> M5ReviewPackSharedConsumersTrustReview {
    M5ReviewPackSharedConsumersTrustReview {
        object_reuse_proven_by_fixtures: true,
        same_subject_same_review_pack_vocabulary_across_surfaces: true,
        review_pack_role_words_stay_in_frozen_vocabulary: true,
        gate_roles_never_let_local_estimate_read_as_provider_authoritative: true,
        ci_only_not_evaluated_here_and_provider_unavailable_never_hidden: true,
        advisory_and_enforced_owner_never_flattened: true,
        ai_review_never_runs_under_a_different_pack_version_without_disclosure: true,
        pack_version_digest_and_template_attribution_never_lost: true,
        narrowing_disclosed_across_representations: true,
        support_export_point_canonical_contracts: true,
        copy_export_open_provider_preserve_one_payload: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> M5ReviewPackSharedConsumersProjection {
    M5ReviewPackSharedConsumersProjection {
        review_detail_consumes_shared_review_pack_vocabulary: true,
        merge_readiness_consumes_shared_review_pack_vocabulary: true,
        ai_review_panel_consumes_shared_review_pack_vocabulary: true,
        provider_handoff_consumes_shared_review_pack_vocabulary: true,
        review_pack_summary_consumes_shared_review_pack_vocabulary: true,
        ownership_overlay_consumes_shared_review_pack_vocabulary: true,
        local_ci_parity_strip_consumes_shared_review_pack_vocabulary: true,
        support_export_packet_consumes_shared_review_pack_vocabulary: true,
        help_docs_consumes_shared_review_pack_vocabulary: true,
        every_object_adopted_by_two_or_more_consumers: true,
        review_pack_vocabulary_identical_for_same_subject: true,
        narrowing_disclosed_not_hidden: true,
        export_maps_back_to_one_review_pack_object: true,
    }
}

fn proof_freshness() -> M5ReviewPackSharedConsumersProofFreshness {
    M5ReviewPackSharedConsumersProofFreshness {
        proof_freshness_slo_hours: M5_REVIEW_PACK_SHARED_CONSUMERS_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_REVIEW_PACK_SHARED_CONSUMERS_SCHEMA_REF.to_owned(),
        M5_REVIEW_PACK_SHARED_CONSUMERS_DOC_REF.to_owned(),
        M5_REVIEW_PACK_MATRIX_SCHEMA_REF.to_owned(),
        M5_REVIEW_PACK_MATRIX_DOC_REF.to_owned(),
    ];
    // The six objects each map to their own canonical domain schema; include each distinct one once.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object in M5ReviewPackObject::ALL {
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
    consumer_bindings: Vec<ReviewPackSharedConsumerBinding>,
) -> M5ReviewPackSharedConsumersPacket {
    M5ReviewPackSharedConsumersPacket::new(M5ReviewPackSharedConsumersPacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        consumer_bindings,
        downgrade_triggers: M5ReviewPackSharedConsumersDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5ReviewPackConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in review-pack shared-consumer parity packet.
pub fn seeded_m5_review_pack_shared_consumers() -> M5ReviewPackSharedConsumersPacket {
    packet_from_bindings(
        M5_REVIEW_PACK_SHARED_CONSUMERS_PACKET_ID,
        "M5 review-pack shared consumers (one vocabulary across surfaces)",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same subjects with two more desktop surfaces narrowed to compact and remote
/// representations, proving vocabulary survives compact and remote forms.
pub fn seeded_m5_review_pack_shared_consumers_compact_remote_narrowed(
) -> M5ReviewPackSharedConsumersPacket {
    packet_from_bindings(
        "m5-review-pack-shared-consumers:compact-remote:0001",
        "M5 review-pack shared consumers (compact / remote narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "rpsc-record-summary" => ReviewPackSharedRepresentation::CompactNarrowed,
            "rpsc-parity-provider-handoff" => ReviewPackSharedRepresentation::RemoteProjected,
            _ => default,
        }),
    )
}

/// Fixture: the same subjects with two more surfaces narrowed to exported, export-safe
/// representations, proving vocabulary survives into exported forms.
pub fn seeded_m5_review_pack_shared_consumers_exported_redaction_narrowed(
) -> M5ReviewPackSharedConsumersPacket {
    packet_from_bindings(
        "m5-review-pack-shared-consumers:exported-redaction:0001",
        "M5 review-pack shared consumers (exported redaction narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "rpsc-record-review-detail" => ReviewPackSharedRepresentation::ExportedRedacted,
            "rpsc-aihook-ai-panel" => ReviewPackSharedRepresentation::ExportedRedacted,
            _ => default,
        }),
    )
}
