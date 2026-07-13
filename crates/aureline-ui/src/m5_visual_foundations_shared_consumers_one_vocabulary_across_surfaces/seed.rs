//! Canonical seed for the visual-foundation shared-consumer parity packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix
//! CSV, Markdown summary, and narrowed fixtures. Every binding is derived from one per-object
//! [`VisualFoundationStateFacetValues`] so the same foundation object always carries the same
//! vocabulary across surfaces, and every narrowed representation derives its disclosure from
//! [`resolve_visual_foundation_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

fn facets(
    semantic_role: &str,
    family: &str,
    token_reference: &str,
    theme_variant: &str,
    density_context: &str,
    non_color_cue: &str,
) -> VisualFoundationStateFacetValues {
    VisualFoundationStateFacetValues {
        semantic_role_word: semantic_role.to_owned(),
        family_word: family.to_owned(),
        token_reference_word: token_reference.to_owned(),
        theme_variant_word: theme_variant.to_owned(),
        density_context_word: density_context.to_owned(),
        non_color_cue_word: non_color_cue.to_owned(),
    }
}

fn preserved_note_for(reason: VisualFoundationNarrowReason) -> String {
    match reason {
        VisualFoundationNarrowReason::CompactionNarrowed => {
            "semantic-role, family, token-reference, theme-variant, density-context, and non-color-cue words preserved; only disclosure depth compacted"
        }
        VisualFoundationNarrowReason::RemoteProjectionNarrowed => {
            "all visual-foundation vocabulary preserved; the family is projected from the remote source of truth"
        }
        VisualFoundationNarrowReason::ExportRedactionNarrowed => {
            "all visual-foundation vocabulary preserved; only surrounding detail is redacted export-safe"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: VisualFoundationNarrowNextAction) -> String {
    match action {
        VisualFoundationNarrowNextAction::ExpandInDesktop => "Expand in the desktop surface",
        VisualFoundationNarrowNextAction::OpenRemoteSource => "Open the remote source",
        VisualFoundationNarrowNextAction::OpenFullDetail => "Open the full detail",
    }
    .to_owned()
}

fn binding_refs(family: M5VisualFoundationFamily) -> Vec<String> {
    vec![
        M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF.to_owned(),
        family.canonical_domain_schema_ref().to_owned(),
    ]
}

fn make_binding(
    binding_id: &str,
    foundation_object_id: &str,
    foundation_object_label: &str,
    family: M5VisualFoundationFamily,
    consumer: M5VisualFoundationConsumerSurface,
    representation: VisualFoundationRepresentation,
    state_facets: VisualFoundationStateFacetValues,
) -> VisualFoundationConsumerBinding {
    let disclosure = resolve_visual_foundation_render_disclosure(representation);
    let narrow_note = disclosure.narrow_reason.map(|reason| {
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrow reason always carries a next action");
        VisualFoundationNarrowNote {
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

    VisualFoundationConsumerBinding {
        binding_id: binding_id.to_owned(),
        foundation_object_id: foundation_object_id.to_owned(),
        foundation_object_label: foundation_object_label.to_owned(),
        family,
        consumer,
        representation,
        state_facets,
        parity_state: disclosure.parity_state,
        narrow_note,
        remote_source_note,
        export_detail_note,
        relies_on_hue_alone_for_meaning: false,
        lets_syntax_or_diff_palette_collide_with_diagnostics: false,
        shrinks_hit_target_below_supported_minimum: false,
        lets_chart_meaning_depend_on_color_alone: false,
        forks_local_spacing_or_elevation_from_shared_geometry: false,
        source_contract_refs: binding_refs(family),
    }
}

/// One consumer-surface adoption of a foundation object, before any representation override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5VisualFoundationConsumerSurface,
    representation: VisualFoundationRepresentation,
}

/// One foundation object rendered across several consumer surfaces at one vocabulary.
struct ObjectSpec {
    object_id: &'static str,
    object_label: &'static str,
    family: M5VisualFoundationFamily,
    facets: VisualFoundationStateFacetValues,
    bindings: Vec<BindingSpec>,
}

fn spec(
    object_id: &'static str,
    object_label: &'static str,
    family: M5VisualFoundationFamily,
    facets: VisualFoundationStateFacetValues,
    bindings: Vec<BindingSpec>,
) -> ObjectSpec {
    ObjectSpec {
        object_id,
        object_label,
        family,
        facets,
        bindings,
    }
}

fn bs(
    binding_id: &'static str,
    consumer: M5VisualFoundationConsumerSurface,
    representation: VisualFoundationRepresentation,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        representation,
    }
}

/// The eight foundation objects — one per B136 visual-foundation family — and the surfaces that
/// adopt each, drawn from the first claimed shell, editor, review, data, docs, settings, and
/// export/support consumers.
fn object_specs() -> Vec<ObjectSpec> {
    use M5VisualFoundationConsumerSurface::*;
    use M5VisualFoundationFamily::*;
    use VisualFoundationRepresentation::*;

    vec![
        spec(
            "color-system/status-palette",
            "Status color-system palette",
            ColorSystem,
            facets(
                "status",
                "color_system",
                "color_system_domain",
                "dark_light_high_contrast",
                "density_aware",
                "status_icon_and_label",
            ),
            vec![
                bs("vf-color-shell", ShellUi, DesktopFull),
                bs("vf-color-settings", SettingsUi, DesktopFull),
                bs("vf-color-support", SupportExport, ExportedRedacted),
            ],
        ),
        spec(
            "semantic-theme-token/surface-role",
            "Semantic theme surface-role token",
            SemanticThemeToken,
            facets(
                "neutral",
                "semantic_theme_token",
                "color_system_domain",
                "dark_light_high_contrast",
                "density_aware",
                "token_name_reference",
            ),
            vec![
                bs("vf-theme-shell", ShellUi, DesktopFull),
                bs("vf-theme-editor", EditorUi, DesktopFull),
            ],
        ),
        spec(
            "syntax-token/keyword-scope",
            "Syntax keyword-scope token",
            SyntaxToken,
            facets(
                "syntax",
                "syntax_token",
                "syntax_diff_chart_domain",
                "dark_light_high_contrast",
                "density_aware",
                "font_style_and_scope_label",
            ),
            vec![
                bs("vf-syntax-editor", EditorUi, DesktopFull),
                bs("vf-syntax-review", ReviewUi, CompactNarrowed),
                bs("vf-syntax-cli", CliExport, ExportedRedacted),
            ],
        ),
        spec(
            "diff-token/addition-region",
            "Diff addition-region token",
            DiffToken,
            facets(
                "diff",
                "diff_token",
                "syntax_diff_chart_domain",
                "dark_light_high_contrast",
                "density_aware",
                "gutter_marker_and_sign",
            ),
            vec![
                bs("vf-diff-review", ReviewUi, DesktopFull),
                bs("vf-diff-editor", EditorUi, DesktopFull),
            ],
        ),
        spec(
            "chart-token/categorical-series",
            "Chart categorical-series token",
            ChartToken,
            facets(
                "chart",
                "chart_token",
                "syntax_diff_chart_domain",
                "dark_light_high_contrast",
                "density_aware",
                "shape_and_legend",
            ),
            vec![
                bs("vf-chart-data", DataUi, DesktopFull),
                bs("vf-chart-docs", DocsUi, DesktopFull),
            ],
        ),
        spec(
            "typography/body-scale",
            "Typography body-scale token",
            Typography,
            facets(
                "neutral",
                "typography",
                "typography_geometry_domain",
                "dark_light_high_contrast",
                "density_aware",
                "type_scale_step",
            ),
            vec![
                bs("vf-type-docs", DocsUi, DesktopFull),
                bs("vf-type-editor", EditorUi, DesktopFull),
                bs("vf-type-product", ProductUi, DesktopFull),
            ],
        ),
        spec(
            "geometry/spacing-scale",
            "Spacing-scale geometry step",
            SpacingSizingRadiiElevation,
            facets(
                "neutral",
                "spacing_sizing_radii_elevation",
                "typography_geometry_domain",
                "dark_light_high_contrast",
                "density_aware",
                "density_token_step",
            ),
            vec![
                bs("vf-geometry-shell", ShellUi, DesktopFull),
                bs("vf-geometry-data", DataUi, RemoteProjected),
            ],
        ),
        spec(
            "hit-target/minimum-control",
            "Minimum hit-target control",
            HitTarget,
            facets(
                "interactive",
                "hit_target",
                "typography_geometry_domain",
                "dark_light_high_contrast",
                "density_aware",
                "focus_ring_and_min_size",
            ),
            vec![
                bs("vf-hittarget-settings", SettingsUi, DesktopFull),
                bs("vf-hittarget-product", ProductUi, DesktopFull),
                bs("vf-hittarget-support", SupportExport, ExportedRedacted),
            ],
        ),
    ]
}

/// Builds all consumer bindings, applying `rep` to override a binding's representation.
fn build_bindings<F>(rep: F) -> Vec<VisualFoundationConsumerBinding>
where
    F: Fn(&str, VisualFoundationRepresentation) -> VisualFoundationRepresentation,
{
    let mut bindings = Vec::new();
    for object in object_specs() {
        for spec in &object.bindings {
            let representation = rep(spec.binding_id, spec.representation);
            bindings.push(make_binding(
                spec.binding_id,
                object.object_id,
                object.object_label,
                object.family,
                spec.consumer,
                representation,
                object.facets.clone(),
            ));
        }
    }
    bindings
}

fn trust_review() -> VisualFoundationSharedConsumersTrustReview {
    VisualFoundationSharedConsumersTrustReview {
        family_reuse_proven_by_fixtures: true,
        same_object_same_vocabulary_across_surfaces: true,
        semantic_role_words_stay_in_frozen_vocabulary: true,
        meaning_never_relies_on_hue_alone: true,
        syntax_diff_never_collide_with_diagnostics: true,
        chart_meaning_never_depends_on_color_alone: true,
        hit_targets_never_shrink_below_minimum: true,
        geometry_never_forks_from_shared_foundation: true,
        typography_and_geometry_stay_density_aware: true,
        narrowing_disclosed_across_representations: true,
        support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> VisualFoundationSharedConsumersProjection {
    VisualFoundationSharedConsumersProjection {
        shell_ui_consumes_shared_foundation: true,
        editor_ui_consumes_shared_foundation: true,
        review_ui_consumes_shared_foundation: true,
        data_ui_consumes_shared_foundation: true,
        docs_ui_consumes_shared_foundation: true,
        settings_ui_consumes_shared_foundation: true,
        support_export_consumes_shared_foundation: true,
        every_family_adopted_by_two_or_more_consumers: true,
        vocabulary_identical_for_same_object: true,
        narrowing_disclosed_not_hidden: true,
        export_maps_back_to_one_foundation_family: true,
    }
}

fn proof_freshness() -> VisualFoundationSharedConsumersProofFreshness {
    VisualFoundationSharedConsumersProofFreshness {
        proof_freshness_slo_hours: M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_SCHEMA_REF.to_owned(),
        M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_DOC_REF.to_owned(),
        M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF.to_owned(),
        M5_VISUAL_FOUNDATION_MATRIX_DOC_REF.to_owned(),
    ];
    // The eight families map to three canonical domain schemas; include each distinct one once.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for family in M5VisualFoundationFamily::ALL {
        domains.insert(family.canonical_domain_schema_ref());
    }
    for domain in domains {
        refs.push(domain.to_owned());
    }
    refs
}

fn packet_from_bindings(
    packet_id: &str,
    surface_label: &str,
    consumer_bindings: Vec<VisualFoundationConsumerBinding>,
) -> M5VisualFoundationSharedConsumersPacket {
    M5VisualFoundationSharedConsumersPacket::new(M5VisualFoundationSharedConsumersPacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        consumer_bindings,
        downgrade_triggers: VisualFoundationSharedConsumersDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5VisualFoundationConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in visual-foundation shared-consumer parity packet.
pub fn seeded_m5_visual_foundations_shared_consumers() -> M5VisualFoundationSharedConsumersPacket {
    packet_from_bindings(
        M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_PACKET_ID,
        "M5 visual-foundation shared consumers",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same objects with two more desktop surfaces narrowed to compact and remote
/// representations, proving vocabulary survives compact and remote forms.
pub fn seeded_m5_visual_foundations_shared_consumers_compact_remote_narrowed(
) -> M5VisualFoundationSharedConsumersPacket {
    packet_from_bindings(
        "m5-visual-foundations-shared-consumers:compact-remote:0001",
        "M5 visual-foundation shared consumers (compact / remote narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "vf-color-shell" => VisualFoundationRepresentation::CompactNarrowed,
            "vf-theme-editor" => VisualFoundationRepresentation::RemoteProjected,
            _ => default,
        }),
    )
}

/// Fixture: the same objects with two more surfaces narrowed to exported, export-safe
/// representations, proving vocabulary survives into exported forms.
pub fn seeded_m5_visual_foundations_shared_consumers_exported_redaction_narrowed(
) -> M5VisualFoundationSharedConsumersPacket {
    packet_from_bindings(
        "m5-visual-foundations-shared-consumers:exported-redaction:0001",
        "M5 visual-foundation shared consumers (exported redaction narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "vf-diff-review" => VisualFoundationRepresentation::ExportedRedacted,
            "vf-chart-docs" => VisualFoundationRepresentation::ExportedRedacted,
            _ => default,
        }),
    )
}
