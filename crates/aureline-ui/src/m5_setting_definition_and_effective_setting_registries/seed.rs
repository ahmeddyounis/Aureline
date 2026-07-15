//! Canonical seed builders for the M5 setting-definition and effective-setting registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean setting-definition and effective-setting entries are
//! built so the one stable setting-definition object resolving per setting, stable setting IDs staying
//! non-recycled, the sensitivity posture disclosed before any sensitive setting is surfaced, the canonical /
//! accessible / audit resolution forms, and the complete resolved-value / shadow-chain / lock-state /
//! validation-status / restart-state / capability-availability / last-applied-revision effective-setting object
//! are proven across the settings-resolver, shell, sync, policy, diagnostics, and support surfaces without any
//! hand-copied per-setting assumption, ID recycle, incomplete object, hidden shadow chain, or resolution-form
//! gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_PACKET_ID: &str =
    "m5-setting-definition-and-effective-setting-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn definition(input: M5SettingDefinitionEntryResolutionInput) -> M5ResolvedSettingDefinitionEntry {
    resolve_setting_definition_entry(input).expect("seed setting-definition entry resolves")
}

fn record(input: M5EffectiveSettingEntryResolutionInput) -> M5ResolvedEffectiveSettingEntry {
    resolve_effective_setting_entry(input).expect("seed effective-setting entry resolves")
}

fn all_forms() -> Vec<M5SettingResolutionForm> {
    M5SettingResolutionForm::ALL.to_vec()
}

// -- Clean setting-definition entries (stable object, non-recycled ID, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_definition_base(
    entry_id: &str,
    setting_binding_id: &str,
    token_name: &str,
    semantic_role: M5SettingsGovernanceRole,
    setting_definition_type: M5SettingDefinitionKind,
    surface_context: M5SettingSurfaceContext,
    stable_setting_id: &str,
    allowed_scopes: &str,
    declared_default: &str,
    migration_aliases: &str,
    restart_posture: &str,
    sensitivity_class: &str,
    capability_dependencies: &str,
) -> M5SettingDefinitionEntryResolutionInput {
    M5SettingDefinitionEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        setting_binding_id: setting_binding_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        setting_definition_type,
        surface_context,
        resolution_form_coverage: all_forms(),
        stable_setting_id: stable_setting_id.to_owned(),
        allowed_scopes: allowed_scopes.to_owned(),
        declared_default: declared_default.to_owned(),
        migration_aliases: migration_aliases.to_owned(),
        restart_posture: restart_posture.to_owned(),
        sensitivity_class: sensitivity_class.to_owned(),
        capability_dependencies: capability_dependencies.to_owned(),
        bound_to_registry: true,
        setting_id_preserved: true,
        is_sensitive_setting: false,
        sensitivity_disclosed: true,
        proof_fresh: true,
    }
}

fn definition_settings_boolean_clean() -> M5ResolvedSettingDefinitionEntry {
    definition(clean_definition_base(
        "definition:settings:boolean",
        "settings.acme.editor.format-on-save",
        "setting.definition.editor.format_on_save",
        M5SettingsGovernanceRole::SettingDefinition,
        M5SettingDefinitionKind::BooleanSetting,
        M5SettingSurfaceContext::SettingsSurface,
        "editor.format_on_save",
        "scopes.machine-user-workspace",
        "default.false",
        "alias.editor.formatOnSave.v1",
        "restart.none",
        "sensitivity.public",
        "capability.editor.core",
    ))
}

fn definition_shell_enum_clean() -> M5ResolvedSettingDefinitionEntry {
    definition(clean_definition_base(
        "definition:shell:enum",
        "settings.acme.workbench.theme-mode",
        "setting.definition.workbench.theme_mode",
        M5SettingsGovernanceRole::EffectiveResolution,
        M5SettingDefinitionKind::EnumSetting,
        M5SettingSurfaceContext::ShellSurface,
        "workbench.theme_mode",
        "scopes.user-workspace",
        "default.system",
        "alias.workbench.themeMode.v1",
        "restart.none",
        "sensitivity.public",
        "capability.workbench.core",
    ))
}

fn definition_diagnostics_number_clean() -> M5ResolvedSettingDefinitionEntry {
    definition(clean_definition_base(
        "definition:diagnostics:number",
        "settings.acme.telemetry.sample-rate",
        "setting.definition.telemetry.sample_rate",
        M5SettingsGovernanceRole::SchemaMigration,
        M5SettingDefinitionKind::NumberSetting,
        M5SettingSurfaceContext::DiagnosticsSurface,
        "telemetry.sample_rate",
        "scopes.machine",
        "default.0",
        "alias.telemetry.sampleRate.v1",
        "restart.on-next-launch",
        "sensitivity.internal",
        "capability.telemetry.core",
    ))
}

fn definition_admin_path_clean() -> M5ResolvedSettingDefinitionEntry {
    // A path setting is sensitivity-bearing and discloses its redaction posture before it is surfaced.
    let mut base = clean_definition_base(
        "definition:admin:path",
        "settings.acme.tools.plugin-root",
        "setting.definition.tools.plugin_root",
        M5SettingsGovernanceRole::PolicyConstraint,
        M5SettingDefinitionKind::PathSetting,
        M5SettingSurfaceContext::AdminSurface,
        "tools.plugin_root",
        "scopes.machine-user",
        "default.redacted-path",
        "alias.tools.pluginRoot.v1",
        "restart.on-next-launch",
        "sensitivity.location-bearing",
        "capability.tools.core",
    );
    base.is_sensitive_setting = true;
    base.sensitivity_disclosed = true;
    definition(base)
}

fn definition_support_secretref_clean() -> M5ResolvedSettingDefinitionEntry {
    // A secret-reference setting is sensitivity-bearing and discloses its redaction posture; the reference
    // handle never carries the secret itself.
    let mut base = clean_definition_base(
        "definition:support:secretref",
        "settings.acme.sync.credential-handle",
        "setting.definition.sync.credential_handle",
        M5SettingsGovernanceRole::PolicyConstraint,
        M5SettingDefinitionKind::SecretReferenceSetting,
        M5SettingSurfaceContext::SupportOrExportForm,
        "sync.credential_handle",
        "scopes.machine",
        "default.redacted-handle",
        "alias.sync.credentialHandle.v1",
        "restart.none",
        "sensitivity.credential-reference",
        "capability.sync.core",
    );
    base.is_sensitive_setting = true;
    base.sensitivity_disclosed = true;
    definition(base)
}

// -- Degraded setting-definition entries --------------------------------------------------------

/// Degraded definition entry: the resolved definition object is incomplete — the declared default is unstated.
fn definition_object_incomplete() -> M5ResolvedSettingDefinitionEntry {
    let mut base = clean_definition_base(
        "definition:settings:incomplete",
        "settings.acme.editor.format-on-save",
        "setting.definition.editor.format_on_save",
        M5SettingsGovernanceRole::SettingDefinition,
        M5SettingDefinitionKind::BooleanSetting,
        M5SettingSurfaceContext::SettingsSurface,
        "editor.format_on_save",
        "scopes.machine-user-workspace",
        "default.false",
        "alias.editor.formatOnSave.v1",
        "restart.none",
        "sensitivity.public",
        "capability.editor.core",
    );
    base.declared_default = "   ".to_owned();
    definition(base)
}

/// Degraded definition entry: the stable setting ID was recycled into a different meaning.
fn definition_id_recycled() -> M5ResolvedSettingDefinitionEntry {
    let mut base = clean_definition_base(
        "definition:sync:id-recycled",
        "settings.acme.telemetry.sample-rate",
        "setting.definition.telemetry.sample_rate",
        M5SettingsGovernanceRole::SchemaMigration,
        M5SettingDefinitionKind::NumberSetting,
        M5SettingSurfaceContext::DiagnosticsSurface,
        "telemetry.sample_rate",
        "scopes.machine",
        "default.0",
        "alias.telemetry.sampleRate.v1",
        "restart.on-next-launch",
        "sensitivity.internal",
        "capability.telemetry.core",
    );
    base.setting_id_preserved = false;
    definition(base)
}

/// Degraded definition entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn definition_unbound() -> M5ResolvedSettingDefinitionEntry {
    let mut base = clean_definition_base(
        "definition:policy:unbound",
        "settings.acme.tools.plugin-root",
        "setting.definition.tools.plugin_root",
        M5SettingsGovernanceRole::PolicyConstraint,
        M5SettingDefinitionKind::PathSetting,
        M5SettingSurfaceContext::AdminSurface,
        "tools.plugin_root",
        "scopes.machine-user",
        "default.redacted-path",
        "alias.tools.pluginRoot.v1",
        "restart.on-next-launch",
        "sensitivity.location-bearing",
        "capability.tools.core",
    );
    base.is_sensitive_setting = true;
    base.sensitivity_disclosed = true;
    base.bound_to_registry = false;
    definition(base)
}

/// Degraded definition entry: the canonical / accessible / audit resolution-form coverage is incomplete.
fn definition_form_incomplete() -> M5ResolvedSettingDefinitionEntry {
    let mut base = clean_definition_base(
        "definition:shell:form-incomplete",
        "settings.acme.workbench.theme-mode",
        "setting.definition.workbench.theme_mode",
        M5SettingsGovernanceRole::EffectiveResolution,
        M5SettingDefinitionKind::EnumSetting,
        M5SettingSurfaceContext::ShellSurface,
        "workbench.theme_mode",
        "scopes.user-workspace",
        "default.system",
        "alias.workbench.themeMode.v1",
        "restart.none",
        "sensitivity.public",
        "capability.workbench.core",
    );
    base.resolution_form_coverage = vec![M5SettingResolutionForm::CanonicalObject];
    definition(base)
}

/// Degraded definition entry: the canonical registry token name is unstated.
fn definition_token_unstated() -> M5ResolvedSettingDefinitionEntry {
    let mut base = clean_definition_base(
        "definition:support:token-unstated",
        "settings.acme.sync.credential-handle",
        "  ",
        M5SettingsGovernanceRole::PolicyConstraint,
        M5SettingDefinitionKind::SecretReferenceSetting,
        M5SettingSurfaceContext::SupportOrExportForm,
        "sync.credential_handle",
        "scopes.machine",
        "default.redacted-handle",
        "alias.sync.credentialHandle.v1",
        "restart.none",
        "sensitivity.credential-reference",
        "capability.sync.core",
    );
    base.is_sensitive_setting = true;
    base.sensitivity_disclosed = true;
    base.token_name = "  ".to_owned();
    definition(base)
}

// -- Clean effective-setting entries ------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_record_base(
    entry_id: &str,
    setting_ref: &str,
    token_name: &str,
    semantic_role: M5SettingsGovernanceRole,
    winning_scope: M5EffectiveSettingScope,
    surface_context: M5SettingSurfaceContext,
    resolved_value_summary: &str,
    shadow_chain: &str,
    lock_or_constraint_state: &str,
    validation_status: &str,
    restart_state: &str,
    capability_availability: &str,
    last_applied_revision: &str,
) -> M5EffectiveSettingEntryResolutionInput {
    M5EffectiveSettingEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        setting_ref: setting_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        winning_scope,
        surface_context,
        resolution_form_coverage: all_forms(),
        resolved_value_summary: resolved_value_summary.to_owned(),
        shadow_chain: shadow_chain.to_owned(),
        lock_or_constraint_state: lock_or_constraint_state.to_owned(),
        validation_status: validation_status.to_owned(),
        restart_state: restart_state.to_owned(),
        capability_availability: capability_availability.to_owned(),
        last_applied_revision: last_applied_revision.to_owned(),
        keeps_shadow_chain_visible: true,
        resolution_is_truthful: true,
        lock_present: false,
        lock_source_disclosed: false,
        machine_only_value_present: false,
        machine_only_flagged_not_portable: false,
        proof_fresh: true,
    }
}

fn record_machine_settings_clean() -> M5ResolvedEffectiveSettingEntry {
    record(clean_record_base(
        "record:settings:machine",
        "editor.format_on_save",
        "effective.editor.format_on_save",
        M5SettingsGovernanceRole::EffectiveResolution,
        M5EffectiveSettingScope::MachineScope,
        M5SettingSurfaceContext::SettingsSurface,
        "value.true",
        "shadow.user-default-lost",
        "lock.none",
        "validation.ok",
        "restart.none",
        "capability.available",
        "revision.0007",
    ))
}

fn record_user_shell_clean() -> M5ResolvedEffectiveSettingEntry {
    // A locked value discloses its lock source rather than masking it.
    let mut base = clean_record_base(
        "record:shell:user",
        "workbench.theme_mode",
        "effective.workbench.theme_mode",
        M5SettingsGovernanceRole::PolicyConstraint,
        M5EffectiveSettingScope::UserScope,
        M5SettingSurfaceContext::ShellSurface,
        "value.dark",
        "shadow.workspace-and-default-lost",
        "lock.policy-managed",
        "validation.ok",
        "restart.none",
        "capability.available",
        "revision.0007",
    );
    base.lock_present = true;
    base.lock_source_disclosed = true;
    record(base)
}

fn record_workspace_diagnostics_clean() -> M5ResolvedEffectiveSettingEntry {
    // A machine-only value is flagged non-portable rather than masquerading as portable.
    let mut base = clean_record_base(
        "record:diagnostics:workspace",
        "telemetry.sample_rate",
        "effective.telemetry.sample_rate",
        M5SettingsGovernanceRole::EffectiveResolution,
        M5EffectiveSettingScope::WorkspaceScope,
        M5SettingSurfaceContext::DiagnosticsSurface,
        "value.0",
        "shadow.user-and-default-lost",
        "lock.none",
        "validation.ok",
        "restart.on-next-launch",
        "capability.available",
        "revision.0007",
    );
    base.machine_only_value_present = true;
    base.machine_only_flagged_not_portable = true;
    record(base)
}

fn record_user_admin_clean() -> M5ResolvedEffectiveSettingEntry {
    record(clean_record_base(
        "record:admin:user",
        "tools.plugin_root",
        "effective.tools.plugin_root",
        M5SettingsGovernanceRole::PolicyConstraint,
        M5EffectiveSettingScope::UserScope,
        M5SettingSurfaceContext::AdminSurface,
        "value.redacted-path",
        "shadow.machine-default-lost",
        "lock.none",
        "validation.ok",
        "restart.on-next-launch",
        "capability.available",
        "revision.0007",
    ))
}

fn record_machine_support_clean() -> M5ResolvedEffectiveSettingEntry {
    record(clean_record_base(
        "record:support:machine",
        "sync.credential_handle",
        "effective.sync.credential_handle",
        M5SettingsGovernanceRole::EffectiveResolution,
        M5EffectiveSettingScope::MachineScope,
        M5SettingSurfaceContext::SupportOrExportForm,
        "value.redacted-handle",
        "shadow.none",
        "lock.none",
        "validation.ok",
        "restart.none",
        "capability.available",
        "revision.0007",
    ))
}

// -- Degraded effective-setting entries ---------------------------------------------------------

/// Degraded record entry: the record would mask a locked value without disclosing its lock source — the
/// effective setting reads as trustworthy when it has quietly hidden the reason another scope lost.
fn record_hides_shadow() -> M5ResolvedEffectiveSettingEntry {
    let mut base = clean_record_base(
        "record:settings:hides-shadow",
        "editor.format_on_save",
        "effective.editor.format_on_save",
        M5SettingsGovernanceRole::EffectiveResolution,
        M5EffectiveSettingScope::MachineScope,
        M5SettingSurfaceContext::SettingsSurface,
        "value.true",
        "shadow.user-default-lost",
        "lock.policy-managed",
        "validation.ok",
        "restart.none",
        "capability.available",
        "revision.0007",
    );
    base.lock_present = true;
    base.lock_source_disclosed = false;
    record(base)
}

/// Degraded record entry: the canonical / accessible / audit resolution-form coverage of the record is
/// incomplete.
fn record_form_incomplete() -> M5ResolvedEffectiveSettingEntry {
    let mut base = clean_record_base(
        "record:shell:form-incomplete",
        "workbench.theme_mode",
        "effective.workbench.theme_mode",
        M5SettingsGovernanceRole::EffectiveResolution,
        M5EffectiveSettingScope::UserScope,
        M5SettingSurfaceContext::ShellSurface,
        "value.dark",
        "shadow.workspace-and-default-lost",
        "lock.none",
        "validation.ok",
        "restart.none",
        "capability.available",
        "revision.0007",
    );
    base.resolution_form_coverage = vec![M5SettingResolutionForm::CanonicalObject];
    record(base)
}

/// Degraded record entry: the winning scope is unclassified.
fn record_scope_unclassified() -> M5ResolvedEffectiveSettingEntry {
    record(clean_record_base(
        "record:policy:scope-unclassified",
        "tools.plugin_root",
        "effective.tools.plugin_root",
        M5SettingsGovernanceRole::EffectiveResolution,
        M5EffectiveSettingScope::ScopeUnclassified,
        M5SettingSurfaceContext::AdminSurface,
        "value.redacted-path",
        "shadow.machine-default-lost",
        "lock.none",
        "validation.ok",
        "restart.on-next-launch",
        "capability.available",
        "revision.0007",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5SettingDefinitionEffectiveSettingRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5SettingsGovernanceDowngradeTrigger>,
    setting_definition_entries: Vec<M5ResolvedSettingDefinitionEntry>,
    effective_setting_entries: Vec<M5ResolvedEffectiveSettingEntry>,
) -> M5SettingDefinitionEffectiveSettingRegistriesRow {
    M5SettingDefinitionEffectiveSettingRegistriesRow {
        consumer_surface,
        qualification: M5SettingsGovernanceQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5SettingsGovernanceDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5SettingsGovernanceRequiredLabel::Identity,
            M5SettingsGovernanceRequiredLabel::SemanticRole,
            M5SettingsGovernanceRequiredLabel::RegistryReference,
            M5SettingsGovernanceRequiredLabel::WinningScope,
            M5SettingsGovernanceRequiredLabel::LifecycleState,
        ],
        accessibility_routes: M5SettingsGovernanceAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5SettingAnatomyPart::ALL.to_vec(),
        export_fields: M5SettingExportField::ALL.to_vec(),
        downgrade_triggers,
        setting_definition_entries,
        effective_setting_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_SCHEMA_REF,
            M5_SETTING_DEFINITION_DOMAIN_SCHEMA_REF,
            M5_EFFECTIVE_SETTING_SCHEMA_REF,
        ]),
        recycles_a_retired_setting_id: false,
        resolves_an_effective_value_without_an_inspectable_shadow_chain: false,
        hides_restart_posture_lock_source_or_sensitivity_before_resolution: false,
        collapses_distinct_settings_scopes_into_one_resolution_path: false,
    }
}

fn registry_rows() -> Vec<M5SettingDefinitionEffectiveSettingRegistriesRow> {
    use M5SettingsGovernanceConsumerSurface as C;
    use M5SettingsGovernanceDowngradeTrigger as D;

    vec![
        base_row(
            C::SettingsResolver,
            "Settings-resolver owner",
            "The settings resolver resolves the boolean setting definition to one stable object — declared type, stable setting ID, allowed scopes, declared default, migration aliases, restart posture, sensitivity class, and capability dependencies — from the shared registry and derives the effective setting for the winning scope; a definition object missing its declared default and an effective setting that masks a locked value without disclosing its lock source degrade honestly instead of reading as a clean pass",
            "evidence:m5-settings-governance-settings-resolver:001",
            vec![
                D::RecycledARetiredSettingId,
                D::SilentlyOverwroteLockedOrMachineOnlyStateDuringSync,
                D::ProofStale,
            ],
            vec![
                definition_settings_boolean_clean(),
                definition_object_incomplete(),
            ],
            vec![record_machine_settings_clean(), record_hides_shadow()],
        ),
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell resolves the enum setting definition and renders the user-scope effective setting while keeping the shadow chain of scopes that lost visible; a resolution-form gap on a definition entry and on an effective setting is caught before a screenshot can reintroduce a false-truth reading",
            "evidence:m5-settings-governance-shell-ui:001",
            vec![
                D::RegistryReferenceUnstated,
                D::ScopeBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![definition_shell_enum_clean(), definition_form_incomplete()],
            vec![record_user_shell_clean(), record_form_incomplete()],
        ),
        base_row(
            C::SyncService,
            "Sync-service owner",
            "The sync service reports the number setting definition and the workspace-scope effective setting without manual reconstruction; a stable setting ID recycled into a different meaning is caught as an ID recycle before it can drift a scope",
            "evidence:m5-settings-governance-sync-service:001",
            vec![
                D::RecycledARetiredSettingId,
                D::ScopeBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![
                definition_diagnostics_number_clean(),
                definition_id_recycled(),
            ],
            vec![record_workspace_diagnostics_clean()],
        ),
        base_row(
            C::PolicyService,
            "Policy-service owner",
            "The policy service resolves the path setting definition while disclosing its sensitivity posture and bound to the registry; a definition that is a hand-copied per-entry assumption and an effective setting on an unclassified scope degrade honestly",
            "evidence:m5-settings-governance-policy-service:001",
            vec![
                D::ScopeBoundaryDriftedBySurface,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![definition_admin_path_clean(), definition_unbound()],
            vec![record_user_admin_clean(), record_scope_unclassified()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved setting-definition and effective-setting truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied settings table",
            "evidence:m5-settings-governance-diagnostics:001",
            vec![
                D::RegistryReferenceUnstated,
                D::ScopeBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![
                definition_diagnostics_number_clean(),
                definition_form_incomplete(),
            ],
            vec![record_user_shell_clean(), record_form_incomplete()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved setting-definition and effective-setting truth, so a hand-copied constant, an unstated registry token, an ID recycle, or a hidden shadow chain is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-settings-governance-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::WinningScopeUnstated,
                D::ProofStale,
            ],
            vec![
                definition_support_secretref_clean(),
                definition_token_unstated(),
            ],
            vec![record_machine_support_clean()],
        ),
    ]
}

fn governance_review() -> M5SettingDefinitionEffectiveSettingRegistriesGovernanceReview {
    M5SettingDefinitionEffectiveSettingRegistriesGovernanceReview {
        definition_registry_names_token_role_and_type: true,
        setting_resolves_to_stable_object_from_shared_registry: true,
        setting_id_type_scopes_default_and_sensitivity_published: true,
        stable_setting_ids_stay_non_recycled: true,
        effective_record_keeps_shadow_chain_visible_and_discloses_lock_source: true,
        sensitivity_disclosed_for_sensitive_settings: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        settings_shell_diagnostics_admin_read_single_source: true,
        definition_or_record_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5SettingDefinitionEffectiveSettingRegistriesConsumerProjection {
    M5SettingDefinitionEffectiveSettingRegistriesConsumerProjection {
        settings_and_shell_consume_shared_registries: true,
        diagnostics_and_admin_consume_shared_registries: true,
        sync_and_capability_services_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5SettingDefinitionEffectiveSettingRegistriesProofFreshness {
    M5SettingDefinitionEffectiveSettingRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SettingDefinitionEffectiveSettingRegistriesReleasePosture {
    M5SettingDefinitionEffectiveSettingRegistriesReleasePosture {
        proof_packet_ref: M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        settings_governance_audit_ref:
            M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_SCHEMA_REF,
        M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_DOC_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_DOC_REF,
        M5_SETTING_DEFINITION_DOMAIN_SCHEMA_REF,
        M5_EFFECTIVE_SETTING_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 setting-definition and effective-setting registries packet.
pub fn seeded_m5_setting_definition_and_effective_setting_registries(
) -> M5SettingDefinitionEffectiveSettingRegistriesPacket {
    M5SettingDefinitionEffectiveSettingRegistriesPacket::new(
        M5SettingDefinitionEffectiveSettingRegistriesPacketInput {
            packet_id: M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 setting-definition and effective-setting registries with one stable setting-definition object resolving per setting, stable setting IDs staying non-recycled, the sensitivity posture disclosed before any sensitive setting is surfaced, canonical / accessible / audit resolution-form coverage, and the complete resolved-value / shadow-chain / lock-state / validation-status / restart-state / capability-availability / last-applied-revision effective-setting object across settings-resolver, shell, sync, policy, diagnostics, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set:
                M5SettingDefinitionEffectiveSettingRegistriesVocabularySet::canonical(),
            governance_review: governance_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            release_posture: release_posture(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Narrowed variant: the settings-resolver row is held at Beta pending setting-definition parity on every
/// platform; every row stays visible and every example stays honest.
pub fn seeded_m5_setting_definition_and_effective_setting_registries_setting_definition_beta_narrowed(
) -> M5SettingDefinitionEffectiveSettingRegistriesPacket {
    let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
    packet.packet_id =
        "m5-setting-definition-and-effective-setting-registries:setting-definition-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5SettingsGovernanceConsumerSurface::SettingsResolver)
        .expect("settings-resolver row present");
    row.qualification = M5SettingsGovernanceQualificationClass::Beta;
    packet
}

/// Narrowed variant: the sync-service row is narrowed to Preview pending effective-setting parity on every
/// platform; every row stays visible and every example stays honest.
pub fn seeded_m5_setting_definition_and_effective_setting_registries_effective_setting_preview_narrowed(
) -> M5SettingDefinitionEffectiveSettingRegistriesPacket {
    let mut packet = seeded_m5_setting_definition_and_effective_setting_registries();
    packet.packet_id =
        "m5-setting-definition-and-effective-setting-registries:effective-setting-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5SettingsGovernanceConsumerSurface::SyncService)
        .expect("sync-service row present");
    row.qualification = M5SettingsGovernanceQualificationClass::Preview;
    packet
}
