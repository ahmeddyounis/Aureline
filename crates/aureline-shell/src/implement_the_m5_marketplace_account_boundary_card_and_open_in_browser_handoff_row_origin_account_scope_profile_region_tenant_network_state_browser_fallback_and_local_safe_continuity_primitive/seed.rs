//! Canonical seed builders for the M5 marketplace/account boundary-card and open-in-browser
//! handoff-row controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_MARKETPLACE_HANDOFF_CONTROLS_PACKET_ID: &str =
    "m5-marketplace-account-boundary-open-in-browser-handoff-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn card(
    input: M5MarketplaceAccountBoundaryCardResolutionInput,
) -> M5ResolvedMarketplaceAccountBoundaryCard {
    resolve_marketplace_account_boundary_card(input)
        .expect("seed marketplace/account boundary card input resolves")
}

fn handoff_row(
    input: M5OpenInBrowserHandoffRowResolutionInput,
) -> M5ResolvedOpenInBrowserHandoffRow {
    resolve_open_in_browser_handoff_row(input).expect("seed open-in-browser handoff row input resolves")
}

// -- Canonical marketplace/account boundary card examples -------------------------------------

/// Clean card for a provider-owned marketplace listing scoped to a personal account.
fn card_marketplace_clean() -> M5ResolvedMarketplaceAccountBoundaryCard {
    card(M5MarketplaceAccountBoundaryCardResolutionInput {
        card_id: "boundary-card:marketplace".to_owned(),
        owner_class: WebviewOwnerClass::ProviderOwned,
        owner_origin_disclosed: true,
        account_scope: M5EmbeddedAccountScope::PersonalAccount,
        account_scope_disclosed: true,
        current_profile: "personal profile".to_owned(),
        region_or_tenant: String::new(),
        network_state: M5MarketplaceNetworkState::Online,
        browser_fallback_kind: Some(BrowserHandoffKind::ProviderContentView),
        browser_fallback_available: true,
        conceals_identity_behind_generic_chrome: false,
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        proof_fresh: true,
    })
}

/// Clean card for a provider-owned account surface scoped to an org workspace with a region cue.
fn card_account_org_clean() -> M5ResolvedMarketplaceAccountBoundaryCard {
    card(M5MarketplaceAccountBoundaryCardResolutionInput {
        card_id: "boundary-card:account-org".to_owned(),
        owner_class: WebviewOwnerClass::ProviderOwned,
        owner_origin_disclosed: true,
        account_scope: M5EmbeddedAccountScope::OrgWorkspace,
        account_scope_disclosed: true,
        current_profile: "org member profile".to_owned(),
        region_or_tenant: "eu-west workspace".to_owned(),
        network_state: M5MarketplaceNetworkState::Online,
        browser_fallback_kind: Some(BrowserHandoffKind::ProviderContentView),
        browser_fallback_available: true,
        conceals_identity_behind_generic_chrome: false,
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        proof_fresh: true,
    })
}

/// Clean card for a provider-owned managed tenant with a tenant cue and degraded connectivity.
fn card_managed_tenant_clean() -> M5ResolvedMarketplaceAccountBoundaryCard {
    card(M5MarketplaceAccountBoundaryCardResolutionInput {
        card_id: "boundary-card:managed-tenant".to_owned(),
        owner_class: WebviewOwnerClass::ProviderOwned,
        owner_origin_disclosed: true,
        account_scope: M5EmbeddedAccountScope::ManagedTenant,
        account_scope_disclosed: true,
        current_profile: "managed member profile".to_owned(),
        region_or_tenant: "acme managed tenant".to_owned(),
        network_state: M5MarketplaceNetworkState::DegradedConnectivity,
        browser_fallback_kind: Some(BrowserHandoffKind::ProviderContentView),
        browser_fallback_available: true,
        conceals_identity_behind_generic_chrome: false,
        freshness: M5EmbeddedFreshnessState::WarmSnapshot,
        proof_fresh: true,
    })
}

/// Clean card for a first-party local account surface with no account, honestly offline.
fn card_offline_local() -> M5ResolvedMarketplaceAccountBoundaryCard {
    card(M5MarketplaceAccountBoundaryCardResolutionInput {
        card_id: "boundary-card:offline-local".to_owned(),
        owner_class: WebviewOwnerClass::FirstPartyEmbedded,
        owner_origin_disclosed: true,
        account_scope: M5EmbeddedAccountScope::NoAccountLocal,
        account_scope_disclosed: true,
        current_profile: String::new(),
        region_or_tenant: String::new(),
        network_state: M5MarketplaceNetworkState::Offline,
        browser_fallback_kind: Some(BrowserHandoffKind::VendorOutboundLink),
        browser_fallback_available: true,
        conceals_identity_behind_generic_chrome: false,
        freshness: M5EmbeddedFreshnessState::OfflineSnapshot,
        proof_fresh: true,
    })
}

/// Degraded card: the owner / origin (service ownership) is undisclosed — proves AC1's ownership
/// half.
fn card_owner_undisclosed() -> M5ResolvedMarketplaceAccountBoundaryCard {
    card(M5MarketplaceAccountBoundaryCardResolutionInput {
        card_id: "boundary-card:owner-hidden".to_owned(),
        owner_class: WebviewOwnerClass::UnknownUntrusted,
        owner_origin_disclosed: false,
        account_scope: M5EmbeddedAccountScope::PersonalAccount,
        account_scope_disclosed: true,
        current_profile: "personal profile".to_owned(),
        region_or_tenant: String::new(),
        network_state: M5MarketplaceNetworkState::Online,
        browser_fallback_kind: Some(BrowserHandoffKind::VendorOutboundLink),
        browser_fallback_available: true,
        conceals_identity_behind_generic_chrome: false,
        freshness: M5EmbeddedFreshnessState::FreshnessUnknown,
        proof_fresh: true,
    })
}

/// Degraded card: generic product chrome conceals identity, region, or ownership — proves AC1's
/// generic-chrome half.
fn card_generic_chrome() -> M5ResolvedMarketplaceAccountBoundaryCard {
    card(M5MarketplaceAccountBoundaryCardResolutionInput {
        card_id: "boundary-card:generic-chrome".to_owned(),
        owner_class: WebviewOwnerClass::ProviderOwned,
        owner_origin_disclosed: true,
        account_scope: M5EmbeddedAccountScope::PersonalAccount,
        account_scope_disclosed: true,
        current_profile: "personal profile".to_owned(),
        region_or_tenant: String::new(),
        network_state: M5MarketplaceNetworkState::Online,
        browser_fallback_kind: Some(BrowserHandoffKind::ProviderContentView),
        browser_fallback_available: true,
        conceals_identity_behind_generic_chrome: true,
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        proof_fresh: true,
    })
}

/// Degraded card: the account scope is unstated.
fn card_account_scope_unstated() -> M5ResolvedMarketplaceAccountBoundaryCard {
    card(M5MarketplaceAccountBoundaryCardResolutionInput {
        card_id: "boundary-card:scope-hidden".to_owned(),
        owner_class: WebviewOwnerClass::ProviderOwned,
        owner_origin_disclosed: true,
        account_scope: M5EmbeddedAccountScope::AccountScopeUnknown,
        account_scope_disclosed: false,
        current_profile: "personal profile".to_owned(),
        region_or_tenant: String::new(),
        network_state: M5MarketplaceNetworkState::Online,
        browser_fallback_kind: Some(BrowserHandoffKind::ProviderContentView),
        browser_fallback_available: true,
        conceals_identity_behind_generic_chrome: false,
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        proof_fresh: true,
    })
}

// -- Canonical open-in-browser handoff row examples ------------------------------------------

/// Clean row for a provider-content handoff that preserves object identity and continuity.
fn row_provider_content_clean() -> M5ResolvedOpenInBrowserHandoffRow {
    handoff_row(M5OpenInBrowserHandoffRowResolutionInput {
        row_id: "handoff-row:provider-content".to_owned(),
        handoff_kind: BrowserHandoffKind::ProviderContentView,
        handoff_reason: HandoffReasonClass::ViewProviderContent,
        object_ref: "object:listing-1042".to_owned(),
        object_label: "Marketplace listing #1042".to_owned(),
        object_identity_preserved: true,
        handoff_reason_stated: true,
        fallback_state: FallbackStateClass::LocalContinuityPreserved,
        local_continuity_explicit: true,
        browser_fallback_available: true,
        lands_on_generic_page: false,
        proof_fresh: true,
    })
}

/// Clean row for a vendor-outbound handoff that preserves object identity and continuity.
fn row_vendor_link_clean() -> M5ResolvedOpenInBrowserHandoffRow {
    handoff_row(M5OpenInBrowserHandoffRowResolutionInput {
        row_id: "handoff-row:vendor-link".to_owned(),
        handoff_kind: BrowserHandoffKind::VendorOutboundLink,
        handoff_reason: HandoffReasonClass::OpenVendorResource,
        object_ref: "object:account-billing".to_owned(),
        object_label: "Account billing portal".to_owned(),
        object_identity_preserved: true,
        handoff_reason_stated: true,
        fallback_state: FallbackStateClass::RetryHandoffInApp,
        local_continuity_explicit: true,
        browser_fallback_available: true,
        lands_on_generic_page: false,
        proof_fresh: true,
    })
}

/// Degraded row: the current object identity is dropped — proves AC2's object-identity half.
fn row_object_identity_dropped() -> M5ResolvedOpenInBrowserHandoffRow {
    handoff_row(M5OpenInBrowserHandoffRowResolutionInput {
        row_id: "handoff-row:identity-dropped".to_owned(),
        handoff_kind: BrowserHandoffKind::VendorOutboundLink,
        handoff_reason: HandoffReasonClass::OpenVendorResource,
        object_ref: String::new(),
        object_label: "Account billing portal".to_owned(),
        object_identity_preserved: false,
        handoff_reason_stated: true,
        fallback_state: FallbackStateClass::RetryHandoffInApp,
        local_continuity_explicit: true,
        browser_fallback_available: true,
        lands_on_generic_page: false,
        proof_fresh: true,
    })
}

/// Degraded row: the handoff lands on a generic page — proves AC2's generic-landing half.
fn row_lands_generic() -> M5ResolvedOpenInBrowserHandoffRow {
    handoff_row(M5OpenInBrowserHandoffRowResolutionInput {
        row_id: "handoff-row:generic-landing".to_owned(),
        handoff_kind: BrowserHandoffKind::ProviderContentView,
        handoff_reason: HandoffReasonClass::ViewProviderContent,
        object_ref: "object:listing-1042".to_owned(),
        object_label: "Marketplace listing #1042".to_owned(),
        object_identity_preserved: true,
        handoff_reason_stated: true,
        fallback_state: FallbackStateClass::LocalContinuityPreserved,
        local_continuity_explicit: true,
        browser_fallback_available: true,
        lands_on_generic_page: true,
        proof_fresh: true,
    })
}

/// Degraded row: the reason the in-product lane ended is unstated.
fn row_reason_unstated() -> M5ResolvedOpenInBrowserHandoffRow {
    handoff_row(M5OpenInBrowserHandoffRowResolutionInput {
        row_id: "handoff-row:reason-unstated".to_owned(),
        handoff_kind: BrowserHandoffKind::ProviderContentView,
        handoff_reason: HandoffReasonClass::ViewProviderContent,
        object_ref: "object:listing-2087".to_owned(),
        object_label: "Marketplace listing #2087".to_owned(),
        object_identity_preserved: true,
        handoff_reason_stated: false,
        fallback_state: FallbackStateClass::LocalContinuityPreserved,
        local_continuity_explicit: true,
        browser_fallback_available: true,
        lands_on_generic_page: false,
        proof_fresh: true,
    })
}

/// Degraded row: the local-safe continuity after handoff is left implicit.
fn row_continuity_unstated() -> M5ResolvedOpenInBrowserHandoffRow {
    handoff_row(M5OpenInBrowserHandoffRowResolutionInput {
        row_id: "handoff-row:continuity-unstated".to_owned(),
        handoff_kind: BrowserHandoffKind::VendorOutboundLink,
        handoff_reason: HandoffReasonClass::OpenVendorResource,
        object_ref: "object:support-case".to_owned(),
        object_label: "Support case handoff".to_owned(),
        object_identity_preserved: true,
        handoff_reason_stated: true,
        fallback_state: FallbackStateClass::CopyLinkForManualOpen,
        local_continuity_explicit: false,
        browser_fallback_available: true,
        lands_on_generic_page: false,
        proof_fresh: true,
    })
}

// -- Row builders ----------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5MarketplaceHandoffConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5EmbeddedDowngradeTrigger>,
    card_examples: Vec<M5ResolvedMarketplaceAccountBoundaryCard>,
    row_examples: Vec<M5ResolvedOpenInBrowserHandoffRow>,
) -> M5MarketplaceHandoffControlsRow {
    M5MarketplaceHandoffControlsRow {
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
            M5EmbeddedRequiredLabel::DataBoundaryAndFallback,
        ],
        accessibility_routes: M5EmbeddedAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5MarketplaceHandoffAnatomyPart::ALL.to_vec(),
        export_fields: M5MarketplaceHandoffExportField::ALL.to_vec(),
        downgrade_triggers,
        marketplace_account_boundary_card_examples: card_examples,
        open_in_browser_handoff_row_examples: row_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_MARKETPLACE_HANDOFF_CONTROLS_SCHEMA_REF,
            M5_MARKETPLACE_ACCOUNT_BOUNDARY_CARD_SCHEMA_REF,
            M5_OPEN_IN_BROWSER_HANDOFF_ROW_SCHEMA_REF,
        ]),
        masquerades_as_native_approval_chrome: false,
        hides_owner_origin_or_handoff_in_menus_only: false,
        renders_stale_or_blocked_as_fresh_first_party_truth: false,
        embeds_high_risk_approval_without_native_step_up: false,
    }
}

fn controls_rows() -> Vec<M5MarketplaceHandoffControlsRow> {
    use M5EmbeddedConsumerSurface as C;
    use M5EmbeddedDowngradeTrigger as D;

    vec![
        base_row(
            C::MarketplaceUi,
            "Marketplace surface owner",
            "Every marketplace listing card names its provider ownership, account scope, current profile, network state, and browser fallback, and degrades honestly when generic product chrome conceals identity, region, or ownership; its open-in-browser handoff rows preserve the current listing identity and never land on a generic page",
            "evidence:m5-marketplace-handoff-marketplace:001",
            vec![
                D::OwnerOrOriginUnstated,
                D::GenericChromeWordingUsed,
                D::AccountScopeUnstated,
                D::ProofStale,
            ],
            vec![card_marketplace_clean(), card_generic_chrome()],
            vec![row_provider_content_clean(), row_lands_generic()],
        ),
        base_row(
            C::AccountUi,
            "Account surface owner",
            "Account panes name owner/origin, account scope, current profile, and region/tenant cues where relevant, and degrade when the service ownership is undisclosed; their outbound handoff rows preserve the current object identity rather than dropping the user onto an anonymous portal",
            "evidence:m5-marketplace-handoff-account:001",
            vec![
                D::OwnerOrOriginUnstated,
                D::AccountScopeUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![card_account_org_clean(), card_owner_undisclosed()],
            vec![row_vendor_link_clean(), row_object_identity_dropped()],
        ),
        base_row(
            C::RemoteDashboardUi,
            "Remote dashboard owner",
            "Remote / service dashboard account cards disclose the managed tenant and region cue and degrade when the account scope is unstated; their handoff rows explain why the in-product lane ended rather than silently opening a generic page",
            "evidence:m5-marketplace-handoff-remote-dashboard:001",
            vec![
                D::AccountScopeUnstated,
                D::GenericChromeWordingUsed,
                D::BrowserFallbackHiddenInMenusOnly,
                D::ProofStale,
            ],
            vec![card_managed_tenant_clean(), card_account_scope_unstated()],
            vec![row_provider_content_clean(), row_reason_unstated()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved boundary-card and handoff-row truth, so a concealed account scope or an implicit local-safe continuity is visible in evidence rather than hidden behind generic chrome",
            "evidence:m5-marketplace-handoff-support-export:001",
            vec![
                D::AccountScopeUnstated,
                D::BrowserFallbackHiddenInMenusOnly,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![card_offline_local()],
            vec![row_vendor_link_clean(), row_continuity_unstated()],
        ),
        base_row(
            C::ProductUi,
            "In-product surface owner",
            "In-product account and marketplace surfaces reuse the same owner/origin, account-scope, and browser-fallback vocabulary the marketplace shows, keeping local-safe continuity explicit after every browser handoff rather than inventing local prose",
            "evidence:m5-marketplace-handoff-product-ui:001",
            vec![
                D::GenericChromeWordingUsed,
                D::AccountScopeUnstated,
                D::BrowserFallbackHiddenInMenusOnly,
                D::ProofStale,
            ],
            vec![card_marketplace_clean()],
            vec![row_provider_content_clean(), row_vendor_link_clean()],
        ),
    ]
}

fn governance_review() -> M5MarketplaceHandoffGovernanceReview {
    M5MarketplaceHandoffGovernanceReview {
        card_names_owner_and_ownership: true,
        card_discloses_account_scope: true,
        profile_and_region_always_explicit: true,
        network_state_and_fallback_always_exposed: true,
        generic_chrome_never_conceals_identity: true,
        handoff_row_preserves_object_identity: true,
        handoff_row_states_reason: true,
        handoff_never_lands_on_generic_page: true,
        local_continuity_always_explicit: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5MarketplaceHandoffConsumerProjection {
    M5MarketplaceHandoffConsumerProjection {
        surfaces_consume_owner_origin_vocabulary: true,
        surfaces_consume_account_scope_vocabulary: true,
        handoff_rows_consume_shared_fallback_vocabulary: true,
        support_export_reads_single_boundary_source: true,
    }
}

fn proof_freshness() -> M5MarketplaceHandoffProofFreshness {
    M5MarketplaceHandoffProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5MarketplaceHandoffReleasePosture {
    M5MarketplaceHandoffReleasePosture {
        proof_packet_ref: M5_MARKETPLACE_HANDOFF_CONTROLS_ARTIFACT_REF.to_owned(),
        boundary_audit_ref: M5_MARKETPLACE_HANDOFF_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_MARKETPLACE_HANDOFF_CONTROLS_SCHEMA_REF,
        M5_MARKETPLACE_HANDOFF_CONTROLS_DOC_REF,
        M5_EMBEDDED_BOUNDARY_COMPONENT_SCHEMA_REF,
        M5_EMBEDDED_BOUNDARY_COMPONENT_DOC_REF,
        M5_MARKETPLACE_ACCOUNT_BOUNDARY_CARD_SCHEMA_REF,
        M5_OPEN_IN_BROWSER_HANDOFF_ROW_SCHEMA_REF,
        M5_AUTH_BOUNDARY_CONTRACT_DOC_REF,
    ])
}

/// Builds the canonical M5 marketplace/account boundary-card and open-in-browser handoff-row
/// controls packet.
pub fn seeded_m5_marketplace_handoff_controls() -> M5MarketplaceHandoffControlsPacket {
    M5MarketplaceHandoffControlsPacket::new(M5MarketplaceHandoffControlsPacketInput {
        packet_id: M5_MARKETPLACE_HANDOFF_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 marketplace/account boundary-card and open-in-browser handoff-row controls with origin, account scope, current profile, region/tenant, network state, browser fallback, object identity, reason-for-handoff, and local-safe continuity truth"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5MarketplaceHandoffVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the marketplace row is held at Beta pending owner/origin parity on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_marketplace_handoff_controls_marketplace_beta_narrowed(
) -> M5MarketplaceHandoffControlsPacket {
    let mut packet = seeded_m5_marketplace_handoff_controls();
    packet.packet_id =
        "m5-marketplace-account-boundary-open-in-browser-handoff-controls:marketplace-beta:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EmbeddedConsumerSurface::MarketplaceUi)
        .expect("marketplace row present");
    row.qualification = M5EmbeddedQualificationClass::Beta;
    packet
}

/// Narrowed variant: the account row is narrowed to Preview pending account-scope parity on every
/// surface; every row stays visible and every example stays honest.
pub fn seeded_m5_marketplace_handoff_controls_account_preview_narrowed(
) -> M5MarketplaceHandoffControlsPacket {
    let mut packet = seeded_m5_marketplace_handoff_controls();
    packet.packet_id =
        "m5-marketplace-account-boundary-open-in-browser-handoff-controls:account-preview:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EmbeddedConsumerSurface::AccountUi)
        .expect("account row present");
    row.qualification = M5EmbeddedQualificationClass::Preview;
    packet
}
