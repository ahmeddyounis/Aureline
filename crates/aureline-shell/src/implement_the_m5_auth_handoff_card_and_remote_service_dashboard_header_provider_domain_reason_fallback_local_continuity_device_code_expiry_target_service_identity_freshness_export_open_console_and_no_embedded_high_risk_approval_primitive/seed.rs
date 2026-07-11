//! Canonical seed builders for the M5 auth handoff-card and remote/service dashboard-header
//! controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_AUTH_DASHBOARD_CONTROLS_PACKET_ID: &str =
    "m5-auth-handoff-card-remote-service-dashboard-header-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn card(input: M5AuthHandoffCardResolutionInput) -> M5ResolvedAuthHandoffCard {
    resolve_auth_handoff_card(input).expect("seed auth handoff card input resolves")
}

fn header(
    input: M5RemoteServiceDashboardHeaderResolutionInput,
) -> M5ResolvedRemoteServiceDashboardHeader {
    resolve_remote_service_dashboard_header(input)
        .expect("seed remote/service dashboard header input resolves")
}

// -- Canonical auth handoff card examples ------------------------------------------------------

/// Clean embedded sign-in checkpoint that names its provider and keeps local continuity explicit.
fn card_embedded_checkpoint_clean() -> M5ResolvedAuthHandoffCard {
    card(M5AuthHandoffCardResolutionInput {
        card_id: "auth-card:embedded-checkpoint".to_owned(),
        posture: M5AuthHandoffPosture::EmbeddedSignInCheckpoint,
        handoff_kind: BrowserHandoffKind::SystemBrowserAuth,
        handoff_reason: HandoffReasonClass::AuthenticateWithProvider,
        provider_label: "Aureline identity provider".to_owned(),
        provider_domain_label: "id.aureline.example".to_owned(),
        reason_stated: true,
        fallback_state: FallbackStateClass::RetryHandoffInApp,
        fallback_stated: true,
        local_continuity_stated: true,
        expiry_disclosure: ExpiryDisclosureClass::NoExpiryApplicable,
        device_code_stated: false,
        imitates_native_approval_ui: false,
        embeds_high_risk_approval_without_step_up: false,
        proof_fresh: true,
    })
}

/// Clean system-browser sign-in handoff.
fn card_system_browser_clean() -> M5ResolvedAuthHandoffCard {
    card(M5AuthHandoffCardResolutionInput {
        card_id: "auth-card:system-browser".to_owned(),
        posture: M5AuthHandoffPosture::SystemBrowserHandoff,
        handoff_kind: BrowserHandoffKind::SystemBrowserAuth,
        handoff_reason: HandoffReasonClass::AuthenticateWithProvider,
        provider_label: "Provider single sign-on".to_owned(),
        provider_domain_label: "login.provider.example".to_owned(),
        reason_stated: true,
        fallback_state: FallbackStateClass::LocalContinuityPreserved,
        fallback_stated: true,
        local_continuity_stated: true,
        expiry_disclosure: ExpiryDisclosureClass::NoExpiryApplicable,
        device_code_stated: false,
        imitates_native_approval_ui: false,
        embeds_high_risk_approval_without_step_up: false,
        proof_fresh: true,
    })
}

/// Clean passkey / platform-credential handoff.
fn card_passkey_clean() -> M5ResolvedAuthHandoffCard {
    card(M5AuthHandoffCardResolutionInput {
        card_id: "auth-card:passkey".to_owned(),
        posture: M5AuthHandoffPosture::PasskeyHandoff,
        handoff_kind: BrowserHandoffKind::SystemBrowserAuth,
        handoff_reason: HandoffReasonClass::AuthenticateWithProvider,
        provider_label: "Platform passkey".to_owned(),
        provider_domain_label: "passkey.local".to_owned(),
        reason_stated: true,
        fallback_state: FallbackStateClass::RetryHandoffInApp,
        fallback_stated: true,
        local_continuity_stated: true,
        expiry_disclosure: ExpiryDisclosureClass::NoExpiryApplicable,
        device_code_stated: false,
        imitates_native_approval_ui: false,
        embeds_high_risk_approval_without_step_up: false,
        proof_fresh: true,
    })
}

/// Clean device-code authorization handoff that discloses its code and expiry.
fn card_device_code_clean() -> M5ResolvedAuthHandoffCard {
    card(M5AuthHandoffCardResolutionInput {
        card_id: "auth-card:device-code".to_owned(),
        posture: M5AuthHandoffPosture::DeviceCodeHandoff,
        handoff_kind: BrowserHandoffKind::DeviceCodeAuth,
        handoff_reason: HandoffReasonClass::AuthorizeDeviceCode,
        provider_label: "Device-code provider".to_owned(),
        provider_domain_label: "device.provider.example".to_owned(),
        reason_stated: true,
        fallback_state: FallbackStateClass::ManualCodeEntry,
        fallback_stated: true,
        local_continuity_stated: true,
        expiry_disclosure: ExpiryDisclosureClass::ExpiresWithCountdown,
        device_code_stated: true,
        imitates_native_approval_ui: false,
        embeds_high_risk_approval_without_step_up: false,
        proof_fresh: true,
    })
}

/// Clean provider-content authorization handoff.
fn card_provider_content_clean() -> M5ResolvedAuthHandoffCard {
    card(M5AuthHandoffCardResolutionInput {
        card_id: "auth-card:provider-content".to_owned(),
        posture: M5AuthHandoffPosture::ProviderContentHandoff,
        handoff_kind: BrowserHandoffKind::ProviderContentView,
        handoff_reason: HandoffReasonClass::ViewProviderContent,
        provider_label: "Provider content portal".to_owned(),
        provider_domain_label: "content.provider.example".to_owned(),
        reason_stated: true,
        fallback_state: FallbackStateClass::CopyLinkForManualOpen,
        fallback_stated: true,
        local_continuity_stated: true,
        expiry_disclosure: ExpiryDisclosureClass::NoExpiryApplicable,
        device_code_stated: false,
        imitates_native_approval_ui: false,
        embeds_high_risk_approval_without_step_up: false,
        proof_fresh: true,
    })
}

/// Degraded card: the local-safe continuity note is missing — proves AC1's continuity half.
fn card_continuity_unstated() -> M5ResolvedAuthHandoffCard {
    card(M5AuthHandoffCardResolutionInput {
        card_id: "auth-card:continuity-missing".to_owned(),
        posture: M5AuthHandoffPosture::SystemBrowserHandoff,
        handoff_kind: BrowserHandoffKind::SystemBrowserAuth,
        handoff_reason: HandoffReasonClass::AuthenticateWithProvider,
        provider_label: "Provider single sign-on".to_owned(),
        provider_domain_label: "login.provider.example".to_owned(),
        reason_stated: true,
        fallback_state: FallbackStateClass::LocalContinuityPreserved,
        fallback_stated: true,
        local_continuity_stated: false,
        expiry_disclosure: ExpiryDisclosureClass::NoExpiryApplicable,
        device_code_stated: false,
        imitates_native_approval_ui: false,
        embeds_high_risk_approval_without_step_up: false,
        proof_fresh: true,
    })
}

/// Degraded card: the embedded surface imitates native permission / approval chrome — proves AC1's
/// no-security-theater half.
fn card_imitates_native() -> M5ResolvedAuthHandoffCard {
    card(M5AuthHandoffCardResolutionInput {
        card_id: "auth-card:imitates-native".to_owned(),
        posture: M5AuthHandoffPosture::EmbeddedSignInCheckpoint,
        handoff_kind: BrowserHandoffKind::SystemBrowserAuth,
        handoff_reason: HandoffReasonClass::AuthenticateWithProvider,
        provider_label: "Aureline identity provider".to_owned(),
        provider_domain_label: "id.aureline.example".to_owned(),
        reason_stated: true,
        fallback_state: FallbackStateClass::RetryHandoffInApp,
        fallback_stated: true,
        local_continuity_stated: true,
        expiry_disclosure: ExpiryDisclosureClass::NoExpiryApplicable,
        device_code_stated: false,
        imitates_native_approval_ui: true,
        embeds_high_risk_approval_without_step_up: false,
        proof_fresh: true,
    })
}

/// Degraded card: the provider or domain is unstated.
fn card_provider_unstated() -> M5ResolvedAuthHandoffCard {
    card(M5AuthHandoffCardResolutionInput {
        card_id: "auth-card:provider-unstated".to_owned(),
        posture: M5AuthHandoffPosture::SystemBrowserHandoff,
        handoff_kind: BrowserHandoffKind::SystemBrowserAuth,
        handoff_reason: HandoffReasonClass::AuthenticateWithProvider,
        provider_label: String::new(),
        provider_domain_label: String::new(),
        reason_stated: true,
        fallback_state: FallbackStateClass::LocalContinuityPreserved,
        fallback_stated: true,
        local_continuity_stated: true,
        expiry_disclosure: ExpiryDisclosureClass::NoExpiryApplicable,
        device_code_stated: false,
        imitates_native_approval_ui: false,
        embeds_high_risk_approval_without_step_up: false,
        proof_fresh: true,
    })
}

/// Degraded card: a device-code posture omits its code / expiry disclosure.
fn card_device_code_missing() -> M5ResolvedAuthHandoffCard {
    card(M5AuthHandoffCardResolutionInput {
        card_id: "auth-card:device-code-missing".to_owned(),
        posture: M5AuthHandoffPosture::DeviceCodeHandoff,
        handoff_kind: BrowserHandoffKind::DeviceCodeAuth,
        handoff_reason: HandoffReasonClass::AuthorizeDeviceCode,
        provider_label: "Device-code provider".to_owned(),
        provider_domain_label: "device.provider.example".to_owned(),
        reason_stated: true,
        fallback_state: FallbackStateClass::ManualCodeEntry,
        fallback_stated: true,
        local_continuity_stated: true,
        expiry_disclosure: ExpiryDisclosureClass::NoExpiryApplicable,
        device_code_stated: false,
        imitates_native_approval_ui: false,
        embeds_high_risk_approval_without_step_up: false,
        proof_fresh: true,
    })
}

/// Degraded card: a high-risk approval is embedded without a native step-up.
fn card_high_risk_embedded() -> M5ResolvedAuthHandoffCard {
    card(M5AuthHandoffCardResolutionInput {
        card_id: "auth-card:high-risk-embedded".to_owned(),
        posture: M5AuthHandoffPosture::EmbeddedSignInCheckpoint,
        handoff_kind: BrowserHandoffKind::SystemBrowserAuth,
        handoff_reason: HandoffReasonClass::AuthenticateWithProvider,
        provider_label: "Aureline identity provider".to_owned(),
        provider_domain_label: "id.aureline.example".to_owned(),
        reason_stated: true,
        fallback_state: FallbackStateClass::RetryHandoffInApp,
        fallback_stated: true,
        local_continuity_stated: true,
        expiry_disclosure: ExpiryDisclosureClass::NoExpiryApplicable,
        device_code_stated: false,
        imitates_native_approval_ui: false,
        embeds_high_risk_approval_without_step_up: true,
        proof_fresh: true,
    })
}

// -- Canonical remote/service dashboard header examples ---------------------------------------

/// Clean first-party dashboard header that names its service identity and keeps local recovery
/// reachable.
fn header_first_party_clean() -> M5ResolvedRemoteServiceDashboardHeader {
    header(M5RemoteServiceDashboardHeaderResolutionInput {
        header_id: "dashboard-header:first-party".to_owned(),
        owner_class: WebviewOwnerClass::FirstPartyEmbedded,
        owner_origin_disclosed: true,
        service_identity_label: "Aureline build service".to_owned(),
        service_identity_stated: true,
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        freshness_stated: true,
        export_action_available: true,
        open_console_action_available: true,
        primary_local_recovery_available: true,
        substitutes_for_local_recovery: false,
        allows_high_risk_approval_in_embedded_chrome: false,
        proof_fresh: true,
    })
}

/// Clean provider-owned dashboard header.
fn header_provider_clean() -> M5ResolvedRemoteServiceDashboardHeader {
    header(M5RemoteServiceDashboardHeaderResolutionInput {
        header_id: "dashboard-header:provider".to_owned(),
        owner_class: WebviewOwnerClass::ProviderOwned,
        owner_origin_disclosed: true,
        service_identity_label: "Provider deployment service".to_owned(),
        service_identity_stated: true,
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        freshness_stated: true,
        export_action_available: true,
        open_console_action_available: false,
        primary_local_recovery_available: true,
        substitutes_for_local_recovery: false,
        allows_high_risk_approval_in_embedded_chrome: false,
        proof_fresh: true,
    })
}

/// Clean dashboard header that is honestly offline.
fn header_offline_clean() -> M5ResolvedRemoteServiceDashboardHeader {
    header(M5RemoteServiceDashboardHeaderResolutionInput {
        header_id: "dashboard-header:offline".to_owned(),
        owner_class: WebviewOwnerClass::FirstPartyEmbedded,
        owner_origin_disclosed: true,
        service_identity_label: "Aureline build service".to_owned(),
        service_identity_stated: true,
        freshness: M5EmbeddedFreshnessState::OfflineSnapshot,
        freshness_stated: true,
        export_action_available: true,
        open_console_action_available: false,
        primary_local_recovery_available: true,
        substitutes_for_local_recovery: false,
        allows_high_risk_approval_in_embedded_chrome: false,
        proof_fresh: true,
    })
}

/// Degraded header: the dashboard substitutes for the primary local recovery controls — proves
/// AC2's local-recovery half.
fn header_substitutes() -> M5ResolvedRemoteServiceDashboardHeader {
    header(M5RemoteServiceDashboardHeaderResolutionInput {
        header_id: "dashboard-header:substitutes".to_owned(),
        owner_class: WebviewOwnerClass::ProviderOwned,
        owner_origin_disclosed: true,
        service_identity_label: "Provider deployment service".to_owned(),
        service_identity_stated: true,
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        freshness_stated: true,
        export_action_available: true,
        open_console_action_available: true,
        primary_local_recovery_available: false,
        substitutes_for_local_recovery: true,
        allows_high_risk_approval_in_embedded_chrome: false,
        proof_fresh: true,
    })
}

/// Degraded header: the freshness / offline state is hidden — proves AC2's freshness half.
fn header_freshness_unstated() -> M5ResolvedRemoteServiceDashboardHeader {
    header(M5RemoteServiceDashboardHeaderResolutionInput {
        header_id: "dashboard-header:freshness-hidden".to_owned(),
        owner_class: WebviewOwnerClass::ProviderOwned,
        owner_origin_disclosed: true,
        service_identity_label: "Provider deployment service".to_owned(),
        service_identity_stated: true,
        freshness: M5EmbeddedFreshnessState::FreshnessUnknown,
        freshness_stated: false,
        export_action_available: true,
        open_console_action_available: true,
        primary_local_recovery_available: true,
        substitutes_for_local_recovery: false,
        allows_high_risk_approval_in_embedded_chrome: false,
        proof_fresh: true,
    })
}

/// Degraded header: the target / service identity is unstated.
fn header_identity_unstated() -> M5ResolvedRemoteServiceDashboardHeader {
    header(M5RemoteServiceDashboardHeaderResolutionInput {
        header_id: "dashboard-header:identity-hidden".to_owned(),
        owner_class: WebviewOwnerClass::ProviderOwned,
        owner_origin_disclosed: true,
        service_identity_label: String::new(),
        service_identity_stated: false,
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        freshness_stated: true,
        export_action_available: true,
        open_console_action_available: true,
        primary_local_recovery_available: true,
        substitutes_for_local_recovery: false,
        allows_high_risk_approval_in_embedded_chrome: false,
        proof_fresh: true,
    })
}

/// Degraded header: no export / open-console action is available.
fn header_no_export() -> M5ResolvedRemoteServiceDashboardHeader {
    header(M5RemoteServiceDashboardHeaderResolutionInput {
        header_id: "dashboard-header:no-export".to_owned(),
        owner_class: WebviewOwnerClass::ProviderOwned,
        owner_origin_disclosed: true,
        service_identity_label: "Provider deployment service".to_owned(),
        service_identity_stated: true,
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        freshness_stated: true,
        export_action_available: false,
        open_console_action_available: false,
        primary_local_recovery_available: true,
        substitutes_for_local_recovery: false,
        allows_high_risk_approval_in_embedded_chrome: false,
        proof_fresh: true,
    })
}

/// Degraded header: a high-risk approval is allowed inside embedded chrome.
fn header_high_risk() -> M5ResolvedRemoteServiceDashboardHeader {
    header(M5RemoteServiceDashboardHeaderResolutionInput {
        header_id: "dashboard-header:high-risk".to_owned(),
        owner_class: WebviewOwnerClass::ProviderOwned,
        owner_origin_disclosed: true,
        service_identity_label: "Provider deployment service".to_owned(),
        service_identity_stated: true,
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        freshness_stated: true,
        export_action_available: true,
        open_console_action_available: true,
        primary_local_recovery_available: true,
        substitutes_for_local_recovery: false,
        allows_high_risk_approval_in_embedded_chrome: true,
        proof_fresh: true,
    })
}

// -- Row builders ----------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5AuthDashboardConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5EmbeddedDowngradeTrigger>,
    card_examples: Vec<M5ResolvedAuthHandoffCard>,
    header_examples: Vec<M5ResolvedRemoteServiceDashboardHeader>,
) -> M5AuthDashboardControlsRow {
    M5AuthDashboardControlsRow {
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
        anatomy_parts: M5AuthDashboardAnatomyPart::ALL.to_vec(),
        export_fields: M5AuthDashboardExportField::ALL.to_vec(),
        downgrade_triggers,
        auth_handoff_card_examples: card_examples,
        remote_service_dashboard_header_examples: header_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_AUTH_DASHBOARD_CONTROLS_SCHEMA_REF,
            M5_AUTH_HANDOFF_CARD_SCHEMA_REF,
            M5_REMOTE_SERVICE_DASHBOARD_HEADER_SCHEMA_REF,
        ]),
        masquerades_as_native_approval_chrome: false,
        hides_owner_origin_or_handoff_in_menus_only: false,
        renders_stale_or_blocked_as_fresh_first_party_truth: false,
        embeds_high_risk_approval_without_native_step_up: false,
    }
}

fn controls_rows() -> Vec<M5AuthDashboardControlsRow> {
    use M5EmbeddedConsumerSurface as C;
    use M5EmbeddedDowngradeTrigger as D;

    vec![
        base_row(
            C::AuthHandoffUi,
            "Auth-handoff surface owner",
            "Every auth handoff card distinguishes an embedded sign-in checkpoint from a system-browser or passkey handoff, names its provider/domain and reason, and keeps the local-safe continuity note explicit; an embedded surface that imitates native approval chrome or embeds a high-risk approval without a native step-up degrades rather than reading as a clean checkpoint",
            "evidence:m5-auth-dashboard-auth-handoff:001",
            vec![
                D::OwnerOrOriginUnstated,
                D::ImitatesNativeApprovalChrome,
                D::HighRiskApprovalEmbedded,
                D::ProofStale,
            ],
            vec![card_embedded_checkpoint_clean(), card_imitates_native()],
            vec![header_first_party_clean(), header_high_risk()],
        ),
        base_row(
            C::RemoteDashboardUi,
            "Remote dashboard owner",
            "Remote / service dashboard headers name their target/service identity and ownership boundary, disclose freshness/offline state, and offer export/open-console actions; a dashboard that substitutes for the primary local recovery controls degrades rather than replacing them, and its auth cards never leave the provider unstated",
            "evidence:m5-auth-dashboard-remote-dashboard:001",
            vec![
                D::OwnerOrOriginUnstated,
                D::GenericChromeWordingUsed,
                D::FreshnessOrLastUpdatedUnstated,
                D::ProofStale,
            ],
            vec![card_system_browser_clean(), card_provider_unstated()],
            vec![header_provider_clean(), header_substitutes()],
        ),
        base_row(
            C::AccountUi,
            "Account surface owner",
            "Account sign-in cards distinguish passkey handoff from embedded checkpoints and keep continuity explicit; their service dashboards disclose freshness/offline state, degrading when the freshness signal is hidden so an offline snapshot is never rendered as fresh first-party truth",
            "evidence:m5-auth-dashboard-account:001",
            vec![
                D::FreshnessOrLastUpdatedUnstated,
                D::BrowserFallbackHiddenInMenusOnly,
                D::StaleOrBlockedShownAsFresh,
                D::ProofStale,
            ],
            vec![card_passkey_clean(), card_continuity_unstated()],
            vec![header_offline_clean(), header_freshness_unstated()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved auth-card and dashboard-header truth, so a device-code posture that omits its code/expiry disclosure or a dashboard header that hides its service identity is visible in evidence rather than obscured behind chrome",
            "evidence:m5-auth-dashboard-support-export:001",
            vec![
                D::FreshnessOrLastUpdatedUnstated,
                D::OwnerOrOriginUnstated,
                D::CapabilityLimitsUnstated,
                D::ProofStale,
            ],
            vec![card_device_code_clean(), card_device_code_missing()],
            vec![header_first_party_clean(), header_identity_unstated()],
        ),
        base_row(
            C::ProductUi,
            "In-product surface owner",
            "In-product auth and service-dashboard surfaces reuse the same handoff-posture, reason, freshness, and ownership vocabulary the auth-handoff surface shows, keeping a high-risk approval out of embedded chrome and an export/open-console path always reachable rather than inventing local prose",
            "evidence:m5-auth-dashboard-product-ui:001",
            vec![
                D::HighRiskApprovalEmbedded,
                D::BrowserFallbackHiddenInMenusOnly,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![card_provider_content_clean(), card_high_risk_embedded()],
            vec![header_provider_clean(), header_no_export()],
        ),
    ]
}

fn governance_review() -> M5AuthDashboardGovernanceReview {
    M5AuthDashboardGovernanceReview {
        card_distinguishes_checkpoint_from_handoff: true,
        card_names_provider_and_reason: true,
        card_keeps_local_continuity_explicit: true,
        card_discloses_device_code_or_expiry: true,
        card_never_imitates_native_approval: true,
        header_names_identity_and_ownership: true,
        header_discloses_freshness: true,
        dashboard_never_substitutes_for_local_recovery: true,
        no_high_risk_approval_in_embedded_chrome: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5AuthDashboardConsumerProjection {
    M5AuthDashboardConsumerProjection {
        surfaces_consume_handoff_posture_vocabulary: true,
        surfaces_consume_owner_freshness_vocabulary: true,
        cards_consume_shared_fallback_expiry_vocabulary: true,
        support_export_reads_single_boundary_source: true,
    }
}

fn proof_freshness() -> M5AuthDashboardProofFreshness {
    M5AuthDashboardProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5AuthDashboardReleasePosture {
    M5AuthDashboardReleasePosture {
        proof_packet_ref: M5_AUTH_DASHBOARD_CONTROLS_ARTIFACT_REF.to_owned(),
        boundary_audit_ref: M5_AUTH_DASHBOARD_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_AUTH_DASHBOARD_CONTROLS_SCHEMA_REF,
        M5_AUTH_DASHBOARD_CONTROLS_DOC_REF,
        M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF,
        M5_EMBEDDED_BOUNDARY_COMPONENT_DOC_REF,
        M5_AUTH_HANDOFF_CARD_SCHEMA_REF,
        M5_REMOTE_SERVICE_DASHBOARD_HEADER_SCHEMA_REF,
        M5_AUTH_BOUNDARY_CONTRACT_DOC_REF,
    ])
}

/// Builds the canonical M5 auth handoff-card and remote/service dashboard-header controls packet.
pub fn seeded_m5_auth_dashboard_controls() -> M5AuthDashboardControlsPacket {
    M5AuthDashboardControlsPacket::new(M5AuthDashboardControlsPacketInput {
        packet_id: M5_AUTH_DASHBOARD_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 auth handoff-card and remote/service dashboard-header controls with provider/domain, reason-for-handoff, fallback state, local-safe continuity, device-code expiry, target/service identity, freshness, export/open-console, and no-embedded-high-risk-approval truth"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5AuthDashboardVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the auth-handoff row is held at Beta pending native-step-up parity on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_auth_dashboard_controls_auth_handoff_beta_narrowed(
) -> M5AuthDashboardControlsPacket {
    let mut packet = seeded_m5_auth_dashboard_controls();
    packet.packet_id =
        "m5-auth-handoff-card-remote-service-dashboard-header-controls:auth-handoff-beta:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EmbeddedConsumerSurface::AuthHandoffUi)
        .expect("auth-handoff row present");
    row.qualification = M5EmbeddedQualificationClass::Beta;
    packet
}

/// Narrowed variant: the remote-dashboard row is narrowed to Preview pending freshness parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_auth_dashboard_controls_remote_dashboard_preview_narrowed(
) -> M5AuthDashboardControlsPacket {
    let mut packet = seeded_m5_auth_dashboard_controls();
    packet.packet_id =
        "m5-auth-handoff-card-remote-service-dashboard-header-controls:remote-dashboard-preview:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EmbeddedConsumerSurface::RemoteDashboardUi)
        .expect("remote-dashboard row present");
    row.qualification = M5EmbeddedQualificationClass::Preview;
    packet
}
