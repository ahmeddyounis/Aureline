//! Canonical seed for the settings-governance shared-consumer parity packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix CSV,
//! Markdown summary, and narrowed fixtures. Every binding is derived from one per-profile
//! [`SettingsGovernanceStateFacetValues`] so the same configuration profile always carries the same grammar
//! across surfaces, and every narrowed representation derives its disclosure from
//! [`resolve_settings_governance_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

fn facets(
    settings_governance_role: &str,
    family: &str,
    registry_reference: &str,
    resolution_context: &str,
    surface_context: &str,
    evidence_continuity: &str,
) -> SettingsGovernanceStateFacetValues {
    SettingsGovernanceStateFacetValues {
        settings_governance_role_word: settings_governance_role.to_owned(),
        family_word: family.to_owned(),
        registry_reference_word: registry_reference.to_owned(),
        resolution_context_word: resolution_context.to_owned(),
        surface_context_word: surface_context.to_owned(),
        evidence_continuity_word: evidence_continuity.to_owned(),
    }
}

fn preserved_note_for(reason: SettingsGovernanceNarrowReason) -> String {
    match reason {
        SettingsGovernanceNarrowReason::CompactionNarrowed => {
            "settings-governance-role, family, registry-reference, resolution-context, surface-context, and evidence-continuity words preserved; only disclosure depth compacted"
        }
        SettingsGovernanceNarrowReason::RemoteProjectionNarrowed => {
            "all settings-governance grammar preserved; the family is projected from the remote source of truth"
        }
        SettingsGovernanceNarrowReason::ExportRedactionNarrowed => {
            "all settings-governance grammar preserved; only surrounding detail is redacted export-safe"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: SettingsGovernanceNarrowNextAction) -> String {
    match action {
        SettingsGovernanceNarrowNextAction::ExpandInDesktop => "Expand in the desktop surface",
        SettingsGovernanceNarrowNextAction::OpenRemoteSource => "Open the remote source",
        SettingsGovernanceNarrowNextAction::OpenFullDetail => "Open the full detail",
    }
    .to_owned()
}

fn binding_refs(family: M5SettingsGovernanceFamily) -> Vec<String> {
    vec![
        M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF.to_owned(),
        family.canonical_domain_schema_ref().to_owned(),
    ]
}

fn make_binding(
    binding_id: &str,
    governance_profile_id: &str,
    governance_profile_label: &str,
    family: M5SettingsGovernanceFamily,
    consumer: M5SettingsGovernanceConsumerSurface,
    representation: SettingsGovernanceRepresentation,
    state_facets: SettingsGovernanceStateFacetValues,
) -> SettingsGovernanceConsumerBinding {
    let disclosure = resolve_settings_governance_render_disclosure(representation);
    let narrow_note = disclosure.narrow_reason.map(|reason| {
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrow reason always carries a next action");
        SettingsGovernanceNarrowNote {
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

    SettingsGovernanceConsumerBinding {
        binding_id: binding_id.to_owned(),
        governance_profile_id: governance_profile_id.to_owned(),
        governance_profile_label: governance_profile_label.to_owned(),
        family,
        consumer,
        representation,
        state_facets,
        parity_state: disclosure.parity_state,
        narrow_note,
        remote_source_note,
        export_detail_note,
        recycles_a_retired_setting_id: false,
        rewrites_a_scoped_write_into_a_broader_scope: false,
        silently_overwrites_locked_or_machine_only_state_during_sync: false,
        hides_lifecycle_or_experiment_dependency_behind_unpublished_markers: false,
        hides_kill_switch_or_policy_disable_cause_behind_generic_unavailable_copy: false,
        source_contract_refs: binding_refs(family),
    }
}

/// One consumer-surface adoption of a configuration profile, before any representation override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5SettingsGovernanceConsumerSurface,
    representation: SettingsGovernanceRepresentation,
}

/// One configuration profile rendered across several consumer surfaces at one grammar.
struct ProfileSpec {
    profile_id: &'static str,
    profile_label: &'static str,
    family: M5SettingsGovernanceFamily,
    facets: SettingsGovernanceStateFacetValues,
    bindings: Vec<BindingSpec>,
}

fn spec(
    profile_id: &'static str,
    profile_label: &'static str,
    family: M5SettingsGovernanceFamily,
    facets: SettingsGovernanceStateFacetValues,
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
    consumer: M5SettingsGovernanceConsumerSurface,
    representation: SettingsGovernanceRepresentation,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        representation,
    }
}

/// The five configuration profiles — one per B143 settings-governance family — and the surfaces that adopt
/// each, drawn from the settings-resolver, shell, sync-service, policy-service, capability-service,
/// diagnostics, docs / help, CLI / export, and support-export consumers.
fn profile_specs() -> Vec<ProfileSpec> {
    use M5SettingsGovernanceConsumerSurface::*;
    use M5SettingsGovernanceFamily::*;
    use SettingsGovernanceRepresentation::*;

    let setting_definition_registry = "setting_definition_registry";
    let write_intent_registry = "write_intent_registry";
    let sync_conflict_registry = "sync_conflict_registry";
    let capability_lifecycle_registry = "capability_lifecycle_registry";
    let evidence_continuity = "evidence_preserved_and_cause_disclosed_before_applying";
    let settings_scoped_descriptor = "settings_scoped_descriptor";

    vec![
        spec(
            "resolve-setting/winning-scope-and-shadow-chain-inspectable",
            "Resolve setting (effective value from the winning scope, shadow chain inspectable)",
            ResolveSetting,
            facets(
                "effective_resolution",
                "resolve_setting",
                setting_definition_registry,
                "returning_profile",
                "settings_resolver_and_shell",
                settings_scoped_descriptor,
            ),
            vec![
                bs("sgsc-resolve-setting-resolver", SettingsResolver, DesktopFull),
                bs("sgsc-resolve-setting-shell", ShellUi, DesktopFull),
                bs("sgsc-resolve-setting-cli", CliExport, ExportedRedacted),
            ],
        ),
        spec(
            "write-setting/scope-preserved-with-recovery-evidence",
            "Write setting (write intent lands in the chosen scope with preview / checkpoint / rollback evidence)",
            WriteSetting,
            facets(
                "write_intent",
                "write_setting",
                write_intent_registry,
                "fresh_install",
                "settings_ui_and_policy",
                evidence_continuity,
            ),
            vec![
                bs("sgsc-write-setting-policy", PolicyService, DesktopFull),
                bs("sgsc-write-setting-shell", ShellUi, DesktopFull),
                bs("sgsc-write-setting-support", SupportExport, ExportedRedacted),
            ],
        ),
        spec(
            "sync-scope/local-authoritative-state-preserved-during-outage",
            "Sync scope (conflict packet surfaced field-by-field, local authoritative state preserved during an outage)",
            SyncScope,
            facets(
                "sync_conflict",
                "sync_scope",
                sync_conflict_registry,
                "offline_or_outage",
                "sync_service_and_outage_recovery",
                evidence_continuity,
            ),
            vec![
                bs("sgsc-sync-scope-sync", SyncService, DesktopFull),
                bs("sgsc-sync-scope-diagnostics", Diagnostics, DesktopFull),
                bs("sgsc-sync-scope-docs", DocsHelp, RemoteProjected),
            ],
        ),
        spec(
            "migrate-schema/setting-id-continuity-across-versions",
            "Migrate schema (schema-migration record preserves setting-ID continuity with a compare surface)",
            MigrateSchema,
            facets(
                "schema_migration",
                "migrate_schema",
                setting_definition_registry,
                "policy_managed_fleet",
                "migration_and_admin",
                settings_scoped_descriptor,
            ),
            vec![
                bs("sgsc-migrate-schema-capability", CapabilityService, RemoteProjected),
                bs("sgsc-migrate-schema-diagnostics", Diagnostics, DesktopFull),
                bs("sgsc-migrate-schema-sync", SyncService, DesktopFull),
            ],
        ),
        spec(
            "rollout-capability/kill-switch-and-policy-disable-self-explaining",
            "Rollout capability (lifecycle dependency published, kill-switch / policy-disable cause preserved)",
            RolloutCapability,
            facets(
                "capability_lifecycle",
                "rollout_capability",
                capability_lifecycle_registry,
                "resumed_after_sync_conflict",
                "capability_sheet_and_support",
                evidence_continuity,
            ),
            vec![
                bs("sgsc-rollout-capability-docs", DocsHelp, DesktopFull),
                bs("sgsc-rollout-capability-capability", CapabilityService, CompactNarrowed),
                bs("sgsc-rollout-capability-support", SupportExport, ExportedRedacted),
            ],
        ),
    ]
}

/// Builds all consumer bindings, applying `rep` to override a binding's representation.
fn build_bindings<F>(rep: F) -> Vec<SettingsGovernanceConsumerBinding>
where
    F: Fn(&str, SettingsGovernanceRepresentation) -> SettingsGovernanceRepresentation,
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

fn trust_review() -> SettingsGovernanceSharedConsumersTrustReview {
    SettingsGovernanceSharedConsumersTrustReview {
        family_reuse_proven_by_fixtures: true,
        same_profile_same_settings_governance_across_surfaces: true,
        settings_governance_role_words_stay_in_frozen_vocabulary: true,
        trust_roles_never_widen_scope_or_hide_cause: true,
        setting_id_never_recycled_across_surfaces: true,
        write_never_widens_a_scoped_write_into_a_broader_scope: true,
        sync_never_silently_overwrites_locked_or_machine_only_state: true,
        lifecycle_dependency_never_hidden_behind_unpublished_markers: true,
        kill_switch_or_policy_disable_cause_never_hidden_behind_generic_copy: true,
        narrowing_disclosed_across_representations: true,
        support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> SettingsGovernanceSharedConsumersProjection {
    SettingsGovernanceSharedConsumersProjection {
        settings_resolver_consumes_shared_settings_governance: true,
        shell_ui_consumes_shared_settings_governance: true,
        sync_service_consumes_shared_settings_governance: true,
        policy_service_consumes_shared_settings_governance: true,
        capability_service_consumes_shared_settings_governance: true,
        diagnostics_consumes_shared_settings_governance: true,
        docs_help_consumes_shared_settings_governance: true,
        cli_export_consumes_shared_settings_governance: true,
        support_export_consumes_shared_settings_governance: true,
        every_family_adopted_by_two_or_more_consumers: true,
        settings_governance_identical_for_same_profile: true,
        narrowing_disclosed_not_hidden: true,
        export_maps_back_to_one_settings_governance_family: true,
    }
}

fn proof_freshness() -> SettingsGovernanceSharedConsumersProofFreshness {
    SettingsGovernanceSharedConsumersProofFreshness {
        proof_freshness_slo_hours: M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_SCHEMA_REF.to_owned(),
        M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_DOC_REF.to_owned(),
        M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF.to_owned(),
        M5_SETTINGS_GOVERNANCE_MATRIX_DOC_REF.to_owned(),
    ];
    // The five families map to four canonical domain schemas; include each distinct one once.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for family in M5SettingsGovernanceFamily::ALL {
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
    consumer_bindings: Vec<SettingsGovernanceConsumerBinding>,
) -> M5SettingsGovernanceSharedConsumersPacket {
    M5SettingsGovernanceSharedConsumersPacket::new(M5SettingsGovernanceSharedConsumersPacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        consumer_bindings,
        downgrade_triggers: SettingsGovernanceSharedConsumersDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5SettingsGovernanceConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in settings-governance shared-consumer parity packet.
pub fn seeded_m5_settings_governance_shared_consumers() -> M5SettingsGovernanceSharedConsumersPacket
{
    packet_from_bindings(
        M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_PACKET_ID,
        "M5 settings-governance shared consumers (one registry across surfaces)",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same profiles with two more desktop surfaces narrowed to compact and remote
/// representations, proving grammar survives compact and remote forms.
pub fn seeded_m5_settings_governance_shared_consumers_compact_remote_narrowed(
) -> M5SettingsGovernanceSharedConsumersPacket {
    packet_from_bindings(
        "m5-settings-governance-shared-consumers:compact-remote:0001",
        "M5 settings-governance shared consumers (compact / remote narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "sgsc-resolve-setting-shell" => SettingsGovernanceRepresentation::CompactNarrowed,
            "sgsc-write-setting-policy" => SettingsGovernanceRepresentation::RemoteProjected,
            _ => default,
        }),
    )
}

/// Fixture: the same profiles with two more surfaces narrowed to exported, export-safe
/// representations, proving grammar survives into exported forms.
pub fn seeded_m5_settings_governance_shared_consumers_exported_redaction_narrowed(
) -> M5SettingsGovernanceSharedConsumersPacket {
    packet_from_bindings(
        "m5-settings-governance-shared-consumers:exported-redaction:0001",
        "M5 settings-governance shared consumers (exported redaction narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "sgsc-sync-scope-diagnostics" => SettingsGovernanceRepresentation::ExportedRedacted,
            "sgsc-write-setting-shell" => SettingsGovernanceRepresentation::ExportedRedacted,
            _ => default,
        }),
    )
}
