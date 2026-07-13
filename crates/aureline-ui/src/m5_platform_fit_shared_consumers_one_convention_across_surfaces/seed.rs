//! Canonical seed for the platform-fit shared-consumer parity packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix
//! CSV, Markdown summary, and narrowed fixtures. Every binding is derived from one per-object
//! [`PlatformFitStateFacetValues`] so the same platform-fit object always carries the same grammar
//! across surfaces, and every narrowed representation derives its disclosure from
//! [`resolve_platform_fit_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

fn facets(
    platform_fit_role: &str,
    family: &str,
    registry_reference: &str,
    host_platform: &str,
    surface_context: &str,
    command_identity: &str,
) -> PlatformFitStateFacetValues {
    PlatformFitStateFacetValues {
        platform_fit_role_word: platform_fit_role.to_owned(),
        family_word: family.to_owned(),
        registry_reference_word: registry_reference.to_owned(),
        host_platform_word: host_platform.to_owned(),
        surface_context_word: surface_context.to_owned(),
        command_identity_word: command_identity.to_owned(),
    }
}

fn preserved_note_for(reason: PlatformFitNarrowReason) -> String {
    match reason {
        PlatformFitNarrowReason::CompactionNarrowed => {
            "platform-fit-role, family, registry-reference, host-platform, surface-context, and command-identity words preserved; only disclosure depth compacted"
        }
        PlatformFitNarrowReason::RemoteProjectionNarrowed => {
            "all platform-fit grammar preserved; the family is projected from the remote source of truth"
        }
        PlatformFitNarrowReason::ExportRedactionNarrowed => {
            "all platform-fit grammar preserved; only surrounding detail is redacted export-safe"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: PlatformFitNarrowNextAction) -> String {
    match action {
        PlatformFitNarrowNextAction::ExpandInDesktop => "Expand in the desktop surface",
        PlatformFitNarrowNextAction::OpenRemoteSource => "Open the remote source",
        PlatformFitNarrowNextAction::OpenFullDetail => "Open the full detail",
    }
    .to_owned()
}

fn binding_refs(family: M5PlatformFitFamily) -> Vec<String> {
    vec![
        M5_PLATFORM_FIT_MATRIX_SCHEMA_REF.to_owned(),
        family.canonical_domain_schema_ref().to_owned(),
    ]
}

fn make_binding(
    binding_id: &str,
    platform_fit_object_id: &str,
    platform_fit_object_label: &str,
    family: M5PlatformFitFamily,
    consumer: M5PlatformFitConsumerSurface,
    representation: PlatformFitRepresentation,
    state_facets: PlatformFitStateFacetValues,
) -> PlatformFitConsumerBinding {
    let disclosure = resolve_platform_fit_render_disclosure(representation);
    let narrow_note = disclosure.narrow_reason.map(|reason| {
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrow reason always carries a next action");
        PlatformFitNarrowNote {
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

    PlatformFitConsumerBinding {
        binding_id: binding_id.to_owned(),
        platform_fit_object_id: platform_fit_object_id.to_owned(),
        platform_fit_object_label: platform_fit_object_label.to_owned(),
        family,
        consumer,
        representation,
        state_facets,
        parity_state: disclosure.parity_state,
        narrow_note,
        remote_source_note,
        export_detail_note,
        platform_wording_changes_command_or_permission_meaning: false,
        hides_primary_action_only_in_os_chrome: false,
        falls_back_to_plaintext_credential_storage_silently: false,
        input_method_corrupts_text_or_trust_fidelity: false,
        screenshot_or_docs_mislabels_shortcut_or_path_verb: false,
        source_contract_refs: binding_refs(family),
    }
}

/// One consumer-surface adoption of a platform-fit object, before any representation override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5PlatformFitConsumerSurface,
    representation: PlatformFitRepresentation,
}

/// One platform-fit object rendered across several consumer surfaces at one grammar.
struct ObjectSpec {
    object_id: &'static str,
    object_label: &'static str,
    family: M5PlatformFitFamily,
    facets: PlatformFitStateFacetValues,
    bindings: Vec<BindingSpec>,
}

fn spec(
    object_id: &'static str,
    object_label: &'static str,
    family: M5PlatformFitFamily,
    facets: PlatformFitStateFacetValues,
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
    consumer: M5PlatformFitConsumerSurface,
    representation: PlatformFitRepresentation,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        representation,
    }
}

/// The six platform-fit objects — one per B139 platform-fit family — and the surfaces that adopt
/// each, drawn from the Start Center / shell, settings, auth, input, docs / help, onboarding, CLI /
/// export, support-export, and general product consumers.
fn object_specs() -> Vec<ObjectSpec> {
    use M5PlatformFitConsumerSurface::*;
    use M5PlatformFitFamily::*;
    use PlatformFitRepresentation::*;

    vec![
        spec(
            "platform-convention/window-and-menu",
            "Window controls and menu-bar convention",
            PlatformConvention,
            facets(
                "window_menu",
                "platform_convention",
                "platform_convention_registry",
                "macos_windows_linux_adaptive",
                "window_and_menu_chrome",
                "stable_command_id",
            ),
            vec![
                bs("pf-convention-shell", ShellUi, DesktopFull),
                bs("pf-convention-docs", DocsHelp, DesktopFull),
                bs("pf-convention-cli", CliExport, ExportedRedacted),
            ],
        ),
        spec(
            "shortcut-notation/platform-native",
            "Platform-native shortcut notation",
            ShortcutNotation,
            facets(
                "shortcut",
                "shortcut_notation",
                "shortcut_notation_registry",
                "macos_windows_linux_adaptive",
                "command_palette_and_menu",
                "stable_command_id",
            ),
            vec![
                bs("pf-shortcut-shell", ShellUi, DesktopFull),
                bs("pf-shortcut-settings", SettingsUi, DesktopFull),
                bs("pf-shortcut-support", SupportExport, ExportedRedacted),
            ],
        ),
        spec(
            "file-path-reveal/host-terminology",
            "File / path / reveal / save terminology",
            FilePathReveal,
            facets(
                "path_terminology",
                "file_path_reveal",
                "file_path_reveal_registry",
                "macos_windows_linux_adaptive",
                "open_save_reveal_dialog",
                "host_matched_terminology",
            ),
            vec![
                bs("pf-path-settings", SettingsUi, DesktopFull),
                bs("pf-path-docs", DocsHelp, DesktopFull),
                bs("pf-path-cli", CliExport, ExportedRedacted),
            ],
        ),
        spec(
            "theme-contrast/live-response",
            "Live theme / contrast / accent / text-scale response",
            ThemeContrastLiveChange,
            facets(
                "appearance",
                "theme_contrast_live_change",
                "appearance_response_registry",
                "macos_windows_linux_adaptive",
                "appearance_settings_and_chrome",
                "live_or_explained_response",
            ),
            vec![
                bs("pf-theme-shell", ShellUi, DesktopFull),
                bs("pf-theme-settings", SettingsUi, DesktopFull),
                bs("pf-theme-product", ProductUi, RemoteProjected),
            ],
        ),
        spec(
            "credential-wording/truthful-non-leaky",
            "Credential-store wording",
            CredentialStoreWording,
            facets(
                "credential_wording",
                "credential_store_wording",
                "credential_store_wording_registry",
                "macos_windows_linux_adaptive",
                "auth_and_credential_dialog",
                "truthful_non_leaky_wording",
            ),
            vec![
                bs("pf-credential-auth", AuthUi, DesktopFull),
                bs("pf-credential-settings", SettingsUi, DesktopFull),
                bs("pf-credential-support", SupportExport, ExportedRedacted),
            ],
        ),
        spec(
            "input-method/text-and-trust-fidelity",
            "IME / dead-key / AltGr / dictation / emoji / layout input",
            InputMethod,
            facets(
                "input_fidelity",
                "input_method",
                "input_method_behavior_registry",
                "macos_windows_linux_adaptive",
                "text_input_field",
                "text_and_trust_fidelity_preserved",
            ),
            vec![
                bs("pf-input-input", InputUi, DesktopFull),
                bs("pf-input-onboarding", Onboarding, CompactNarrowed),
                bs("pf-input-product", ProductUi, RemoteProjected),
            ],
        ),
    ]
}

/// Builds all consumer bindings, applying `rep` to override a binding's representation.
fn build_bindings<F>(rep: F) -> Vec<PlatformFitConsumerBinding>
where
    F: Fn(&str, PlatformFitRepresentation) -> PlatformFitRepresentation,
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

fn trust_review() -> PlatformFitSharedConsumersTrustReview {
    PlatformFitSharedConsumersTrustReview {
        family_reuse_proven_by_fixtures: true,
        same_object_same_platform_fit_across_surfaces: true,
        platform_fit_role_words_stay_in_frozen_vocabulary: true,
        adaptation_roles_never_change_command_or_permission_meaning: true,
        platform_wording_never_changes_command_or_permission_meaning: true,
        primary_actions_never_hidden_only_in_os_chrome: true,
        credentials_never_fall_back_to_plaintext_silently: true,
        input_methods_never_corrupt_text_or_trust_fidelity: true,
        screenshots_and_docs_never_mislabel_shortcut_or_path_verb: true,
        narrowing_disclosed_across_representations: true,
        support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> PlatformFitSharedConsumersProjection {
    PlatformFitSharedConsumersProjection {
        shell_ui_consumes_shared_platform_fit: true,
        settings_ui_consumes_shared_platform_fit: true,
        auth_ui_consumes_shared_platform_fit: true,
        input_ui_consumes_shared_platform_fit: true,
        docs_help_consumes_shared_platform_fit: true,
        support_export_consumes_shared_platform_fit: true,
        product_ui_consumes_shared_platform_fit: true,
        every_family_adopted_by_two_or_more_consumers: true,
        platform_fit_identical_for_same_object: true,
        narrowing_disclosed_not_hidden: true,
        export_maps_back_to_one_platform_fit_family: true,
    }
}

fn proof_freshness() -> PlatformFitSharedConsumersProofFreshness {
    PlatformFitSharedConsumersProofFreshness {
        proof_freshness_slo_hours: M5_PLATFORM_FIT_SHARED_CONSUMERS_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_PLATFORM_FIT_SHARED_CONSUMERS_SCHEMA_REF.to_owned(),
        M5_PLATFORM_FIT_SHARED_CONSUMERS_DOC_REF.to_owned(),
        M5_PLATFORM_FIT_MATRIX_SCHEMA_REF.to_owned(),
        M5_PLATFORM_FIT_MATRIX_DOC_REF.to_owned(),
    ];
    // The six families map to three canonical domain schemas; include each distinct one once.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for family in M5PlatformFitFamily::ALL {
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
    consumer_bindings: Vec<PlatformFitConsumerBinding>,
) -> M5PlatformFitSharedConsumersPacket {
    M5PlatformFitSharedConsumersPacket::new(M5PlatformFitSharedConsumersPacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        consumer_bindings,
        downgrade_triggers: PlatformFitSharedConsumersDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5PlatformFitConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in platform-fit shared-consumer parity packet.
pub fn seeded_m5_platform_fit_shared_consumers() -> M5PlatformFitSharedConsumersPacket {
    packet_from_bindings(
        M5_PLATFORM_FIT_SHARED_CONSUMERS_PACKET_ID,
        "M5 platform-fit shared consumers (one convention across surfaces)",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same objects with two more desktop surfaces narrowed to compact and remote
/// representations, proving grammar survives compact and remote forms.
pub fn seeded_m5_platform_fit_shared_consumers_compact_remote_narrowed(
) -> M5PlatformFitSharedConsumersPacket {
    packet_from_bindings(
        "m5-platform-fit-shared-consumers:compact-remote:0001",
        "M5 platform-fit shared consumers (compact / remote narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "pf-shortcut-settings" => PlatformFitRepresentation::CompactNarrowed,
            "pf-convention-docs" => PlatformFitRepresentation::RemoteProjected,
            _ => default,
        }),
    )
}

/// Fixture: the same objects with two more surfaces narrowed to exported, export-safe
/// representations, proving grammar survives into exported forms.
pub fn seeded_m5_platform_fit_shared_consumers_exported_redaction_narrowed(
) -> M5PlatformFitSharedConsumersPacket {
    packet_from_bindings(
        "m5-platform-fit-shared-consumers:exported-redaction:0001",
        "M5 platform-fit shared consumers (exported redaction narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "pf-theme-settings" => PlatformFitRepresentation::ExportedRedacted,
            "pf-credential-settings" => PlatformFitRepresentation::ExportedRedacted,
            _ => default,
        }),
    )
}
