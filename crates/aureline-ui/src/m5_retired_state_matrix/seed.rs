//! Canonical seed builders for the frozen M5 retired-state matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code matrix, the artifact, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical retired-state matrix.
pub const M5_RETIRED_STATE_MATRIX_PACKET_ID: &str = "m5-retired-state:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn mandatory_labels() -> Vec<M5RetiredStateRequiredLabel> {
    M5RetiredStateRequiredLabel::MANDATORY.to_vec()
}

fn labels_with(extra: &[M5RetiredStateRequiredLabel]) -> Vec<M5RetiredStateRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

#[allow(clippy::too_many_arguments)]
fn base_row(
    object_class: M5RetiredStateObject,
    qualification: M5RetiredStateQualificationClass,
    owner_role: &str,
    backup_owner_role: &str,
    scope_summary: &str,
    closure_ref: &str,
    source_refs: &[&str],
    retirement_transition: M5RetiredStateTransition,
) -> M5RetiredStateRow {
    M5RetiredStateRow {
        object_class,
        qualification,
        lifecycle_state: M5RetiredStateLifecycleState::Retired,
        owner_role: owner_role.to_owned(),
        backup_owner_role: backup_owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        retirement_transition,
        surface_families: M5RetiredStateSurfaceFamily::ALL.to_vec(),
        removal_horizon_stages: M5RetiredStateRemovalHorizonStage::ALL.to_vec(),
        required_labels: mandatory_labels(),
        semantic_roles: vec![],
        supported_line_roles: vec![],
        stable_capability_roles: vec![],
        bundle_roles: vec![],
        command_deep_link_roles: vec![],
        schema_bearing_surface_roles: vec![],
        registry_visible_package_roles: vec![],
        managed_tenant_feature_roles: vec![],
        degraded_reasons: M5RetiredStateDegradedReason::ALL.to_vec(),
        accessibility_routes: M5RetiredStateAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5RetiredStateConsumerSurface::Support,
            M5RetiredStateConsumerSurface::HelpDocs,
        ],
        downgrade_triggers: vec![M5RetiredStateDowngradeTrigger::RetirementManifestStale],
        required_closure_artifact_refs: strings(&[closure_ref]),
        source_contract_refs: strings(source_refs),
        lets_a_retired_surface_disappear_without_tombstone_archival_route_or_successor_pointer: false,
        keeps_a_retired_class_selectable_in_new_install_new_tenant_marketplace_or_upgrade_flow: false,
        destroys_last_supported_docs_schemas_or_evidence_before_support_note_closure: false,
        leaves_retirement_state_unjoined_to_build_line_identity_deployment_profile_and_migration_outcome: false,
        retires_a_surface_through_silent_disappearance_stale_selection_ui_or_orphaned_support_truth: false,
    }
}

fn txn(f: [&str; 8]) -> M5RetiredStateTransition {
    M5RetiredStateTransition {
        last_supported_version_or_channel: f[0].to_owned(),
        cutoff_date: f[1].to_owned(),
        successor_path: f[2].to_owned(),
        disable_path: f[3].to_owned(),
        export_rollback_route: f[4].to_owned(),
        archival_note: f[5].to_owned(),
        migration_outcome: f[6].to_owned(),
        support_note_closure_state: f[7].to_owned(),
    }
}

fn retired_state_rows() -> Vec<M5RetiredStateRow> {
    use M5RetiredStateConsumerSurface as C;
    use M5RetiredStateDowngradeTrigger as D;
    use M5RetiredStateObject as O;
    use M5RetiredStateQualificationClass as Q;
    use M5RetiredStateRequiredLabel as L;
    use M5RetiredStateRole as R;

    let mut rows = Vec::new();

    // 1. SupportedLine.
    let mut row = base_row(
        O::SupportedLine,
        Q::Stable,
        "Supported-line retirement owner",
        "Release-governance backup owner",
        "One supported line moving to Retired names the last-supported version pinned to an exact build, the successor line routed forward, the disable path published, and the no-new-install gating enforced so the line never disappears silently and no new install can still select it",
        "evidence:m5-supported-line-closure:001",
        &[
            M5_RETIRED_STATE_MATRIX_SCHEMA_REF,
            M5_RETIREMENT_MANIFEST_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "v6.4 last supported build 6.4.219",
            "2026-09-30",
            "supported line v7.0",
            "settings lifecycle disable retired line",
            "export bundle via cli export, rollback to build 6.4.219",
            "archived under artifacts release m5-retirements",
            "migrated with documented migration scoreboard",
            "support note closed",
        ]),
    );
    row.supported_line_roles = M5RetiredStateSupportedLineRole::ALL.to_vec();
    row.semantic_roles = vec![R::LastSupportedPin, R::SuccessorRouting];
    row.required_labels = labels_with(&[L::LastSupportedVersion]);
    row.consumer_surfaces = vec![
        C::ReleaseCenter,
        C::HelpDocs,
        C::Support,
        C::InstallUpdate,
        C::PartnerProcurement,
        C::ProgramGovernance,
    ];
    row.downgrade_triggers = vec![
        D::RetiredSurfaceDisappearedWithoutTombstone,
        D::RetiredClassStayedSelectableInNewInstall,
        D::SuccessorPathUnnamed,
        D::RegistryReferenceUnstated,
        D::RetirementManifestStale,
    ];
    rows.push(row);

    // 2. StableCapability.
    let mut row = base_row(
        O::StableCapability,
        Q::Stable,
        "Stable-capability retirement owner",
        "Capability-governance backup owner",
        "One stable-facing capability moving to Retired names the last-supported channel pinned, the successor capability named, the export/rollback route ready, and the no-new-tenant gating enforced so its support and docs truth is never orphaned",
        "evidence:m5-stable-capability-closure:001",
        &[
            M5_RETIRED_STATE_MATRIX_SCHEMA_REF,
            M5_RETIREMENT_MANIFEST_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "stable channel last supported release 6.4",
            "2026-10-15",
            "successor capability v7.0",
            "admin lifecycle disable capability",
            "export capability state via cli export, rollback to release 6.4",
            "archival note filed with tombstone",
            "migrated with recorded outcome",
            "support note closed",
        ]),
    );
    row.stable_capability_roles = M5RetiredStateStableCapabilityRole::ALL.to_vec();
    row.semantic_roles = vec![R::SuccessorRouting, R::DisablePath];
    row.required_labels = labels_with(&[L::SuccessorPath]);
    row.consumer_surfaces = vec![
        C::ReleaseCenter,
        C::HelpDocs,
        C::Support,
        C::InstallUpdate,
        C::ProgramGovernance,
        C::Diagnostics,
    ];
    row.downgrade_triggers = vec![
        D::RetiredClassStayedSelectableForNewTenant,
        D::DisablePathUnnamed,
        D::SuccessorPathUnnamed,
        D::RegistryReferenceUnstated,
        D::RetirementManifestStale,
    ];
    rows.push(row);

    // 3. Bundle.
    let mut row = base_row(
        O::Bundle,
        Q::Stable,
        "Bundle retirement owner",
        "Release-engineering backup owner",
        "One bundle moving to Retired names the last-supported bundle snapshotted to exact build identity, the archival note written, the export route ready, and the bundle removed from the upgrade flow so no stale bundle stays selectable",
        "evidence:m5-bundle-closure:001",
        &[
            M5_RETIRED_STATE_MATRIX_SCHEMA_REF,
            M5_LAST_SUPPORTED_SNAPSHOT_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "last supported bundle snapshot build 6.4.219",
            "2026-09-30",
            "successor bundle v7.0",
            "update flow disable retired bundle",
            "export bundle artifact via cli export, rollback to snapshot 6.4.219",
            "archival note and tombstone filed",
            "migrated with documented outcome",
            "support note closed",
        ]),
    );
    row.bundle_roles = M5RetiredStateBundleRole::ALL.to_vec();
    row.semantic_roles = vec![R::ArchivalNote, R::ExportRollbackRoute];
    row.required_labels = labels_with(&[L::CutoffDate]);
    row.consumer_surfaces = vec![
        C::ReleaseCenter,
        C::MarketplaceRegistry,
        C::InstallUpdate,
        C::Support,
        C::CliExport,
        C::Diagnostics,
    ];
    row.downgrade_triggers = vec![
        D::LastSupportedSnapshotMissing,
        D::ArchivalNoteMissing,
        D::CutoffDateUnstated,
        D::RegistryReferenceUnstated,
        D::RetirementManifestStale,
    ];
    rows.push(row);

    // 4. CommandDeepLink.
    let mut row = base_row(
        O::CommandDeepLink,
        Q::Stable,
        "Command / deep-link retirement owner",
        "Shell-governance backup owner",
        "One command or deep link moving to Retired names the command tombstone registered, the successor redirect named, the disable path ready, and its removal from the command palette so no dangling deep link is left without a tombstone",
        "evidence:m5-command-deep-link-closure:001",
        &[
            M5_RETIRED_STATE_MATRIX_SCHEMA_REF,
            M5_RETIREMENT_IMPACT_REPORT_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "last supported command surface build 6.4",
            "2026-10-01",
            "successor command redirect v7.0",
            "palette disable retired command",
            "export command mapping via cli export, rollback to build 6.4",
            "tombstone and archival redirect filed",
            "migrated with recorded redirect outcome",
            "support note closed",
        ]),
    );
    row.command_deep_link_roles = M5RetiredStateCommandDeepLinkRole::ALL.to_vec();
    row.semantic_roles = vec![R::DisablePath, R::SuccessorRouting];
    row.required_labels = labels_with(&[L::SuccessorPath]);
    row.consumer_surfaces = vec![
        C::ReleaseCenter,
        C::HelpDocs,
        C::Support,
        C::Diagnostics,
        C::CliExport,
        C::ProgramGovernance,
    ];
    row.downgrade_triggers = vec![
        D::RetiredSurfaceDisappearedWithoutTombstone,
        D::DisablePathUnnamed,
        D::SuccessorPathUnnamed,
        D::RegistryReferenceUnstated,
        D::RetirementManifestStale,
    ];
    rows.push(row);

    // 5. SchemaBearingSurface.
    let mut row = base_row(
        O::SchemaBearingSurface,
        Q::Stable,
        "Schema-bearing-surface retirement owner",
        "Data-contract backup owner",
        "One schema-bearing surface moving to Retired names the last-supported schema snapshotted, the migration outcome recorded, the export route ready, and the archival note written so no last-supported schema is destroyed before support-note closure",
        "evidence:m5-schema-bearing-surface-closure:001",
        &[
            M5_RETIRED_STATE_MATRIX_SCHEMA_REF,
            M5_LAST_SUPPORTED_SNAPSHOT_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "last supported schema snapshot build 6.4.219",
            "2026-11-01",
            "successor schema v2",
            "schema registry disable retired surface",
            "export schema snapshot via cli export, rollback to snapshot 6.4.219",
            "archival note filed with schema tombstone",
            "migrated with recorded schema outcome",
            "support note closed after export-safe handoff",
        ]),
    );
    row.schema_bearing_surface_roles = M5RetiredStateSchemaBearingSurfaceRole::ALL.to_vec();
    row.semantic_roles = vec![R::ArchivalNote, R::MigrationOutcome];
    row.required_labels = labels_with(&[L::CutoffDate]);
    row.consumer_surfaces = vec![
        C::ReleaseCenter,
        C::HelpDocs,
        C::Support,
        C::Diagnostics,
        C::CliExport,
        C::ProgramGovernance,
    ];
    row.downgrade_triggers = vec![
        D::LastSupportedSnapshotMissing,
        D::ArchivalNoteMissing,
        D::RetirementUnjoinedFromBuildIdentity,
        D::RegistryReferenceUnstated,
        D::RetirementManifestStale,
    ];
    rows.push(row);

    // 6. RegistryVisiblePackage.
    let mut row = base_row(
        O::RegistryVisiblePackage,
        Q::Stable,
        "Registry-visible-package retirement owner",
        "Marketplace-governance backup owner",
        "One registry-visible package moving to Retired names the package marked Retired in the registry, the successor package named, its removal from the marketplace listing, and the no-new-install gating from the registry so no stale marketplace listing stays selectable",
        "evidence:m5-registry-visible-package-closure:001",
        &[
            M5_RETIRED_STATE_MATRIX_SCHEMA_REF,
            M5_RETIREMENT_IMPACT_REPORT_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "last supported package version 6.4.219",
            "2026-09-30",
            "successor package v7.0",
            "registry disable retired package",
            "export package manifest via cli export, rollback to version 6.4.219",
            "archival note and registry tombstone filed",
            "migrated with documented outcome",
            "support note closed",
        ]),
    );
    row.registry_visible_package_roles = M5RetiredStateRegistryVisiblePackageRole::ALL.to_vec();
    row.semantic_roles = vec![R::LastSupportedPin, R::SuccessorRouting];
    row.required_labels = labels_with(&[L::LastSupportedVersion]);
    row.consumer_surfaces = vec![
        C::ReleaseCenter,
        C::MarketplaceRegistry,
        C::InstallUpdate,
        C::Support,
        C::PartnerProcurement,
        C::ProgramGovernance,
    ];
    row.downgrade_triggers = vec![
        D::RetiredClassStayedSelectableInNewInstall,
        D::RetiredClassStayedSelectableForNewTenant,
        D::SuccessorPathUnnamed,
        D::RegistryReferenceUnstated,
        D::RetirementManifestStale,
    ];
    rows.push(row);

    // 7. ManagedTenantFeature.
    let mut row = base_row(
        O::ManagedTenantFeature,
        Q::Stable,
        "Managed / new-tenant-feature retirement owner",
        "Tenant-governance backup owner",
        "One managed / new-tenant-gated feature moving to Retired names the feature disabled by policy for new tenants, the successor named, the export/rollback route ready, and the support note closed so new-tenant gating is never bypassed",
        "evidence:m5-managed-tenant-feature-closure:001",
        &[
            M5_RETIRED_STATE_MATRIX_SCHEMA_REF,
            M5_RETIREMENT_CLOSURE_LEDGER_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "last supported feature build 6.4",
            "2026-12-01",
            "successor feature v7.0",
            "policy disable feature for new tenants",
            "export tenant feature state via cli export, rollback to build 6.4",
            "archival route and tombstone filed",
            "migrated with recorded tenant outcome",
            "support note closed",
        ]),
    );
    row.managed_tenant_feature_roles = M5RetiredStateManagedTenantFeatureRole::ALL.to_vec();
    row.semantic_roles = vec![R::SupportNoteClosure, R::DisablePath];
    row.required_labels = labels_with(&[L::SuccessorPath]);
    row.consumer_surfaces = vec![
        C::ReleaseCenter,
        C::Support,
        C::InstallUpdate,
        C::PartnerProcurement,
        C::ProgramGovernance,
        C::CliExport,
    ];
    row.downgrade_triggers = vec![
        D::RetiredClassStayedSelectableForNewTenant,
        D::SupportNoteClosureIncomplete,
        D::DisablePathUnnamed,
        D::RegistryReferenceUnstated,
        D::RetirementManifestStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5RetiredStateGovernanceReview {
    M5RetiredStateGovernanceReview {
        no_retired_surface_disappears_without_tombstone_archival_route_or_successor_pointer: true,
        every_covered_object_class_names_owner_closure_artifacts_and_first_consumer: true,
        retired_is_mechanically_distinct_from_deprecated_disabled_and_narrowed: true,
        last_supported_snapshots_are_captured_before_retirement: true,
        successor_routing_and_cutoff_review_precede_every_retirement: true,
        support_and_public_proof_surfaces_close_cleanly_on_retirement: true,
        archival_and_tombstone_truth_is_preserved_after_retirement: true,
        no_new_installs_or_new_tenants_can_select_a_retired_class: true,
        retirement_state_stays_joined_to_exact_build_and_line_identity: true,
        every_object_declares_removal_horizon_stages: true,
        every_object_declares_accessibility_route: true,
        support_export_reads_single_retirement_source: true,
        release_help_and_support_bind_to_single_retirement_source: true,
        later_rows_cannot_invent_parallel_retirement_vocabulary: true,
        retirement_truth_survives_zoom_and_high_contrast: true,
        claims_narrow_automatically_when_matrix_row_missing_or_stale: true,
    }
}

fn consumer_projection() -> M5RetiredStateConsumerProjection {
    M5RetiredStateConsumerProjection {
        release_and_help_consume_shared_retirement_truth: true,
        support_and_marketplace_consume_shared_closure_and_snapshot_truth: true,
        install_update_and_tenant_gating_consume_shared_no_new_install_truth: true,
        docs_help_and_screenshots_read_single_retirement_source: true,
        archives_and_tombstones_bind_to_shared_build_identity: true,
        support_export_reads_single_retirement_source: true,
    }
}

fn proof_freshness() -> M5RetiredStateProofFreshness {
    M5RetiredStateProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5RetiredStateReleasePosture {
    M5RetiredStateReleasePosture {
        proof_packet_ref: M5_RETIRED_STATE_ARTIFACT_REF.to_owned(),
        retired_state_audit_ref: M5_RETIRED_STATE_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_RETIRED_STATE_MATRIX_SCHEMA_REF,
        M5_RETIRED_STATE_MATRIX_DOC_REF,
        M5_RETIREMENT_MANIFEST_DOMAIN_SCHEMA_REF,
        M5_RETIREMENT_IMPACT_REPORT_DOMAIN_SCHEMA_REF,
        M5_LAST_SUPPORTED_SNAPSHOT_DOMAIN_SCHEMA_REF,
        M5_RETIREMENT_CLOSURE_LEDGER_DOMAIN_SCHEMA_REF,
        M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 retired-state matrix packet.
pub fn seeded_m5_retired_state_matrix() -> M5RetiredStateMatrixPacket {
    M5RetiredStateMatrixPacket::new(M5RetiredStateMatrixPacketInput {
        packet_id: M5_RETIRED_STATE_MATRIX_PACKET_ID.to_owned(),
        matrix_label: "M5 retired-state, end-of-support closure, successor-routing, and tombstone/archive matrix"
            .to_owned(),
        retired_state_rows: retired_state_rows(),
        vocabulary_set: M5RetiredStateVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the registry-visible package is held at Beta because its marketplace tombstone and
/// no-new-install gating are not yet fully proven; every object class stays visible.
pub fn seeded_m5_retired_state_matrix_registry_visible_package_beta_narrowed(
) -> M5RetiredStateMatrixPacket {
    let mut packet = seeded_m5_retired_state_matrix();
    packet.packet_id = "m5-retired-state:registry-visible-package-beta:0001".to_owned();
    let row = packet
        .retired_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5RetiredStateObject::RegistryVisiblePackage)
        .expect("registry-visible-package row present");
    row.qualification = M5RetiredStateQualificationClass::Beta;
    packet
}

/// Narrowed variant: the managed / new-tenant feature is narrowed to Preview pending support-note closure
/// and new-tenant gating proof; every object class stays visible.
pub fn seeded_m5_retired_state_matrix_managed_tenant_feature_preview_narrowed(
) -> M5RetiredStateMatrixPacket {
    let mut packet = seeded_m5_retired_state_matrix();
    packet.packet_id = "m5-retired-state:managed-tenant-feature-preview:0001".to_owned();
    let row = packet
        .retired_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5RetiredStateObject::ManagedTenantFeature)
        .expect("managed-tenant-feature row present");
    row.qualification = M5RetiredStateQualificationClass::Preview;
    packet
}
