//! Canonical seed for the install-topology shared-consumer parity packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix
//! CSV, Markdown summary, and narrowed fixtures. Every binding is derived from one per-profile
//! [`InstallTopologyStateFacetValues`] so the same delivery profile always carries the same grammar
//! across surfaces, and every narrowed representation derives its disclosure from
//! [`resolve_install_topology_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

fn facets(
    install_topology_role: &str,
    family: &str,
    registry_reference: &str,
    channel: &str,
    surface_context: &str,
    ownership_identity: &str,
) -> InstallTopologyStateFacetValues {
    InstallTopologyStateFacetValues {
        install_topology_role_word: install_topology_role.to_owned(),
        family_word: family.to_owned(),
        registry_reference_word: registry_reference.to_owned(),
        channel_word: channel.to_owned(),
        surface_context_word: surface_context.to_owned(),
        ownership_identity_word: ownership_identity.to_owned(),
    }
}

fn preserved_note_for(reason: InstallTopologyNarrowReason) -> String {
    match reason {
        InstallTopologyNarrowReason::CompactionNarrowed => {
            "install-topology-role, family, registry-reference, channel, surface-context, and ownership-identity words preserved; only disclosure depth compacted"
        }
        InstallTopologyNarrowReason::RemoteProjectionNarrowed => {
            "all install-topology grammar preserved; the family is projected from the remote source of truth"
        }
        InstallTopologyNarrowReason::ExportRedactionNarrowed => {
            "all install-topology grammar preserved; only surrounding detail is redacted export-safe"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: InstallTopologyNarrowNextAction) -> String {
    match action {
        InstallTopologyNarrowNextAction::ExpandInDesktop => "Expand in the desktop surface",
        InstallTopologyNarrowNextAction::OpenRemoteSource => "Open the remote source",
        InstallTopologyNarrowNextAction::OpenFullDetail => "Open the full detail",
    }
    .to_owned()
}

fn binding_refs(family: M5InstallTopologyFamily) -> Vec<String> {
    vec![
        M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF.to_owned(),
        family.canonical_domain_schema_ref().to_owned(),
    ]
}

fn make_binding(
    binding_id: &str,
    delivery_profile_id: &str,
    delivery_profile_label: &str,
    family: M5InstallTopologyFamily,
    consumer: M5InstallTopologyConsumerSurface,
    representation: InstallTopologyRepresentation,
    state_facets: InstallTopologyStateFacetValues,
) -> InstallTopologyConsumerBinding {
    let disclosure = resolve_install_topology_render_disclosure(representation);
    let narrow_note = disclosure.narrow_reason.map(|reason| {
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrow reason always carries a next action");
        InstallTopologyNarrowNote {
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

    InstallTopologyConsumerBinding {
        binding_id: binding_id.to_owned(),
        delivery_profile_id: delivery_profile_id.to_owned(),
        delivery_profile_label: delivery_profile_label.to_owned(),
        family,
        consumer,
        representation,
        state_facets,
        parity_state: disclosure.parity_state,
        narrow_note,
        remote_source_note,
        export_detail_note,
        portable_mode_writes_hidden_machine_global_durable_state: false,
        preview_channel_reuses_stable_state_namespace_without_handoff: false,
        rollback_targets_primary_executable_while_sidecars_drift: false,
        hides_updater_ownership_or_admin_control_in_managed_flow: false,
        publishes_deployment_claim_outpacing_ring_or_repair_verify_evidence: false,
        source_contract_refs: binding_refs(family),
    }
}

/// One consumer-surface adoption of a delivery profile, before any representation override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5InstallTopologyConsumerSurface,
    representation: InstallTopologyRepresentation,
}

/// One delivery profile rendered across several consumer surfaces at one grammar.
struct ProfileSpec {
    profile_id: &'static str,
    profile_label: &'static str,
    family: M5InstallTopologyFamily,
    facets: InstallTopologyStateFacetValues,
    bindings: Vec<BindingSpec>,
}

fn spec(
    profile_id: &'static str,
    profile_label: &'static str,
    family: M5InstallTopologyFamily,
    facets: InstallTopologyStateFacetValues,
    bindings: Vec<BindingSpec>,
) -> ProfileSpec {
    ProfileSpec {
        profile_id,
        profile_label,
        family,
        facets,
        bindings,
    }
}

fn bs(
    binding_id: &'static str,
    consumer: M5InstallTopologyConsumerSurface,
    representation: InstallTopologyRepresentation,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        representation,
    }
}

/// The five delivery profiles — one per B140 install-topology family — and the surfaces that adopt
/// each, drawn from the installer / package-manager, About / shell, update-center / updater,
/// diagnostics, admin, docs / help, CLI / export, support-export, and general product / fleet-rollout
/// consumers.
fn profile_specs() -> Vec<ProfileSpec> {
    use InstallTopologyRepresentation::*;
    use M5InstallTopologyConsumerSurface::*;
    use M5InstallTopologyFamily::*;

    let install_registry = "install_topology_registry";
    let state_root_registry = "state_root_boundaries_registry";
    let owned_isolated = "inspectable_owner_and_isolated_state";

    vec![
        spec(
            "per-user-managed/user-scoped-ownership",
            "Per-user managed install (user-scoped updater ownership)",
            PerUserManaged,
            facets(
                "updater_owner",
                "per_user_managed",
                install_registry,
                "stable",
                "about_and_update_center",
                owned_isolated,
            ),
            vec![
                bs("itsc-per-user-updater", UpdaterService, DesktopFull),
                bs("itsc-per-user-about", ShellUi, DesktopFull),
                bs("itsc-per-user-cli", CliExport, ExportedRedacted),
            ],
        ),
        spec(
            "per-machine-managed/machine-policy-roots",
            "Per-machine managed install (machine policy roots)",
            PerMachineManaged,
            facets(
                "policy_roots",
                "per_machine_managed",
                install_registry,
                "stable",
                "admin_and_installer",
                owned_isolated,
            ),
            vec![
                bs("itsc-per-machine-admin", Admin, DesktopFull),
                bs("itsc-per-machine-installer", Installer, DesktopFull),
                bs("itsc-per-machine-support", SupportExport, ExportedRedacted),
            ],
        ),
        spec(
            "side-by-side/isolated-state-namespace",
            "Side-by-side stable-plus-preview (isolated channel state namespace)",
            SideBySideStablePreview,
            facets(
                "writable_state_roots",
                "side_by_side_stable_preview",
                install_registry,
                "preview",
                "diagnostics_and_channel_review",
                owned_isolated,
            ),
            vec![
                bs("itsc-side-by-side-diagnostics", Diagnostics, DesktopFull),
                bs("itsc-side-by-side-about", ShellUi, DesktopFull),
                bs("itsc-side-by-side-product", ProductUi, RemoteProjected),
            ],
        ),
        spec(
            "portable-mode/colocated-install",
            "Portable mode (colocated install-mode disclosure)",
            PortableMode,
            facets(
                "install_mode",
                "portable_mode",
                state_root_registry,
                "stable",
                "portable_diagnostics",
                "colocated_portable_state",
            ),
            vec![
                bs("itsc-portable-docs", DocsHelp, DesktopFull),
                bs("itsc-portable-diagnostics", Diagnostics, DesktopFull),
                bs("itsc-portable-product", ProductUi, RemoteProjected),
            ],
        ),
        spec(
            "offline-airgap/complete-rollback-target",
            "Offline / air-gap bundle (complete rollback-target set)",
            OfflineAirgapBundle,
            facets(
                "rollback_target",
                "offline_airgap_bundle",
                state_root_registry,
                "stable",
                "airgap_admin_and_installer",
                owned_isolated,
            ),
            vec![
                bs("itsc-offline-admin", Admin, DesktopFull),
                bs("itsc-offline-installer", Installer, CompactNarrowed),
                bs("itsc-offline-support", SupportExport, ExportedRedacted),
            ],
        ),
    ]
}

/// Builds all consumer bindings, applying `rep` to override a binding's representation.
fn build_bindings<F>(rep: F) -> Vec<InstallTopologyConsumerBinding>
where
    F: Fn(&str, InstallTopologyRepresentation) -> InstallTopologyRepresentation,
{
    let mut bindings = Vec::new();
    for profile in profile_specs() {
        for spec in &profile.bindings {
            let representation = rep(spec.binding_id, spec.representation);
            bindings.push(make_binding(
                spec.binding_id,
                profile.profile_id,
                profile.profile_label,
                profile.family,
                spec.consumer,
                representation,
                profile.facets.clone(),
            ));
        }
    }
    bindings
}

fn trust_review() -> InstallTopologySharedConsumersTrustReview {
    InstallTopologySharedConsumersTrustReview {
        family_reuse_proven_by_fixtures: true,
        same_profile_same_install_topology_across_surfaces: true,
        install_topology_role_words_stay_in_frozen_vocabulary: true,
        ownership_roles_never_hide_owner_or_spill_state: true,
        portable_mode_never_spills_hidden_machine_global_durable_state: true,
        preview_channel_never_reuses_stable_state_namespace_without_handoff: true,
        rollback_never_targets_primary_executable_while_sidecars_drift: true,
        updater_ownership_or_admin_control_never_hidden_in_managed_flow: true,
        deployment_claims_never_outpace_ring_or_repair_verify_evidence: true,
        narrowing_disclosed_across_representations: true,
        support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> InstallTopologySharedConsumersProjection {
    InstallTopologySharedConsumersProjection {
        updater_service_consumes_shared_install_topology: true,
        shell_about_consumes_shared_install_topology: true,
        diagnostics_consumes_shared_install_topology: true,
        admin_consumes_shared_install_topology: true,
        installer_consumes_shared_install_topology: true,
        docs_help_consumes_shared_install_topology: true,
        cli_export_consumes_shared_install_topology: true,
        support_export_consumes_shared_install_topology: true,
        product_ui_consumes_shared_install_topology: true,
        every_family_adopted_by_two_or_more_consumers: true,
        install_topology_identical_for_same_profile: true,
        narrowing_disclosed_not_hidden: true,
        export_maps_back_to_one_install_topology_family: true,
    }
}

fn proof_freshness() -> InstallTopologySharedConsumersProofFreshness {
    InstallTopologySharedConsumersProofFreshness {
        proof_freshness_slo_hours: M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_SCHEMA_REF.to_owned(),
        M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_DOC_REF.to_owned(),
        M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF.to_owned(),
        M5_INSTALL_TOPOLOGY_MATRIX_DOC_REF.to_owned(),
    ];
    // The five families map to two canonical domain schemas; include each distinct one once.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for family in M5InstallTopologyFamily::ALL {
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
    consumer_bindings: Vec<InstallTopologyConsumerBinding>,
) -> M5InstallTopologySharedConsumersPacket {
    M5InstallTopologySharedConsumersPacket::new(M5InstallTopologySharedConsumersPacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        consumer_bindings,
        downgrade_triggers: InstallTopologySharedConsumersDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5InstallTopologyConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in install-topology shared-consumer parity packet.
pub fn seeded_m5_install_topology_shared_consumers() -> M5InstallTopologySharedConsumersPacket {
    packet_from_bindings(
        M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_PACKET_ID,
        "M5 install-topology shared consumers (one registry across surfaces)",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same profiles with two more desktop surfaces narrowed to compact and remote
/// representations, proving grammar survives compact and remote forms.
pub fn seeded_m5_install_topology_shared_consumers_compact_remote_narrowed(
) -> M5InstallTopologySharedConsumersPacket {
    packet_from_bindings(
        "m5-install-topology-shared-consumers:compact-remote:0001",
        "M5 install-topology shared consumers (compact / remote narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "itsc-per-user-about" => InstallTopologyRepresentation::CompactNarrowed,
            "itsc-per-machine-admin" => InstallTopologyRepresentation::RemoteProjected,
            _ => default,
        }),
    )
}

/// Fixture: the same profiles with two more surfaces narrowed to exported, export-safe
/// representations, proving grammar survives into exported forms.
pub fn seeded_m5_install_topology_shared_consumers_exported_redaction_narrowed(
) -> M5InstallTopologySharedConsumersPacket {
    packet_from_bindings(
        "m5-install-topology-shared-consumers:exported-redaction:0001",
        "M5 install-topology shared consumers (exported redaction narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "itsc-side-by-side-diagnostics" => InstallTopologyRepresentation::ExportedRedacted,
            "itsc-portable-docs" => InstallTopologyRepresentation::ExportedRedacted,
            _ => default,
        }),
    )
}
