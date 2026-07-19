// Sequential pushes keep each contract scenario adjacent to its rationale.
#![allow(clippy::vec_init_then_push)]

//! Canonical seed builders for the frozen M5 marketplace-install component matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical marketplace-install component matrix.
pub const M5_MARKETPLACE_INSTALL_COMPONENT_MATRIX_PACKET_ID: &str =
    "m5-marketplace-install-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5MarketplaceInstallRequiredLabel> {
    M5MarketplaceInstallRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(
    extra: &[M5MarketplaceInstallRequiredLabel],
) -> Vec<M5MarketplaceInstallRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every family-specific
/// vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5MarketplaceInstallComponentFamily,
    qualification: M5MarketplaceInstallQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5MarketplaceInstallComponentRow {
    M5MarketplaceInstallComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5MarketplaceInstallSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5MarketplaceInstallDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        dispositions: M5MarketplaceInstallDisposition::ALL.to_vec(),
        registry_source_classes: vec![],
        compatibility_states: vec![],
        host_runtime_models: vec![],
        permission_postures: vec![],
        activation_budget_bands: vec![],
        publisher_continuity_states: vec![],
        disable_scope_classes: vec![],
        rollback_compatibility_states: vec![],
        quarantine_states: vec![],
        degraded_reasons: M5MarketplaceInstallDegradedReason::ALL.to_vec(),
        accessibility_routes: M5MarketplaceInstallAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5MarketplaceInstallConsumerSurface::SupportExport,
            M5MarketplaceInstallConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5MarketplaceInstallDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        hides_permission_widening_or_activation_cost: false,
        hides_publisher_transfer_disable_scope_or_rollback_incompatibility: false,
        collapses_registry_source_class_across_public_mirrored_enterprise: false,
        presents_incompatible_or_over_budget_as_ready: false,
    }
}

fn component_rows() -> Vec<M5MarketplaceInstallComponentRow> {
    use M5ActivationBudgetBandState as AB;
    use M5CompatibilityState as CO;
    use M5DisableScopeClass as DS;
    use M5HostRuntimeModel as HM;
    use M5MarketplaceInstallComponentFamily as F;
    use M5MarketplaceInstallConsumerSurface as C;
    use M5MarketplaceInstallDisposition as BD;
    use M5MarketplaceInstallDowngradeTrigger as D;
    use M5MarketplaceInstallQualificationClass as Q;
    use M5MarketplaceInstallRequiredLabel as L;
    use M5PermissionPostureState as PP;
    use M5PublisherContinuityState as PC;
    use M5QuarantineState as QU;
    use M5RegistrySourceClass as RS;
    use M5RollbackCompatibilityState as RB;

    let mut rows = Vec::new();

    // 1. Marketplace result row.
    let mut row = base_row(
        F::MarketplaceResultRow,
        Q::Stable,
        "Marketplace catalog owner",
        "One marketplace-result-row model naming the registry source class (public, mirrored, enterprise, side-load, or verified partner), the compatibility state, and the publisher continuity for one listed artifact, so a mirrored or side-loaded artifact never reads with the same authority as a verified public listing",
        "evidence:m5-marketplace-result-row-parity:001",
        &[
            M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
            M5_MARKETPLACE_RESULT_ROW_SCHEMA_REF,
        ],
    );
    row.registry_source_classes = RS::ALL.to_vec();
    row.compatibility_states = vec![
        CO::Compatible,
        CO::CompatibleWithWarnings,
        CO::Incompatible,
        CO::CompatibilityUnknown,
    ];
    row.publisher_continuity_states = vec![
        PC::Continuous,
        PC::Transferred,
        PC::Deprecated,
        PC::VerifiedPublisher,
    ];
    row.dispositions = vec![
        BD::Public,
        BD::Mirrored,
        BD::Enterprise,
        BD::SideLoad,
        BD::Verified,
        BD::Deprecated,
        BD::Limited,
    ];
    row.required_labels = labels_with(&[L::CompatibilityAndHost, L::PublisherAndSourceClass]);
    row.consumer_surfaces = vec![C::MarketplaceUi, C::HelpUi, C::SupportExport, C::ProductUi];
    row.downgrade_triggers = vec![
        D::CompatibilityRangeUnstated,
        D::RegistrySourceClassCollapsed,
        D::PublisherTransferHidden,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Marketplace detail fact grid.
    let mut row = base_row(
        F::MarketplaceDetailFactGrid,
        Q::Stable,
        "Marketplace catalog owner",
        "One marketplace-detail-fact-grid model naming registry source class, compatibility and host/runtime model, permission posture, activation-budget band, and publisher continuity together in one place, so a user can read every governable marketplace fact about an artifact without hunting through disconnected surfaces",
        "evidence:m5-marketplace-detail-fact-grid-parity:001",
        &[
            M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
            M5_MARKETPLACE_DETAIL_FACT_GRID_SCHEMA_REF,
        ],
    );
    row.registry_source_classes = RS::ALL.to_vec();
    row.compatibility_states = CO::ALL.to_vec();
    row.host_runtime_models = vec![HM::InProcess, HM::Sandboxed, HM::RemoteHost, HM::NativeHost];
    row.permission_postures = vec![
        PP::Minimal,
        PP::Standard,
        PP::Elevated,
        PP::WidenedTransitive,
        PP::PolicyRestricted,
    ];
    row.activation_budget_bands = vec![
        AB::WithinBudget,
        AB::NearBudget,
        AB::OverBudget,
        AB::Throttled,
    ];
    row.publisher_continuity_states = PC::ALL.to_vec();
    row.dispositions = vec![
        BD::Public,
        BD::Mirrored,
        BD::Enterprise,
        BD::Verified,
        BD::Transferred,
        BD::Deprecated,
        BD::Incompatible,
    ];
    row.required_labels = labels_with(&[
        L::CompatibilityAndHost,
        L::PermissionAndBudget,
        L::PublisherAndSourceClass,
    ]);
    row.consumer_surfaces = vec![
        C::MarketplaceUi,
        C::ExtensionsUi,
        C::HelpUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::CompatibilityRangeUnstated,
        D::PermissionWideningHidden,
        D::ActivationCostHidden,
        D::PublisherTransferHidden,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Compatibility-label strip.
    let mut row = base_row(
        F::CompatibilityLabelStrip,
        Q::Stable,
        "Extension runtime owner",
        "One compatibility-label-strip model naming the compatibility range and the runtime/host model (in-process, sandboxed, remote, web-worker, or native), so an incompatible, degraded-host, or unsupported-runtime artifact is never presented as freely installable",
        "evidence:m5-compatibility-label-strip-parity:001",
        &[
            M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
            M5_COMPATIBILITY_LABEL_STRIP_SCHEMA_REF,
        ],
    );
    row.compatibility_states = CO::ALL.to_vec();
    row.host_runtime_models = HM::ALL.to_vec();
    row.dispositions = vec![BD::Incompatible, BD::Limited, BD::Verified];
    row.required_labels = labels_with(&[L::CompatibilityAndHost]);
    row.consumer_surfaces = vec![
        C::MarketplaceUi,
        C::ExtensionsUi,
        C::InstallReviewUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::CompatibilityRangeUnstated,
        D::HostModelUnstated,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Permission-manifest summary.
    let mut row = base_row(
        F::PermissionManifestSummary,
        Q::Stable,
        "Extension security owner",
        "One permission-manifest-summary model naming the permission posture (minimal, standard, elevated, or policy-restricted) and any transitive widening introduced through dependencies, so permission widening is always named rather than hidden behind compact install chrome",
        "evidence:m5-permission-manifest-summary-parity:001",
        &[
            M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
            M5_PERMISSION_MANIFEST_SUMMARY_SCHEMA_REF,
        ],
    );
    row.permission_postures = PP::ALL.to_vec();
    row.dispositions = vec![BD::Limited, BD::Verified, BD::Enterprise];
    row.required_labels = labels_with(&[L::PermissionAndBudget]);
    row.consumer_surfaces = vec![
        C::InstallReviewUi,
        C::ExtensionsUi,
        C::AiContextUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::PermissionWideningHidden,
        D::TransitivePermissionHidden,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Activation-budget band.
    let mut row = base_row(
        F::ActivationBudgetBand,
        Q::Stable,
        "Extension runtime owner",
        "One activation-budget-band model naming whether the artifact is within, near, or over its activation budget, or throttled or suspended for exceeding it, so an over-budget or throttled artifact never reads as cost-free",
        "evidence:m5-activation-budget-band-parity:001",
        &[
            M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
            M5_ACTIVATION_BUDGET_BAND_SCHEMA_REF,
        ],
    );
    row.activation_budget_bands = AB::ALL.to_vec();
    row.dispositions = vec![BD::OverBudget, BD::Throttled, BD::Limited];
    row.required_labels = labels_with(&[L::PermissionAndBudget]);
    row.consumer_surfaces = vec![
        C::ExtensionsUi,
        C::SettingsUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ActivationCostHidden,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Install/update/disable/rollback review sheet.
    let mut row = base_row(
        F::InstallUpdateDisableRollbackReviewSheet,
        Q::Stable,
        "Install review owner",
        "One install/update/disable/rollback-review-sheet model naming the disable scope (workspace, global, profile, keep-data, or full uninstall) and the rollback-compatibility class before any mutation, so a workspace-only disable is never mistaken for a blanket removal and a rollback with data loss or no prior version is never implied to be a clean revert",
        "evidence:m5-install-review-sheet-parity:001",
        &[
            M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
            M5_INSTALL_UPDATE_DISABLE_ROLLBACK_REVIEW_SHEET_SCHEMA_REF,
        ],
    );
    row.disable_scope_classes = DS::ALL.to_vec();
    row.rollback_compatibility_states = RB::ALL.to_vec();
    row.dispositions = vec![BD::DisableScope, BD::RollbackCompatibility, BD::Limited];
    row.required_labels = labels_with(&[L::PublisherAndSourceClass]);
    row.consumer_surfaces = vec![
        C::InstallReviewUi,
        C::ExtensionsUi,
        C::RegistryUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::DisableScopeUnstated,
        D::RollbackIncompatibilityHidden,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 7. Publisher-continuity row.
    let mut row = base_row(
        F::PublisherContinuityRow,
        Q::Stable,
        "Registry integrity owner",
        "One publisher-continuity-row model naming publisher continuity (continuous, transferred, deprecated, abandoned, or verified) plus the registry source class, so a transferred, deprecated, or abandoned publisher is never presented as continuous and a publisher transfer never stays implicit",
        "evidence:m5-publisher-continuity-row-parity:001",
        &[
            M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
            M5_PUBLISHER_CONTINUITY_ROW_SCHEMA_REF,
        ],
    );
    row.publisher_continuity_states = PC::ALL.to_vec();
    row.registry_source_classes = vec![
        RS::PublicRegistry,
        RS::MirroredRegistry,
        RS::EnterpriseRegistry,
        RS::VerifiedPartner,
    ];
    row.dispositions = vec![BD::Transferred, BD::Deprecated, BD::Verified, BD::Public];
    row.required_labels = labels_with(&[L::PublisherAndSourceClass]);
    row.consumer_surfaces = vec![
        C::MarketplaceUi,
        C::RegistryUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::PublisherTransferHidden,
        D::RegistrySourceClassCollapsed,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 8. Installed-state diagnostics card.
    let mut row = base_row(
        F::InstalledStateDiagnosticsCard,
        Q::Stable,
        "Extension runtime owner",
        "One installed-state-diagnostics-card model naming quarantine history, the activation-budget band, and the compatibility state of an installed artifact, so quarantine history is never hidden behind an otherwise healthy card and an over-budget or incompatible installed artifact is never presented as ready",
        "evidence:m5-installed-state-diagnostics-card-parity:001",
        &[
            M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
            M5_INSTALLED_STATE_DIAGNOSTICS_CARD_SCHEMA_REF,
        ],
    );
    row.quarantine_states = QU::ALL.to_vec();
    row.activation_budget_bands = vec![
        AB::WithinBudget,
        AB::OverBudget,
        AB::Throttled,
        AB::SuspendedOverBudget,
    ];
    row.compatibility_states = vec![
        CO::Compatible,
        CO::Incompatible,
        CO::DegradedHost,
        CO::CompatibilityUnknown,
    ];
    row.dispositions = vec![BD::Quarantined, BD::Throttled, BD::OverBudget, BD::Limited];
    row.required_labels = labels_with(&[L::CompatibilityAndHost, L::PermissionAndBudget]);
    row.consumer_surfaces = vec![
        C::ExtensionsUi,
        C::SettingsUi,
        C::AiContextUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::QuarantineHistoryHidden,
        D::ActivationCostHidden,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5MarketplaceInstallGovernanceReview {
    M5MarketplaceInstallGovernanceReview {
        marketplace_result_row_shows_source_class_and_compatibility: true,
        marketplace_detail_fact_grid_shows_all_facts_together: true,
        compatibility_label_strip_shows_range_and_host_model: true,
        permission_manifest_summary_shows_posture_and_transitive_widening: true,
        activation_budget_band_shows_band_and_throttle: true,
        install_review_sheet_shows_disable_scope_and_rollback: true,
        publisher_continuity_row_shows_transfer_and_deprecation: true,
        installed_state_diagnostics_card_shows_quarantine_and_health: true,
        no_component_hides_permission_widening: true,
        activation_cost_always_explicit: true,
        publisher_transfer_never_hidden: true,
        registry_source_class_always_explicit: true,
        rollback_incompatibility_never_hidden: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_marketplace_vocabulary: true,
    }
}

fn consumer_projection() -> M5MarketplaceInstallConsumerProjection {
    M5MarketplaceInstallConsumerProjection {
        marketplace_surfaces_consume_source_class_vocabulary: true,
        extension_manager_consumes_permission_posture_vocabulary: true,
        install_review_consumes_disable_scope_and_rollback_vocabulary: true,
        registry_admin_consumes_publisher_continuity_vocabulary: true,
        help_consumes_compatibility_vocabulary: true,
        support_export_reads_single_marketplace_source: true,
    }
}

fn proof_freshness() -> M5MarketplaceInstallProofFreshness {
    M5MarketplaceInstallProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5MarketplaceInstallReleasePosture {
    M5MarketplaceInstallReleasePosture {
        proof_packet_ref: M5_MARKETPLACE_INSTALL_COMPONENT_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_MARKETPLACE_INSTALL_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_DOC_REF,
        M5_MARKETPLACE_RESULT_ROW_SCHEMA_REF,
        M5_MARKETPLACE_DETAIL_FACT_GRID_SCHEMA_REF,
        M5_COMPATIBILITY_LABEL_STRIP_SCHEMA_REF,
        M5_PERMISSION_MANIFEST_SUMMARY_SCHEMA_REF,
        M5_ACTIVATION_BUDGET_BAND_SCHEMA_REF,
        M5_INSTALL_UPDATE_DISABLE_ROLLBACK_REVIEW_SHEET_SCHEMA_REF,
        M5_PUBLISHER_CONTINUITY_ROW_SCHEMA_REF,
        M5_INSTALLED_STATE_DIAGNOSTICS_CARD_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 marketplace-install component matrix packet.
pub fn seeded_m5_marketplace_install_component_matrix() -> M5MarketplaceInstallComponentMatrixPacket
{
    M5MarketplaceInstallComponentMatrixPacket::new(M5MarketplaceInstallComponentMatrixPacketInput {
        packet_id: M5_MARKETPLACE_INSTALL_COMPONENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 marketplace-result-row, marketplace-detail-fact-grid, compatibility-label-strip, permission-manifest-summary, activation-budget-band, install/update/disable/rollback review-sheet, publisher-continuity-row, and installed-state-diagnostics-card component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5MarketplaceInstallVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the compatibility-label strip is held at Beta because runtime/host-model
/// parity round-trips are not yet proven across every deployment line; every component stays
/// visible.
pub fn seeded_m5_marketplace_install_component_matrix_compatibility_label_strip_beta_narrowed(
) -> M5MarketplaceInstallComponentMatrixPacket {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    packet.packet_id =
        "m5-marketplace-install-components:compatibility-label-strip-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5MarketplaceInstallComponentFamily::CompatibilityLabelStrip
        })
        .expect("compatibility-label-strip row present");
    row.qualification = M5MarketplaceInstallQualificationClass::Beta;
    packet
}

/// Narrowed variant: the install/update/disable/rollback review sheet is narrowed to Preview
/// pending rollback-compatibility parity across every deployment line; every component stays
/// visible.
pub fn seeded_m5_marketplace_install_component_matrix_install_review_sheet_preview_narrowed(
) -> M5MarketplaceInstallComponentMatrixPacket {
    let mut packet = seeded_m5_marketplace_install_component_matrix();
    packet.packet_id =
        "m5-marketplace-install-components:install-review-sheet-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family
                == M5MarketplaceInstallComponentFamily::InstallUpdateDisableRollbackReviewSheet
        })
        .expect("install-review-sheet row present");
    row.qualification = M5MarketplaceInstallQualificationClass::Preview;
    packet
}
