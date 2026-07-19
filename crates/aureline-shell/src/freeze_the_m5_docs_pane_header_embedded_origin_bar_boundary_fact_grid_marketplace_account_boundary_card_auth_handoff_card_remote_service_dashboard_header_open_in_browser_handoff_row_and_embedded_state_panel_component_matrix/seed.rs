// Sequential pushes keep each contract scenario adjacent to its rationale.
#![allow(clippy::vec_init_then_push)]

//! Canonical seed builders for the frozen M5 embedded-boundary component matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical embedded-boundary component matrix.
pub const M5_EMBEDDED_BOUNDARY_COMPONENT_MATRIX_PACKET_ID: &str =
    "m5-embedded-boundary-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5EmbeddedRequiredLabel> {
    M5EmbeddedRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5EmbeddedRequiredLabel]) -> Vec<M5EmbeddedRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every family-specific
/// vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5EmbeddedBoundaryComponentFamily,
    qualification: M5EmbeddedQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5EmbeddedBoundaryComponentRow {
    M5EmbeddedBoundaryComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5EmbeddedSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5EmbeddedDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        boundary_dispositions: M5EmbeddedBoundaryDisposition::ALL.to_vec(),
        owner_classes: vec![],
        data_exit_boundaries: vec![],
        browser_handoff_kinds: vec![],
        capability_limits: vec![],
        freshness_states: vec![],
        account_scopes: vec![],
        degraded_reasons: M5EmbeddedDegradedReason::ALL.to_vec(),
        accessibility_routes: M5EmbeddedAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5EmbeddedConsumerSurface::SupportExport,
            M5EmbeddedConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5EmbeddedDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        imitates_native_permission_or_approval_ui: false,
        hides_owner_origin_or_browser_fallback_in_menus_only: false,
        renders_stale_or_blocked_as_fresh_first_party_truth: false,
        embeds_high_risk_approval_without_native_step_up: false,
    }
}

fn component_rows() -> Vec<M5EmbeddedBoundaryComponentRow> {
    use BrowserHandoffKind as BH;
    use CapabilityLimitClass as CL;
    use DataExitBoundary as DE;
    use M5EmbeddedAccountScope as AS;
    use M5EmbeddedBoundaryComponentFamily as F;
    use M5EmbeddedBoundaryDisposition as BD;
    use M5EmbeddedConsumerSurface as C;
    use M5EmbeddedDowngradeTrigger as D;
    use M5EmbeddedFreshnessState as FR;
    use M5EmbeddedQualificationClass as Q;
    use M5EmbeddedRequiredLabel as L;
    use WebviewOwnerClass as OW;

    let mut rows = Vec::new();

    // 1. Docs-pane header.
    let mut row = base_row(
        F::DocsPaneHeader,
        Q::Stable,
        "Docs / help pane owner",
        "One docs-pane-header model naming whose documentation is shown (first-party, first-party hosted, or connected-provider docs) plus the source, version, last-updated, and freshness, so a user never mistakes a stale or provider-owned docs pane for fresh first-party local help",
        "evidence:m5-docs-pane-header-parity:001",
        &[
            M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF,
            M5_DOCS_PANE_HEADER_SCHEMA_REF,
            M5_WEBVIEW_ORIGIN_BAR_SCHEMA_REF,
        ],
    );
    row.owner_classes = vec![OW::FirstPartyEmbedded, OW::ProviderOwned];
    row.capability_limits = vec![CL::NotNativeTrustChrome, CL::CannotVerifyUpdates];
    row.freshness_states = vec![
        FR::LiveFresh,
        FR::WarmSnapshot,
        FR::StaleSnapshot,
        FR::OfflineSnapshot,
    ];
    row.boundary_dispositions = vec![
        BD::LiveFirstPartyLocal,
        BD::LiveFirstPartyHosted,
        BD::LiveProviderOwned,
        BD::StaleSnapshot,
        BD::OfflineSnapshot,
        BD::NotEvaluated,
    ];
    row.required_labels = labels_with(&[L::OwnerAndOrigin, L::FreshnessAndCapabilityLimits]);
    row.consumer_surfaces = vec![
        C::DocsBrowserUi,
        C::EmbeddedWebviewUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::OwnerOrOriginUnstated,
        D::FreshnessOrLastUpdatedUnstated,
        D::StaleOrBlockedShownAsFresh,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Embedded-origin bar.
    let mut row = base_row(
        F::EmbeddedOriginBar,
        Q::Stable,
        "Embedded webview owner",
        "One embedded-origin-bar model naming who owns the embedded content (extension, provider, first-party embedded, or unknown/untrusted) and the capability limits the embedded surface has relative to native trusted chrome, so an embedded webview can never impersonate native trust UI",
        "evidence:m5-embedded-origin-bar-parity:001",
        &[
            M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF,
            M5_EMBEDDED_ORIGIN_BAR_SCHEMA_REF,
            M5_WEBVIEW_ORIGIN_BAR_SCHEMA_REF,
        ],
    );
    row.owner_classes = OW::ALL.to_vec();
    row.capability_limits = CL::ALL.to_vec();
    row.boundary_dispositions = vec![
        BD::LiveProviderOwned,
        BD::CapabilityLimited,
        BD::ProviderBlocked,
        BD::NotEvaluated,
    ];
    row.required_labels = labels_with(&[L::OwnerAndOrigin, L::FreshnessAndCapabilityLimits]);
    row.consumer_surfaces = vec![
        C::EmbeddedWebviewUi,
        C::MarketplaceUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::OwnerOrOriginUnstated,
        D::CapabilityLimitsUnstated,
        D::ImitatesNativeApprovalChrome,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Boundary-fact grid.
    let mut row = base_row(
        F::BoundaryFactGrid,
        Q::Stable,
        "Embedded boundary owner",
        "One boundary-fact-grid model naming owner/origin, the data boundary that governs what leaves the product, and the freshness in one place, so a user can read every boundary fact about a surface without hunting through menus",
        "evidence:m5-boundary-fact-grid-parity:001",
        &[
            M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF,
            M5_BOUNDARY_FACT_GRID_SCHEMA_REF,
            M5_WEBVIEW_ORIGIN_BAR_SCHEMA_REF,
        ],
    );
    row.owner_classes = vec![
        OW::FirstPartyEmbedded,
        OW::ProviderOwned,
        OW::ExtensionOwned,
    ];
    row.data_exit_boundaries = BOUND_DATA_EXIT_BOUNDARIES.to_vec();
    row.freshness_states = FR::ALL.to_vec();
    row.boundary_dispositions = M5EmbeddedBoundaryDisposition::ALL.to_vec();
    row.required_labels = labels_with(&[
        L::OwnerAndOrigin,
        L::DataBoundaryAndFallback,
        L::FreshnessAndCapabilityLimits,
    ]);
    row.consumer_surfaces = vec![
        C::EmbeddedWebviewUi,
        C::RemoteDashboardUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::OwnerOrOriginUnstated,
        D::DataBoundaryUnstated,
        D::FreshnessOrLastUpdatedUnstated,
        D::StaleOrBlockedShownAsFresh,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Marketplace / account boundary card.
    let mut row = base_row(
        F::MarketplaceAccountBoundaryCard,
        Q::Stable,
        "Marketplace / account owner",
        "One marketplace-account-boundary-card model naming the account scope (no account local, personal, org workspace, managed tenant, or unknown) and the data boundary, so marketplace and account content never hides whose account it is scoped to",
        "evidence:m5-marketplace-account-boundary-card-parity:001",
        &[
            M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF,
            M5_MARKETPLACE_ACCOUNT_BOUNDARY_CARD_SCHEMA_REF,
            M5_WEBVIEW_ORIGIN_BAR_SCHEMA_REF,
        ],
    );
    row.owner_classes = vec![OW::ProviderOwned, OW::FirstPartyEmbedded];
    row.data_exit_boundaries = vec![
        DE::MetadataSafeObjectRefs,
        DE::VendorOrThirdPartyOutbound,
        DE::ExternalPublicBrowse,
    ];
    row.account_scopes = AS::ALL.to_vec();
    row.boundary_dispositions = vec![
        BD::LiveFirstPartyHosted,
        BD::LiveProviderOwned,
        BD::StaleSnapshot,
        BD::ProviderBlocked,
        BD::NotEvaluated,
    ];
    row.required_labels = labels_with(&[L::OwnerAndOrigin, L::DataBoundaryAndFallback]);
    row.consumer_surfaces = vec![
        C::MarketplaceUi,
        C::AccountUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::AccountScopeUnstated,
        D::DataBoundaryUnstated,
        D::OwnerOrOriginUnstated,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Auth-handoff card.
    let mut row = base_row(
        F::AuthHandoffCard,
        Q::Stable,
        "Auth boundary owner",
        "One auth-handoff-card model naming the browser fallback for a sign-in (system browser, device code, provider content, or vendor link), the data boundary, and the account scope, so authentication that leaves native chrome is always explicit and never imitates a native approval sheet",
        "evidence:m5-auth-handoff-card-parity:001",
        &[
            M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF,
            M5_AUTH_HANDOFF_CARD_SCHEMA_REF,
            M5_BROWSER_HANDOFF_CARD_SCHEMA_REF,
        ],
    );
    row.data_exit_boundaries = vec![DE::NoPayloadLeavesProduct, DE::VendorOrThirdPartyOutbound];
    row.browser_handoff_kinds = BH::ALL.to_vec();
    row.account_scopes = vec![AS::PersonalAccount, AS::OrgWorkspace, AS::ManagedTenant];
    row.boundary_dispositions = vec![
        BD::BrowserHandoffOnly,
        BD::LiveProviderOwned,
        BD::NotEvaluated,
    ];
    row.required_labels = labels_with(&[L::DataBoundaryAndFallback]);
    row.consumer_surfaces = vec![
        C::AuthHandoffUi,
        C::AccountUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::BrowserFallbackHiddenInMenusOnly,
        D::DataBoundaryUnstated,
        D::AccountScopeUnstated,
        D::ImitatesNativeApprovalChrome,
        D::HighRiskApprovalEmbedded,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Remote / service dashboard header.
    let mut row = base_row(
        F::RemoteServiceDashboardHeader,
        Q::Stable,
        "Remote / service dashboard owner",
        "One remote-service-dashboard-header model naming who owns the dashboard, the data boundary, the provider health, and the freshness, so a remote or service dashboard never renders a stale, offline, or provider-blocked view as fresh first-party local truth",
        "evidence:m5-remote-service-dashboard-header-parity:001",
        &[
            M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF,
            M5_REMOTE_SERVICE_DASHBOARD_HEADER_SCHEMA_REF,
            M5_WEBVIEW_ORIGIN_BAR_SCHEMA_REF,
        ],
    );
    row.owner_classes = vec![OW::ProviderOwned, OW::FirstPartyEmbedded];
    row.data_exit_boundaries = vec![DE::MetadataSafeObjectRefs, DE::VendorOrThirdPartyOutbound];
    row.capability_limits = vec![CL::NotNativeTrustChrome, CL::CannotDisplayProductSecurity];
    row.freshness_states = vec![
        FR::LiveFresh,
        FR::WarmSnapshot,
        FR::StaleSnapshot,
        FR::OfflineSnapshot,
    ];
    row.boundary_dispositions = vec![
        BD::LiveProviderOwned,
        BD::StaleSnapshot,
        BD::OfflineSnapshot,
        BD::ProviderBlocked,
        BD::NotEvaluated,
    ];
    row.required_labels = labels_with(&[L::OwnerAndOrigin, L::FreshnessAndCapabilityLimits]);
    row.consumer_surfaces = vec![
        C::RemoteDashboardUi,
        C::EmbeddedWebviewUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::OwnerOrOriginUnstated,
        D::ProviderHealthUnstated,
        D::FreshnessOrLastUpdatedUnstated,
        D::StaleOrBlockedShownAsFresh,
        D::ProofStale,
    ];
    rows.push(row);

    // 7. Open-in-browser handoff row.
    let mut row = base_row(
        F::OpenInBrowserHandoffRow,
        Q::Stable,
        "Browser handoff owner",
        "One open-in-browser-handoff-row model naming the browser fallback for a surface (provider content or vendor link) and the data boundary, so the escape hatch into the real browser is always a first-class row and never hidden behind menus only",
        "evidence:m5-open-in-browser-handoff-row-parity:001",
        &[
            M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF,
            M5_OPEN_IN_BROWSER_HANDOFF_ROW_SCHEMA_REF,
            M5_BROWSER_HANDOFF_CARD_SCHEMA_REF,
        ],
    );
    row.data_exit_boundaries = vec![DE::ExternalPublicBrowse, DE::VendorOrThirdPartyOutbound];
    row.browser_handoff_kinds = vec![BH::ProviderContentView, BH::VendorOutboundLink];
    row.boundary_dispositions = vec![BD::BrowserHandoffOnly, BD::NotEvaluated];
    row.required_labels = labels_with(&[L::DataBoundaryAndFallback]);
    row.consumer_surfaces = vec![
        C::EmbeddedWebviewUi,
        C::DocsBrowserUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::BrowserFallbackHiddenInMenusOnly,
        D::DataBoundaryUnstated,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 8. Embedded-state panel.
    let mut row = base_row(
        F::EmbeddedStatePanel,
        Q::Stable,
        "Embedded boundary owner",
        "One embedded-state-panel model naming whether an embedded surface is live-first-party, stale, offline, provider-blocked, or capability-limited, plus who owns it and its capability limits, so a stale, offline, or blocked pane is always shown explicitly and never as fresh first-party local truth",
        "evidence:m5-embedded-state-panel-parity:001",
        &[
            M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF,
            M5_EMBEDDED_STATE_PANEL_SCHEMA_REF,
            M5_WEBVIEW_ORIGIN_BAR_SCHEMA_REF,
        ],
    );
    row.owner_classes = vec![
        OW::FirstPartyEmbedded,
        OW::ProviderOwned,
        OW::UnknownUntrusted,
    ];
    row.capability_limits = vec![
        CL::NotNativeTrustChrome,
        CL::CannotGrantDevicePermission,
        CL::CannotEnterProductCredentials,
    ];
    row.freshness_states = vec![
        FR::LiveFresh,
        FR::StaleSnapshot,
        FR::OfflineSnapshot,
        FR::FreshnessUnknown,
    ];
    row.boundary_dispositions = vec![
        BD::LiveFirstPartyLocal,
        BD::StaleSnapshot,
        BD::OfflineSnapshot,
        BD::ProviderBlocked,
        BD::CapabilityLimited,
        BD::NotEvaluated,
    ];
    row.required_labels = labels_with(&[L::OwnerAndOrigin, L::FreshnessAndCapabilityLimits]);
    row.consumer_surfaces = vec![
        C::EmbeddedWebviewUi,
        C::RemoteDashboardUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::StaleOrBlockedShownAsFresh,
        D::OwnerOrOriginUnstated,
        D::CapabilityLimitsUnstated,
        D::ImitatesNativeApprovalChrome,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5EmbeddedBoundaryComponentGovernanceReview {
    M5EmbeddedBoundaryComponentGovernanceReview {
        docs_pane_header_shows_owner_origin_and_freshness: true,
        embedded_origin_bar_shows_owner_and_capability_limits: true,
        boundary_fact_grid_shows_owner_origin_data_boundary_freshness: true,
        marketplace_account_boundary_card_shows_account_scope: true,
        auth_handoff_card_shows_browser_fallback_and_data_boundary: true,
        remote_service_dashboard_header_shows_provider_health_and_freshness: true,
        open_in_browser_handoff_row_shows_browser_fallback: true,
        embedded_state_panel_shows_stale_offline_blocked_explicitly: true,
        no_embedded_surface_imitates_native_approval_chrome: true,
        owner_and_origin_always_explicit: true,
        data_boundary_always_explicit: true,
        browser_fallback_never_menu_only: true,
        stale_offline_blocked_never_shown_as_fresh: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_boundary_vocabulary: true,
    }
}

fn consumer_projection() -> M5EmbeddedBoundaryComponentConsumerProjection {
    M5EmbeddedBoundaryComponentConsumerProjection {
        docs_and_help_surfaces_consume_owner_origin_vocabulary: true,
        marketplace_and_account_surfaces_consume_account_scope_vocabulary: true,
        remote_dashboard_surfaces_consume_freshness_vocabulary: true,
        webview_surfaces_consume_capability_limit_vocabulary: true,
        auth_handoff_surfaces_consume_browser_handoff_vocabulary: true,
        support_export_reads_single_boundary_source: true,
    }
}

fn proof_freshness() -> M5EmbeddedBoundaryComponentProofFreshness {
    M5EmbeddedBoundaryComponentProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5EmbeddedBoundaryComponentReleasePosture {
    M5EmbeddedBoundaryComponentReleasePosture {
        proof_packet_ref: M5_EMBEDDED_BOUNDARY_COMPONENT_ARTIFACT_REF.to_owned(),
        boundary_audit_ref: M5_EMBEDDED_BOUNDARY_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF,
        M5_EMBEDDED_BOUNDARY_COMPONENT_DOC_REF,
        M5_DOCS_PANE_HEADER_SCHEMA_REF,
        M5_EMBEDDED_ORIGIN_BAR_SCHEMA_REF,
        M5_BOUNDARY_FACT_GRID_SCHEMA_REF,
        M5_MARKETPLACE_ACCOUNT_BOUNDARY_CARD_SCHEMA_REF,
        M5_AUTH_HANDOFF_CARD_SCHEMA_REF,
        M5_REMOTE_SERVICE_DASHBOARD_HEADER_SCHEMA_REF,
        M5_OPEN_IN_BROWSER_HANDOFF_ROW_SCHEMA_REF,
        M5_EMBEDDED_STATE_PANEL_SCHEMA_REF,
        M5_AUTH_BOUNDARY_CONTRACT_DOC_REF,
        M5_BROWSER_HANDOFF_CARD_SCHEMA_REF,
        M5_WEBVIEW_ORIGIN_BAR_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 embedded-boundary component matrix packet.
pub fn seeded_m5_embedded_boundary_component_matrix() -> M5EmbeddedBoundaryComponentMatrixPacket {
    M5EmbeddedBoundaryComponentMatrixPacket::new(M5EmbeddedBoundaryComponentMatrixPacketInput {
        packet_id: M5_EMBEDDED_BOUNDARY_COMPONENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 docs-pane-header, embedded-origin-bar, boundary-fact-grid, marketplace-account-boundary-card, auth-handoff-card, remote-service-dashboard-header, open-in-browser-handoff-row, and embedded-state-panel component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5EmbeddedBoundaryVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the docs-pane header is held at Beta because provider-owned docs freshness
/// round-trips are not yet proven across every deployment line; every component stays visible.
pub fn seeded_m5_embedded_boundary_component_matrix_docs_pane_header_beta_narrowed(
) -> M5EmbeddedBoundaryComponentMatrixPacket {
    let mut packet = seeded_m5_embedded_boundary_component_matrix();
    packet.packet_id = "m5-embedded-boundary-components:docs-pane-header-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5EmbeddedBoundaryComponentFamily::DocsPaneHeader)
        .expect("docs-pane-header row present");
    row.qualification = M5EmbeddedQualificationClass::Beta;
    packet
}

/// Narrowed variant: the embedded-state panel is narrowed to Preview pending stale/offline/blocked
/// continuity parity on every surface; every component stays visible.
pub fn seeded_m5_embedded_boundary_component_matrix_embedded_state_panel_preview_narrowed(
) -> M5EmbeddedBoundaryComponentMatrixPacket {
    let mut packet = seeded_m5_embedded_boundary_component_matrix();
    packet.packet_id =
        "m5-embedded-boundary-components:embedded-state-panel-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5EmbeddedBoundaryComponentFamily::EmbeddedStatePanel)
        .expect("embedded-state-panel row present");
    row.qualification = M5EmbeddedQualificationClass::Preview;
    packet
}
