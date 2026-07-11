//! Canonical seed builders for the M5 embedded-origin-bar / embedded-state-panel controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_EMBEDDED_ORIGIN_STATE_CONTROLS_PACKET_ID: &str =
    "m5-embedded-origin-bar-state-panel-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn bar(input: M5EmbeddedOriginBarResolutionInput) -> M5ResolvedEmbeddedOriginBar {
    resolve_embedded_origin_bar(input).expect("seed embedded origin bar input resolves")
}

fn panel(input: M5EmbeddedStatePanelResolutionInput) -> M5ResolvedEmbeddedStatePanel {
    resolve_embedded_state_panel(input).expect("seed embedded-state panel input resolves")
}

// -- Canonical embedded origin bar examples ---------------------------------------------------

/// Clean bar for an extension-owned webview — names the extension, publisher, capability limits,
/// and offers reload plus an open-in-browser path.
fn bar_extension_clean() -> M5ResolvedEmbeddedOriginBar {
    bar(M5EmbeddedOriginBarResolutionInput {
        bar_id: "origin-bar:extension".to_owned(),
        owner_class: WebviewOwnerClass::ExtensionOwned,
        origin_disclosure: OriginDisclosureClass::NamedExtensionOrigin,
        extension_name: "Acme Language Pack".to_owned(),
        publisher: "Acme Tools".to_owned(),
        owner_origin_disclosed: true,
        permission_state: WebviewPermissionState::ScopedPermissionsGranted,
        capability_limits: vec![
            CapabilityLimitClass::NotNativeTrustChrome,
            CapabilityLimitClass::CannotDisplayProductSecurity,
        ],
        capability_limits_disclosed: true,
        reload_available: true,
        open_in_browser_kind: Some(BrowserHandoffKind::ProviderContentView),
        open_in_browser_available: true,
        imitates_native_permission_ui: false,
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        proof_fresh: true,
    })
}

/// Clean bar for a provider-owned webview.
fn bar_provider_clean() -> M5ResolvedEmbeddedOriginBar {
    bar(M5EmbeddedOriginBarResolutionInput {
        bar_id: "origin-bar:provider".to_owned(),
        owner_class: WebviewOwnerClass::ProviderOwned,
        origin_disclosure: OriginDisclosureClass::NamedProviderOrigin,
        extension_name: String::new(),
        publisher: "Provider Cloud".to_owned(),
        owner_origin_disclosed: true,
        permission_state: WebviewPermissionState::NoElevatedPermissions,
        capability_limits: vec![
            CapabilityLimitClass::NotNativeTrustChrome,
            CapabilityLimitClass::CannotVerifyUpdates,
        ],
        capability_limits_disclosed: true,
        reload_available: true,
        open_in_browser_kind: Some(BrowserHandoffKind::ProviderContentView),
        open_in_browser_available: true,
        imitates_native_permission_ui: false,
        freshness: M5EmbeddedFreshnessState::WarmSnapshot,
        proof_fresh: true,
    })
}

/// Clean bar for a first-party embedded webview.
fn bar_first_party_clean() -> M5ResolvedEmbeddedOriginBar {
    bar(M5EmbeddedOriginBarResolutionInput {
        bar_id: "origin-bar:first-party".to_owned(),
        owner_class: WebviewOwnerClass::FirstPartyEmbedded,
        origin_disclosure: OriginDisclosureClass::FirstPartyOrigin,
        extension_name: String::new(),
        publisher: "Aureline".to_owned(),
        owner_origin_disclosed: true,
        permission_state: WebviewPermissionState::ScopedPermissionsGranted,
        capability_limits: vec![CapabilityLimitClass::NotNativeTrustChrome],
        capability_limits_disclosed: true,
        reload_available: true,
        open_in_browser_kind: Some(BrowserHandoffKind::ProviderContentView),
        open_in_browser_available: true,
        imitates_native_permission_ui: false,
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        proof_fresh: true,
    })
}

/// Degraded bar: the owner / origin is undisclosed — proves AC1's owner-chrome half.
fn bar_owner_undisclosed() -> M5ResolvedEmbeddedOriginBar {
    bar(M5EmbeddedOriginBarResolutionInput {
        bar_id: "origin-bar:owner-hidden".to_owned(),
        owner_class: WebviewOwnerClass::UnknownUntrusted,
        origin_disclosure: OriginDisclosureClass::UndisclosedOriginBlocked,
        extension_name: String::new(),
        publisher: String::new(),
        owner_origin_disclosed: false,
        permission_state: WebviewPermissionState::PermissionDenied,
        capability_limits: vec![CapabilityLimitClass::NotNativeTrustChrome],
        capability_limits_disclosed: true,
        reload_available: true,
        open_in_browser_kind: Some(BrowserHandoffKind::VendorOutboundLink),
        open_in_browser_available: true,
        imitates_native_permission_ui: false,
        freshness: M5EmbeddedFreshnessState::FreshnessUnknown,
        proof_fresh: true,
    })
}

/// Degraded bar: the capability limits are undisclosed — proves AC1's capability-limit half.
fn bar_capability_undisclosed() -> M5ResolvedEmbeddedOriginBar {
    bar(M5EmbeddedOriginBarResolutionInput {
        bar_id: "origin-bar:capability-hidden".to_owned(),
        owner_class: WebviewOwnerClass::ExtensionOwned,
        origin_disclosure: OriginDisclosureClass::NamedExtensionOrigin,
        extension_name: "Acme Language Pack".to_owned(),
        publisher: "Acme Tools".to_owned(),
        owner_origin_disclosed: true,
        permission_state: WebviewPermissionState::ScopedPermissionsGranted,
        capability_limits: vec![CapabilityLimitClass::NotNativeTrustChrome],
        capability_limits_disclosed: false,
        reload_available: true,
        open_in_browser_kind: Some(BrowserHandoffKind::ProviderContentView),
        open_in_browser_available: true,
        imitates_native_permission_ui: false,
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        proof_fresh: true,
    })
}

/// Degraded bar: an extension-owned surface hides its publisher.
fn bar_publisher_missing() -> M5ResolvedEmbeddedOriginBar {
    bar(M5EmbeddedOriginBarResolutionInput {
        bar_id: "origin-bar:no-publisher".to_owned(),
        owner_class: WebviewOwnerClass::ExtensionOwned,
        origin_disclosure: OriginDisclosureClass::NamedExtensionOrigin,
        extension_name: "Acme Language Pack".to_owned(),
        publisher: String::new(),
        owner_origin_disclosed: true,
        permission_state: WebviewPermissionState::ScopedPermissionsGranted,
        capability_limits: vec![CapabilityLimitClass::NotNativeTrustChrome],
        capability_limits_disclosed: true,
        reload_available: true,
        open_in_browser_kind: Some(BrowserHandoffKind::ProviderContentView),
        open_in_browser_available: true,
        imitates_native_permission_ui: false,
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        proof_fresh: true,
    })
}

/// Degraded bar: it imitates native permission / trust UI — proves AC2's origin-bar half.
fn bar_imitates_native() -> M5ResolvedEmbeddedOriginBar {
    bar(M5EmbeddedOriginBarResolutionInput {
        bar_id: "origin-bar:imitates-native".to_owned(),
        owner_class: WebviewOwnerClass::ExtensionOwned,
        origin_disclosure: OriginDisclosureClass::NamedExtensionOrigin,
        extension_name: "Acme Language Pack".to_owned(),
        publisher: "Acme Tools".to_owned(),
        owner_origin_disclosed: true,
        permission_state: WebviewPermissionState::PermissionRequestPending,
        capability_limits: vec![CapabilityLimitClass::NotNativeTrustChrome],
        capability_limits_disclosed: true,
        reload_available: true,
        open_in_browser_kind: Some(BrowserHandoffKind::ProviderContentView),
        open_in_browser_available: true,
        imitates_native_permission_ui: true,
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        proof_fresh: true,
    })
}

// -- Canonical embedded-state panel examples --------------------------------------------------

fn state_panel(
    panel_id: &str,
    state_class: M5EmbeddedStateClass,
    owner_class: WebviewOwnerClass,
    state_explained: bool,
    severity_and_support_boundary_shared: bool,
    shown_as_fresh_first_party: bool,
    imitates_native_permission_ui: bool,
) -> M5ResolvedEmbeddedStatePanel {
    panel(M5EmbeddedStatePanelResolutionInput {
        panel_id: panel_id.to_owned(),
        state_class,
        owner_class,
        state_explained,
        severity_and_support_boundary_shared,
        recovery_action_available: true,
        shown_as_fresh_first_party,
        imitates_native_permission_ui,
        proof_fresh: true,
    })
}

/// Clean panel explaining a stale state.
fn panel_stale() -> M5ResolvedEmbeddedStatePanel {
    state_panel(
        "state-panel:stale",
        M5EmbeddedStateClass::StaleSnapshot,
        WebviewOwnerClass::ProviderOwned,
        true,
        true,
        false,
        false,
    )
}

/// Clean panel explaining an offline state.
fn panel_offline() -> M5ResolvedEmbeddedStatePanel {
    state_panel(
        "state-panel:offline",
        M5EmbeddedStateClass::OfflineSnapshot,
        WebviewOwnerClass::ProviderOwned,
        true,
        true,
        false,
        false,
    )
}

/// Clean panel explaining a policy-blocked state.
fn panel_policy_blocked() -> M5ResolvedEmbeddedStatePanel {
    state_panel(
        "state-panel:policy-blocked",
        M5EmbeddedStateClass::PolicyBlocked,
        WebviewOwnerClass::ProviderOwned,
        true,
        true,
        false,
        false,
    )
}

/// Clean panel explaining a certificate-denied state.
fn panel_certificate_denied() -> M5ResolvedEmbeddedStatePanel {
    state_panel(
        "state-panel:certificate-denied",
        M5EmbeddedStateClass::CertificateDenied,
        WebviewOwnerClass::ProviderOwned,
        true,
        true,
        false,
        false,
    )
}

/// Clean panel explaining a cross-origin-limited state.
fn panel_cross_origin_limited() -> M5ResolvedEmbeddedStatePanel {
    state_panel(
        "state-panel:cross-origin",
        M5EmbeddedStateClass::CrossOriginLimited,
        WebviewOwnerClass::ExtensionOwned,
        true,
        true,
        false,
        false,
    )
}

/// Clean panel for a live-healthy embedded surface.
fn panel_live_clean() -> M5ResolvedEmbeddedStatePanel {
    state_panel(
        "state-panel:live",
        M5EmbeddedStateClass::LiveHealthy,
        WebviewOwnerClass::FirstPartyEmbedded,
        true,
        true,
        false,
        false,
    )
}

/// Degraded panel: a stale state is rendered as fresh first-party truth.
fn panel_blocked_as_fresh() -> M5ResolvedEmbeddedStatePanel {
    state_panel(
        "state-panel:blocked-as-fresh",
        M5EmbeddedStateClass::StaleSnapshot,
        WebviewOwnerClass::ProviderOwned,
        true,
        true,
        true,
        false,
    )
}

/// Degraded panel: it imitates native permission / trust UI — proves AC2's panel half.
fn panel_imitates_native() -> M5ResolvedEmbeddedStatePanel {
    state_panel(
        "state-panel:imitates-native",
        M5EmbeddedStateClass::LiveHealthy,
        WebviewOwnerClass::ExtensionOwned,
        true,
        true,
        false,
        true,
    )
}

/// Degraded panel: the state is not explained.
fn panel_not_explained() -> M5ResolvedEmbeddedStatePanel {
    state_panel(
        "state-panel:not-explained",
        M5EmbeddedStateClass::OfflineSnapshot,
        WebviewOwnerClass::ProviderOwned,
        false,
        true,
        false,
        false,
    )
}

// -- Row builders ----------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5EmbeddedOriginStateConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5EmbeddedDowngradeTrigger>,
    embedded_origin_bar_examples: Vec<M5ResolvedEmbeddedOriginBar>,
    embedded_state_panel_examples: Vec<M5ResolvedEmbeddedStatePanel>,
) -> M5EmbeddedOriginStateControlsRow {
    M5EmbeddedOriginStateControlsRow {
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
        anatomy_parts: M5EmbeddedOriginStateAnatomyPart::ALL.to_vec(),
        export_fields: M5EmbeddedOriginStateExportField::ALL.to_vec(),
        downgrade_triggers,
        embedded_origin_bar_examples,
        embedded_state_panel_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_EMBEDDED_ORIGIN_STATE_CONTROLS_SCHEMA_REF,
            M5_EMBEDDED_ORIGIN_BAR_SCHEMA_REF,
            M5_EMBEDDED_STATE_PANEL_SCHEMA_REF,
        ]),
        masquerades_as_native_approval_chrome: false,
        hides_owner_origin_or_handoff_in_menus_only: false,
        renders_stale_or_blocked_as_fresh_first_party_truth: false,
        embeds_high_risk_approval_without_native_step_up: false,
    }
}

fn controls_rows() -> Vec<M5EmbeddedOriginStateControlsRow> {
    use M5EmbeddedConsumerSurface as C;
    use M5EmbeddedDowngradeTrigger as D;

    vec![
        base_row(
            C::EmbeddedWebviewUi,
            "Embedded webview owner",
            "Every extension-owned webview renders an origin bar naming the extension, publisher, origin class, permission state, and capability limits, and never imitates native permission or trust chrome; its embedded-state panel explains stale and offline states with the shared first-party vocabulary",
            "evidence:m5-embedded-origin-state-embedded-webview:001",
            vec![
                D::OwnerOrOriginUnstated,
                D::ImitatesNativeApprovalChrome,
                D::CapabilityLimitsUnstated,
                D::ProofStale,
            ],
            vec![bar_extension_clean(), bar_imitates_native()],
            vec![panel_stale(), panel_offline()],
        ),
        base_row(
            C::MarketplaceUi,
            "Marketplace webview owner",
            "Marketplace listing webviews name their provider or publisher on the origin bar and degrade honestly when an extension-owned surface hides its publisher; policy-blocked content is explained rather than shown as fresh first-party truth",
            "evidence:m5-embedded-origin-state-marketplace:001",
            vec![
                D::OwnerOrOriginUnstated,
                D::StaleOrBlockedShownAsFresh,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![bar_provider_clean(), bar_publisher_missing()],
            vec![panel_policy_blocked()],
        ),
        base_row(
            C::RemoteDashboardUi,
            "Remote dashboard owner",
            "Remote / service dashboard webviews disclose the owner/origin chrome or degrade when it is undisclosed, explain cross-origin-limited states with the shared severity vocabulary, and never imitate native permission UI",
            "evidence:m5-embedded-origin-state-remote-dashboard:001",
            vec![
                D::OwnerOrOriginUnstated,
                D::ImitatesNativeApprovalChrome,
                D::ProviderHealthUnstated,
                D::ProofStale,
            ],
            vec![bar_owner_undisclosed()],
            vec![panel_cross_origin_limited(), panel_imitates_native()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved origin-bar and state-panel truth, so an undisclosed capability limit or an unexplained embedded state is visible in evidence rather than hidden",
            "evidence:m5-embedded-origin-state-support-export:001",
            vec![
                D::CapabilityLimitsUnstated,
                D::GenericChromeWordingUsed,
                D::FreshnessOrLastUpdatedUnstated,
                D::ProofStale,
            ],
            vec![bar_capability_undisclosed()],
            vec![panel_certificate_denied(), panel_not_explained()],
        ),
        base_row(
            C::ProductUi,
            "In-product surface owner",
            "In-product embedded surfaces reuse the same owner/origin and capability-limit vocabulary the embedded webview shows, degrading honestly when a stale state is rendered as fresh first-party truth rather than inventing local prose",
            "evidence:m5-embedded-origin-state-product-ui:001",
            vec![
                D::StaleOrBlockedShownAsFresh,
                D::GenericChromeWordingUsed,
                D::CapabilityLimitsUnstated,
                D::ProofStale,
            ],
            vec![bar_first_party_clean()],
            vec![panel_blocked_as_fresh(), panel_live_clean()],
        ),
    ]
}

fn governance_review() -> M5EmbeddedOriginStateGovernanceReview {
    M5EmbeddedOriginStateGovernanceReview {
        origin_bar_names_owner_and_publisher: true,
        origin_bar_discloses_capability_limits: true,
        owner_and_origin_always_explicit: true,
        open_in_browser_always_exposed: true,
        no_surface_imitates_native_ui: true,
        state_panel_explains_state_with_shared_vocabulary: true,
        stale_or_blocked_never_shown_as_fresh: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5EmbeddedOriginStateConsumerProjection {
    M5EmbeddedOriginStateConsumerProjection {
        embedded_surfaces_consume_owner_origin_vocabulary: true,
        embedded_surfaces_consume_capability_limit_vocabulary: true,
        state_panels_consume_shared_severity_vocabulary: true,
        support_export_reads_single_boundary_source: true,
    }
}

fn proof_freshness() -> M5EmbeddedOriginStateProofFreshness {
    M5EmbeddedOriginStateProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5EmbeddedOriginStateReleasePosture {
    M5EmbeddedOriginStateReleasePosture {
        proof_packet_ref: M5_EMBEDDED_ORIGIN_STATE_CONTROLS_ARTIFACT_REF.to_owned(),
        boundary_audit_ref: M5_EMBEDDED_ORIGIN_STATE_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_EMBEDDED_ORIGIN_STATE_CONTROLS_SCHEMA_REF,
        M5_EMBEDDED_ORIGIN_STATE_CONTROLS_DOC_REF,
        M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF,
        M5_EMBEDDED_BOUNDARY_COMPONENT_DOC_REF,
        M5_EMBEDDED_ORIGIN_BAR_SCHEMA_REF,
        M5_EMBEDDED_STATE_PANEL_SCHEMA_REF,
        M5_AUTH_BOUNDARY_CONTRACT_DOC_REF,
    ])
}

/// Builds the canonical M5 embedded-origin-bar / embedded-state-panel controls packet.
pub fn seeded_m5_embedded_origin_state_controls() -> M5EmbeddedOriginStateControlsPacket {
    M5EmbeddedOriginStateControlsPacket::new(M5EmbeddedOriginStateControlsPacketInput {
        packet_id: M5_EMBEDDED_ORIGIN_STATE_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 embedded-origin-bar and embedded-state-panel controls with extension/publisher, origin, permission, capability-limit, reload, open-in-browser, and offline/stale/policy-blocked/certificate-denied/cross-origin-limited state truth"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5EmbeddedOriginStateVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the embedded-webview row is held at Beta pending owner/origin parity on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_embedded_origin_state_controls_embedded_webview_beta_narrowed(
) -> M5EmbeddedOriginStateControlsPacket {
    let mut packet = seeded_m5_embedded_origin_state_controls();
    packet.packet_id =
        "m5-embedded-origin-bar-state-panel-controls:embedded-webview-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EmbeddedConsumerSurface::EmbeddedWebviewUi)
        .expect("embedded-webview row present");
    row.qualification = M5EmbeddedQualificationClass::Beta;
    packet
}

/// Narrowed variant: the remote-dashboard row is narrowed to Preview pending state-panel parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_embedded_origin_state_controls_remote_dashboard_preview_narrowed(
) -> M5EmbeddedOriginStateControlsPacket {
    let mut packet = seeded_m5_embedded_origin_state_controls();
    packet.packet_id =
        "m5-embedded-origin-bar-state-panel-controls:remote-dashboard-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EmbeddedConsumerSurface::RemoteDashboardUi)
        .expect("remote-dashboard row present");
    row.qualification = M5EmbeddedQualificationClass::Preview;
    packet
}
