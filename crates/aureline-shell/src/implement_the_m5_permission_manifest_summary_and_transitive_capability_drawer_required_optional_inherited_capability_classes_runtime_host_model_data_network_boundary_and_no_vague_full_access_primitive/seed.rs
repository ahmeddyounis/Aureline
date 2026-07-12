//! Canonical seed builders for the M5 permission-manifest-summary / transitive-capability-drawer
//! controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce. Clean summaries
//! group their capabilities into required / optional / inherited classes and name their data /
//! network boundaries; clean drawers disclose transitive widening and attribute dependency-contributed
//! permissions, so trust is never quietly continued behind one vague full-access label.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_PERMISSION_MANIFEST_CONTROLS_PACKET_ID: &str =
    "m5-permission-manifest-summary-transitive-capability-drawer-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn summary(
    input: M5PermissionManifestSummaryResolutionInput,
) -> M5ResolvedPermissionManifestSummary {
    resolve_permission_manifest_summary(input).expect("seed permission manifest summary resolves")
}

fn drawer(
    input: M5TransitiveCapabilityDrawerResolutionInput,
) -> M5ResolvedTransitiveCapabilityDrawer {
    resolve_transitive_capability_drawer(input).expect("seed transitive capability drawer resolves")
}

// -- Clean permission-manifest summary examples ------------------------------------------------

/// Clean summary: a standard posture in a sandboxed host, grouped into required / optional classes,
/// with data and network boundaries and a canonical manifest digest.
fn summary_standard_clean() -> M5ResolvedPermissionManifestSummary {
    summary(M5PermissionManifestSummaryResolutionInput {
        summary_id: "perm-summary:acme-linter".to_owned(),
        artifact_identity: "acme-linter".to_owned(),
        posture: M5PermissionPostureState::Standard,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        required_capabilities: strings(&["read workspace files", "read diagnostics"]),
        optional_capabilities: strings(&["write formatter settings"]),
        inherited_capabilities: Vec::new(),
        data_boundary: "reads workspace files, no data leaves the sandbox".to_owned(),
        network_boundary: "no network access".to_owned(),
        manifest_digest: "sha256-manifest-v3-acme-linter".to_owned(),
        flattens_into_full_access: false,
        proof_fresh: true,
    })
}

/// Clean summary: a minimal posture in the main process, tiny required set, boundaries and digest
/// stated.
fn summary_minimal_clean() -> M5ResolvedPermissionManifestSummary {
    summary(M5PermissionManifestSummaryResolutionInput {
        summary_id: "perm-summary:tiny-tool".to_owned(),
        artifact_identity: "tiny-tool".to_owned(),
        posture: M5PermissionPostureState::Minimal,
        host_runtime_model: M5HostRuntimeModel::InProcess,
        required_capabilities: strings(&["read active file"]),
        optional_capabilities: Vec::new(),
        inherited_capabilities: Vec::new(),
        data_boundary: "reads the active file only".to_owned(),
        network_boundary: "no network access".to_owned(),
        manifest_digest: "sha256-manifest-v1-tiny-tool".to_owned(),
        flattens_into_full_access: false,
        proof_fresh: true,
    })
}

/// Clean summary: a transitively-widened posture, grouping the inherited (dependency-contributed)
/// class explicitly, with boundaries and a digest.
fn summary_widened_clean() -> M5ResolvedPermissionManifestSummary {
    summary(M5PermissionManifestSummaryResolutionInput {
        summary_id: "perm-summary:corp-suite".to_owned(),
        artifact_identity: "corp-suite".to_owned(),
        posture: M5PermissionPostureState::WidenedTransitive,
        host_runtime_model: M5HostRuntimeModel::RemoteHost,
        required_capabilities: strings(&["read workspace files"]),
        optional_capabilities: strings(&["write workspace files"]),
        inherited_capabilities: strings(&["network read from telemetry-sdk"]),
        data_boundary: "reads and writes workspace files".to_owned(),
        network_boundary: "outbound to declared telemetry hosts via telemetry-sdk".to_owned(),
        manifest_digest: "sha256-manifest-v4-corp-suite".to_owned(),
        flattens_into_full_access: false,
        proof_fresh: true,
    })
}

// -- Degraded permission-manifest summary examples ---------------------------------------------

/// Degraded summary: the artifact identity is unstated.
fn summary_identity_unstated() -> M5ResolvedPermissionManifestSummary {
    summary(M5PermissionManifestSummaryResolutionInput {
        summary_id: "perm-summary:no-identity".to_owned(),
        artifact_identity: "  ".to_owned(),
        posture: M5PermissionPostureState::Standard,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        required_capabilities: strings(&["read workspace files"]),
        optional_capabilities: Vec::new(),
        inherited_capabilities: Vec::new(),
        data_boundary: "reads workspace files".to_owned(),
        network_boundary: "no network access".to_owned(),
        manifest_digest: "sha256-manifest-vx".to_owned(),
        flattens_into_full_access: false,
        proof_fresh: true,
    })
}

/// Degraded summary: the permission posture cannot be resolved.
fn summary_posture_unknown() -> M5ResolvedPermissionManifestSummary {
    summary(M5PermissionManifestSummaryResolutionInput {
        summary_id: "perm-summary:posture-unknown".to_owned(),
        artifact_identity: "pending-artifact".to_owned(),
        posture: M5PermissionPostureState::PostureUnknown,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        required_capabilities: Vec::new(),
        optional_capabilities: Vec::new(),
        inherited_capabilities: Vec::new(),
        data_boundary: "".to_owned(),
        network_boundary: "".to_owned(),
        manifest_digest: "".to_owned(),
        flattens_into_full_access: false,
        proof_fresh: true,
    })
}

/// Degraded summary: the host / runtime model cannot be resolved.
fn summary_host_unknown() -> M5ResolvedPermissionManifestSummary {
    summary(M5PermissionManifestSummaryResolutionInput {
        summary_id: "perm-summary:host-unknown".to_owned(),
        artifact_identity: "hostless-artifact".to_owned(),
        posture: M5PermissionPostureState::Standard,
        host_runtime_model: M5HostRuntimeModel::HostUnknown,
        required_capabilities: strings(&["read workspace files"]),
        optional_capabilities: Vec::new(),
        inherited_capabilities: Vec::new(),
        data_boundary: "reads workspace files".to_owned(),
        network_boundary: "no network access".to_owned(),
        manifest_digest: "sha256-manifest-vh".to_owned(),
        flattens_into_full_access: false,
        proof_fresh: true,
    })
}

/// Degraded summary: a capability-requesting posture names no required-capability grouping.
fn summary_capability_grouping_unstated() -> M5ResolvedPermissionManifestSummary {
    summary(M5PermissionManifestSummaryResolutionInput {
        summary_id: "perm-summary:grouping-unstated".to_owned(),
        artifact_identity: "ungrouped-artifact".to_owned(),
        posture: M5PermissionPostureState::Standard,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        required_capabilities: Vec::new(),
        optional_capabilities: Vec::new(),
        inherited_capabilities: Vec::new(),
        data_boundary: "reads workspace files".to_owned(),
        network_boundary: "no network access".to_owned(),
        manifest_digest: "sha256-manifest-vg".to_owned(),
        flattens_into_full_access: false,
        proof_fresh: true,
    })
}

/// Degraded summary: the data / network boundary is unstated.
fn summary_boundary_unstated() -> M5ResolvedPermissionManifestSummary {
    summary(M5PermissionManifestSummaryResolutionInput {
        summary_id: "perm-summary:boundary-unstated".to_owned(),
        artifact_identity: "boundaryless-artifact".to_owned(),
        posture: M5PermissionPostureState::Standard,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        required_capabilities: strings(&["read workspace files"]),
        optional_capabilities: Vec::new(),
        inherited_capabilities: Vec::new(),
        data_boundary: "".to_owned(),
        network_boundary: "".to_owned(),
        manifest_digest: "sha256-manifest-vb".to_owned(),
        flattens_into_full_access: false,
        proof_fresh: true,
    })
}

/// Degraded summary: the manifest is flattened into one vague full-access label.
fn summary_flattened() -> M5ResolvedPermissionManifestSummary {
    summary(M5PermissionManifestSummaryResolutionInput {
        summary_id: "perm-summary:flattened".to_owned(),
        artifact_identity: "full-access-artifact".to_owned(),
        posture: M5PermissionPostureState::Elevated,
        host_runtime_model: M5HostRuntimeModel::NativeHost,
        required_capabilities: strings(&["read workspace files", "run native host process"]),
        optional_capabilities: Vec::new(),
        inherited_capabilities: Vec::new(),
        data_boundary: "reads and writes workspace files".to_owned(),
        network_boundary: "outbound to declared hosts".to_owned(),
        manifest_digest: "sha256-manifest-vf".to_owned(),
        flattens_into_full_access: true,
        proof_fresh: true,
    })
}

/// Degraded summary: the summary cannot be traced back to a canonical manifest digest.
fn summary_digest_unstated() -> M5ResolvedPermissionManifestSummary {
    summary(M5PermissionManifestSummaryResolutionInput {
        summary_id: "perm-summary:digest-unstated".to_owned(),
        artifact_identity: "digestless-artifact".to_owned(),
        posture: M5PermissionPostureState::Standard,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        required_capabilities: strings(&["read workspace files"]),
        optional_capabilities: Vec::new(),
        inherited_capabilities: Vec::new(),
        data_boundary: "reads workspace files".to_owned(),
        network_boundary: "no network access".to_owned(),
        manifest_digest: "".to_owned(),
        flattens_into_full_access: false,
        proof_fresh: true,
    })
}

// -- Clean transitive-capability drawer examples -----------------------------------------------

/// Clean drawer: a transitively-widened posture disclosing its widening and attributing each
/// dependency-contributed permission.
fn drawer_widened_disclosed_clean() -> M5ResolvedTransitiveCapabilityDrawer {
    drawer(M5TransitiveCapabilityDrawerResolutionInput {
        drawer_id: "transitive-drawer:corp-suite".to_owned(),
        artifact_identity: "corp-suite".to_owned(),
        posture: M5PermissionPostureState::WidenedTransitive,
        transitive_widening_disclosed: true,
        dependency_contributed_capabilities: strings(&["network read", "read cache directory"]),
        dependency_attributions: strings(&[
            "network read contributed by telemetry-sdk >=2.1",
            "read cache directory contributed by cache-helper >=1.4",
        ]),
        manifest_digest: "sha256-manifest-v4-corp-suite".to_owned(),
        flattens_into_full_access: false,
        proof_fresh: true,
    })
}

/// Clean drawer: a minimal posture that widens nothing and contributes no dependency permissions.
fn drawer_minimal_clean() -> M5ResolvedTransitiveCapabilityDrawer {
    drawer(M5TransitiveCapabilityDrawerResolutionInput {
        drawer_id: "transitive-drawer:tiny-tool".to_owned(),
        artifact_identity: "tiny-tool".to_owned(),
        posture: M5PermissionPostureState::Minimal,
        transitive_widening_disclosed: false,
        dependency_contributed_capabilities: Vec::new(),
        dependency_attributions: Vec::new(),
        manifest_digest: "sha256-manifest-v1-tiny-tool".to_owned(),
        flattens_into_full_access: false,
        proof_fresh: true,
    })
}

// -- Degraded transitive-capability drawer examples --------------------------------------------

/// Degraded drawer: a transitively-widened posture hides its widening.
fn drawer_widening_hidden() -> M5ResolvedTransitiveCapabilityDrawer {
    drawer(M5TransitiveCapabilityDrawerResolutionInput {
        drawer_id: "transitive-drawer:widening-hidden".to_owned(),
        artifact_identity: "silently-widened-artifact".to_owned(),
        posture: M5PermissionPostureState::WidenedTransitive,
        transitive_widening_disclosed: false,
        dependency_contributed_capabilities: strings(&["network read"]),
        dependency_attributions: strings(&["network read contributed by telemetry-sdk"]),
        manifest_digest: "sha256-manifest-vw".to_owned(),
        flattens_into_full_access: false,
        proof_fresh: true,
    })
}

/// Degraded drawer: dependency-contributed permissions carry no attribution.
fn drawer_attribution_missing() -> M5ResolvedTransitiveCapabilityDrawer {
    drawer(M5TransitiveCapabilityDrawerResolutionInput {
        drawer_id: "transitive-drawer:attribution-missing".to_owned(),
        artifact_identity: "unattributed-artifact".to_owned(),
        posture: M5PermissionPostureState::Standard,
        transitive_widening_disclosed: false,
        dependency_contributed_capabilities: strings(&["write workspace files"]),
        dependency_attributions: Vec::new(),
        manifest_digest: "sha256-manifest-va".to_owned(),
        flattens_into_full_access: false,
        proof_fresh: true,
    })
}

/// Degraded drawer: the drawer collapses into one vague full-access label.
fn drawer_flattened() -> M5ResolvedTransitiveCapabilityDrawer {
    drawer(M5TransitiveCapabilityDrawerResolutionInput {
        drawer_id: "transitive-drawer:flattened".to_owned(),
        artifact_identity: "full-access-drawer-artifact".to_owned(),
        posture: M5PermissionPostureState::Elevated,
        transitive_widening_disclosed: false,
        dependency_contributed_capabilities: Vec::new(),
        dependency_attributions: Vec::new(),
        manifest_digest: "sha256-manifest-vd".to_owned(),
        flattens_into_full_access: true,
        proof_fresh: true,
    })
}

/// Degraded drawer: the drawer cannot be traced back to a canonical manifest digest.
fn drawer_digest_unstated() -> M5ResolvedTransitiveCapabilityDrawer {
    drawer(M5TransitiveCapabilityDrawerResolutionInput {
        drawer_id: "transitive-drawer:digest-unstated".to_owned(),
        artifact_identity: "digestless-drawer-artifact".to_owned(),
        posture: M5PermissionPostureState::Standard,
        transitive_widening_disclosed: false,
        dependency_contributed_capabilities: Vec::new(),
        dependency_attributions: Vec::new(),
        manifest_digest: "".to_owned(),
        flattens_into_full_access: false,
        proof_fresh: true,
    })
}

/// Degraded drawer: the artifact identity is unstated.
fn drawer_identity_unstated() -> M5ResolvedTransitiveCapabilityDrawer {
    drawer(M5TransitiveCapabilityDrawerResolutionInput {
        drawer_id: "transitive-drawer:no-identity".to_owned(),
        artifact_identity: "  ".to_owned(),
        posture: M5PermissionPostureState::Standard,
        transitive_widening_disclosed: false,
        dependency_contributed_capabilities: Vec::new(),
        dependency_attributions: Vec::new(),
        manifest_digest: "sha256-manifest-vi".to_owned(),
        flattens_into_full_access: false,
        proof_fresh: true,
    })
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5PermissionManifestConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5MarketplaceInstallDowngradeTrigger>,
    permission_manifest_summary_examples: Vec<M5ResolvedPermissionManifestSummary>,
    transitive_capability_drawer_examples: Vec<M5ResolvedTransitiveCapabilityDrawer>,
) -> M5PermissionManifestControlsRow {
    M5PermissionManifestControlsRow {
        consumer_surface,
        qualification: M5MarketplaceInstallQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5MarketplaceInstallDeploymentLine::ALL.to_vec(),
        required_labels: M5MarketplaceInstallRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5MarketplaceInstallAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5PermissionManifestAnatomyPart::ALL.to_vec(),
        export_fields: M5PermissionManifestExportField::ALL.to_vec(),
        downgrade_triggers,
        permission_manifest_summary_examples,
        transitive_capability_drawer_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_PERMISSION_MANIFEST_CONTROLS_SCHEMA_REF,
            M5_PERMISSION_MANIFEST_SUMMARY_SCHEMA_REF,
        ]),
        flattens_permissions_into_vague_full_access: false,
        hides_transitive_or_dependency_contributed_widening: false,
        hides_data_network_or_runtime_boundary: false,
        severs_summary_from_canonical_manifest_digest: false,
    }
}

fn controls_rows() -> Vec<M5PermissionManifestControlsRow> {
    use M5MarketplaceInstallConsumerSurface as C;
    use M5MarketplaceInstallDowngradeTrigger as D;

    vec![
        base_row(
            C::MarketplaceUi,
            "Marketplace catalog owner",
            "The marketplace listing renders one permission-manifest summary per artifact naming the permission posture, required / optional / inherited capability classes, runtime / host model, and data / network boundaries so a compare decision needs no disconnected page, and degrades honestly when a capability-requesting posture names no grouping",
            "evidence:m5-permission-manifest-marketplace-ui:001",
            vec![
                D::PermissionWideningHidden,
                D::TransitivePermissionHidden,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![summary_standard_clean(), summary_capability_grouping_unstated()],
            vec![drawer_minimal_clean(), drawer_widening_hidden()],
        ),
        base_row(
            C::ExtensionsUi,
            "Extensions manager owner",
            "The extensions detail surface reuses the same grouping model, shows a transitively-widened artifact disclosing its widening with each dependency-contributed permission attributed to the dependency that contributed it, and degrades honestly when the data / network boundary or dependency attribution is hidden",
            "evidence:m5-permission-manifest-extensions-ui:001",
            vec![
                D::PermissionWideningHidden,
                D::TransitivePermissionHidden,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![summary_minimal_clean(), summary_boundary_unstated()],
            vec![drawer_widened_disclosed_clean(), drawer_attribution_missing()],
        ),
        base_row(
            C::InstallReviewUi,
            "Install-review owner",
            "The install / update review sheet keeps the permission posture explicit before install trust silently continues, groups the inherited (dependency-contributed) class explicitly, and degrades honestly when the posture cannot be resolved or the drawer is severed from its canonical manifest digest",
            "evidence:m5-permission-manifest-install-review-ui:001",
            vec![
                D::PermissionWideningHidden,
                D::TransitivePermissionHidden,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![summary_widened_clean(), summary_posture_unknown()],
            vec![drawer_minimal_clean(), drawer_digest_unstated()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved summary and drawer truth, so a manifest flattened into one vague full-access label, an unstated host model, a missing manifest digest, or an unattributed dependency-contributed permission is visible in evidence rather than hidden behind compact chrome",
            "evidence:m5-permission-manifest-support-export:001",
            vec![
                D::PermissionWideningHidden,
                D::TransitivePermissionHidden,
                D::HostModelUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                summary_flattened(),
                summary_digest_unstated(),
                summary_host_unknown(),
            ],
            vec![drawer_flattened(), drawer_identity_unstated()],
        ),
        base_row(
            C::ProductUi,
            "In-product diagnostics owner",
            "In-product listing and diagnostics surfaces reuse the same permission grammar, keep the standard posture and its capability classes explicit, and degrade honestly when the artifact identity is missing or a transitively-widened posture hides its widening so no widened trust is quietly carried forward into installed-state diagnostics",
            "evidence:m5-permission-manifest-product-ui:001",
            vec![
                D::GenericChromeWordingUsed,
                D::PermissionWideningHidden,
                D::TransitivePermissionHidden,
                D::ProofStale,
            ],
            vec![summary_standard_clean(), summary_identity_unstated()],
            vec![drawer_minimal_clean(), drawer_widening_hidden()],
        ),
    ]
}

fn governance_review() -> M5PermissionManifestGovernanceReview {
    M5PermissionManifestGovernanceReview {
        summary_names_posture_and_capability_classes: true,
        summary_names_runtime_host_and_boundaries: true,
        drawer_discloses_transitive_widening: true,
        drawer_attributes_dependency_contributed_permissions: true,
        permissions_never_flattened_into_full_access: true,
        transitive_widening_always_visible_and_attributable: true,
        data_network_runtime_boundary_always_explicit: true,
        summaries_trace_to_single_manifest_digest: true,
        posture_explicit_across_all_surfaces: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5PermissionManifestConsumerProjection {
    M5PermissionManifestConsumerProjection {
        marketplace_surfaces_consume_permission_vocabulary: true,
        install_surfaces_consume_transitive_widening_vocabulary: true,
        facts_trace_to_single_component_contract: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5PermissionManifestProofFreshness {
    M5PermissionManifestProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5PermissionManifestReleasePosture {
    M5PermissionManifestReleasePosture {
        proof_packet_ref: M5_PERMISSION_MANIFEST_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_PERMISSION_MANIFEST_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_PERMISSION_MANIFEST_CONTROLS_SCHEMA_REF,
        M5_PERMISSION_MANIFEST_CONTROLS_DOC_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_DOC_REF,
        M5_PERMISSION_MANIFEST_SUMMARY_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 permission-manifest-summary / transitive-capability-drawer controls packet.
pub fn seeded_m5_permission_manifest_controls() -> M5PermissionManifestControlsPacket {
    M5PermissionManifestControlsPacket::new(M5PermissionManifestControlsPacketInput {
        packet_id: M5_PERMISSION_MANIFEST_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 permission-manifest-summary and transitive-capability-drawer controls with required / optional / inherited capability classes, runtime / host model, data / network boundaries, and transitive-widening attribution across listing, detail, install, update, diagnostics, and export"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5PermissionManifestVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the marketplace-UI row is held at Beta pending permission-posture parity on
/// every deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_permission_manifest_controls_marketplace_ui_beta_narrowed(
) -> M5PermissionManifestControlsPacket {
    let mut packet = seeded_m5_permission_manifest_controls();
    packet.packet_id =
        "m5-permission-manifest-summary-transitive-capability-drawer-controls:marketplace-ui-beta:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5MarketplaceInstallConsumerSurface::MarketplaceUi)
        .expect("marketplace-ui row present");
    row.qualification = M5MarketplaceInstallQualificationClass::Beta;
    packet
}

/// Narrowed variant: the install-review row is narrowed to Preview pending transitive-widening
/// parity on every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_permission_manifest_controls_install_review_ui_preview_narrowed(
) -> M5PermissionManifestControlsPacket {
    let mut packet = seeded_m5_permission_manifest_controls();
    packet.packet_id =
        "m5-permission-manifest-summary-transitive-capability-drawer-controls:install-review-preview:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5MarketplaceInstallConsumerSurface::InstallReviewUi)
        .expect("install-review row present");
    row.qualification = M5MarketplaceInstallQualificationClass::Preview;
    packet
}
