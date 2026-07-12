//! Canonical seed for the editor-inline shared-consumer parity packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support
//! export, matrix CSV, Markdown summary, and narrowed fixtures. Every binding is derived
//! from one per-object [`EditorInlineStateFacetValues`] so the same inline object always
//! carries the same vocabulary across surfaces, and every narrowed representation derives
//! its disclosure from [`resolve_editor_inline_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-12T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

fn facets(
    state: &str,
    severity_or_confidence: &str,
    anchor_freshness: &str,
    approval: &str,
    evidence: &str,
) -> EditorInlineStateFacetValues {
    EditorInlineStateFacetValues {
        state_word: state.to_owned(),
        severity_or_confidence_word: severity_or_confidence.to_owned(),
        anchor_freshness_word: anchor_freshness.to_owned(),
        approval_state_word: approval.to_owned(),
        evidence_lineage_word: evidence.to_owned(),
    }
}

fn preserved_note_for(reason: EditorInlineNarrowReason) -> String {
    match reason {
        EditorInlineNarrowReason::CompactionNarrowed => {
            "state, severity/confidence, anchor, approval, and evidence words preserved; only disclosure depth compacted"
        }
        EditorInlineNarrowReason::RemoteProjectionNarrowed => {
            "all inline vocabulary preserved; the state is projected from the remote source of truth"
        }
        EditorInlineNarrowReason::ExportRedactionNarrowed => {
            "all inline vocabulary preserved; only evidence detail is redacted export-safe"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: EditorInlineNarrowNextAction) -> String {
    match action {
        EditorInlineNarrowNextAction::ExpandInDesktop => "Expand in the desktop editor",
        EditorInlineNarrowNextAction::OpenRemoteSource => "Open the remote source",
        EditorInlineNarrowNextAction::OpenFullEvidence => "Open the full evidence",
    }
    .to_owned()
}

fn binding_refs(component: M5EditorInlineComponentFamily) -> Vec<String> {
    vec![
        M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF.to_owned(),
        component.canonical_component_schema_ref().to_owned(),
    ]
}

#[allow(clippy::too_many_arguments)]
fn make_binding(
    binding_id: &str,
    inline_object_id: &str,
    inline_object_label: &str,
    component: M5EditorInlineComponentFamily,
    consumer: M5EditorInlineConsumerSurface,
    representation: EditorInlineRepresentation,
    state_facets: EditorInlineStateFacetValues,
) -> EditorInlineConsumerBinding {
    let disclosure = resolve_editor_inline_render_disclosure(representation);
    let narrow_note = disclosure.narrow_reason.map(|reason| {
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrow reason always carries a next action");
        EditorInlineNarrowNote {
            reason,
            preserved_vocabulary_note: preserved_note_for(reason),
            next_action,
            next_action_label: next_action_label_for(next_action),
        }
    });
    let remote_source_note = if disclosure.needs_remote_source_note {
        "projected from the remote review host; the source of truth stays remote".to_owned()
    } else {
        String::new()
    };
    let export_evidence_note = if disclosure.needs_export_evidence_note {
        "evidence redacted export-safe in this packet; full evidence available on request"
            .to_owned()
    } else {
        String::new()
    };

    EditorInlineConsumerBinding {
        binding_id: binding_id.to_owned(),
        inline_object_id: inline_object_id.to_owned(),
        inline_object_label: inline_object_label.to_owned(),
        component,
        consumer,
        representation,
        state_facets,
        parity_state: disclosure.parity_state,
        narrow_note,
        remote_source_note,
        export_evidence_note,
        encodes_state_by_color_alone: false,
        lets_anchor_or_evidence_pointer_silently_drift: false,
        blurs_outdated_and_resolved_review_state: false,
        presents_inferred_fix_as_exact: false,
        hides_evidence_in_opaque_log: false,
        rewords_inline_vocabulary_per_surface: false,
        source_contract_refs: binding_refs(component),
    }
}

/// One consumer-surface adoption of an inline object, before any representation override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5EditorInlineConsumerSurface,
    representation: EditorInlineRepresentation,
}

/// One inline object rendered across several consumer surfaces at one vocabulary.
struct ObjectSpec {
    object_id: &'static str,
    object_label: &'static str,
    component: M5EditorInlineComponentFamily,
    facets: EditorInlineStateFacetValues,
    bindings: Vec<BindingSpec>,
}

fn spec(
    object_id: &'static str,
    object_label: &'static str,
    component: M5EditorInlineComponentFamily,
    facets: EditorInlineStateFacetValues,
    bindings: Vec<BindingSpec>,
) -> ObjectSpec {
    ObjectSpec {
        object_id,
        object_label,
        component,
        facets,
        bindings,
    }
}

fn bs(
    binding_id: &'static str,
    consumer: M5EditorInlineConsumerSurface,
    representation: EditorInlineRepresentation,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        representation,
    }
}

/// The eight inline objects — one per B133 component — and the surfaces that adopt each.
fn object_specs() -> Vec<ObjectSpec> {
    use EditorInlineRepresentation::*;
    use M5EditorInlineComponentFamily::*;
    use M5EditorInlineConsumerSurface::*;

    vec![
        spec(
            "tab:src/main.rs",
            "src/main.rs (editor tab)",
            EditorTab,
            facets(
                "modified",
                "not_applicable",
                "anchored_exact",
                "not_applicable",
                "not_applicable",
            ),
            vec![
                bs("eb-tab-editor", EditorUi, DesktopFull),
                bs("eb-tab-support", SupportExport, ExportedRedacted),
            ],
        ),
        spec(
            "gutter:src/main.rs:42",
            "src/main.rs:42 gutter marker",
            Gutter,
            facets(
                "modified",
                "warning",
                "anchored_exact",
                "not_applicable",
                "not_applicable",
            ),
            vec![
                bs("eb-gutter-editor", EditorUi, DesktopFull),
                bs("eb-gutter-diff", DiffUi, CompactNarrowed),
            ],
        ),
        spec(
            "diag:src/main.rs:88",
            "src/main.rs:88 diagnostic",
            DiagnosticDecoration,
            facets(
                "outdated",
                "error",
                "drifted_approximate",
                "not_applicable",
                "not_applicable",
            ),
            vec![
                bs("eb-diag-diagnostics", DiagnosticsUi, DesktopFull),
                bs("eb-diag-editor", EditorUi, CompactNarrowed),
                bs("eb-diag-support", SupportExport, ExportedRedacted),
            ],
        ),
        spec(
            "chip:src/main.rs:88",
            "src/main.rs:88 code action",
            CodeActionChip,
            facets(
                "inferred_fix",
                "not_applicable",
                "anchored_exact",
                "review_required",
                "not_applicable",
            ),
            vec![
                bs("eb-chip-editor", EditorUi, DesktopFull),
                bs("eb-chip-diagnostics", DiagnosticsUi, CompactNarrowed),
            ],
        ),
        spec(
            "diff:pr-12/src/main.rs",
            "pr-12 src/main.rs diff",
            DiffView,
            facets(
                "modified",
                "not_applicable",
                "anchored_exact",
                "not_applicable",
                "not_applicable",
            ),
            vec![
                bs("eb-diff-diff", DiffUi, DesktopFull),
                bs("eb-diff-review", ReviewUi, RemoteProjected),
                bs("eb-diff-cli", CliExport, ExportedRedacted),
            ],
        ),
        spec(
            "thread:pr-12/c-3",
            "pr-12 comment 3 thread",
            ReviewThread,
            facets(
                "resolved",
                "not_applicable",
                "re_anchored",
                "resolved",
                "not_applicable",
            ),
            vec![
                bs("eb-thread-review", ReviewUi, DesktopFull),
                bs("eb-thread-support", SupportExport, ExportedRedacted),
                bs("eb-thread-product", ProductUi, RemoteProjected),
            ],
        ),
        spec(
            "card:chat-9/m-2",
            "chat-9 message 2 card",
            AiMessageCard,
            facets(
                "review_required",
                "grounded_high",
                "not_applicable",
                "review_required",
                "export_safe_evidence",
            ),
            vec![
                bs("eb-card-ai", AiUi, DesktopFull),
                bs("eb-card-notebook", NotebookUi, CompactNarrowed),
                bs("eb-card-support", SupportExport, ExportedRedacted),
            ],
        ),
        spec(
            "evidence:chat-9/m-2",
            "chat-9 message 2 evidence",
            EvidenceTimeline,
            facets(
                "export_safe_evidence",
                "not_applicable",
                "anchored_exact",
                "not_applicable",
                "export_safe_evidence",
            ),
            vec![
                bs("eb-evid-ai", AiUi, DesktopFull),
                bs("eb-evid-notebook", NotebookUi, CompactNarrowed),
                bs("eb-evid-cli", CliExport, ExportedRedacted),
            ],
        ),
    ]
}

/// Builds all consumer bindings, applying `rep` to override a binding's representation.
fn build_bindings<F>(rep: F) -> Vec<EditorInlineConsumerBinding>
where
    F: Fn(&str, EditorInlineRepresentation) -> EditorInlineRepresentation,
{
    let mut bindings = Vec::new();
    for object in object_specs() {
        for spec in &object.bindings {
            let representation = rep(spec.binding_id, spec.representation);
            bindings.push(make_binding(
                spec.binding_id,
                object.object_id,
                object.object_label,
                object.component,
                spec.consumer,
                representation,
                object.facets.clone(),
            ));
        }
    }
    bindings
}

fn trust_review() -> EditorInlineSharedConsumersTrustReview {
    EditorInlineSharedConsumersTrustReview {
        component_reuse_proven_by_fixtures: true,
        same_object_same_vocabulary_across_surfaces: true,
        state_words_stay_in_frozen_vocabulary: true,
        state_never_encoded_by_color_alone: true,
        anchors_and_evidence_never_silently_drift: true,
        outdated_and_resolved_stay_distinct: true,
        inferred_fix_never_shown_as_exact: true,
        evidence_keeps_inspectable_structure: true,
        narrowing_disclosed_across_representations: true,
        support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> EditorInlineSharedConsumersProjection {
    EditorInlineSharedConsumersProjection {
        editor_ui_reuses_shared_components: true,
        diff_ui_reuses_shared_components: true,
        review_ui_reuses_shared_components: true,
        notebook_ui_reuses_shared_components: true,
        ai_ui_reuses_shared_components: true,
        diagnostics_ui_reuses_shared_components: true,
        support_export_reuses_shared_components: true,
        every_component_adopted_by_two_or_more_consumers: true,
        vocabulary_identical_for_same_object: true,
        narrowing_disclosed_not_hidden: true,
        export_maps_back_to_one_contract_family: true,
    }
}

fn proof_freshness() -> EditorInlineSharedConsumersProofFreshness {
    EditorInlineSharedConsumersProofFreshness {
        proof_freshness_slo_hours: M5_EDITOR_INLINE_SHARED_CONSUMERS_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_EDITOR_INLINE_SHARED_CONSUMERS_SCHEMA_REF.to_owned(),
        M5_EDITOR_INLINE_SHARED_CONSUMERS_DOC_REF.to_owned(),
        M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF.to_owned(),
        M5_EDITOR_INLINE_COMPONENT_DOC_REF.to_owned(),
    ];
    for family in M5EditorInlineComponentFamily::ALL {
        refs.push(family.canonical_component_schema_ref().to_owned());
    }
    refs
}

fn packet_from_bindings(
    packet_id: &str,
    surface_label: &str,
    consumer_bindings: Vec<EditorInlineConsumerBinding>,
) -> M5EditorInlineSharedConsumersPacket {
    M5EditorInlineSharedConsumersPacket::new(M5EditorInlineSharedConsumersPacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        consumer_bindings,
        downgrade_triggers: EditorInlineSharedConsumersDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5EditorInlineConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in editor-inline shared-consumer parity packet.
pub fn seeded_m5_editor_inline_shared_consumers() -> M5EditorInlineSharedConsumersPacket {
    packet_from_bindings(
        M5_EDITOR_INLINE_SHARED_CONSUMERS_PACKET_ID,
        "M5 editor-inline shared consumers",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same objects with two more surfaces narrowed to compact and remote
/// representations, proving state changes propagate across compact and remote forms.
pub fn seeded_m5_editor_inline_shared_consumers_compact_remote_narrowed(
) -> M5EditorInlineSharedConsumersPacket {
    packet_from_bindings(
        "m5-editor-inline-shared-consumers:compact-remote:0001",
        "M5 editor-inline shared consumers (compact / remote narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "eb-diff-diff" => EditorInlineRepresentation::RemoteProjected,
            "eb-thread-review" => EditorInlineRepresentation::CompactNarrowed,
            _ => default,
        }),
    )
}

/// Fixture: the same objects with two AI surfaces narrowed to exported, export-safe
/// representations, proving state changes propagate into exported forms.
pub fn seeded_m5_editor_inline_shared_consumers_exported_redaction_narrowed(
) -> M5EditorInlineSharedConsumersPacket {
    packet_from_bindings(
        "m5-editor-inline-shared-consumers:exported-redaction:0001",
        "M5 editor-inline shared consumers (exported redaction narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "eb-card-ai" => EditorInlineRepresentation::ExportedRedacted,
            "eb-evid-ai" => EditorInlineRepresentation::ExportedRedacted,
            _ => default,
        }),
    )
}
