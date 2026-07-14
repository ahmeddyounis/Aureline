//! Canonical seed for the window-restore shared-consumer parity packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix CSV,
//! Markdown summary, and narrowed fixtures. Every binding is derived from one per-profile
//! [`WindowRestoreStateFacetValues`] so the same restore profile always carries the same grammar across
//! surfaces, and every narrowed representation derives its disclosure from
//! [`resolve_window_restore_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

fn facets(
    window_restore_role: &str,
    family: &str,
    registry_reference: &str,
    restore_context: &str,
    surface_context: &str,
    session_continuity: &str,
) -> WindowRestoreStateFacetValues {
    WindowRestoreStateFacetValues {
        window_restore_role_word: window_restore_role.to_owned(),
        family_word: family.to_owned(),
        registry_reference_word: registry_reference.to_owned(),
        restore_context_word: restore_context.to_owned(),
        surface_context_word: surface_context.to_owned(),
        session_continuity_word: session_continuity.to_owned(),
    }
}

fn preserved_note_for(reason: WindowRestoreNarrowReason) -> String {
    match reason {
        WindowRestoreNarrowReason::CompactionNarrowed => {
            "window-restore-role, family, registry-reference, restore-context, surface-context, and session-continuity words preserved; only disclosure depth compacted"
        }
        WindowRestoreNarrowReason::RemoteProjectionNarrowed => {
            "all window-restore grammar preserved; the family is projected from the remote source of truth"
        }
        WindowRestoreNarrowReason::ExportRedactionNarrowed => {
            "all window-restore grammar preserved; only surrounding detail is redacted export-safe"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: WindowRestoreNarrowNextAction) -> String {
    match action {
        WindowRestoreNarrowNextAction::ExpandInDesktop => "Expand in the desktop surface",
        WindowRestoreNarrowNextAction::OpenRemoteSource => "Open the remote source",
        WindowRestoreNarrowNextAction::OpenFullDetail => "Open the full detail",
    }
    .to_owned()
}

fn binding_refs(family: M5WindowRestoreFamily) -> Vec<String> {
    vec![
        M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF.to_owned(),
        family.canonical_domain_schema_ref().to_owned(),
    ]
}

fn make_binding(
    binding_id: &str,
    restore_profile_id: &str,
    restore_profile_label: &str,
    family: M5WindowRestoreFamily,
    consumer: M5WindowRestoreConsumerSurface,
    representation: WindowRestoreRepresentation,
    state_facets: WindowRestoreStateFacetValues,
) -> WindowRestoreConsumerBinding {
    let disclosure = resolve_window_restore_render_disclosure(representation);
    let narrow_note = disclosure.narrow_reason.map(|reason| {
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrow reason always carries a next action");
        WindowRestoreNarrowNote {
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

    WindowRestoreConsumerBinding {
        binding_id: binding_id.to_owned(),
        restore_profile_id: restore_profile_id.to_owned(),
        restore_profile_label: restore_profile_label.to_owned(),
        family,
        consumer,
        representation,
        state_facets,
        parity_state: disclosure.parity_state,
        narrow_note,
        remote_source_note,
        export_detail_note,
        reruns_commands_or_reattaches_privileged_sessions_implicitly_during_restore: false,
        deletes_layout_structure_silently_on_missing_extension_or_remote_target: false,
        leaves_windows_or_dialogs_unreachable_after_display_topology_remap: false,
        merges_workspace_authority_and_window_topology_into_one_opaque_blob: false,
        overclaims_restore_fidelity_when_only_context_or_evidence_reopened: false,
        source_contract_refs: binding_refs(family),
    }
}

/// One consumer-surface adoption of a restore profile, before any representation override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5WindowRestoreConsumerSurface,
    representation: WindowRestoreRepresentation,
}

/// One restore profile rendered across several consumer surfaces at one grammar.
struct ProfileSpec {
    profile_id: &'static str,
    profile_label: &'static str,
    family: M5WindowRestoreFamily,
    facets: WindowRestoreStateFacetValues,
    bindings: Vec<BindingSpec>,
}

fn spec(
    profile_id: &'static str,
    profile_label: &'static str,
    family: M5WindowRestoreFamily,
    facets: WindowRestoreStateFacetValues,
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
    consumer: M5WindowRestoreConsumerSurface,
    representation: WindowRestoreRepresentation,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        representation,
    }
}

/// The five restore profiles — one per B141 window-restore family — and the surfaces that adopt each,
/// drawn from the restore-coordinator, shell, workspace, session, diagnostics, docs / help, CLI / export,
/// support-export, and general product consumers.
fn profile_specs() -> Vec<ProfileSpec> {
    use M5WindowRestoreConsumerSurface::*;
    use M5WindowRestoreFamily::*;
    use WindowRestoreRepresentation::*;

    let window_topology_registry = "window_topology_registry";
    let restore_fidelity_registry = "restore_fidelity_registry";
    let preserved_continuity = "window_local_selection_and_no_rerun_preserved";
    let window_scoped_descriptor = "window_scoped_descriptor";

    vec![
        spec(
            "shared-workspace-authority/one-authority-many-windows",
            "Shared workspace authority (one authority backs many windows)",
            SharedWorkspaceAuthority,
            facets(
                "workspace_authority",
                "shared_workspace_authority",
                window_topology_registry,
                "warm_restore",
                "shell_and_workspace",
                preserved_continuity,
            ),
            vec![
                bs(
                    "wrsc-shared-authority-coordinator",
                    RestoreCoordinator,
                    DesktopFull,
                ),
                bs("wrsc-shared-authority-shell", ShellUi, DesktopFull),
                bs("wrsc-shared-authority-cli", CliExport, ExportedRedacted),
            ],
        ),
        spec(
            "window-local-topology/window-scoped-pane-tree",
            "Window-local topology (window-scoped, versioned pane tree)",
            WindowLocalTopology,
            facets(
                "window_topology",
                "window_local_topology",
                window_topology_registry,
                "warm_restore",
                "shell_and_editor",
                window_scoped_descriptor,
            ),
            vec![
                bs(
                    "wrsc-window-topology-workspace",
                    WorkspaceService,
                    DesktopFull,
                ),
                bs("wrsc-window-topology-shell", ShellUi, DesktopFull),
                bs(
                    "wrsc-window-topology-support",
                    SupportExport,
                    ExportedRedacted,
                ),
            ],
        ),
        spec(
            "skeleton-first-restore/layout-skeleton-before-hydration",
            "Skeleton-first restore (layout skeleton before heavy hydration)",
            SkeletonFirstRestore,
            facets(
                "layout_skeleton",
                "skeleton_first_restore",
                restore_fidelity_registry,
                "cold_start",
                "review_and_notebook",
                window_scoped_descriptor,
            ),
            vec![
                bs("wrsc-skeleton-diagnostics", Diagnostics, DesktopFull),
                bs("wrsc-skeleton-coordinator", RestoreCoordinator, DesktopFull),
                bs("wrsc-skeleton-product", ProductUi, RemoteProjected),
            ],
        ),
        spec(
            "no-rerun-session-hydration/never-silently-reruns",
            "No-rerun session hydration (terminals / debug / notebooks never silently rerun)",
            NoRerunSessionHydration,
            facets(
                "session_hydration",
                "no_rerun_session_hydration",
                restore_fidelity_registry,
                "remote_reconnect",
                "terminal_and_debug",
                preserved_continuity,
            ),
            vec![
                bs("wrsc-no-rerun-session", SessionService, DesktopFull),
                bs("wrsc-no-rerun-diagnostics", Diagnostics, DesktopFull),
                bs("wrsc-no-rerun-product", ProductUi, RemoteProjected),
            ],
        ),
        spec(
            "display-topology-recovery/keeps-windows-reachable",
            "Display-topology recovery (monitor remap keeps windows reachable)",
            DisplayTopologyRecovery,
            facets(
                "display_affinity",
                "display_topology_recovery",
                window_topology_registry,
                "multi_monitor",
                "collaboration_and_companion",
                preserved_continuity,
            ),
            vec![
                bs("wrsc-display-docs", DocsHelp, DesktopFull),
                bs("wrsc-display-workspace", WorkspaceService, CompactNarrowed),
                bs("wrsc-display-support", SupportExport, ExportedRedacted),
            ],
        ),
    ]
}

/// Builds all consumer bindings, applying `rep` to override a binding's representation.
fn build_bindings<F>(rep: F) -> Vec<WindowRestoreConsumerBinding>
where
    F: Fn(&str, WindowRestoreRepresentation) -> WindowRestoreRepresentation,
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

fn trust_review() -> WindowRestoreSharedConsumersTrustReview {
    WindowRestoreSharedConsumersTrustReview {
        family_reuse_proven_by_fixtures: true,
        same_profile_same_window_restore_across_surfaces: true,
        window_restore_role_words_stay_in_frozen_vocabulary: true,
        authority_roles_never_clobber_selection_or_rerun_session_work: true,
        restore_never_reruns_or_reattaches_session_scoped_work: true,
        restore_never_deletes_layout_structure_silently: true,
        display_remap_never_leaves_window_or_dialog_unreachable: true,
        workspace_authority_and_window_topology_never_merged_into_blob: true,
        restore_never_overclaims_fidelity_when_only_context_or_evidence_reopened: true,
        narrowing_disclosed_across_representations: true,
        support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> WindowRestoreSharedConsumersProjection {
    WindowRestoreSharedConsumersProjection {
        restore_coordinator_consumes_shared_window_restore: true,
        shell_ui_consumes_shared_window_restore: true,
        workspace_service_consumes_shared_window_restore: true,
        session_service_consumes_shared_window_restore: true,
        diagnostics_consumes_shared_window_restore: true,
        docs_help_consumes_shared_window_restore: true,
        cli_export_consumes_shared_window_restore: true,
        support_export_consumes_shared_window_restore: true,
        product_ui_consumes_shared_window_restore: true,
        every_family_adopted_by_two_or_more_consumers: true,
        window_restore_identical_for_same_profile: true,
        narrowing_disclosed_not_hidden: true,
        export_maps_back_to_one_window_restore_family: true,
    }
}

fn proof_freshness() -> WindowRestoreSharedConsumersProofFreshness {
    WindowRestoreSharedConsumersProofFreshness {
        proof_freshness_slo_hours: M5_WINDOW_RESTORE_SHARED_CONSUMERS_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_WINDOW_RESTORE_SHARED_CONSUMERS_SCHEMA_REF.to_owned(),
        M5_WINDOW_RESTORE_SHARED_CONSUMERS_DOC_REF.to_owned(),
        M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF.to_owned(),
        M5_WINDOW_RESTORE_MATRIX_DOC_REF.to_owned(),
    ];
    // The five families map to two canonical domain schemas; include each distinct one once.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for family in M5WindowRestoreFamily::ALL {
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
    consumer_bindings: Vec<WindowRestoreConsumerBinding>,
) -> M5WindowRestoreSharedConsumersPacket {
    M5WindowRestoreSharedConsumersPacket::new(M5WindowRestoreSharedConsumersPacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        consumer_bindings,
        downgrade_triggers: WindowRestoreSharedConsumersDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5WindowRestoreConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in window-restore shared-consumer parity packet.
pub fn seeded_m5_window_restore_shared_consumers() -> M5WindowRestoreSharedConsumersPacket {
    packet_from_bindings(
        M5_WINDOW_RESTORE_SHARED_CONSUMERS_PACKET_ID,
        "M5 window-restore shared consumers (one registry across surfaces)",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same profiles with two more desktop surfaces narrowed to compact and remote
/// representations, proving grammar survives compact and remote forms.
pub fn seeded_m5_window_restore_shared_consumers_compact_remote_narrowed(
) -> M5WindowRestoreSharedConsumersPacket {
    packet_from_bindings(
        "m5-window-restore-shared-consumers:compact-remote:0001",
        "M5 window-restore shared consumers (compact / remote narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "wrsc-shared-authority-shell" => WindowRestoreRepresentation::CompactNarrowed,
            "wrsc-window-topology-workspace" => WindowRestoreRepresentation::RemoteProjected,
            _ => default,
        }),
    )
}

/// Fixture: the same profiles with two more surfaces narrowed to exported, export-safe
/// representations, proving grammar survives into exported forms.
pub fn seeded_m5_window_restore_shared_consumers_exported_redaction_narrowed(
) -> M5WindowRestoreSharedConsumersPacket {
    packet_from_bindings(
        "m5-window-restore-shared-consumers:exported-redaction:0001",
        "M5 window-restore shared consumers (exported redaction narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "wrsc-skeleton-diagnostics" => WindowRestoreRepresentation::ExportedRedacted,
            "wrsc-display-docs" => WindowRestoreRepresentation::ExportedRedacted,
            _ => default,
        }),
    )
}
