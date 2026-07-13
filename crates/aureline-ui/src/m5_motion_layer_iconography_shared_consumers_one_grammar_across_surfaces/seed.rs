//! Canonical seed for the visual-interaction shared-consumer parity packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix
//! CSV, Markdown summary, and narrowed fixtures. Every binding is derived from one per-object
//! [`VisualInteractionStateFacetValues`] so the same interaction object always carries the same
//! grammar across surfaces, and every narrowed representation derives its disclosure from
//! [`resolve_visual_interaction_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

fn facets(
    interaction_role: &str,
    family: &str,
    token_reference: &str,
    state_variant: &str,
    surface_context: &str,
    accessible_fallback: &str,
) -> VisualInteractionStateFacetValues {
    VisualInteractionStateFacetValues {
        interaction_role_word: interaction_role.to_owned(),
        family_word: family.to_owned(),
        token_reference_word: token_reference.to_owned(),
        state_variant_word: state_variant.to_owned(),
        surface_context_word: surface_context.to_owned(),
        accessible_fallback_word: accessible_fallback.to_owned(),
    }
}

fn preserved_note_for(reason: VisualInteractionNarrowReason) -> String {
    match reason {
        VisualInteractionNarrowReason::CompactionNarrowed => {
            "interaction-role, family, token-reference, state-variant, surface-context, and accessible-fallback words preserved; only disclosure depth compacted"
        }
        VisualInteractionNarrowReason::RemoteProjectionNarrowed => {
            "all visual-interaction grammar preserved; the family is projected from the remote source of truth"
        }
        VisualInteractionNarrowReason::ExportRedactionNarrowed => {
            "all visual-interaction grammar preserved; only surrounding detail is redacted export-safe"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: VisualInteractionNarrowNextAction) -> String {
    match action {
        VisualInteractionNarrowNextAction::ExpandInDesktop => "Expand in the desktop surface",
        VisualInteractionNarrowNextAction::OpenRemoteSource => "Open the remote source",
        VisualInteractionNarrowNextAction::OpenFullDetail => "Open the full detail",
    }
    .to_owned()
}

fn binding_refs(family: M5VisualInteractionFamily) -> Vec<String> {
    vec![
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF.to_owned(),
        family.canonical_domain_schema_ref().to_owned(),
    ]
}

fn make_binding(
    binding_id: &str,
    interaction_object_id: &str,
    interaction_object_label: &str,
    family: M5VisualInteractionFamily,
    consumer: M5VisualInteractionConsumerSurface,
    representation: VisualInteractionRepresentation,
    state_facets: VisualInteractionStateFacetValues,
) -> VisualInteractionConsumerBinding {
    let disclosure = resolve_visual_interaction_render_disclosure(representation);
    let narrow_note = disclosure.narrow_reason.map(|reason| {
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrow reason always carries a next action");
        VisualInteractionNarrowNote {
            reason,
            preserved_grammar_note: preserved_note_for(reason),
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

    VisualInteractionConsumerBinding {
        binding_id: binding_id.to_owned(),
        interaction_object_id: interaction_object_id.to_owned(),
        interaction_object_label: interaction_object_label.to_owned(),
        family,
        consumer,
        representation,
        state_facets,
        parity_state: disclosure.parity_state,
        narrow_note,
        remote_source_note,
        export_detail_note,
        delays_protected_input_with_motion: false,
        lets_scrim_erase_orientation_or_contrast: false,
        lets_overlay_bypass_shared_z_order: false,
        uses_unlabeled_icon_for_uncommon_or_destructive_action: false,
        lets_illustration_impersonate_operational_or_security_truth: false,
        source_contract_refs: binding_refs(family),
    }
}

/// One consumer-surface adoption of an interaction object, before any representation override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5VisualInteractionConsumerSurface,
    representation: VisualInteractionRepresentation,
}

/// One interaction object rendered across several consumer surfaces at one grammar.
struct ObjectSpec {
    object_id: &'static str,
    object_label: &'static str,
    family: M5VisualInteractionFamily,
    facets: VisualInteractionStateFacetValues,
    bindings: Vec<BindingSpec>,
}

fn spec(
    object_id: &'static str,
    object_label: &'static str,
    family: M5VisualInteractionFamily,
    facets: VisualInteractionStateFacetValues,
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
    consumer: M5VisualInteractionConsumerSurface,
    representation: VisualInteractionRepresentation,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        representation,
    }
}

/// The seven interaction objects — one per B137 visual-interaction family — and the surfaces that
/// adopt each, drawn from the first claimed shell, editor, help, marketplace / extension,
/// onboarding, settings, and export/support consumers.
fn object_specs() -> Vec<ObjectSpec> {
    use M5VisualInteractionConsumerSurface::*;
    use M5VisualInteractionFamily::*;
    use VisualInteractionRepresentation::*;

    vec![
        spec(
            "motion-token/panel-transition",
            "Panel-transition motion token",
            MotionToken,
            facets(
                "motion",
                "motion_token",
                "motion_and_reduced_motion_domain",
                "reduced_motion_power_saver_thermal",
                "shell_surface",
                "reduced_motion_static_fallback",
            ),
            vec![
                bs("mi-motion-shell", ShellUi, DesktopFull),
                bs("mi-motion-onboarding", OnboardingUi, DesktopFull),
                bs("mi-motion-support", SupportExport, ExportedRedacted),
            ],
        ),
        spec(
            "reduced-motion/clamp-profile",
            "Reduced-motion clamp profile",
            ReducedMotion,
            facets(
                "motion",
                "reduced_motion",
                "motion_and_reduced_motion_domain",
                "reduced_motion_power_saver_thermal",
                "shell_surface",
                "static_equivalent_state",
            ),
            vec![
                bs("mi-reduced-shell", ShellUi, DesktopFull),
                bs("mi-reduced-editor", EditorUi, DesktopFull),
            ],
        ),
        spec(
            "opacity-scrim/dialog-scrim",
            "Dialog opacity / scrim layer",
            OpacityScrim,
            facets(
                "overlay",
                "opacity_scrim",
                "opacity_scrim_domain",
                "reduced_motion_power_saver_thermal",
                "dialog_surface",
                "focus_trap_and_dismiss_affordance",
            ),
            vec![
                bs("mi-scrim-settings", SettingsUi, DesktopFull),
                bs("mi-scrim-shell", ShellUi, CompactNarrowed),
                bs("mi-scrim-cli", CliExport, ExportedRedacted),
            ],
        ),
        spec(
            "layer-order/z-tier",
            "Shared z-order tier",
            LayerOrder,
            facets(
                "layer",
                "layer_order",
                "layer_order_and_portal_domain",
                "reduced_motion_power_saver_thermal",
                "notification_surface",
                "shared_z_tier_label",
            ),
            vec![
                bs("mi-layer-shell", ShellUi, DesktopFull),
                bs("mi-layer-marketplace", MarketplaceUi, DesktopFull),
            ],
        ),
        spec(
            "portal-ownership/owning-attach",
            "Owning-surface portal attachment",
            PortalOwnership,
            facets(
                "portal",
                "portal_ownership",
                "layer_order_and_portal_domain",
                "reduced_motion_power_saver_thermal",
                "embedded_surface",
                "restore_safe_reattachment",
            ),
            vec![
                bs("mi-portal-shell", ShellUi, DesktopFull),
                bs("mi-portal-marketplace", MarketplaceUi, RemoteProjected),
            ],
        ),
        spec(
            "iconography/action-icon",
            "Semantic action icon",
            Iconography,
            facets(
                "icon",
                "iconography",
                "iconography_and_illustration_domain",
                "reduced_motion_power_saver_thermal",
                "shell_surface",
                "accessible_label_and_tooltip",
            ),
            vec![
                bs("mi-icon-editor", EditorUi, DesktopFull),
                bs("mi-icon-help", HelpUi, DesktopFull),
                bs("mi-icon-cli", CliExport, ExportedRedacted),
            ],
        ),
        spec(
            "illustration/onboarding-illustration",
            "Secondary onboarding illustration",
            Illustration,
            facets(
                "illustration",
                "illustration",
                "iconography_and_illustration_domain",
                "reduced_motion_power_saver_thermal",
                "onboarding_surface",
                "secondary_non_operational_caption",
            ),
            vec![
                bs("mi-illus-onboarding", OnboardingUi, DesktopFull),
                bs("mi-illus-help", HelpUi, DesktopFull),
                bs("mi-illus-product", ProductUi, DesktopFull),
                bs("mi-illus-support", SupportExport, ExportedRedacted),
            ],
        ),
    ]
}

/// Builds all consumer bindings, applying `rep` to override a binding's representation.
fn build_bindings<F>(rep: F) -> Vec<VisualInteractionConsumerBinding>
where
    F: Fn(&str, VisualInteractionRepresentation) -> VisualInteractionRepresentation,
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

fn trust_review() -> VisualInteractionSharedConsumersTrustReview {
    VisualInteractionSharedConsumersTrustReview {
        family_reuse_proven_by_fixtures: true,
        same_object_same_grammar_across_surfaces: true,
        interaction_role_words_stay_in_frozen_vocabulary: true,
        meaning_never_relies_on_motion_or_decoration_alone: true,
        motion_never_delays_protected_input: true,
        scrims_never_erase_orientation_or_contrast: true,
        overlays_never_bypass_shared_z_order: true,
        icons_never_unlabeled_for_uncommon_or_destructive_actions: true,
        illustrations_never_impersonate_operational_truth: true,
        narrowing_disclosed_across_representations: true,
        support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> VisualInteractionSharedConsumersProjection {
    VisualInteractionSharedConsumersProjection {
        shell_ui_consumes_shared_grammar: true,
        editor_ui_consumes_shared_grammar: true,
        help_ui_consumes_shared_grammar: true,
        marketplace_ui_consumes_shared_grammar: true,
        onboarding_ui_consumes_shared_grammar: true,
        settings_ui_consumes_shared_grammar: true,
        support_export_consumes_shared_grammar: true,
        every_family_adopted_by_two_or_more_consumers: true,
        grammar_identical_for_same_object: true,
        narrowing_disclosed_not_hidden: true,
        export_maps_back_to_one_interaction_family: true,
    }
}

fn proof_freshness() -> VisualInteractionSharedConsumersProofFreshness {
    VisualInteractionSharedConsumersProofFreshness {
        proof_freshness_slo_hours: M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_SCHEMA_REF.to_owned(),
        M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_DOC_REF.to_owned(),
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF.to_owned(),
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_DOC_REF.to_owned(),
    ];
    // The seven families map to four canonical domain schemas; include each distinct one once.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for family in M5VisualInteractionFamily::ALL {
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
    consumer_bindings: Vec<VisualInteractionConsumerBinding>,
) -> M5VisualInteractionSharedConsumersPacket {
    M5VisualInteractionSharedConsumersPacket::new(M5VisualInteractionSharedConsumersPacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        consumer_bindings,
        downgrade_triggers: VisualInteractionSharedConsumersDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5VisualInteractionConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in visual-interaction shared-consumer parity packet.
pub fn seeded_m5_motion_layer_iconography_shared_consumers(
) -> M5VisualInteractionSharedConsumersPacket {
    packet_from_bindings(
        M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_PACKET_ID,
        "M5 motion / layer / iconography shared consumers",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same objects with two more desktop surfaces narrowed to compact and remote
/// representations, proving grammar survives compact and remote forms.
pub fn seeded_m5_motion_layer_iconography_shared_consumers_compact_remote_narrowed(
) -> M5VisualInteractionSharedConsumersPacket {
    packet_from_bindings(
        "m5-motion-layer-iconography-shared-consumers:compact-remote:0001",
        "M5 motion / layer / iconography shared consumers (compact / remote narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "mi-motion-shell" => VisualInteractionRepresentation::CompactNarrowed,
            "mi-reduced-editor" => VisualInteractionRepresentation::RemoteProjected,
            _ => default,
        }),
    )
}

/// Fixture: the same objects with two more surfaces narrowed to exported, export-safe
/// representations, proving grammar survives into exported forms.
pub fn seeded_m5_motion_layer_iconography_shared_consumers_exported_redaction_narrowed(
) -> M5VisualInteractionSharedConsumersPacket {
    packet_from_bindings(
        "m5-motion-layer-iconography-shared-consumers:exported-redaction:0001",
        "M5 motion / layer / iconography shared consumers (exported redaction narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "mi-layer-marketplace" => VisualInteractionRepresentation::ExportedRedacted,
            "mi-illus-product" => VisualInteractionRepresentation::ExportedRedacted,
            _ => default,
        }),
    )
}
