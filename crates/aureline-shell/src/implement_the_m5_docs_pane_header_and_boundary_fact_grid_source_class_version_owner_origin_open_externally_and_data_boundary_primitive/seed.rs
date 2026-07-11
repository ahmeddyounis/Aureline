//! Canonical seed builders for the M5 docs-pane-header / boundary-fact-grid controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_DOCS_BOUNDARY_CONTROLS_PACKET_ID: &str =
    "m5-docs-pane-header-boundary-fact-grid-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn header(input: M5DocsPaneHeaderResolutionInput) -> M5ResolvedDocsPaneHeader {
    resolve_docs_pane_header(input).expect("seed docs-pane header input resolves")
}

fn grid(input: M5BoundaryFactGridResolutionInput) -> M5ResolvedBoundaryFactGrid {
    resolve_boundary_fact_grid(input).expect("seed boundary-fact grid input resolves")
}

// -- Canonical docs-pane header examples ------------------------------------------------------

/// Clean header for project-local documentation — proves AC1's project-local distinction.
fn header_project_local_clean() -> M5ResolvedDocsPaneHeader {
    header(M5DocsPaneHeaderResolutionInput {
        header_id: "docs-header:project-local".to_owned(),
        source_class: M5DocsSourceClass::ProjectLocal,
        owner_class: WebviewOwnerClass::FirstPartyEmbedded,
        owner_disclosed: true,
        pack_identity: "aureline-docs v2026.07".to_owned(),
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        last_updated_stated: true,
        capability_limits: vec![CapabilityLimitClass::NotNativeTrustChrome],
        handoff_required: false,
        open_externally_available: true,
        find_in_page_applicable: true,
        find_in_page_available: true,
        proof_fresh: true,
    })
}

/// Clean header for mirrored vendor material — proves AC1's mirrored-vendor distinction.
fn header_mirrored_vendor_clean() -> M5ResolvedDocsPaneHeader {
    header(M5DocsPaneHeaderResolutionInput {
        header_id: "docs-header:mirrored-vendor".to_owned(),
        source_class: M5DocsSourceClass::MirroredVendor,
        owner_class: WebviewOwnerClass::ProviderOwned,
        owner_disclosed: true,
        pack_identity: "vendor-mirror pack 41".to_owned(),
        freshness: M5EmbeddedFreshnessState::WarmSnapshot,
        last_updated_stated: true,
        capability_limits: vec![
            CapabilityLimitClass::NotNativeTrustChrome,
            CapabilityLimitClass::CannotVerifyUpdates,
        ],
        handoff_required: false,
        open_externally_available: true,
        find_in_page_applicable: true,
        find_in_page_available: true,
        proof_fresh: true,
    })
}

/// Clean header for extension-contributed documentation — proves AC1's extension distinction.
fn header_extension_clean() -> M5ResolvedDocsPaneHeader {
    header(M5DocsPaneHeaderResolutionInput {
        header_id: "docs-header:extension".to_owned(),
        source_class: M5DocsSourceClass::ExtensionContributed,
        owner_class: WebviewOwnerClass::ExtensionOwned,
        owner_disclosed: true,
        pack_identity: "acme-lang-pack 3.4.1".to_owned(),
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        last_updated_stated: true,
        capability_limits: vec![
            CapabilityLimitClass::NotNativeTrustChrome,
            CapabilityLimitClass::CannotDisplayProductSecurity,
        ],
        handoff_required: false,
        open_externally_available: true,
        find_in_page_applicable: true,
        find_in_page_available: true,
        proof_fresh: true,
    })
}

/// Clean header for browser-handoff-required content — proves AC1's handoff distinction and that a
/// required handoff is exposed.
fn header_handoff_clean() -> M5ResolvedDocsPaneHeader {
    header(M5DocsPaneHeaderResolutionInput {
        header_id: "docs-header:handoff".to_owned(),
        source_class: M5DocsSourceClass::BrowserHandoffRequired,
        owner_class: WebviewOwnerClass::ProviderOwned,
        owner_disclosed: true,
        pack_identity: "provider-portal handoff".to_owned(),
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        last_updated_stated: true,
        capability_limits: vec![CapabilityLimitClass::NotNativeTrustChrome],
        handoff_required: true,
        open_externally_available: true,
        find_in_page_applicable: false,
        find_in_page_available: false,
        proof_fresh: true,
    })
}

/// Degraded header: the source class is unstated — proves AC1's negative half (an undistinguishable
/// pane never reads clean).
fn header_source_unknown() -> M5ResolvedDocsPaneHeader {
    header(M5DocsPaneHeaderResolutionInput {
        header_id: "docs-header:unknown-source".to_owned(),
        source_class: M5DocsSourceClass::SourceUnknown,
        owner_class: WebviewOwnerClass::UnknownUntrusted,
        owner_disclosed: false,
        pack_identity: "".to_owned(),
        freshness: M5EmbeddedFreshnessState::FreshnessUnknown,
        last_updated_stated: false,
        capability_limits: vec![CapabilityLimitClass::NotNativeTrustChrome],
        handoff_required: false,
        open_externally_available: true,
        find_in_page_applicable: true,
        find_in_page_available: false,
        proof_fresh: true,
    })
}

/// Degraded header: a required browser handoff is not exposed — proves AC2's handoff half.
fn header_handoff_not_exposed() -> M5ResolvedDocsPaneHeader {
    header(M5DocsPaneHeaderResolutionInput {
        header_id: "docs-header:handoff-hidden".to_owned(),
        source_class: M5DocsSourceClass::BrowserHandoffRequired,
        owner_class: WebviewOwnerClass::ProviderOwned,
        owner_disclosed: true,
        pack_identity: "provider-portal handoff".to_owned(),
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        last_updated_stated: true,
        capability_limits: vec![CapabilityLimitClass::NotNativeTrustChrome],
        handoff_required: true,
        open_externally_available: false,
        find_in_page_applicable: false,
        find_in_page_available: false,
        proof_fresh: true,
    })
}

/// Degraded header: the owner / origin is undisclosed.
fn header_owner_undisclosed() -> M5ResolvedDocsPaneHeader {
    header(M5DocsPaneHeaderResolutionInput {
        header_id: "docs-header:owner-hidden".to_owned(),
        source_class: M5DocsSourceClass::MirroredVendor,
        owner_class: WebviewOwnerClass::ProviderOwned,
        owner_disclosed: false,
        pack_identity: "vendor-mirror pack 41".to_owned(),
        freshness: M5EmbeddedFreshnessState::WarmSnapshot,
        last_updated_stated: true,
        capability_limits: vec![CapabilityLimitClass::NotNativeTrustChrome],
        handoff_required: false,
        open_externally_available: true,
        find_in_page_applicable: true,
        find_in_page_available: true,
        proof_fresh: true,
    })
}

// -- Canonical boundary-fact grid examples ----------------------------------------------------

/// Clean grid for local reading — names the data boundary, posture, and reading trust.
fn grid_local_clean() -> M5ResolvedBoundaryFactGrid {
    grid(M5BoundaryFactGridResolutionInput {
        grid_id: "boundary-grid:local".to_owned(),
        source_class: M5DocsSourceClass::ProjectLocal,
        data_exit_boundary: DataExitBoundary::NoPayloadLeavesProduct,
        data_boundary_stated: true,
        reading_posture: M5PaneReadingPosture::LocalReadingSafe,
        posture_stated: true,
        reading_trust_explained: true,
        trustworthy_for_in_product_reading: true,
        claims_approval_or_policy_authority: false,
        suitable_for_high_risk_approval: false,
        proof_fresh: true,
    })
}

/// Clean grid for a mirrored snapshot — names the mirrored posture explicitly.
fn grid_mirrored_clean() -> M5ResolvedBoundaryFactGrid {
    grid(M5BoundaryFactGridResolutionInput {
        grid_id: "boundary-grid:mirrored".to_owned(),
        source_class: M5DocsSourceClass::MirroredVendor,
        data_exit_boundary: DataExitBoundary::ExternalPublicBrowse,
        data_boundary_stated: true,
        reading_posture: M5PaneReadingPosture::MirroredReadingSafe,
        posture_stated: true,
        reading_trust_explained: true,
        trustworthy_for_in_product_reading: true,
        claims_approval_or_policy_authority: false,
        suitable_for_high_risk_approval: false,
        proof_fresh: true,
    })
}

/// Degraded grid: it claims approval / policy authority — proves AC2's masquerade half.
fn grid_masquerade() -> M5ResolvedBoundaryFactGrid {
    grid(M5BoundaryFactGridResolutionInput {
        grid_id: "boundary-grid:masquerade".to_owned(),
        source_class: M5DocsSourceClass::MirroredVendor,
        data_exit_boundary: DataExitBoundary::ExternalPublicBrowse,
        data_boundary_stated: true,
        reading_posture: M5PaneReadingPosture::MirroredReadingSafe,
        posture_stated: true,
        reading_trust_explained: true,
        trustworthy_for_in_product_reading: true,
        claims_approval_or_policy_authority: true,
        suitable_for_high_risk_approval: true,
        proof_fresh: true,
    })
}

/// Degraded grid: the data boundary is unstated.
fn grid_data_boundary_unstated() -> M5ResolvedBoundaryFactGrid {
    grid(M5BoundaryFactGridResolutionInput {
        grid_id: "boundary-grid:no-boundary".to_owned(),
        source_class: M5DocsSourceClass::MirroredVendor,
        data_exit_boundary: DataExitBoundary::ExternalPublicBrowse,
        data_boundary_stated: false,
        reading_posture: M5PaneReadingPosture::MirroredReadingSafe,
        posture_stated: true,
        reading_trust_explained: true,
        trustworthy_for_in_product_reading: true,
        claims_approval_or_policy_authority: false,
        suitable_for_high_risk_approval: false,
        proof_fresh: true,
    })
}

/// Degraded grid: why the pane is trustworthy for in-product reading is not explained.
fn grid_reading_trust_unexplained() -> M5ResolvedBoundaryFactGrid {
    grid(M5BoundaryFactGridResolutionInput {
        grid_id: "boundary-grid:no-trust".to_owned(),
        source_class: M5DocsSourceClass::ExtensionContributed,
        data_exit_boundary: DataExitBoundary::VendorOrThirdPartyOutbound,
        data_boundary_stated: true,
        reading_posture: M5PaneReadingPosture::HostedReadingSafe,
        posture_stated: true,
        reading_trust_explained: false,
        trustworthy_for_in_product_reading: false,
        claims_approval_or_policy_authority: false,
        suitable_for_high_risk_approval: false,
        proof_fresh: true,
    })
}

/// Degraded grid: the offline / mirrored posture is unstated.
fn grid_posture_unknown() -> M5ResolvedBoundaryFactGrid {
    grid(M5BoundaryFactGridResolutionInput {
        grid_id: "boundary-grid:no-posture".to_owned(),
        source_class: M5DocsSourceClass::ProjectLocal,
        data_exit_boundary: DataExitBoundary::NoPayloadLeavesProduct,
        data_boundary_stated: true,
        reading_posture: M5PaneReadingPosture::PostureUnknown,
        posture_stated: false,
        reading_trust_explained: true,
        trustworthy_for_in_product_reading: true,
        claims_approval_or_policy_authority: false,
        suitable_for_high_risk_approval: false,
        proof_fresh: true,
    })
}

// -- Row builders ----------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5DocsBoundaryConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5EmbeddedDowngradeTrigger>,
    docs_pane_header_examples: Vec<M5ResolvedDocsPaneHeader>,
    boundary_fact_grid_examples: Vec<M5ResolvedBoundaryFactGrid>,
) -> M5DocsBoundaryControlsRow {
    M5DocsBoundaryControlsRow {
        consumer_surface,
        qualification: M5EmbeddedQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5EmbeddedDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5EmbeddedRequiredLabel::Identity,
            M5EmbeddedRequiredLabel::State,
            M5EmbeddedRequiredLabel::KeyboardRoute,
            M5EmbeddedRequiredLabel::OwnerAndOrigin,
            M5EmbeddedRequiredLabel::FreshnessAndCapabilityLimits,
        ],
        accessibility_routes: M5EmbeddedAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5DocsBoundaryAnatomyPart::ALL.to_vec(),
        export_fields: M5DocsBoundaryExportField::ALL.to_vec(),
        downgrade_triggers,
        docs_pane_header_examples,
        boundary_fact_grid_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_DOCS_BOUNDARY_CONTROLS_SCHEMA_REF,
            M5_DOCS_PANE_HEADER_SCHEMA_REF,
            M5_BOUNDARY_FACT_GRID_SCHEMA_REF,
        ]),
        masquerades_as_native_approval_chrome: false,
        hides_owner_origin_or_handoff_in_menus_only: false,
        renders_stale_or_blocked_as_fresh_first_party_truth: false,
        embeds_high_risk_approval_without_native_step_up: false,
    }
}

fn controls_rows() -> Vec<M5DocsBoundaryControlsRow> {
    use M5EmbeddedConsumerSurface as C;
    use M5EmbeddedDowngradeTrigger as D;

    vec![
        base_row(
            C::DocsBrowserUi,
            "Docs browser owner",
            "The docs browser renders one docs-pane header per pane naming the source class, owner/origin, version, and last-updated state, so a user reads whether they are looking at project-local or mirrored vendor material without leaving the pane",
            "evidence:m5-docs-boundary-docs-browser:001",
            vec![
                D::OwnerOrOriginUnstated,
                D::FreshnessOrLastUpdatedUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![header_project_local_clean(), header_mirrored_vendor_clean()],
            vec![grid_local_clean()],
        ),
        base_row(
            C::EmbeddedWebviewUi,
            "Embedded webview owner",
            "Extension-contributed docs render inside an embedded webview whose header names the contributing extension and capability limits, and whose boundary-fact grid never masquerades as an approval or policy-authority surface",
            "evidence:m5-docs-boundary-embedded-webview:001",
            vec![
                D::OwnerOrOriginUnstated,
                D::ImitatesNativeApprovalChrome,
                D::CapabilityLimitsUnstated,
                D::ProofStale,
            ],
            vec![header_extension_clean()],
            vec![grid_mirrored_clean(), grid_masquerade()],
        ),
        base_row(
            C::MarketplaceUi,
            "Marketplace docs owner",
            "Marketplace listing docs distinguish browser-handoff-required content from an undistinguishable source, degrading honestly when the source class cannot be told or the data boundary is unstated",
            "evidence:m5-docs-boundary-marketplace:001",
            vec![
                D::GenericChromeWordingUsed,
                D::DataBoundaryUnstated,
                D::BrowserFallbackHiddenInMenusOnly,
                D::ProofStale,
            ],
            vec![header_handoff_clean(), header_source_unknown()],
            vec![grid_data_boundary_unstated()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved header and grid truth, so a required browser handoff that is not exposed or an unexplained reading-trust claim is visible in evidence rather than hidden",
            "evidence:m5-docs-boundary-support-export:001",
            vec![
                D::BrowserFallbackHiddenInMenusOnly,
                D::GenericChromeWordingUsed,
                D::DataBoundaryUnstated,
                D::ProofStale,
            ],
            vec![header_handoff_not_exposed()],
            vec![grid_reading_trust_unexplained()],
        ),
        base_row(
            C::ProductUi,
            "In-product help owner",
            "In-product help panes reuse the same source-class and data-boundary vocabulary a user sees in the docs browser, degrading honestly when the owner/origin is undisclosed or the reading posture is unstated rather than inventing local prose",
            "evidence:m5-docs-boundary-product-ui:001",
            vec![
                D::OwnerOrOriginUnstated,
                D::StaleOrBlockedShownAsFresh,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![header_project_local_clean(), header_owner_undisclosed()],
            vec![grid_posture_unknown()],
        ),
    ]
}

fn governance_review() -> M5DocsBoundaryGovernanceReview {
    M5DocsBoundaryGovernanceReview {
        docs_pane_header_names_source_class_and_owner: true,
        boundary_fact_grid_names_data_boundary_and_posture: true,
        source_class_always_distinguishable_or_degraded: true,
        owner_and_origin_always_explicit: true,
        external_handoff_exposed_when_required: true,
        no_pane_masquerades_as_approval_authority: true,
        stale_or_offline_never_shown_as_fresh: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5DocsBoundaryConsumerProjection {
    M5DocsBoundaryConsumerProjection {
        docs_surfaces_consume_source_class_vocabulary: true,
        embedded_surfaces_consume_capability_limit_vocabulary: true,
        boundary_grids_consume_single_data_boundary_source: true,
        support_export_reads_single_boundary_source: true,
    }
}

fn proof_freshness() -> M5DocsBoundaryProofFreshness {
    M5DocsBoundaryProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DocsBoundaryReleasePosture {
    M5DocsBoundaryReleasePosture {
        proof_packet_ref: M5_DOCS_BOUNDARY_CONTROLS_ARTIFACT_REF.to_owned(),
        boundary_audit_ref: M5_DOCS_BOUNDARY_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DOCS_BOUNDARY_CONTROLS_SCHEMA_REF,
        M5_DOCS_BOUNDARY_CONTROLS_DOC_REF,
        M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF,
        M5_EMBEDDED_BOUNDARY_COMPONENT_DOC_REF,
        M5_DOCS_PANE_HEADER_SCHEMA_REF,
        M5_BOUNDARY_FACT_GRID_SCHEMA_REF,
        M5_AUTH_BOUNDARY_CONTRACT_DOC_REF,
    ])
}

/// Builds the canonical M5 docs-pane-header / boundary-fact-grid controls packet.
pub fn seeded_m5_docs_boundary_controls() -> M5DocsBoundaryControlsPacket {
    M5DocsBoundaryControlsPacket::new(M5DocsBoundaryControlsPacketInput {
        packet_id: M5_DOCS_BOUNDARY_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 docs-pane-header and boundary-fact-grid controls with source class, version/pack identity, owner/origin, open-externally, find-in-page, and data-boundary truth"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5DocsBoundaryVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the docs-browser row is held at Beta pending source-class parity on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_docs_boundary_controls_docs_browser_beta_narrowed() -> M5DocsBoundaryControlsPacket
{
    let mut packet = seeded_m5_docs_boundary_controls();
    packet.packet_id =
        "m5-docs-pane-header-boundary-fact-grid-controls:docs-browser-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EmbeddedConsumerSurface::DocsBrowserUi)
        .expect("docs-browser row present");
    row.qualification = M5EmbeddedQualificationClass::Beta;
    packet
}

/// Narrowed variant: the embedded-webview row is narrowed to Preview pending boundary-fact-grid
/// parity on every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_docs_boundary_controls_embedded_webview_preview_narrowed(
) -> M5DocsBoundaryControlsPacket {
    let mut packet = seeded_m5_docs_boundary_controls();
    packet.packet_id =
        "m5-docs-pane-header-boundary-fact-grid-controls:embedded-webview-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EmbeddedConsumerSurface::EmbeddedWebviewUi)
        .expect("embedded-webview row present");
    row.qualification = M5EmbeddedQualificationClass::Preview;
    packet
}
