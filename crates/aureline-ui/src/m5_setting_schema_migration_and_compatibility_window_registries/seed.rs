//! Canonical seed builders for the M5 schema-migration and compatibility-window registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean schema-migration and compatibility-window entries are
//! built so the one migration record landing per version change, migration labels that never overstate fidelity,
//! the compare-before-apply surface materialized before any lossy or manual-review migration applies, the
//! canonical / accessible / audit resolution forms, and the complete window-source / supported-version-range /
//! deprecation-review / validation-status / review-state / docs-pointer / last-review-revision compatibility
//! window object are proven across the settings-resolver, shell, sync, policy, diagnostics, and support surfaces
//! without any hand-copied per-version assumption, overstated fidelity, incomplete record, masked window, or
//! resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_PACKET_ID: &str =
    "m5-setting-schema-migration-and-compatibility-window-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn migration(
    input: M5SchemaMigrationRecordEntryResolutionInput,
) -> M5ResolvedSchemaMigrationRecordEntry {
    resolve_schema_migration_record_entry(input).expect("seed schema-migration entry resolves")
}

fn window(input: M5CompatibilityWindowEntryResolutionInput) -> M5ResolvedCompatibilityWindowEntry {
    resolve_compatibility_window_entry(input).expect("seed compatibility-window entry resolves")
}

fn all_forms() -> Vec<M5ConfigMigrationResolutionForm> {
    M5ConfigMigrationResolutionForm::ALL.to_vec()
}

// -- Clean schema-migration entries (one record, fidelity honest, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_migration_base(
    entry_id: &str,
    migration_ref: &str,
    token_name: &str,
    semantic_role: M5SettingsGovernanceRole,
    fidelity_class: M5SchemaMigrationFidelityClass,
    surface_context: M5ConfigMigrationSurfaceContext,
    old_key_or_alias: &str,
    new_key: &str,
    transform: &str,
    compatibility_window: &str,
    rollback_note: &str,
    compare_before_apply_reference: &str,
    migration_provenance_reference: &str,
) -> M5SchemaMigrationRecordEntryResolutionInput {
    M5SchemaMigrationRecordEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        migration_ref: migration_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        fidelity_class,
        surface_context,
        resolution_form_coverage: all_forms(),
        old_key_or_alias: old_key_or_alias.to_owned(),
        new_key: new_key.to_owned(),
        transform: transform.to_owned(),
        compatibility_window: compatibility_window.to_owned(),
        rollback_note: rollback_note.to_owned(),
        compare_before_apply_reference: compare_before_apply_reference.to_owned(),
        migration_provenance_reference: migration_provenance_reference.to_owned(),
        bound_to_registry: true,
        fidelity_label_honest: true,
        is_lossy_or_manual_review: false,
        compare_surface_materialized: true,
        proof_fresh: true,
    }
}

fn migration_exact_upgrade_clean() -> M5ResolvedSchemaMigrationRecordEntry {
    migration(clean_migration_base(
        "migration:upgrade:exact",
        "settings.acme.editor.font-size@v1-to-v2",
        "migration.editor.font_size",
        M5SettingsGovernanceRole::SchemaMigration,
        M5SchemaMigrationFidelityClass::ExactMigration,
        M5ConfigMigrationSurfaceContext::UpgradeFlow,
        "old.editor.fontSize",
        "new.editor.font-size",
        "transform.rename-key-verbatim",
        "window.v1-through-v3",
        "rollback.restore-v1-key",
        "compare.before-apply-0007",
        "provenance.migration-record-0007",
    ))
}

fn migration_compatible_import_clean() -> M5ResolvedSchemaMigrationRecordEntry {
    migration(clean_migration_base(
        "migration:import:compatible",
        "settings.acme.workbench.theme-mode@v1-to-v2",
        "migration.workbench.theme_mode",
        M5SettingsGovernanceRole::SchemaMigration,
        M5SchemaMigrationFidelityClass::CompatibleMigration,
        M5ConfigMigrationSurfaceContext::ImportFlow,
        "old.workbench.themeMode",
        "new.workbench.theme-mode",
        "transform.coerce-enum-compatible",
        "window.v1-through-v3",
        "rollback.restore-v1-enum",
        "compare.before-apply-0007",
        "provenance.migration-record-0007",
    ))
}

fn migration_lossy_restore_clean() -> M5ResolvedSchemaMigrationRecordEntry {
    // A lossy migration materializes the compare-before-apply surface before it applies.
    let mut base = clean_migration_base(
        "migration:restore:lossy",
        "settings.acme.telemetry.sample-rate@v2-to-v3",
        "migration.telemetry.sample_rate",
        M5SettingsGovernanceRole::SchemaMigration,
        M5SchemaMigrationFidelityClass::LossyMigration,
        M5ConfigMigrationSurfaceContext::RestoreFlow,
        "old.telemetry.sampleRateBuckets",
        "new.telemetry.sample-rate",
        "transform.collapse-buckets-lossy",
        "window.v2-through-v3",
        "rollback.restore-v2-buckets",
        "compare.before-apply-0007",
        "provenance.migration-record-0007",
    );
    base.is_lossy_or_manual_review = true;
    base.compare_surface_materialized = true;
    migration(base)
}

fn migration_manual_review_downgrade_clean() -> M5ResolvedSchemaMigrationRecordEntry {
    // A manual-review migration materializes the compare-before-apply surface before it applies.
    let mut base = clean_migration_base(
        "migration:downgrade:manual-review",
        "settings.acme.tools.plugin-root@v3-to-v2",
        "migration.tools.plugin_root",
        M5SettingsGovernanceRole::SchemaMigration,
        M5SchemaMigrationFidelityClass::ManualReviewMigration,
        M5ConfigMigrationSurfaceContext::DowngradeFlow,
        "old.tools.pluginRoots",
        "new.tools.plugin-root",
        "transform.manual-review-required",
        "window.v2-through-v3",
        "rollback.restore-v3-roots",
        "compare.before-apply-0007",
        "provenance.migration-record-0007",
    );
    base.is_lossy_or_manual_review = true;
    base.compare_surface_materialized = true;
    migration(base)
}

fn migration_compatible_support_clean() -> M5ResolvedSchemaMigrationRecordEntry {
    migration(clean_migration_base(
        "migration:support:compatible",
        "settings.acme.sync.state-shape@v1-to-v2",
        "migration.sync.state_shape",
        M5SettingsGovernanceRole::SchemaMigration,
        M5SchemaMigrationFidelityClass::CompatibleMigration,
        M5ConfigMigrationSurfaceContext::SupportOrExportForm,
        "old.sync.stateShape",
        "new.sync.state-shape",
        "transform.reshape-compatible",
        "window.v1-through-v3",
        "rollback.restore-v1-shape",
        "compare.before-apply-0007",
        "provenance.migration-record-0007",
    ))
}

// -- Degraded schema-migration entries ----------------------------------------------------------

/// Degraded migration entry: the resolved migration record is incomplete — the compare-before-apply reference is
/// unstated.
fn migration_record_incomplete() -> M5ResolvedSchemaMigrationRecordEntry {
    let mut base = clean_migration_base(
        "migration:upgrade:incomplete",
        "settings.acme.editor.font-size@v1-to-v2",
        "migration.editor.font_size",
        M5SettingsGovernanceRole::SchemaMigration,
        M5SchemaMigrationFidelityClass::ExactMigration,
        M5ConfigMigrationSurfaceContext::UpgradeFlow,
        "old.editor.fontSize",
        "new.editor.font-size",
        "transform.rename-key-verbatim",
        "window.v1-through-v3",
        "rollback.restore-v1-key",
        "compare.before-apply-0007",
        "provenance.migration-record-0007",
    );
    base.compare_before_apply_reference = "   ".to_owned();
    migration(base)
}

/// Degraded migration entry: the fidelity label overstates what the lossy transform preserves.
fn migration_fidelity_overstated() -> M5ResolvedSchemaMigrationRecordEntry {
    let mut base = clean_migration_base(
        "migration:sync:fidelity-overstated",
        "settings.acme.telemetry.sample-rate@v2-to-v3",
        "migration.telemetry.sample_rate",
        M5SettingsGovernanceRole::SchemaMigration,
        M5SchemaMigrationFidelityClass::LossyMigration,
        M5ConfigMigrationSurfaceContext::RestoreFlow,
        "old.telemetry.sampleRateBuckets",
        "new.telemetry.sample-rate",
        "transform.collapse-buckets-lossy",
        "window.v2-through-v3",
        "rollback.restore-v2-buckets",
        "compare.before-apply-0007",
        "provenance.migration-record-0007",
    );
    base.is_lossy_or_manual_review = true;
    base.compare_surface_materialized = true;
    base.fidelity_label_honest = false;
    migration(base)
}

/// Degraded migration entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn migration_unbound() -> M5ResolvedSchemaMigrationRecordEntry {
    let mut base = clean_migration_base(
        "migration:policy:unbound",
        "settings.acme.tools.plugin-root@v3-to-v2",
        "migration.tools.plugin_root",
        M5SettingsGovernanceRole::SchemaMigration,
        M5SchemaMigrationFidelityClass::ManualReviewMigration,
        M5ConfigMigrationSurfaceContext::DowngradeFlow,
        "old.tools.pluginRoots",
        "new.tools.plugin-root",
        "transform.manual-review-required",
        "window.v2-through-v3",
        "rollback.restore-v3-roots",
        "compare.before-apply-0007",
        "provenance.migration-record-0007",
    );
    base.is_lossy_or_manual_review = true;
    base.compare_surface_materialized = true;
    base.bound_to_registry = false;
    migration(base)
}

/// Degraded migration entry: the canonical / accessible / audit resolution-form coverage is incomplete.
fn migration_form_incomplete() -> M5ResolvedSchemaMigrationRecordEntry {
    let mut base = clean_migration_base(
        "migration:import:form-incomplete",
        "settings.acme.workbench.theme-mode@v1-to-v2",
        "migration.workbench.theme_mode",
        M5SettingsGovernanceRole::SchemaMigration,
        M5SchemaMigrationFidelityClass::CompatibleMigration,
        M5ConfigMigrationSurfaceContext::ImportFlow,
        "old.workbench.themeMode",
        "new.workbench.theme-mode",
        "transform.coerce-enum-compatible",
        "window.v1-through-v3",
        "rollback.restore-v1-enum",
        "compare.before-apply-0007",
        "provenance.migration-record-0007",
    );
    base.resolution_form_coverage = vec![M5ConfigMigrationResolutionForm::CanonicalObject];
    migration(base)
}

/// Degraded migration entry: the canonical registry token name is unstated.
fn migration_token_unstated() -> M5ResolvedSchemaMigrationRecordEntry {
    let mut base = clean_migration_base(
        "migration:support:token-unstated",
        "settings.acme.sync.state-shape@v1-to-v2",
        "  ",
        M5SettingsGovernanceRole::SchemaMigration,
        M5SchemaMigrationFidelityClass::CompatibleMigration,
        M5ConfigMigrationSurfaceContext::SupportOrExportForm,
        "old.sync.stateShape",
        "new.sync.state-shape",
        "transform.reshape-compatible",
        "window.v1-through-v3",
        "rollback.restore-v1-shape",
        "compare.before-apply-0007",
        "provenance.migration-record-0007",
    );
    base.token_name = "  ".to_owned();
    migration(base)
}

// -- Clean compatibility-window entries ---------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_window_base(
    entry_id: &str,
    window_ref: &str,
    token_name: &str,
    semantic_role: M5SettingsGovernanceRole,
    window_class: M5CompatibilityWindowClass,
    surface_context: M5ConfigMigrationSurfaceContext,
    window_source: &str,
    supported_version_range: &str,
    deprecation_review: &str,
    validation_status: &str,
    review_state: &str,
    docs_pointer: &str,
    last_review_revision: &str,
) -> M5CompatibilityWindowEntryResolutionInput {
    M5CompatibilityWindowEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        window_ref: window_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        window_class,
        surface_context,
        resolution_form_coverage: all_forms(),
        window_source: window_source.to_owned(),
        supported_version_range: supported_version_range.to_owned(),
        deprecation_review: deprecation_review.to_owned(),
        validation_status: validation_status.to_owned(),
        review_state: review_state.to_owned(),
        docs_pointer: docs_pointer.to_owned(),
        last_review_revision: last_review_revision.to_owned(),
        keeps_window_source_visible: true,
        window_is_truthful: true,
        deprecation_present: false,
        deprecation_source_disclosed: false,
        unsupported_present: false,
        downgrade_guidance_disclosed: false,
        proof_fresh: true,
    }
}

fn window_within_upgrade_clean() -> M5ResolvedCompatibilityWindowEntry {
    window(clean_window_base(
        "window:upgrade:within",
        "editor.font_size",
        "window.editor.font_size",
        M5SettingsGovernanceRole::SchemaMigration,
        M5CompatibilityWindowClass::WithinCompatibilityWindow,
        M5ConfigMigrationSurfaceContext::UpgradeFlow,
        "window.schema-version-registry",
        "range.v1-to-v3",
        "review.deprecates-2026-12-31",
        "validation.ok",
        "review.current",
        "docs.migration-compatibility",
        "revision.0007",
    ))
}

fn window_deprecated_import_clean() -> M5ResolvedCompatibilityWindowEntry {
    // A deprecated window discloses its window source rather than masking it.
    let mut base = clean_window_base(
        "window:import:deprecated",
        "workbench.theme_mode",
        "window.workbench.theme_mode",
        M5SettingsGovernanceRole::SchemaMigration,
        M5CompatibilityWindowClass::DeprecatedButSupported,
        M5ConfigMigrationSurfaceContext::ImportFlow,
        "window.schema-version-registry",
        "range.v1-to-v2",
        "review.deprecates-2026-12-31",
        "validation.ok",
        "review.current",
        "docs.migration-compatibility",
        "revision.0007",
    );
    base.deprecation_present = true;
    base.deprecation_source_disclosed = true;
    window(base)
}

fn window_outside_restore_clean() -> M5ResolvedCompatibilityWindowEntry {
    // An outside-window migration discloses its downgrade guidance rather than reading as ambiguous failure copy.
    let mut base = clean_window_base(
        "window:restore:outside",
        "telemetry.sample_rate",
        "window.telemetry.sample_rate",
        M5SettingsGovernanceRole::SchemaMigration,
        M5CompatibilityWindowClass::OutsideCompatibilityWindow,
        M5ConfigMigrationSurfaceContext::RestoreFlow,
        "window.schema-version-registry",
        "range.v3-only",
        "review.deprecates-2026-12-31",
        "validation.warn",
        "review.current",
        "docs.migration-downgrade-guidance",
        "revision.0007",
    );
    base.unsupported_present = true;
    base.downgrade_guidance_disclosed = true;
    window(base)
}

fn window_within_downgrade_clean() -> M5ResolvedCompatibilityWindowEntry {
    window(clean_window_base(
        "window:downgrade:within",
        "tools.plugin_root",
        "window.tools.plugin_root",
        M5SettingsGovernanceRole::SchemaMigration,
        M5CompatibilityWindowClass::WithinCompatibilityWindow,
        M5ConfigMigrationSurfaceContext::DowngradeFlow,
        "window.schema-version-registry",
        "range.v2-to-v3",
        "review.deprecates-2026-12-31",
        "validation.ok",
        "review.current",
        "docs.migration-compatibility",
        "revision.0007",
    ))
}

fn window_deprecated_support_clean() -> M5ResolvedCompatibilityWindowEntry {
    let mut base = clean_window_base(
        "window:support:deprecated",
        "sync.state_shape",
        "window.sync.state_shape",
        M5SettingsGovernanceRole::SchemaMigration,
        M5CompatibilityWindowClass::DeprecatedButSupported,
        M5ConfigMigrationSurfaceContext::SupportOrExportForm,
        "window.schema-version-registry",
        "range.v1-to-v2",
        "review.deprecates-2026-12-31",
        "validation.ok",
        "review.current",
        "docs.migration-compatibility",
        "revision.0007",
    );
    base.deprecation_present = true;
    base.deprecation_source_disclosed = true;
    window(base)
}

// -- Degraded compatibility-window entries ------------------------------------------------------

/// Degraded window entry: the record would mask a deprecated window without disclosing its window source — a
/// deprecated migration reads as ambiguously unavailable when it has quietly hidden the cause.
fn window_masks_window() -> M5ResolvedCompatibilityWindowEntry {
    let mut base = clean_window_base(
        "window:upgrade:masks-window",
        "editor.font_size",
        "window.editor.font_size",
        M5SettingsGovernanceRole::SchemaMigration,
        M5CompatibilityWindowClass::DeprecatedButSupported,
        M5ConfigMigrationSurfaceContext::UpgradeFlow,
        "window.schema-version-registry",
        "range.v1-to-v2",
        "review.deprecates-2026-12-31",
        "validation.ok",
        "review.current",
        "docs.migration-compatibility",
        "revision.0007",
    );
    base.deprecation_present = true;
    base.deprecation_source_disclosed = false;
    window(base)
}

/// Degraded window entry: the canonical / accessible / audit resolution-form coverage of the record is
/// incomplete.
fn window_form_incomplete() -> M5ResolvedCompatibilityWindowEntry {
    let mut base = clean_window_base(
        "window:import:form-incomplete",
        "workbench.theme_mode",
        "window.workbench.theme_mode",
        M5SettingsGovernanceRole::SchemaMigration,
        M5CompatibilityWindowClass::DeprecatedButSupported,
        M5ConfigMigrationSurfaceContext::ImportFlow,
        "window.schema-version-registry",
        "range.v1-to-v2",
        "review.deprecates-2026-12-31",
        "validation.ok",
        "review.current",
        "docs.migration-compatibility",
        "revision.0007",
    );
    base.deprecation_present = true;
    base.deprecation_source_disclosed = true;
    base.resolution_form_coverage = vec![M5ConfigMigrationResolutionForm::CanonicalObject];
    window(base)
}

/// Degraded window entry: the window class is unclassified.
fn window_class_unclassified() -> M5ResolvedCompatibilityWindowEntry {
    window(clean_window_base(
        "window:policy:class-unclassified",
        "tools.plugin_root",
        "window.tools.plugin_root",
        M5SettingsGovernanceRole::SchemaMigration,
        M5CompatibilityWindowClass::WindowClassUnclassified,
        M5ConfigMigrationSurfaceContext::DowngradeFlow,
        "window.schema-version-registry",
        "range.v2-to-v3",
        "review.deprecates-2026-12-31",
        "validation.ok",
        "review.current",
        "docs.migration-compatibility",
        "revision.0007",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5SettingSchemaMigrationCompatibilityWindowRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5SettingsGovernanceDowngradeTrigger>,
    schema_migration_entries: Vec<M5ResolvedSchemaMigrationRecordEntry>,
    compatibility_window_entries: Vec<M5ResolvedCompatibilityWindowEntry>,
) -> M5SettingSchemaMigrationCompatibilityWindowRegistriesRow {
    M5SettingSchemaMigrationCompatibilityWindowRegistriesRow {
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
        anatomy_parts: M5ConfigMigrationAnatomyPart::ALL.to_vec(),
        export_fields: M5ConfigMigrationExportField::ALL.to_vec(),
        downgrade_triggers,
        schema_migration_entries,
        compatibility_window_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_SCHEMA_REF,
            M5_SETTING_DEFINITION_DOMAIN_SCHEMA_REF,
            M5_SCHEMA_MIGRATION_LANDED_SCHEMA_REF,
        ]),
        implies_full_fidelity_when_migration_is_lossy: false,
        alters_stored_meaning_without_a_checked_in_migration_record: false,
        applies_a_lossy_migration_without_a_compare_before_apply_surface: false,
        hides_the_compatibility_window_or_downgrade_cause_behind_generic_copy: false,
    }
}

fn registry_rows() -> Vec<M5SettingSchemaMigrationCompatibilityWindowRegistriesRow> {
    use M5SettingsGovernanceConsumerSurface as C;
    use M5SettingsGovernanceDowngradeTrigger as D;

    vec![
        base_row(
            C::SettingsResolver,
            "Settings-resolver owner",
            "The settings resolver lands the exact upgrade migration record — old key / alias, new key, transform, compatibility window, rollback note, compare-before-apply reference, and migration provenance reference — from the shared registry and resolves the within-window compatibility label for that setting; a migration record missing its compare-before-apply reference and a compatibility window that masks a deprecated window without disclosing its window source degrade honestly instead of reading as a clean pass",
            "evidence:m5-settings-governance-settings-resolver:001",
            vec![
                D::RewroteAScopedWriteIntoABroaderScope,
                D::HidKillSwitchOrPolicyDisableCauseBehindGenericUnavailableCopy,
                D::ProofStale,
            ],
            vec![migration_exact_upgrade_clean(), migration_record_incomplete()],
            vec![window_within_upgrade_clean(), window_masks_window()],
        ),
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell lands the compatible import migration record while disclosing the deprecated-but-supported compatibility window and its window source; a resolution-form gap on a migration entry and on a compatibility window is caught before a screenshot can reintroduce a false-fidelity reading",
            "evidence:m5-settings-governance-shell-ui:001",
            vec![
                D::RegistryReferenceUnstated,
                D::ScopeBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![
                migration_compatible_import_clean(),
                migration_form_incomplete(),
            ],
            vec![window_deprecated_import_clean(), window_form_incomplete()],
        ),
        base_row(
            C::SyncService,
            "Sync-service owner",
            "The sync service lands the lossy restore migration with a materialized compare-before-apply surface and reports the outside-window compatibility label with downgrade guidance; a migration whose fidelity label overstates what the lossy transform preserves is caught before it can imply full fidelity",
            "evidence:m5-settings-governance-sync-service:001",
            vec![
                D::RewroteAScopedWriteIntoABroaderScope,
                D::ScopeBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![
                migration_lossy_restore_clean(),
                migration_fidelity_overstated(),
            ],
            vec![window_outside_restore_clean()],
        ),
        base_row(
            C::PolicyService,
            "Policy-service owner",
            "The policy service lands the manual-review downgrade migration with a materialized compare surface and bound to the registry while resolving the within-window compatibility label; a migration that is a hand-copied per-entry assumption and a compatibility window on an unclassified window class degrade honestly",
            "evidence:m5-settings-governance-policy-service:001",
            vec![
                D::ScopeBoundaryDriftedBySurface,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![
                migration_manual_review_downgrade_clean(),
                migration_unbound(),
            ],
            vec![window_within_downgrade_clean(), window_class_unclassified()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved schema-migration and compatibility-window truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied migration table",
            "evidence:m5-settings-governance-diagnostics:001",
            vec![
                D::RegistryReferenceUnstated,
                D::ScopeBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![
                migration_lossy_restore_clean(),
                migration_form_incomplete(),
            ],
            vec![window_deprecated_import_clean(), window_form_incomplete()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved schema-migration and compatibility-window truth, so a hand-copied constant, an unstated registry token, an overstated fidelity label, or a masked window is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-settings-governance-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::LifecycleStateUnstated,
                D::ProofStale,
            ],
            vec![
                migration_compatible_support_clean(),
                migration_token_unstated(),
            ],
            vec![window_deprecated_support_clean()],
        ),
    ]
}

fn governance_review() -> M5SettingSchemaMigrationCompatibilityWindowRegistriesGovernanceReview {
    M5SettingSchemaMigrationCompatibilityWindowRegistriesGovernanceReview {
        schema_migration_registry_names_token_role_and_label: true,
        migration_resolves_to_one_record_from_shared_registry: true,
        old_key_new_key_transform_window_rollback_and_compare_surface_published: true,
        migration_labels_never_overstate_fidelity: true,
        compatibility_window_keeps_window_source_visible_and_discloses_downgrade_guidance: true,
        compare_before_apply_surface_materialized_for_lossy_or_manual_migrations: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        upgrade_import_restore_downgrade_read_single_source: true,
        migration_or_window_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5SettingSchemaMigrationCompatibilityWindowRegistriesConsumerProjection
{
    M5SettingSchemaMigrationCompatibilityWindowRegistriesConsumerProjection {
        upgrade_and_import_consume_shared_registries: true,
        restore_and_downgrade_consume_shared_registries: true,
        migration_and_compat_services_consume_shared_registries: true,
        docs_migration_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5SettingSchemaMigrationCompatibilityWindowRegistriesProofFreshness {
    M5SettingSchemaMigrationCompatibilityWindowRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SettingSchemaMigrationCompatibilityWindowRegistriesReleasePosture {
    M5SettingSchemaMigrationCompatibilityWindowRegistriesReleasePosture {
        proof_packet_ref: M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        settings_governance_audit_ref:
            M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_SCHEMA_REF,
        M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_DOC_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_DOC_REF,
        M5_SETTING_DEFINITION_DOMAIN_SCHEMA_REF,
        M5_SCHEMA_MIGRATION_LANDED_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 schema-migration and compatibility-window registries packet.
pub fn seeded_m5_setting_schema_migration_and_compatibility_window_registries(
) -> M5SettingSchemaMigrationCompatibilityWindowRegistriesPacket {
    M5SettingSchemaMigrationCompatibilityWindowRegistriesPacket::new(
        M5SettingSchemaMigrationCompatibilityWindowRegistriesPacketInput {
            packet_id: M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_PACKET_ID
                .to_owned(),
            registries_label:
                "M5 schema-migration and compatibility-window registries with one migration record landing per version change, migration labels that never overstate fidelity, a compare-before-apply surface materialized before any lossy or manual-review migration applies, canonical / accessible / audit resolution-form coverage, and the complete window-source / supported-version-range / deprecation-review / validation-status / review-state / docs-pointer / last-review-revision compatibility-window object across settings-resolver, shell, sync, policy, diagnostics, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set:
                M5SettingSchemaMigrationCompatibilityWindowRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the settings-resolver row is held at Beta pending schema-migration parity on every
/// platform; every row stays visible and every example stays honest.
pub fn seeded_m5_setting_schema_migration_and_compatibility_window_registries_schema_migration_beta_narrowed(
) -> M5SettingSchemaMigrationCompatibilityWindowRegistriesPacket {
    let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    packet.packet_id =
        "m5-setting-schema-migration-and-compatibility-window-registries:schema-migration-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5SettingsGovernanceConsumerSurface::SettingsResolver)
        .expect("settings-resolver row present");
    row.qualification = M5SettingsGovernanceQualificationClass::Beta;
    packet
}

/// Narrowed variant: the sync-service row is narrowed to Preview pending compatibility-window parity on every
/// platform; every row stays visible and every example stays honest.
pub fn seeded_m5_setting_schema_migration_and_compatibility_window_registries_compatibility_window_preview_narrowed(
) -> M5SettingSchemaMigrationCompatibilityWindowRegistriesPacket {
    let mut packet = seeded_m5_setting_schema_migration_and_compatibility_window_registries();
    packet.packet_id =
        "m5-setting-schema-migration-and-compatibility-window-registries:compatibility-window-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5SettingsGovernanceConsumerSurface::SyncService)
        .expect("sync-service row present");
    row.qualification = M5SettingsGovernanceQualificationClass::Preview;
    packet
}
