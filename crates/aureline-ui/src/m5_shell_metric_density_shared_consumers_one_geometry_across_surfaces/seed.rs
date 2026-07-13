//! Canonical seed for the shell-geometry shared-consumer parity packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix
//! CSV, Markdown summary, and narrowed fixtures. Every binding is derived from one per-object
//! [`ShellGeometryStateFacetValues`] so the same geometry object always carries the same grammar
//! across surfaces, and every narrowed representation derives its disclosure from
//! [`resolve_shell_geometry_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

fn facets(
    geometry_role: &str,
    family: &str,
    registry_reference: &str,
    width_or_density_class: &str,
    surface_context: &str,
    minimum_guarantee: &str,
) -> ShellGeometryStateFacetValues {
    ShellGeometryStateFacetValues {
        geometry_role_word: geometry_role.to_owned(),
        family_word: family.to_owned(),
        registry_reference_word: registry_reference.to_owned(),
        width_or_density_class_word: width_or_density_class.to_owned(),
        surface_context_word: surface_context.to_owned(),
        minimum_guarantee_word: minimum_guarantee.to_owned(),
    }
}

fn preserved_note_for(reason: ShellGeometryNarrowReason) -> String {
    match reason {
        ShellGeometryNarrowReason::CompactionNarrowed => {
            "geometry-role, family, registry-reference, width/density-class, surface-context, and minimum-guarantee words preserved; only disclosure depth compacted"
        }
        ShellGeometryNarrowReason::RemoteProjectionNarrowed => {
            "all shell-geometry grammar preserved; the family is projected from the remote source of truth"
        }
        ShellGeometryNarrowReason::ExportRedactionNarrowed => {
            "all shell-geometry grammar preserved; only surrounding detail is redacted export-safe"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: ShellGeometryNarrowNextAction) -> String {
    match action {
        ShellGeometryNarrowNextAction::ExpandInDesktop => "Expand in the desktop surface",
        ShellGeometryNarrowNextAction::OpenRemoteSource => "Open the remote source",
        ShellGeometryNarrowNextAction::OpenFullDetail => "Open the full detail",
    }
    .to_owned()
}

fn binding_refs(family: M5ShellGeometryFamily) -> Vec<String> {
    vec![
        M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF.to_owned(),
        family.canonical_domain_schema_ref().to_owned(),
    ]
}

fn make_binding(
    binding_id: &str,
    geometry_object_id: &str,
    geometry_object_label: &str,
    family: M5ShellGeometryFamily,
    consumer: M5ShellGeometryConsumerSurface,
    representation: ShellGeometryRepresentation,
    state_facets: ShellGeometryStateFacetValues,
) -> ShellGeometryConsumerBinding {
    let disclosure = resolve_shell_geometry_render_disclosure(representation);
    let narrow_note = disclosure.narrow_reason.map(|reason| {
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrow reason always carries a next action");
        ShellGeometryNarrowNote {
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

    ShellGeometryConsumerBinding {
        binding_id: binding_id.to_owned(),
        geometry_object_id: geometry_object_id.to_owned(),
        geometry_object_label: geometry_object_label.to_owned(),
        family,
        consumer,
        representation,
        state_facets,
        parity_state: disclosure.parity_state,
        narrow_note,
        remote_source_note,
        export_detail_note,
        density_or_collapse_changes_command_focus_or_trust: false,
        extension_or_embedded_sets_private_fracturing_width: false,
        shrinks_hit_target_below_supported_minimum: false,
        hides_primary_workflow_behind_overlay_only_fallback: false,
        lets_zone_starve_main_workspace_below_minimum: false,
        source_contract_refs: binding_refs(family),
    }
}

/// One consumer-surface adoption of a geometry object, before any representation override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5ShellGeometryConsumerSurface,
    representation: ShellGeometryRepresentation,
}

/// One geometry object rendered across several consumer surfaces at one grammar.
struct ObjectSpec {
    object_id: &'static str,
    object_label: &'static str,
    family: M5ShellGeometryFamily,
    facets: ShellGeometryStateFacetValues,
    bindings: Vec<BindingSpec>,
}

fn spec(
    object_id: &'static str,
    object_label: &'static str,
    family: M5ShellGeometryFamily,
    facets: ShellGeometryStateFacetValues,
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
    consumer: M5ShellGeometryConsumerSurface,
    representation: ShellGeometryRepresentation,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        representation,
    }
}

/// The five geometry objects — one per B138 shell-geometry family — and the surfaces that adopt
/// each, drawn from the first claimed shell, editor, review, notebook, data, settings, and
/// export/support consumers.
fn object_specs() -> Vec<ObjectSpec> {
    use M5ShellGeometryConsumerSurface::*;
    use M5ShellGeometryFamily::*;
    use ShellGeometryRepresentation::*;

    vec![
        spec(
            "shell-metric/sidebar-width",
            "Sidebar shell-metric size",
            ShellMetric,
            facets(
                "metric",
                "shell_metric",
                "shell_metrics_registry",
                "standard_density_zoom_snapped_safe",
                "shell_surface",
                "declared_minimum_and_recommended_size",
            ),
            vec![
                bs("sg-metric-shell", ShellUi, DesktopFull),
                bs("sg-metric-editor", EditorUi, DesktopFull),
                bs("sg-metric-support", SupportExport, ExportedRedacted),
            ],
        ),
        spec(
            "minimum-size/tab-and-hit-target",
            "Tab and hit-target minimum size",
            MinimumSize,
            facets(
                "hit_target",
                "minimum_size",
                "shell_metrics_registry",
                "standard_density_zoom_snapped_safe",
                "editor_surface",
                "pointer_and_keyboard_reachable_minimum",
            ),
            vec![
                bs("sg-min-editor", EditorUi, DesktopFull),
                bs("sg-min-review", ReviewUi, DesktopFull),
                bs("sg-min-cli", CliExport, ExportedRedacted),
            ],
        ),
        spec(
            "density-mode/comfortable-standard-compact",
            "Comfortable / standard / compact density mode",
            DensityMode,
            facets(
                "density",
                "density_mode",
                "density_mode_registry",
                "compact_standard_comfortable_zoom_safe",
                "notebook_surface",
                "information_architecture_preserved",
            ),
            vec![
                bs("sg-density-shell", ShellUi, DesktopFull),
                bs("sg-density-notebook", NotebookUi, DesktopFull),
                bs("sg-density-data", DataUi, CompactNarrowed),
            ],
        ),
        spec(
            "responsive-geometry/window-class",
            "Compact / standard / expanded window class",
            ResponsiveGeometry,
            facets(
                "responsive",
                "responsive_geometry",
                "density_mode_registry",
                "compact_standard_expanded_width_class",
                "data_surface",
                "task_identity_and_recovery_state_preserved",
            ),
            vec![
                bs("sg-responsive-shell", ShellUi, DesktopFull),
                bs("sg-responsive-data", DataUi, DesktopFull),
                bs("sg-responsive-product", ProductUi, RemoteProjected),
            ],
        ),
        spec(
            "collapse-priority/adaptive-collapse",
            "Adaptive-collapse priority order",
            CollapsePriority,
            facets(
                "collapse",
                "collapse_priority",
                "density_mode_registry",
                "compact_standard_expanded_width_class",
                "review_surface",
                "main_workspace_dominant_no_overlay_only_fallback",
            ),
            vec![
                bs("sg-collapse-shell", ShellUi, DesktopFull),
                bs("sg-collapse-settings", SettingsUi, DesktopFull),
                bs("sg-collapse-review", ReviewUi, DesktopFull),
            ],
        ),
    ]
}

/// Builds all consumer bindings, applying `rep` to override a binding's representation.
fn build_bindings<F>(rep: F) -> Vec<ShellGeometryConsumerBinding>
where
    F: Fn(&str, ShellGeometryRepresentation) -> ShellGeometryRepresentation,
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

fn trust_review() -> ShellGeometrySharedConsumersTrustReview {
    ShellGeometrySharedConsumersTrustReview {
        family_reuse_proven_by_fixtures: true,
        same_object_same_geometry_across_surfaces: true,
        geometry_role_words_stay_in_frozen_vocabulary: true,
        adaptive_roles_never_drop_task_identity_or_recovery_state: true,
        density_or_collapse_never_changes_command_focus_or_trust: true,
        extension_or_embedded_never_sets_private_fracturing_width: true,
        hit_targets_never_shrink_below_supported_minimum: true,
        primary_workflows_never_hidden_behind_overlay_only_fallback: true,
        zones_never_starve_main_workspace_below_minimum: true,
        narrowing_disclosed_across_representations: true,
        support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> ShellGeometrySharedConsumersProjection {
    ShellGeometrySharedConsumersProjection {
        shell_ui_consumes_shared_geometry: true,
        editor_ui_consumes_shared_geometry: true,
        review_ui_consumes_shared_geometry: true,
        notebook_ui_consumes_shared_geometry: true,
        data_ui_consumes_shared_geometry: true,
        settings_ui_consumes_shared_geometry: true,
        support_export_consumes_shared_geometry: true,
        every_family_adopted_by_two_or_more_consumers: true,
        geometry_identical_for_same_object: true,
        narrowing_disclosed_not_hidden: true,
        export_maps_back_to_one_geometry_family: true,
    }
}

fn proof_freshness() -> ShellGeometrySharedConsumersProofFreshness {
    ShellGeometrySharedConsumersProofFreshness {
        proof_freshness_slo_hours: M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_SCHEMA_REF.to_owned(),
        M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_DOC_REF.to_owned(),
        M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF.to_owned(),
        M5_SHELL_METRIC_DENSITY_MATRIX_DOC_REF.to_owned(),
    ];
    // The five families map to two canonical domain schemas; include each distinct one once.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for family in M5ShellGeometryFamily::ALL {
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
    consumer_bindings: Vec<ShellGeometryConsumerBinding>,
) -> M5ShellGeometrySharedConsumersPacket {
    M5ShellGeometrySharedConsumersPacket::new(M5ShellGeometrySharedConsumersPacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        consumer_bindings,
        downgrade_triggers: ShellGeometrySharedConsumersDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5ShellGeometryConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in shell-geometry shared-consumer parity packet.
pub fn seeded_m5_shell_metric_density_shared_consumers() -> M5ShellGeometrySharedConsumersPacket {
    packet_from_bindings(
        M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_PACKET_ID,
        "M5 shell-metric / density / geometry shared consumers",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same objects with two more desktop surfaces narrowed to compact and remote
/// representations, proving grammar survives compact and remote forms.
pub fn seeded_m5_shell_metric_density_shared_consumers_compact_remote_narrowed(
) -> M5ShellGeometrySharedConsumersPacket {
    packet_from_bindings(
        "m5-shell-metric-density-shared-consumers:compact-remote:0001",
        "M5 shell-metric / density / geometry shared consumers (compact / remote narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "sg-metric-editor" => ShellGeometryRepresentation::CompactNarrowed,
            "sg-collapse-review" => ShellGeometryRepresentation::RemoteProjected,
            _ => default,
        }),
    )
}

/// Fixture: the same objects with two more surfaces narrowed to exported, export-safe
/// representations, proving grammar survives into exported forms.
pub fn seeded_m5_shell_metric_density_shared_consumers_exported_redaction_narrowed(
) -> M5ShellGeometrySharedConsumersPacket {
    packet_from_bindings(
        "m5-shell-metric-density-shared-consumers:exported-redaction:0001",
        "M5 shell-metric / density / geometry shared consumers (exported redaction narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "sg-min-review" => ShellGeometryRepresentation::ExportedRedacted,
            "sg-responsive-data" => ShellGeometryRepresentation::ExportedRedacted,
            _ => default,
        }),
    )
}
