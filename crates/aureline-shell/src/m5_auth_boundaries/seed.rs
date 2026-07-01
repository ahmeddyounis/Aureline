//! Canonical seed for the M5 auth-boundary lane: the browser / device-code
//! handoff card set, the webview origin bar set, and the two narrowed scenario
//! fixtures.
//!
//! The seed builders are the single mint-from-truth path: the checked-in support
//! exports, governance summaries, matrix CSVs, and fixtures are projections of
//! these functions, and the module tests prove the on-disk artifacts deserialize
//! back to exactly these values.

use super::{
    BrowserHandoffCard, BrowserHandoffKind, CapabilityLimit, CapabilityLimitClass,
    DataExitBoundary, DeviceCodeDisclosure, ExpiryDisclosureClass, FallbackStateClass,
    HandoffReasonClass, LocalContinuity, M5BrowserHandoffCardSet, M5WebviewOriginBarSet,
    OpenInBrowserAction, OriginDisclosureClass, ReturnAnchor, WebviewOriginBar, WebviewOwnerClass,
    WebviewPermissionState, BROWSER_HANDOFF_CARD_RECORD_KIND, BROWSER_HANDOFF_CARD_SCHEMA_VERSION,
    M5_AUTH_BOUNDARY_COMMUNITY_HANDOFF_REF, M5_AUTH_BOUNDARY_CONTRACT_DOC_REF,
    M5_AUTH_BOUNDARY_DEVICE_PERMISSION_REF, M5_BROWSER_HANDOFF_CARD_SCHEMA_REF,
    M5_BROWSER_HANDOFF_CARD_SET_RECORD_KIND, M5_BROWSER_HANDOFF_CARD_SET_SCHEMA_VERSION,
    M5_WEBVIEW_ORIGIN_BAR_SCHEMA_REF, M5_WEBVIEW_ORIGIN_BAR_SET_RECORD_KIND,
    M5_WEBVIEW_ORIGIN_BAR_SET_SCHEMA_VERSION, WEBVIEW_ORIGIN_BAR_RECORD_KIND,
    WEBVIEW_ORIGIN_BAR_SCHEMA_VERSION,
};

/// Stable id of the canonical browser-handoff card set.
pub const M5_BROWSER_HANDOFF_CARD_SET_ID: &str = "m5_browser_handoff_card_set:default";

/// Stable id of the canonical webview origin bar set.
pub const M5_WEBVIEW_ORIGIN_BAR_SET_ID: &str = "m5_webview_origin_bar_set:default";

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_BROWSER_HANDOFF_CARD_SCHEMA_REF.to_owned(),
        M5_WEBVIEW_ORIGIN_BAR_SCHEMA_REF.to_owned(),
        M5_AUTH_BOUNDARY_CONTRACT_DOC_REF.to_owned(),
        M5_AUTH_BOUNDARY_COMMUNITY_HANDOFF_REF.to_owned(),
        M5_AUTH_BOUNDARY_DEVICE_PERMISSION_REF.to_owned(),
    ]
}

fn local_continuity(ref_token: &str, note: &str) -> LocalContinuity {
    LocalContinuity {
        continuity_ref: ref_token.to_owned(),
        work_preserved_locally: true,
        continuity_note: note.to_owned(),
    }
}

fn return_anchor(ref_token: &str, label: &str, note: &str) -> ReturnAnchor {
    ReturnAnchor {
        anchor_ref: ref_token.to_owned(),
        anchor_label: label.to_owned(),
        return_path_truth_note: note.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Browser / device-code handoff cards.
// ---------------------------------------------------------------------------

/// System-browser sign-in card.
fn card_system_browser_auth() -> BrowserHandoffCard {
    BrowserHandoffCard {
        browser_handoff_card_schema_version: BROWSER_HANDOFF_CARD_SCHEMA_VERSION,
        record_kind: BROWSER_HANDOFF_CARD_RECORD_KIND.to_owned(),
        card_id: "browser_handoff_card:system_browser_auth".to_owned(),
        handoff_kind: BrowserHandoffKind::SystemBrowserAuth,
        handoff_reason: HandoffReasonClass::AuthenticateWithProvider,
        reason_note: "You are being handed to your system browser to sign in with the provider."
            .to_owned(),
        provider_identity_ref: "provider.identity.connected_auth_provider".to_owned(),
        provider_label: "Connected authentication provider".to_owned(),
        provider_domain_label: "auth.provider.example".to_owned(),
        data_exit_boundary: DataExitBoundary::VendorOrThirdPartyOutbound,
        data_exit_note: "Sign-in happens on the provider in your system browser; only the sign-in request leaves Aureline.".to_owned(),
        device_code_disclosure: None,
        fallback_state: FallbackStateClass::RetryHandoffInApp,
        fallback_note: "If the browser does not open, you can retry the sign-in from within Aureline.".to_owned(),
        local_continuity: local_continuity(
            "continuity.system_browser_auth",
            "Your current work stays open in Aureline while you sign in.",
        ),
        return_anchor: return_anchor(
            "anchor.return.after_sign_in",
            "Return to Aureline",
            "After you sign in, the browser returns you to this Aureline window.",
        ),
        opens_outside_native_chrome: true,
        impersonates_native_chrome: false,
        presents_provider_owned_content_labeled: true,
        headline_label: "Sign in with your provider".to_owned(),
        card_summary: "Opens your system browser to sign in with the provider, then returns you to Aureline.".to_owned(),
        contract_doc_ref: M5_AUTH_BOUNDARY_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

/// Device-code authorization card.
fn card_device_code_auth() -> BrowserHandoffCard {
    BrowserHandoffCard {
        browser_handoff_card_schema_version: BROWSER_HANDOFF_CARD_SCHEMA_VERSION,
        record_kind: BROWSER_HANDOFF_CARD_RECORD_KIND.to_owned(),
        card_id: "browser_handoff_card:device_code_auth".to_owned(),
        handoff_kind: BrowserHandoffKind::DeviceCodeAuth,
        handoff_reason: HandoffReasonClass::AuthorizeDeviceCode,
        reason_note: "Enter the one-time device code shown here at the provider to authorize this device.".to_owned(),
        provider_identity_ref: "provider.identity.device_code_provider".to_owned(),
        provider_label: "Device-code authorization provider".to_owned(),
        provider_domain_label: "device.provider.example".to_owned(),
        data_exit_boundary: DataExitBoundary::NoPayloadLeavesProduct,
        data_exit_note: "Aureline shows the code for you to type at the provider; no payload is transmitted by Aureline.".to_owned(),
        device_code_disclosure: Some(DeviceCodeDisclosure {
            code_presentation_ref: "disclosure.device_code_presentation".to_owned(),
            code_presentation_label: "One-time device code".to_owned(),
            expiry_disclosure: ExpiryDisclosureClass::ExpiresWithCountdown,
            expiry_note: "The code expires shortly and is shown with a live countdown.".to_owned(),
            code_shown_in_app_not_transmitted: true,
        }),
        fallback_state: FallbackStateClass::ManualCodeEntry,
        fallback_note: "If the browser does not open, you can enter the device code manually at the provider.".to_owned(),
        local_continuity: local_continuity(
            "continuity.device_code_auth",
            "Your work stays open while you authorize the device code.",
        ),
        return_anchor: return_anchor(
            "anchor.return.after_device_code",
            "Return to Aureline",
            "Once you authorize the code, Aureline continues in this window.",
        ),
        opens_outside_native_chrome: true,
        impersonates_native_chrome: false,
        presents_provider_owned_content_labeled: true,
        headline_label: "Authorize with a device code".to_owned(),
        card_summary: "Shows a one-time device code and its expiry to enter at the provider, keeping your work local.".to_owned(),
        contract_doc_ref: M5_AUTH_BOUNDARY_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

/// Provider-content-in-browser card.
fn card_provider_content_view() -> BrowserHandoffCard {
    BrowserHandoffCard {
        browser_handoff_card_schema_version: BROWSER_HANDOFF_CARD_SCHEMA_VERSION,
        record_kind: BROWSER_HANDOFF_CARD_RECORD_KIND.to_owned(),
        card_id: "browser_handoff_card:provider_content_view".to_owned(),
        handoff_kind: BrowserHandoffKind::ProviderContentView,
        handoff_reason: HandoffReasonClass::ViewProviderContent,
        reason_note: "This opens provider-owned content in your browser; the content is not Aureline chrome.".to_owned(),
        provider_identity_ref: "provider.identity.content_provider".to_owned(),
        provider_label: "Content provider".to_owned(),
        provider_domain_label: "content.provider.example".to_owned(),
        data_exit_boundary: DataExitBoundary::ExternalPublicBrowse,
        data_exit_note: "Opens the provider's public content in your browser; you browse it as an external page.".to_owned(),
        device_code_disclosure: None,
        fallback_state: FallbackStateClass::CopyLinkForManualOpen,
        fallback_note: "If the browser does not open, you can copy the link to open it manually.".to_owned(),
        local_continuity: local_continuity(
            "continuity.provider_content_view",
            "Aureline keeps your place while you view the provider content.",
        ),
        return_anchor: return_anchor(
            "anchor.return.after_content_view",
            "Return to Aureline",
            "Close the browser tab to come back to this Aureline window.",
        ),
        opens_outside_native_chrome: true,
        impersonates_native_chrome: false,
        presents_provider_owned_content_labeled: true,
        headline_label: "View provider content in your browser".to_owned(),
        card_summary: "Opens provider-owned content as an external browser page, clearly labeled as provider-owned.".to_owned(),
        contract_doc_ref: M5_AUTH_BOUNDARY_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

/// Vendor / third-party outbound link card.
fn card_vendor_outbound_link() -> BrowserHandoffCard {
    BrowserHandoffCard {
        browser_handoff_card_schema_version: BROWSER_HANDOFF_CARD_SCHEMA_VERSION,
        record_kind: BROWSER_HANDOFF_CARD_RECORD_KIND.to_owned(),
        card_id: "browser_handoff_card:vendor_outbound_link".to_owned(),
        handoff_kind: BrowserHandoffKind::VendorOutboundLink,
        handoff_reason: HandoffReasonClass::OpenVendorResource,
        reason_note: "This opens a vendor / third-party resource in your browser, outside Aureline.".to_owned(),
        provider_identity_ref: "provider.identity.vendor_resource".to_owned(),
        provider_label: "Vendor / third-party resource".to_owned(),
        provider_domain_label: "vendor.example".to_owned(),
        data_exit_boundary: DataExitBoundary::VendorOrThirdPartyOutbound,
        data_exit_note: "Opens a vendor / third-party page in your browser; the outbound request leaves Aureline.".to_owned(),
        device_code_disclosure: None,
        fallback_state: FallbackStateClass::CopyLinkForManualOpen,
        fallback_note: "If the browser does not open, you can copy the link to open it manually.".to_owned(),
        local_continuity: local_continuity(
            "continuity.vendor_outbound_link",
            "Your Aureline session stays intact while the vendor page opens.",
        ),
        return_anchor: return_anchor(
            "anchor.return.after_vendor_link",
            "Return to Aureline",
            "Switch back to Aureline to continue where you left off.",
        ),
        opens_outside_native_chrome: true,
        impersonates_native_chrome: false,
        presents_provider_owned_content_labeled: true,
        headline_label: "Open a vendor link in your browser".to_owned(),
        card_summary: "Opens a vendor / third-party resource in your browser, disclosing that it leaves Aureline.".to_owned(),
        contract_doc_ref: M5_AUTH_BOUNDARY_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

/// Build the canonical M5 browser / device-code handoff card set.
pub fn seeded_m5_browser_handoff_card_set() -> M5BrowserHandoffCardSet {
    M5BrowserHandoffCardSet {
        schema_version: M5_BROWSER_HANDOFF_CARD_SET_SCHEMA_VERSION,
        record_kind: M5_BROWSER_HANDOFF_CARD_SET_RECORD_KIND.to_owned(),
        set_id: M5_BROWSER_HANDOFF_CARD_SET_ID.to_owned(),
        set_label: "M5 browser / device-code handoff cards".to_owned(),
        cards: vec![
            card_system_browser_auth(),
            card_device_code_auth(),
            card_provider_content_view(),
            card_vendor_outbound_link(),
        ],
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_object_refs".to_owned(),
        minted_at: "mint.m5_browser_handoff_card_set".to_owned(),
        contract_doc_ref: M5_AUTH_BOUNDARY_CONTRACT_DOC_REF.to_owned(),
    }
}

/// A standalone device-code card fixture, proving the device-code path discloses
/// the code and its expiry and keeps work local.
pub fn seeded_device_code_card_fixture() -> BrowserHandoffCard {
    let mut card = card_device_code_auth();
    card.card_id = "browser_handoff_card:device_code_auth.expiry_disclosed".to_owned();
    card.notes = Some(
        "Device code and expiry are disclosed in-app; the code is never transmitted by Aureline."
            .to_owned(),
    );
    card
}

// ---------------------------------------------------------------------------
// Webview origin bars.
// ---------------------------------------------------------------------------

fn capability_limit(class: CapabilityLimitClass, note: &str) -> CapabilityLimit {
    CapabilityLimit {
        limit_class: class,
        limit_note: note.to_owned(),
    }
}

fn open_in_browser(ref_token: &str, label: &str, available: bool) -> OpenInBrowserAction {
    OpenInBrowserAction {
        action_ref: ref_token.to_owned(),
        action_label: label.to_owned(),
        available,
    }
}

/// Extension-owned webview origin bar.
fn bar_extension_owned() -> WebviewOriginBar {
    WebviewOriginBar {
        webview_origin_bar_schema_version: WEBVIEW_ORIGIN_BAR_SCHEMA_VERSION,
        record_kind: WEBVIEW_ORIGIN_BAR_RECORD_KIND.to_owned(),
        bar_id: "webview_origin_bar:extension_owned".to_owned(),
        owner_class: WebviewOwnerClass::ExtensionOwned,
        owner_identity_ref: "owner.extension.installed_extension".to_owned(),
        owner_label: "Installed extension".to_owned(),
        origin_label: "extension.example".to_owned(),
        origin_disclosure: OriginDisclosureClass::NamedExtensionOrigin,
        permission_state: WebviewPermissionState::ScopedPermissionsGranted,
        open_in_browser: open_in_browser(
            "action.open_in_browser.extension",
            "Open in browser",
            true,
        ),
        capability_limits: vec![
            capability_limit(
                CapabilityLimitClass::NotNativeTrustChrome,
                "This bar is extension content, not native Aureline trust chrome.",
            ),
            capability_limit(
                CapabilityLimitClass::CannotVerifyUpdates,
                "Extension content cannot verify Aureline updates.",
            ),
        ],
        labeled_as_embedded: true,
        impersonates_native_chrome: false,
        may_show_update_verification: false,
        may_show_device_permission_prompt: false,
        may_show_product_security_messaging: false,
        headline_label: "Extension content".to_owned(),
        bar_summary: "Embedded extension content, labeled with its origin and disclosed as not native trust chrome.".to_owned(),
        contract_doc_ref: M5_AUTH_BOUNDARY_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

/// Provider-owned webview origin bar.
fn bar_provider_owned() -> WebviewOriginBar {
    WebviewOriginBar {
        webview_origin_bar_schema_version: WEBVIEW_ORIGIN_BAR_SCHEMA_VERSION,
        record_kind: WEBVIEW_ORIGIN_BAR_RECORD_KIND.to_owned(),
        bar_id: "webview_origin_bar:provider_owned".to_owned(),
        owner_class: WebviewOwnerClass::ProviderOwned,
        owner_identity_ref: "owner.provider.connected_provider".to_owned(),
        owner_label: "Connected provider".to_owned(),
        origin_label: "app.provider.example".to_owned(),
        origin_disclosure: OriginDisclosureClass::NamedProviderOrigin,
        permission_state: WebviewPermissionState::NoElevatedPermissions,
        open_in_browser: open_in_browser(
            "action.open_in_browser.provider",
            "Open in browser",
            true,
        ),
        capability_limits: vec![
            capability_limit(
                CapabilityLimitClass::NotNativeTrustChrome,
                "This bar is provider content, not native Aureline trust chrome.",
            ),
            capability_limit(
                CapabilityLimitClass::CannotGrantDevicePermission,
                "Provider content cannot grant device permissions.",
            ),
            capability_limit(
                CapabilityLimitClass::CannotEnterProductCredentials,
                "Never enter your Aureline credentials into provider content.",
            ),
        ],
        labeled_as_embedded: true,
        impersonates_native_chrome: false,
        may_show_update_verification: false,
        may_show_device_permission_prompt: false,
        may_show_product_security_messaging: false,
        headline_label: "Provider content".to_owned(),
        bar_summary: "Embedded provider content, labeled with its origin and disclosed as not native trust chrome.".to_owned(),
        contract_doc_ref: M5_AUTH_BOUNDARY_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

/// First-party embedded webview origin bar.
fn bar_first_party_embedded() -> WebviewOriginBar {
    WebviewOriginBar {
        webview_origin_bar_schema_version: WEBVIEW_ORIGIN_BAR_SCHEMA_VERSION,
        record_kind: WEBVIEW_ORIGIN_BAR_RECORD_KIND.to_owned(),
        bar_id: "webview_origin_bar:first_party_embedded".to_owned(),
        owner_class: WebviewOwnerClass::FirstPartyEmbedded,
        owner_identity_ref: "owner.first_party.aureline_embedded".to_owned(),
        owner_label: "Aureline first-party embedded content".to_owned(),
        origin_label: "docs.aureline.example".to_owned(),
        origin_disclosure: OriginDisclosureClass::FirstPartyOrigin,
        permission_state: WebviewPermissionState::NoElevatedPermissions,
        open_in_browser: open_in_browser(
            "action.open_in_browser.first_party",
            "Open in browser",
            true,
        ),
        capability_limits: vec![
            capability_limit(
                CapabilityLimitClass::NotNativeTrustChrome,
                "Even first-party embedded content is not native trust chrome; trust prompts live in native chrome.",
            ),
            capability_limit(
                CapabilityLimitClass::CannotDisplayProductSecurity,
                "Embedded content does not display product-security messaging.",
            ),
        ],
        labeled_as_embedded: true,
        impersonates_native_chrome: false,
        may_show_update_verification: false,
        may_show_device_permission_prompt: false,
        may_show_product_security_messaging: false,
        headline_label: "Aureline embedded content".to_owned(),
        bar_summary: "First-party embedded content, still labeled embedded and disclosed as not native trust chrome.".to_owned(),
        contract_doc_ref: M5_AUTH_BOUNDARY_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

/// Unknown / untrusted webview origin bar.
fn bar_unknown_untrusted() -> WebviewOriginBar {
    WebviewOriginBar {
        webview_origin_bar_schema_version: WEBVIEW_ORIGIN_BAR_SCHEMA_VERSION,
        record_kind: WEBVIEW_ORIGIN_BAR_RECORD_KIND.to_owned(),
        bar_id: "webview_origin_bar:unknown_untrusted".to_owned(),
        owner_class: WebviewOwnerClass::UnknownUntrusted,
        owner_identity_ref: "owner.unknown.undisclosed_origin".to_owned(),
        owner_label: "Unknown origin".to_owned(),
        origin_label: "Origin could not be disclosed".to_owned(),
        origin_disclosure: OriginDisclosureClass::UndisclosedOriginBlocked,
        permission_state: WebviewPermissionState::PermissionDenied,
        open_in_browser: open_in_browser(
            "action.open_in_browser.unknown",
            "Open in browser",
            true,
        ),
        capability_limits: vec![
            capability_limit(
                CapabilityLimitClass::NotNativeTrustChrome,
                "This is untrusted content and is never native Aureline trust chrome.",
            ),
            capability_limit(
                CapabilityLimitClass::CannotVerifyUpdates,
                "Untrusted content cannot verify updates.",
            ),
            capability_limit(
                CapabilityLimitClass::CannotGrantDevicePermission,
                "Untrusted content cannot grant device permissions.",
            ),
            capability_limit(
                CapabilityLimitClass::CannotDisplayProductSecurity,
                "Untrusted content cannot display product-security messaging.",
            ),
            capability_limit(
                CapabilityLimitClass::CannotEnterProductCredentials,
                "Never enter your Aureline credentials into untrusted content.",
            ),
        ],
        labeled_as_embedded: true,
        impersonates_native_chrome: false,
        may_show_update_verification: false,
        may_show_device_permission_prompt: false,
        may_show_product_security_messaging: false,
        headline_label: "Untrusted content blocked".to_owned(),
        bar_summary: "Untrusted content whose origin could not be disclosed; permissions are denied and it is not native trust chrome.".to_owned(),
        contract_doc_ref: M5_AUTH_BOUNDARY_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

/// Build the canonical M5 webview origin bar set.
pub fn seeded_m5_webview_origin_bar_set() -> M5WebviewOriginBarSet {
    M5WebviewOriginBarSet {
        schema_version: M5_WEBVIEW_ORIGIN_BAR_SET_SCHEMA_VERSION,
        record_kind: M5_WEBVIEW_ORIGIN_BAR_SET_RECORD_KIND.to_owned(),
        set_id: M5_WEBVIEW_ORIGIN_BAR_SET_ID.to_owned(),
        set_label: "M5 webview origin bars".to_owned(),
        bars: vec![
            bar_extension_owned(),
            bar_provider_owned(),
            bar_first_party_embedded(),
            bar_unknown_untrusted(),
        ],
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_object_refs".to_owned(),
        minted_at: "mint.m5_webview_origin_bar_set".to_owned(),
        contract_doc_ref: M5_AUTH_BOUNDARY_CONTRACT_DOC_REF.to_owned(),
    }
}

/// A standalone untrusted webview origin bar fixture, proving an undisclosed
/// origin is blocked, denied, and never impersonates native trust chrome.
pub fn seeded_untrusted_webview_origin_bar_fixture() -> WebviewOriginBar {
    let mut bar = bar_unknown_untrusted();
    bar.bar_id = "webview_origin_bar:unknown_untrusted.blocked".to_owned();
    bar.notes = Some(
        "Undisclosed origin: content is blocked, permissions denied, and it discloses it is not native trust chrome.".to_owned(),
    );
    bar
}
