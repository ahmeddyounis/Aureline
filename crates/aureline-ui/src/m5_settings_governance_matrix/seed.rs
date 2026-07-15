//! Canonical seed builders for the frozen M5 settings-governance matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code matrix, the artifact, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical settings-governance matrix.
pub const M5_SETTINGS_GOVERNANCE_MATRIX_PACKET_ID: &str = "m5-settings-governance:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every family must be able to show.
fn mandatory_labels() -> Vec<M5SettingsGovernanceRequiredLabel> {
    M5SettingsGovernanceRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a family carries.
fn labels_with(
    extra: &[M5SettingsGovernanceRequiredLabel],
) -> Vec<M5SettingsGovernanceRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every family filled in and every family-specific vocabulary left
/// empty for the caller to populate.
fn base_row(
    settings_governance_family: M5SettingsGovernanceFamily,
    qualification: M5SettingsGovernanceQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5SettingsGovernanceRow {
    M5SettingsGovernanceRow {
        settings_governance_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5SettingsGovernanceSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5SettingsGovernanceDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        semantic_roles: vec![],
        resolve_setting_roles: vec![],
        write_setting_roles: vec![],
        sync_scope_roles: vec![],
        migrate_schema_roles: vec![],
        rollout_capability_roles: vec![],
        degraded_reasons: M5SettingsGovernanceDegradedReason::ALL.to_vec(),
        accessibility_routes: M5SettingsGovernanceAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5SettingsGovernanceConsumerSurface::SupportExport,
            M5SettingsGovernanceConsumerSurface::DocsHelp,
        ],
        downgrade_triggers: vec![M5SettingsGovernanceDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        recycles_a_retired_setting_id: false,
        rewrites_a_scoped_write_into_a_broader_scope: false,
        silently_overwrites_locked_or_machine_only_state_during_sync: false,
        hides_lifecycle_or_experiment_dependency_behind_unpublished_markers: false,
        hides_kill_switch_or_policy_disable_cause_behind_generic_unavailable_copy: false,
    }
}

fn settings_governance_rows() -> Vec<M5SettingsGovernanceRow> {
    use M5SettingsGovernanceConsumerSurface as C;
    use M5SettingsGovernanceDowngradeTrigger as D;
    use M5SettingsGovernanceFamily as F;
    use M5SettingsGovernanceQualificationClass as Q;
    use M5SettingsGovernanceRequiredLabel as L;
    use M5SettingsGovernanceRole as R;

    let mut rows = Vec::new();

    // 1. Resolve setting.
    let mut row = base_row(
        F::ResolveSetting,
        Q::Stable,
        "Settings-resolver owner",
        "One resolve-setting profile naming the effective value resolved from the winning scope, the shadowed values and scope chain kept inspectable, the restart posture and lock source disclosed, and the stable setting ID preserved so resolving a setting stays inspectable and never recycles a retired setting ID",
        "evidence:m5-resolve-setting-parity:001",
        &[
            M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
            M5_SETTING_DEFINITION_DOMAIN_SCHEMA_REF,
            M5_EFFECTIVE_SETTING_SCHEMA_REF,
        ],
    );
    row.resolve_setting_roles = M5ResolveSettingRole::ALL.to_vec();
    row.semantic_roles = vec![R::SettingDefinition, R::EffectiveResolution];
    row.required_labels = labels_with(&[L::WinningScope]);
    row.consumer_surfaces = vec![
        C::SettingsResolver,
        C::ShellUi,
        C::PolicyService,
        C::Diagnostics,
        C::SupportExport,
        C::DocsHelp,
    ];
    row.downgrade_triggers = vec![
        D::RecycledARetiredSettingId,
        D::WinningScopeUnstated,
        D::ScopeBoundaryDriftedBySurface,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Write setting.
    let mut row = base_row(
        F::WriteSetting,
        Q::Stable,
        "Settings-write owner",
        "One write-setting profile naming the write intent targeting the chosen artifact and scope, the preview / checkpoint / rollback evidence created, the material behavior change disclosed before apply, and the chosen scope preserved so a write lands only in the chosen artifact and scope and never widens a scoped write into a broader scope because it is easier downstream",
        "evidence:m5-write-setting-parity:001",
        &[
            M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
            M5_SETTING_WRITE_INTENT_DOMAIN_SCHEMA_REF,
            M5_EFFECTIVE_SETTING_SCHEMA_REF,
        ],
    );
    row.write_setting_roles = M5WriteSettingRole::ALL.to_vec();
    row.semantic_roles = vec![R::WriteIntent, R::PolicyConstraint];
    row.required_labels = labels_with(&[L::WriteIntent]);
    row.consumer_surfaces = vec![
        C::SettingsResolver,
        C::ShellUi,
        C::PolicyService,
        C::Diagnostics,
        C::SupportExport,
        C::CliExport,
    ];
    row.downgrade_triggers = vec![
        D::RewroteAScopedWriteIntoABroaderScope,
        D::WriteIntentUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Sync scope.
    let mut row = base_row(
        F::SyncScope,
        Q::Stable,
        "Sync-service owner",
        "One sync-scope profile naming the sync scope bundle and session resolved, the conflict packet surfaced rather than auto-overwritten, the local authoritative state preserved during an outage, and machine-only state never marked portable so syncing a scope bundle never silently overwrites local authoritative state and never lets machine-only state masquerade as portable",
        "evidence:m5-sync-scope-parity:001",
        &[
            M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
            M5_SYNC_CONFLICT_PACKET_DOMAIN_SCHEMA_REF,
            M5_EFFECTIVE_SETTING_SCHEMA_REF,
        ],
    );
    row.sync_scope_roles = M5SyncScopeRole::ALL.to_vec();
    row.semantic_roles = vec![R::SyncConflict, R::PolicyConstraint];
    row.required_labels = labels_with(&[L::WinningScope]);
    row.consumer_surfaces = vec![
        C::SyncService,
        C::ShellUi,
        C::PolicyService,
        C::Diagnostics,
        C::SupportExport,
        C::CliExport,
    ];
    row.downgrade_triggers = vec![
        D::SilentlyOverwroteLockedOrMachineOnlyStateDuringSync,
        D::SyncConflictRuleUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Migrate schema.
    let mut row = base_row(
        F::MigrateSchema,
        Q::Stable,
        "Migration-service owner",
        "One migrate-schema profile naming the schema-migration record resolved, the setting-ID continuity preserved across versions, the migration preview shown before rewrite, and the reversible migration checkpoint recorded so migrating a settings schema preserves setting-ID continuity and never silently rewrites a schema without a checkpoint",
        "evidence:m5-migrate-schema-parity:001",
        &[
            M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
            M5_SETTING_DEFINITION_DOMAIN_SCHEMA_REF,
            M5_EFFECTIVE_SETTING_SCHEMA_REF,
        ],
    );
    row.migrate_schema_roles = M5MigrateSchemaRole::ALL.to_vec();
    row.semantic_roles = vec![R::SettingDefinition, R::SchemaMigration];
    row.required_labels = labels_with(&[L::WinningScope]);
    row.consumer_surfaces = vec![
        C::SettingsResolver,
        C::PolicyService,
        C::Diagnostics,
        C::SupportExport,
        C::DocsHelp,
        C::CliExport,
    ];
    row.downgrade_triggers = vec![
        D::RecycledARetiredSettingId,
        D::ScopeBoundaryDriftedBySurface,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Rollout capability.
    let mut row = base_row(
        F::RolloutCapability,
        Q::Stable,
        "Capability-lifecycle owner",
        "One rollout-capability profile naming the capability lifecycle state resolved, the Labs and rollout dependency markers published, the kill-switch and policy-disable cause explained, and the disabled state preserving user data so a capability rollout keeps lifecycle and experiment dependencies visible and never hides a kill-switch or policy-disable cause behind generic unavailable copy",
        "evidence:m5-rollout-capability-parity:001",
        &[
            M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
            M5_CAPABILITY_LIFECYCLE_DOMAIN_SCHEMA_REF,
            M5_CAPABILITY_LIFECYCLE_LANDED_SCHEMA_REF,
        ],
    );
    row.rollout_capability_roles = M5RolloutCapabilityRole::ALL.to_vec();
    row.semantic_roles = vec![R::CapabilityLifecycle, R::PolicyConstraint];
    row.required_labels = labels_with(&[L::LifecycleState]);
    row.consumer_surfaces = vec![
        C::CapabilityService,
        C::PolicyService,
        C::ShellUi,
        C::Diagnostics,
        C::SupportExport,
        C::CliExport,
    ];
    row.downgrade_triggers = vec![
        D::HidLifecycleOrExperimentDependencyBehindUnpublishedMarkers,
        D::HidKillSwitchOrPolicyDisableCauseBehindGenericUnavailableCopy,
        D::LifecycleStateUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5SettingsGovernanceGovernanceReview {
    M5SettingsGovernanceGovernanceReview {
        setting_definition_and_effective_resolution_stay_separately_inspectable: true,
        setting_ids_are_never_recycled: true,
        scoped_writes_are_never_widened_into_broader_scope: true,
        winning_scope_shadowed_values_restart_posture_and_lock_source_stay_inspectable: true,
        writes_land_only_in_chosen_artifact_and_scope_with_preview_checkpoint_rollback: true,
        sync_never_silently_overwrites_local_authoritative_state_during_outages: true,
        machine_only_state_never_masquerades_as_portable: true,
        lifecycle_and_experiment_dependencies_stay_visible_across_surfaces: true,
        kill_switch_and_disabled_by_policy_states_preserve_user_data_and_explain_themselves: true,
        every_family_declares_deployment_contexts: true,
        every_family_declares_accessibility_route: true,
        support_export_reads_single_settings_governance_source: true,
        settings_shell_diagnostics_admin_bind_to_single_settings_governance_source: true,
        later_rows_cannot_invent_parallel_settings_governance_vocabulary: true,
        configuration_truth_survives_zoom_and_high_contrast: true,
        claims_narrow_automatically_when_registry_missing_or_stale: true,
    }
}

fn consumer_projection() -> M5SettingsGovernanceConsumerProjection {
    M5SettingsGovernanceConsumerProjection {
        settings_and_shell_consume_shared_settings_governance_truth: true,
        diagnostics_and_admin_consume_shared_policy_and_lifecycle_boundaries: true,
        sync_and_capability_services_consume_shared_write_intent_and_conflict_classes: true,
        docs_help_and_screenshots_read_single_settings_governance_source: true,
        labs_experiments_and_kill_switches_bind_to_shared_capability_lifecycle_rule: true,
        support_export_reads_single_settings_governance_source: true,
    }
}

fn proof_freshness() -> M5SettingsGovernanceProofFreshness {
    M5SettingsGovernanceProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SettingsGovernanceReleasePosture {
    M5SettingsGovernanceReleasePosture {
        proof_packet_ref: M5_SETTINGS_GOVERNANCE_ARTIFACT_REF.to_owned(),
        settings_governance_audit_ref: M5_SETTINGS_GOVERNANCE_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_DOC_REF,
        M5_SETTING_DEFINITION_DOMAIN_SCHEMA_REF,
        M5_SETTING_WRITE_INTENT_DOMAIN_SCHEMA_REF,
        M5_SYNC_CONFLICT_PACKET_DOMAIN_SCHEMA_REF,
        M5_CAPABILITY_LIFECYCLE_DOMAIN_SCHEMA_REF,
        M5_EFFECTIVE_SETTING_SCHEMA_REF,
        M5_CAPABILITY_LIFECYCLE_LANDED_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 settings-governance matrix packet.
pub fn seeded_m5_settings_governance_matrix() -> M5SettingsGovernanceMatrixPacket {
    M5SettingsGovernanceMatrixPacket::new(M5SettingsGovernanceMatrixPacketInput {
        packet_id: M5_SETTINGS_GOVERNANCE_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 setting-definition, write-intent, sync-conflict, and capability-lifecycle matrix"
                .to_owned(),
        settings_governance_rows: settings_governance_rows(),
        vocabulary_set: M5SettingsGovernanceVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: sync scope is held at Beta because local-authoritative-state continuity is not yet
/// proven across every configuration context; every family stays visible.
pub fn seeded_m5_settings_governance_matrix_sync_scope_beta_narrowed(
) -> M5SettingsGovernanceMatrixPacket {
    let mut packet = seeded_m5_settings_governance_matrix();
    packet.packet_id = "m5-settings-governance:sync-scope-beta:0001".to_owned();
    let row = packet
        .settings_governance_rows
        .iter_mut()
        .find(|row| row.settings_governance_family == M5SettingsGovernanceFamily::SyncScope)
        .expect("sync-scope row present");
    row.qualification = M5SettingsGovernanceQualificationClass::Beta;
    packet
}

/// Narrowed variant: rollout capability is narrowed to Preview pending complete capability-lifecycle
/// evidence across every configuration context; every family stays visible.
pub fn seeded_m5_settings_governance_matrix_rollout_capability_preview_narrowed(
) -> M5SettingsGovernanceMatrixPacket {
    let mut packet = seeded_m5_settings_governance_matrix();
    packet.packet_id = "m5-settings-governance:rollout-capability-preview:0001".to_owned();
    let row = packet
        .settings_governance_rows
        .iter_mut()
        .find(|row| row.settings_governance_family == M5SettingsGovernanceFamily::RolloutCapability)
        .expect("rollout-capability row present");
    row.qualification = M5SettingsGovernanceQualificationClass::Preview;
    packet
}
