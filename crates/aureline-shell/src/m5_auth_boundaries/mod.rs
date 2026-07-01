//! Browser / device-code handoff cards and webview origin bars for the M5
//! auth-boundary honesty lane.
//!
//! This module is the in-product producer of two durable, checked-in record
//! families that keep Aureline honest about the moment authentication,
//! device-code entry, or provider content crosses out of native desktop chrome:
//!
//! - [`BrowserHandoffCard`] — rendered *before* Aureline hands a user to the
//!   system browser or shows a device-code authorization flow. Each card
//!   declares the provider/domain being handed to, the reason for the handoff,
//!   the data-exit boundary the payload obeys, the fallback state if the handoff
//!   is blocked, a local-continuity note so nothing is dropped, the device
//!   code / expiry disclosure where relevant, and the return anchor so the user
//!   knows how they come back. A browser/device-code handoff always leaves
//!   native chrome and never impersonates it.
//! - [`WebviewOriginBar`] — the origin bar rendered on an embedded webview so
//!   the surface can never impersonate native trust UI. Each bar discloses the
//!   extension/provider/origin that owns the content, the permission state, an
//!   open-in-browser action, and the capability limits the embedded surface has
//!   relative to native trusted chrome. Embedded surfaces may never show update
//!   verification, device-permission, or product-security messaging.
//!
//! The [`DataExitBoundary`] vocabulary is reused verbatim from
//! [`crate::public_truth`] so the auth-boundary lane speaks the same
//! redaction-safe export language as the About/help/community destination
//! contract.
//!
//! Two acceptance invariants are enforced structurally:
//!
//! - **Native chrome is distinguishable from browser/provider-owned content.** A
//!   browser handoff card sets `opens_outside_native_chrome = true` and
//!   `impersonates_native_chrome = false`; a webview origin bar sets
//!   `labeled_as_embedded = true`, `impersonates_native_chrome = false`, and
//!   holds every native-only messaging flag (`may_show_update_verification`,
//!   `may_show_device_permission_prompt`, `may_show_product_security_messaging`)
//!   at `false`.
//! - **Boundary truth is preserved.** Device-code and browser handoffs preserve
//!   target identity, expiry, and return-path truth; embedded surfaces disclose
//!   capability limits (always including `not_native_trust_chrome`) instead of
//!   pretending parity with native trusted surfaces.
//!
//! Raw URLs, raw email addresses, raw local paths, raw usernames, raw hostnames,
//! tokens, and raw secret material never cross this boundary; the records carry
//! opaque refs, controlled-vocabulary tokens, and bounded reviewable sentences
//! only.
//!
//! The boundary schemas are
//! [`schemas/help/m5-browser-handoff-card.schema.json`](../../../../schemas/help/m5-browser-handoff-card.schema.json)
//! and
//! [`schemas/help/m5-webview-origin-bar.schema.json`](../../../../schemas/help/m5-webview-origin-bar.schema.json).
//! The contract doc is
//! [`docs/help/m5_auth_boundaries_contract.md`](../../../../docs/help/m5_auth_boundaries_contract.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_device_code_card_fixture, seeded_m5_browser_handoff_card_set,
    seeded_m5_webview_origin_bar_set, seeded_untrusted_webview_origin_bar_fixture,
    M5_BROWSER_HANDOFF_CARD_SET_ID, M5_WEBVIEW_ORIGIN_BAR_SET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use crate::public_truth::DataExitBoundary;

// ---------------------------------------------------------------------------
// Stable identifiers, versions, and cross-contract refs.
// ---------------------------------------------------------------------------

/// Record-kind tag carried by [`BrowserHandoffCard`].
pub const BROWSER_HANDOFF_CARD_RECORD_KIND: &str = "browser_handoff_card_record";

/// Record-kind tag carried by [`M5BrowserHandoffCardSet`].
pub const M5_BROWSER_HANDOFF_CARD_SET_RECORD_KIND: &str = "m5_browser_handoff_card_set";

/// Record-kind tag carried by [`WebviewOriginBar`].
pub const WEBVIEW_ORIGIN_BAR_RECORD_KIND: &str = "webview_origin_bar_record";

/// Record-kind tag carried by [`M5WebviewOriginBarSet`].
pub const M5_WEBVIEW_ORIGIN_BAR_SET_RECORD_KIND: &str = "m5_webview_origin_bar_set";

/// Schema version for a single browser-handoff card.
pub const BROWSER_HANDOFF_CARD_SCHEMA_VERSION: u32 = 1;

/// Schema version for the bundled browser-handoff card set.
pub const M5_BROWSER_HANDOFF_CARD_SET_SCHEMA_VERSION: u32 = 1;

/// Schema version for a single webview origin bar.
pub const WEBVIEW_ORIGIN_BAR_SCHEMA_VERSION: u32 = 1;

/// Schema version for the bundled webview origin bar set.
pub const M5_WEBVIEW_ORIGIN_BAR_SET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the browser-handoff-card boundary schema.
pub const M5_BROWSER_HANDOFF_CARD_SCHEMA_REF: &str =
    "schemas/help/m5-browser-handoff-card.schema.json";

/// Repo-relative path of the webview-origin-bar boundary schema.
pub const M5_WEBVIEW_ORIGIN_BAR_SCHEMA_REF: &str = "schemas/help/m5-webview-origin-bar.schema.json";

/// Repo-relative path of the contract doc both record families point at.
pub const M5_AUTH_BOUNDARY_CONTRACT_DOC_REF: &str = "docs/help/m5_auth_boundaries_contract.md";

/// Repo-relative path of the sibling community-handoff target contract this lane
/// aligns its handoff vocabulary with.
pub const M5_AUTH_BOUNDARY_COMMUNITY_HANDOFF_REF: &str =
    "schemas/help/m5-handoff-target.schema.json";

/// Repo-relative path of the device-permission-row contract this lane keeps
/// device-permission messaging out of embedded surfaces for.
pub const M5_AUTH_BOUNDARY_DEVICE_PERMISSION_REF: &str =
    "schemas/help/m5-device-permission-row.schema.json";

// ===========================================================================
// Browser / device-code handoff cards.
// ===========================================================================

/// The kind of handoff a card describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserHandoffKind {
    /// Sign in with a provider through the system browser.
    SystemBrowserAuth,
    /// Authorize a device code the user enters at the provider.
    DeviceCodeAuth,
    /// View provider-owned content in the browser.
    ProviderContentView,
    /// Open a vendor / third-party resource in the browser.
    VendorOutboundLink,
}

impl BrowserHandoffKind {
    /// Every handoff kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SystemBrowserAuth,
        Self::DeviceCodeAuth,
        Self::ProviderContentView,
        Self::VendorOutboundLink,
    ];

    /// Stable token recorded on the card.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemBrowserAuth => "system_browser_auth",
            Self::DeviceCodeAuth => "device_code_auth",
            Self::ProviderContentView => "provider_content_view",
            Self::VendorOutboundLink => "vendor_outbound_link",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SystemBrowserAuth => "System browser sign-in",
            Self::DeviceCodeAuth => "Device-code authorization",
            Self::ProviderContentView => "Provider content in browser",
            Self::VendorOutboundLink => "Vendor / third-party link",
        }
    }

    /// The reason class this kind must carry.
    pub const fn required_reason(self) -> HandoffReasonClass {
        match self {
            Self::SystemBrowserAuth => HandoffReasonClass::AuthenticateWithProvider,
            Self::DeviceCodeAuth => HandoffReasonClass::AuthorizeDeviceCode,
            Self::ProviderContentView => HandoffReasonClass::ViewProviderContent,
            Self::VendorOutboundLink => HandoffReasonClass::OpenVendorResource,
        }
    }

    /// True when the card must carry a device-code disclosure.
    pub const fn requires_device_code_disclosure(self) -> bool {
        matches!(self, Self::DeviceCodeAuth)
    }

    /// Whether the given data-exit boundary is honest for this handoff kind.
    pub fn allows_data_exit(self, data_exit: DataExitBoundary) -> bool {
        use DataExitBoundary as D;
        match self {
            Self::SystemBrowserAuth => {
                matches!(
                    data_exit,
                    D::VendorOrThirdPartyOutbound | D::NoPayloadLeavesProduct
                )
            }
            Self::DeviceCodeAuth => {
                matches!(
                    data_exit,
                    D::NoPayloadLeavesProduct | D::VendorOrThirdPartyOutbound
                )
            }
            Self::ProviderContentView => {
                matches!(
                    data_exit,
                    D::ExternalPublicBrowse | D::VendorOrThirdPartyOutbound
                )
            }
            Self::VendorOutboundLink => {
                matches!(
                    data_exit,
                    D::ExternalPublicBrowse | D::VendorOrThirdPartyOutbound
                )
            }
        }
    }
}

/// Why the handoff happens; kept consistent with the handoff kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffReasonClass {
    /// Authenticate with the provider.
    AuthenticateWithProvider,
    /// Authorize a device code.
    AuthorizeDeviceCode,
    /// View provider-owned content.
    ViewProviderContent,
    /// Open a vendor / third-party resource.
    OpenVendorResource,
}

impl HandoffReasonClass {
    /// Stable token recorded on the card.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthenticateWithProvider => "authenticate_with_provider",
            Self::AuthorizeDeviceCode => "authorize_device_code",
            Self::ViewProviderContent => "view_provider_content",
            Self::OpenVendorResource => "open_vendor_resource",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AuthenticateWithProvider => "Authenticate with provider",
            Self::AuthorizeDeviceCode => "Authorize device code",
            Self::ViewProviderContent => "View provider content",
            Self::OpenVendorResource => "Open vendor resource",
        }
    }
}

/// What survives if the handoff is blocked or the browser cannot open.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackStateClass {
    /// Local continuity is preserved; the user can resume in-app.
    LocalContinuityPreserved,
    /// The user can retry the handoff from within Aureline.
    RetryHandoffInApp,
    /// The user can enter the device code manually.
    ManualCodeEntry,
    /// The handoff degrades to a labeled copy-link path.
    CopyLinkForManualOpen,
}

impl FallbackStateClass {
    /// Stable token recorded on the card.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalContinuityPreserved => "local_continuity_preserved",
            Self::RetryHandoffInApp => "retry_handoff_in_app",
            Self::ManualCodeEntry => "manual_code_entry",
            Self::CopyLinkForManualOpen => "copy_link_for_manual_open",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalContinuityPreserved => "Local continuity preserved",
            Self::RetryHandoffInApp => "Retry handoff in app",
            Self::ManualCodeEntry => "Manual code entry",
            Self::CopyLinkForManualOpen => "Copy link to open manually",
        }
    }
}

/// How a device code's expiry is disclosed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpiryDisclosureClass {
    /// Expires with a visible countdown.
    ExpiresWithCountdown,
    /// Expires at a disclosed time.
    ExpiresAtDisclosedTime,
    /// No expiry applies to this handoff.
    NoExpiryApplicable,
}

impl ExpiryDisclosureClass {
    /// Stable token recorded on the card.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpiresWithCountdown => "expires_with_countdown",
            Self::ExpiresAtDisclosedTime => "expires_at_disclosed_time",
            Self::NoExpiryApplicable => "no_expiry_applicable",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ExpiresWithCountdown => "Expires with countdown",
            Self::ExpiresAtDisclosedTime => "Expires at disclosed time",
            Self::NoExpiryApplicable => "No expiry applicable",
        }
    }

    /// True when the disclosure states a real expiry (required for device codes).
    pub const fn discloses_expiry(self) -> bool {
        matches!(
            self,
            Self::ExpiresWithCountdown | Self::ExpiresAtDisclosedTime
        )
    }
}

/// The device-code / expiry disclosure block, present only for a device-code
/// handoff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceCodeDisclosure {
    /// Opaque ref of where and how the one-time device code is presented.
    pub code_presentation_ref: String,
    /// Reviewer-facing label for the code presentation.
    pub code_presentation_label: String,
    /// How the code's expiry is disclosed.
    pub expiry_disclosure: ExpiryDisclosureClass,
    /// A bounded reviewable sentence describing the expiry.
    pub expiry_note: String,
    /// The code is shown in-app for the user to enter at the provider; it is not
    /// transmitted by Aureline. Always true.
    pub code_shown_in_app_not_transmitted: bool,
}

/// The local-continuity note carried on every card so a blocked handoff never
/// drops the user's work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalContinuity {
    /// Opaque ref of the local continuity anchor.
    pub continuity_ref: String,
    /// Whether the user's in-app work is preserved locally. Always true.
    pub work_preserved_locally: bool,
    /// A bounded reviewable sentence describing what stays local.
    pub continuity_note: String,
}

/// The return anchor: how the user comes back to native Aureline after the
/// handoff, so the return path is truthful rather than implied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReturnAnchor {
    /// Opaque ref of the return anchor.
    pub anchor_ref: String,
    /// Reviewer-facing return anchor label.
    pub anchor_label: String,
    /// A bounded reviewable sentence stating the return-path truth.
    pub return_path_truth_note: String,
}

/// One browser / device-code handoff card rendered before Aureline hands a user
/// to the system browser or a device-code flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserHandoffCard {
    /// Schema version for this card shape.
    pub browser_handoff_card_schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable card id; prefixed `browser_handoff_card:`.
    pub card_id: String,
    /// The kind of handoff.
    pub handoff_kind: BrowserHandoffKind,
    /// Why the handoff happens.
    pub handoff_reason: HandoffReasonClass,
    /// A bounded reviewable sentence describing the reason.
    pub reason_note: String,
    /// Opaque ref of the provider identity.
    pub provider_identity_ref: String,
    /// Reviewer-facing provider label.
    pub provider_label: String,
    /// Reviewer-facing provider domain label (no scheme, no credentials).
    pub provider_domain_label: String,
    /// The data-exit boundary the handoff obeys.
    pub data_exit_boundary: DataExitBoundary,
    /// A bounded reviewable sentence naming what leaves the product.
    pub data_exit_note: String,
    /// The device-code / expiry disclosure, present only for device-code
    /// handoffs.
    pub device_code_disclosure: Option<DeviceCodeDisclosure>,
    /// What survives if the handoff is blocked.
    pub fallback_state: FallbackStateClass,
    /// A bounded reviewable sentence describing the fallback.
    pub fallback_note: String,
    /// The local-continuity note.
    pub local_continuity: LocalContinuity,
    /// The return anchor.
    pub return_anchor: ReturnAnchor,
    /// Whether the handoff opens outside native chrome. Always true.
    pub opens_outside_native_chrome: bool,
    /// Whether the card impersonates native chrome. Always false.
    pub impersonates_native_chrome: bool,
    /// Whether provider-owned content is labeled as provider-owned. Always true.
    pub presents_provider_owned_content_labeled: bool,
    /// Reviewer-facing headline label.
    pub headline_label: String,
    /// A bounded reviewable sentence summarizing the card.
    pub card_summary: String,
    /// Frozen contract-doc ref.
    pub contract_doc_ref: String,
    /// Optional reviewer note.
    pub notes: Option<String>,
}

impl BrowserHandoffCard {
    /// Validate the card against the browser-handoff contract.
    pub fn validate(&self) -> Result<(), AuthBoundaryError> {
        if self.browser_handoff_card_schema_version != BROWSER_HANDOFF_CARD_SCHEMA_VERSION {
            return Err(AuthBoundaryError::WrongCardSchemaVersion {
                card_id: self.card_id.clone(),
                actual: self.browser_handoff_card_schema_version,
            });
        }
        if self.record_kind != BROWSER_HANDOFF_CARD_RECORD_KIND {
            return Err(AuthBoundaryError::WrongCardRecordKind {
                card_id: self.card_id.clone(),
                actual: self.record_kind.clone(),
            });
        }
        if !self.card_id.starts_with("browser_handoff_card:") {
            return Err(AuthBoundaryError::MalformedCardId {
                card_id: self.card_id.clone(),
            });
        }
        if self.contract_doc_ref != M5_AUTH_BOUNDARY_CONTRACT_DOC_REF {
            return Err(AuthBoundaryError::WrongContractDocRef {
                record_id: self.card_id.clone(),
                actual: self.contract_doc_ref.clone(),
            });
        }

        for (field, value) in [
            ("reason_note", &self.reason_note),
            ("provider_label", &self.provider_label),
            ("provider_domain_label", &self.provider_domain_label),
            ("data_exit_note", &self.data_exit_note),
            ("fallback_note", &self.fallback_note),
            ("headline_label", &self.headline_label),
            ("card_summary", &self.card_summary),
        ] {
            if non_empty(value).is_none() {
                return Err(AuthBoundaryError::EmptyRequiredField {
                    record_id: self.card_id.clone(),
                    field,
                });
            }
        }
        if !ref_is_opaque(&self.provider_identity_ref) {
            return Err(AuthBoundaryError::RawRefLeak {
                record_id: self.card_id.clone(),
                field: "provider_identity_ref",
            });
        }

        // Reason must be the one required by the handoff kind.
        if self.handoff_reason != self.handoff_kind.required_reason() {
            return Err(AuthBoundaryError::HandoffReasonMismatch {
                card_id: self.card_id.clone(),
                kind: self.handoff_kind,
                reason: self.handoff_reason,
            });
        }
        // Data-exit boundary must be honest for the handoff kind.
        if !self.handoff_kind.allows_data_exit(self.data_exit_boundary) {
            return Err(AuthBoundaryError::HandoffDataExitMismatch {
                card_id: self.card_id.clone(),
                kind: self.handoff_kind,
                data_exit: self.data_exit_boundary,
            });
        }

        // Device-code disclosure present iff the handoff is a device-code flow.
        match (
            &self.device_code_disclosure,
            self.handoff_kind.requires_device_code_disclosure(),
        ) {
            (Some(disclosure), true) => disclosure.validate(&self.card_id)?,
            (None, false) => {}
            (Some(_), false) => {
                return Err(AuthBoundaryError::UnexpectedDeviceCodeDisclosure {
                    card_id: self.card_id.clone(),
                    kind: self.handoff_kind,
                });
            }
            (None, true) => {
                return Err(AuthBoundaryError::MissingDeviceCodeDisclosure {
                    card_id: self.card_id.clone(),
                });
            }
        }

        // Local continuity is preserved and truthfully anchored.
        if !ref_is_opaque(&self.local_continuity.continuity_ref) {
            return Err(AuthBoundaryError::RawRefLeak {
                record_id: self.card_id.clone(),
                field: "local_continuity.continuity_ref",
            });
        }
        if non_empty(&self.local_continuity.continuity_note).is_none() {
            return Err(AuthBoundaryError::EmptyRequiredField {
                record_id: self.card_id.clone(),
                field: "local_continuity.continuity_note",
            });
        }
        if !self.local_continuity.work_preserved_locally {
            return Err(AuthBoundaryError::LocalContinuityNotPreserved {
                card_id: self.card_id.clone(),
            });
        }

        // Return anchor preserves return-path truth.
        if !ref_is_opaque(&self.return_anchor.anchor_ref) {
            return Err(AuthBoundaryError::RawRefLeak {
                record_id: self.card_id.clone(),
                field: "return_anchor.anchor_ref",
            });
        }
        if non_empty(&self.return_anchor.anchor_label).is_none()
            || non_empty(&self.return_anchor.return_path_truth_note).is_none()
        {
            return Err(AuthBoundaryError::EmptyRequiredField {
                record_id: self.card_id.clone(),
                field: "return_anchor",
            });
        }

        // Boundary honesty: a browser/device-code handoff always leaves native
        // chrome and never impersonates it, and provider content stays labeled.
        if !self.opens_outside_native_chrome {
            return Err(AuthBoundaryError::HandoffDoesNotLeaveNativeChrome {
                card_id: self.card_id.clone(),
            });
        }
        if self.impersonates_native_chrome {
            return Err(AuthBoundaryError::ImpersonatesNativeChrome {
                record_id: self.card_id.clone(),
            });
        }
        if !self.presents_provider_owned_content_labeled {
            return Err(AuthBoundaryError::ProviderContentNotLabeled {
                card_id: self.card_id.clone(),
            });
        }

        Ok(())
    }

    /// Render a deterministic plaintext block for support exports and previews.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "[{}] {} — kind={} reason={}\n",
            self.card_id,
            self.headline_label,
            self.handoff_kind.as_str(),
            self.handoff_reason.as_str(),
        ));
        out.push_str(&format!(
            "    provider: {} ({}) | data_exit={}\n",
            self.provider_label,
            self.provider_domain_label,
            self.data_exit_boundary.as_str(),
        ));
        if let Some(disclosure) = &self.device_code_disclosure {
            out.push_str(&format!(
                "    device code: {} (expiry={})\n",
                disclosure.code_presentation_ref,
                disclosure.expiry_disclosure.as_str(),
            ));
        }
        out.push_str(&format!(
            "    fallback: {} | return anchor: {}\n",
            self.fallback_state.as_str(),
            self.return_anchor.anchor_ref,
        ));
        out
    }
}

impl DeviceCodeDisclosure {
    fn validate(&self, card_id: &str) -> Result<(), AuthBoundaryError> {
        if !ref_is_opaque(&self.code_presentation_ref) {
            return Err(AuthBoundaryError::RawRefLeak {
                record_id: card_id.to_owned(),
                field: "device_code_disclosure.code_presentation_ref",
            });
        }
        if non_empty(&self.code_presentation_label).is_none()
            || non_empty(&self.expiry_note).is_none()
        {
            return Err(AuthBoundaryError::EmptyRequiredField {
                record_id: card_id.to_owned(),
                field: "device_code_disclosure",
            });
        }
        // A device code always carries a real expiry.
        if !self.expiry_disclosure.discloses_expiry() {
            return Err(AuthBoundaryError::DeviceCodeMissingExpiry {
                card_id: card_id.to_owned(),
            });
        }
        // Aureline shows the code for the user to enter; it never transmits it.
        if !self.code_shown_in_app_not_transmitted {
            return Err(AuthBoundaryError::DeviceCodeTransmitted {
                card_id: card_id.to_owned(),
            });
        }
        Ok(())
    }
}

/// A bundled set of browser-handoff cards, one per governed handoff kind,
/// checked in as canonical M5 auth-boundary source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BrowserHandoffCardSet {
    /// Schema version for the card-set shape.
    pub schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable id for the card set.
    pub set_id: String,
    /// Reviewer-facing label for the card set.
    pub set_label: String,
    /// One card per governed handoff kind.
    pub cards: Vec<BrowserHandoffCard>,
    /// Source contracts this set binds to by id.
    pub source_contract_refs: Vec<String>,
    /// Redaction-class token covering the export boundary.
    pub redaction_class_token: String,
    /// Opaque mint timestamp ref.
    pub minted_at: String,
    /// Frozen contract-doc ref.
    pub contract_doc_ref: String,
}

impl M5BrowserHandoffCardSet {
    /// Validate the card set: every card validates, every handoff kind is
    /// represented exactly once, at least one card is a device-code flow, no two
    /// cards share an id, and the source contracts are present.
    pub fn validate(&self) -> Result<(), AuthBoundaryError> {
        if self.schema_version != M5_BROWSER_HANDOFF_CARD_SET_SCHEMA_VERSION {
            return Err(AuthBoundaryError::WrongSetSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_BROWSER_HANDOFF_CARD_SET_RECORD_KIND {
            return Err(AuthBoundaryError::WrongSetRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        self.check_set_identity()?;
        if self.contract_doc_ref != M5_AUTH_BOUNDARY_CONTRACT_DOC_REF {
            return Err(AuthBoundaryError::WrongContractDocRef {
                record_id: self.set_id.clone(),
                actual: self.contract_doc_ref.clone(),
            });
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for card in &self.cards {
            card.validate()?;
            if !seen.insert(card.card_id.as_str()) {
                return Err(AuthBoundaryError::DuplicateRecordId {
                    record_id: card.card_id.clone(),
                });
            }
        }

        // Every governed handoff kind is named exactly once.
        for kind in BrowserHandoffKind::ALL {
            let count = self.cards.iter().filter(|c| c.handoff_kind == kind).count();
            if count != 1 {
                return Err(AuthBoundaryError::HandoffKindNotNamedOnce {
                    kind,
                    count: count as u32,
                });
            }
        }
        // At least one device-code card proves the device-code path is covered.
        if !self
            .cards
            .iter()
            .any(|c| c.handoff_kind == BrowserHandoffKind::DeviceCodeAuth)
        {
            return Err(AuthBoundaryError::DeviceCodeCardMissing);
        }

        self.check_source_contracts()?;
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("browser-handoff card set serializes"),
        ) {
            return Err(AuthBoundaryError::RawMaterialInExport);
        }
        Ok(())
    }

    fn check_set_identity(&self) -> Result<(), AuthBoundaryError> {
        if non_empty(&self.set_id).is_none()
            || non_empty(&self.set_label).is_none()
            || non_empty(&self.redaction_class_token).is_none()
            || non_empty(&self.minted_at).is_none()
        {
            return Err(AuthBoundaryError::SetIdentityIncomplete);
        }
        Ok(())
    }

    fn check_source_contracts(&self) -> Result<(), AuthBoundaryError> {
        let refs: BTreeSet<&str> = self
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        for required in [
            M5_BROWSER_HANDOFF_CARD_SCHEMA_REF,
            M5_AUTH_BOUNDARY_CONTRACT_DOC_REF,
            M5_AUTH_BOUNDARY_COMMUNITY_HANDOFF_REF,
            M5_AUTH_BOUNDARY_DEVICE_PERMISSION_REF,
        ] {
            if !refs.contains(required) {
                return Err(AuthBoundaryError::MissingSourceContracts);
            }
        }
        Ok(())
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("browser-handoff card set serializes")
    }

    /// Deterministic CSV: one row per handoff kind.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "handoff_kind,reason,data_exit_boundary,has_device_code,fallback_state,opens_outside_native_chrome,impersonates_native_chrome\n",
        );
        for card in &self.cards {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                card.handoff_kind.as_str(),
                card.handoff_reason.as_str(),
                card.data_exit_boundary.as_str(),
                card.device_code_disclosure.is_some(),
                card.fallback_state.as_str(),
                card.opens_outside_native_chrome,
                card.impersonates_native_chrome,
            ));
        }
        out
    }

    /// Deterministic Markdown governance summary.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 browser / device-code handoff cards\n\n");
        out.push_str(&format!("Card set: `{}`\n\n", self.set_id));
        out.push_str(
            "| Handoff kind | Reason | Data exit | Device code | Fallback | Return anchor |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- |\n");
        for card in &self.cards {
            out.push_str(&format!(
                "| {} | {} | `{}` | {} | {} | `{}` |\n",
                card.handoff_kind.label(),
                card.handoff_reason.label(),
                card.data_exit_boundary.as_str(),
                card.device_code_disclosure.is_some(),
                card.fallback_state.label(),
                card.return_anchor.anchor_ref,
            ));
        }
        out.push('\n');
        out.push_str(
            "Every card opens outside native chrome, never impersonates it, and preserves ",
        );
        out.push_str("local continuity plus a truthful return anchor; device-code cards disclose the code and its expiry.\n");
        out
    }
}

// ===========================================================================
// Webview origin bars.
// ===========================================================================

/// Who owns the content rendered in an embedded webview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebviewOwnerClass {
    /// An installed extension owns the embedded content.
    ExtensionOwned,
    /// A connected provider owns the embedded content.
    ProviderOwned,
    /// First-party Aureline content rendered in a webview, still labeled.
    FirstPartyEmbedded,
    /// Unknown / untrusted origin that could not be disclosed.
    UnknownUntrusted,
}

impl WebviewOwnerClass {
    /// Every owner class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ExtensionOwned,
        Self::ProviderOwned,
        Self::FirstPartyEmbedded,
        Self::UnknownUntrusted,
    ];

    /// Stable token recorded on the bar.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExtensionOwned => "extension_owned",
            Self::ProviderOwned => "provider_owned",
            Self::FirstPartyEmbedded => "first_party_embedded",
            Self::UnknownUntrusted => "unknown_untrusted",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ExtensionOwned => "Extension-owned",
            Self::ProviderOwned => "Provider-owned",
            Self::FirstPartyEmbedded => "First-party embedded",
            Self::UnknownUntrusted => "Unknown / untrusted",
        }
    }

    /// The origin disclosure this owner class must carry.
    pub const fn required_origin_disclosure(self) -> OriginDisclosureClass {
        match self {
            Self::ExtensionOwned => OriginDisclosureClass::NamedExtensionOrigin,
            Self::ProviderOwned => OriginDisclosureClass::NamedProviderOrigin,
            Self::FirstPartyEmbedded => OriginDisclosureClass::FirstPartyOrigin,
            Self::UnknownUntrusted => OriginDisclosureClass::UndisclosedOriginBlocked,
        }
    }
}

/// How the origin is disclosed on the bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginDisclosureClass {
    /// A named extension origin.
    NamedExtensionOrigin,
    /// A named provider origin.
    NamedProviderOrigin,
    /// A first-party origin.
    FirstPartyOrigin,
    /// The origin could not be disclosed and the content is blocked / gated.
    UndisclosedOriginBlocked,
}

impl OriginDisclosureClass {
    /// Stable token recorded on the bar.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NamedExtensionOrigin => "named_extension_origin",
            Self::NamedProviderOrigin => "named_provider_origin",
            Self::FirstPartyOrigin => "first_party_origin",
            Self::UndisclosedOriginBlocked => "undisclosed_origin_blocked",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NamedExtensionOrigin => "Named extension origin",
            Self::NamedProviderOrigin => "Named provider origin",
            Self::FirstPartyOrigin => "First-party origin",
            Self::UndisclosedOriginBlocked => "Undisclosed origin (blocked)",
        }
    }
}

/// The permission state of the embedded surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebviewPermissionState {
    /// No elevated permissions are granted.
    NoElevatedPermissions,
    /// Scoped permissions are granted.
    ScopedPermissionsGranted,
    /// A permission request is pending user review.
    PermissionRequestPending,
    /// Permissions are denied.
    PermissionDenied,
}

impl WebviewPermissionState {
    /// Stable token recorded on the bar.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoElevatedPermissions => "no_elevated_permissions",
            Self::ScopedPermissionsGranted => "scoped_permissions_granted",
            Self::PermissionRequestPending => "permission_request_pending",
            Self::PermissionDenied => "permission_denied",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NoElevatedPermissions => "No elevated permissions",
            Self::ScopedPermissionsGranted => "Scoped permissions granted",
            Self::PermissionRequestPending => "Permission request pending",
            Self::PermissionDenied => "Permission denied",
        }
    }

    /// Permission states an untrusted, undisclosed origin may hold.
    pub const fn allowed_for_untrusted(self) -> bool {
        matches!(self, Self::NoElevatedPermissions | Self::PermissionDenied)
    }
}

/// A capability the embedded surface lacks relative to native trusted chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityLimitClass {
    /// The surface is not native trust chrome.
    NotNativeTrustChrome,
    /// The surface cannot verify updates.
    CannotVerifyUpdates,
    /// The surface cannot grant device permissions.
    CannotGrantDevicePermission,
    /// The surface cannot display product-security messaging.
    CannotDisplayProductSecurity,
    /// The surface cannot collect Aureline product credentials.
    CannotEnterProductCredentials,
}

impl CapabilityLimitClass {
    /// Every capability-limit class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NotNativeTrustChrome,
        Self::CannotVerifyUpdates,
        Self::CannotGrantDevicePermission,
        Self::CannotDisplayProductSecurity,
        Self::CannotEnterProductCredentials,
    ];

    /// Stable token recorded on the bar.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNativeTrustChrome => "not_native_trust_chrome",
            Self::CannotVerifyUpdates => "cannot_verify_updates",
            Self::CannotGrantDevicePermission => "cannot_grant_device_permission",
            Self::CannotDisplayProductSecurity => "cannot_display_product_security",
            Self::CannotEnterProductCredentials => "cannot_enter_product_credentials",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotNativeTrustChrome => "Not native trust chrome",
            Self::CannotVerifyUpdates => "Cannot verify updates",
            Self::CannotGrantDevicePermission => "Cannot grant device permission",
            Self::CannotDisplayProductSecurity => "Cannot display product security messaging",
            Self::CannotEnterProductCredentials => "Cannot enter product credentials",
        }
    }
}

/// One capability-limit disclosure on the bar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityLimit {
    /// The capability the surface lacks.
    pub limit_class: CapabilityLimitClass,
    /// A bounded reviewable sentence describing the limit.
    pub limit_note: String,
}

/// The open-in-browser action a bar offers so the user can escape the embedded
/// surface into their real browser.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenInBrowserAction {
    /// Opaque ref of the open-in-browser action.
    pub action_ref: String,
    /// Reviewer-facing action label.
    pub action_label: String,
    /// Whether the action is available.
    pub available: bool,
}

/// One webview origin bar rendered on an embedded surface so it cannot
/// impersonate native trust UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebviewOriginBar {
    /// Schema version for this bar shape.
    pub webview_origin_bar_schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable bar id; prefixed `webview_origin_bar:`.
    pub bar_id: String,
    /// Who owns the embedded content.
    pub owner_class: WebviewOwnerClass,
    /// Opaque ref of the owner identity.
    pub owner_identity_ref: String,
    /// Reviewer-facing owner label.
    pub owner_label: String,
    /// Reviewer-facing origin label (no scheme, no credentials).
    pub origin_label: String,
    /// How the origin is disclosed.
    pub origin_disclosure: OriginDisclosureClass,
    /// The permission state.
    pub permission_state: WebviewPermissionState,
    /// The open-in-browser action.
    pub open_in_browser: OpenInBrowserAction,
    /// The capability limits disclosed on the bar.
    pub capability_limits: Vec<CapabilityLimit>,
    /// Whether the surface is labeled as embedded. Always true.
    pub labeled_as_embedded: bool,
    /// Whether the bar impersonates native chrome. Always false.
    pub impersonates_native_chrome: bool,
    /// Whether the embedded surface may show update verification. Always false.
    pub may_show_update_verification: bool,
    /// Whether the embedded surface may show a device-permission prompt. Always
    /// false.
    pub may_show_device_permission_prompt: bool,
    /// Whether the embedded surface may show product-security messaging. Always
    /// false.
    pub may_show_product_security_messaging: bool,
    /// Reviewer-facing headline label.
    pub headline_label: String,
    /// A bounded reviewable sentence summarizing the bar.
    pub bar_summary: String,
    /// Frozen contract-doc ref.
    pub contract_doc_ref: String,
    /// Optional reviewer note.
    pub notes: Option<String>,
}

impl WebviewOriginBar {
    /// Validate the bar against the webview-origin-bar contract.
    pub fn validate(&self) -> Result<(), AuthBoundaryError> {
        if self.webview_origin_bar_schema_version != WEBVIEW_ORIGIN_BAR_SCHEMA_VERSION {
            return Err(AuthBoundaryError::WrongBarSchemaVersion {
                bar_id: self.bar_id.clone(),
                actual: self.webview_origin_bar_schema_version,
            });
        }
        if self.record_kind != WEBVIEW_ORIGIN_BAR_RECORD_KIND {
            return Err(AuthBoundaryError::WrongBarRecordKind {
                bar_id: self.bar_id.clone(),
                actual: self.record_kind.clone(),
            });
        }
        if !self.bar_id.starts_with("webview_origin_bar:") {
            return Err(AuthBoundaryError::MalformedBarId {
                bar_id: self.bar_id.clone(),
            });
        }
        if self.contract_doc_ref != M5_AUTH_BOUNDARY_CONTRACT_DOC_REF {
            return Err(AuthBoundaryError::WrongContractDocRef {
                record_id: self.bar_id.clone(),
                actual: self.contract_doc_ref.clone(),
            });
        }

        for (field, value) in [
            ("owner_label", &self.owner_label),
            ("origin_label", &self.origin_label),
            ("headline_label", &self.headline_label),
            ("bar_summary", &self.bar_summary),
            (
                "open_in_browser.action_label",
                &self.open_in_browser.action_label,
            ),
        ] {
            if non_empty(value).is_none() {
                return Err(AuthBoundaryError::EmptyRequiredField {
                    record_id: self.bar_id.clone(),
                    field,
                });
            }
        }
        if !ref_is_opaque(&self.owner_identity_ref) {
            return Err(AuthBoundaryError::RawRefLeak {
                record_id: self.bar_id.clone(),
                field: "owner_identity_ref",
            });
        }
        if !ref_is_opaque(&self.open_in_browser.action_ref) {
            return Err(AuthBoundaryError::RawRefLeak {
                record_id: self.bar_id.clone(),
                field: "open_in_browser.action_ref",
            });
        }

        // Origin disclosure must match the owner class.
        if self.origin_disclosure != self.owner_class.required_origin_disclosure() {
            return Err(AuthBoundaryError::OriginDisclosureMismatch {
                bar_id: self.bar_id.clone(),
                owner: self.owner_class,
                origin: self.origin_disclosure,
            });
        }
        // An unknown/untrusted origin may only hold a non-elevated or denied
        // permission state.
        if self.owner_class == WebviewOwnerClass::UnknownUntrusted
            && !self.permission_state.allowed_for_untrusted()
        {
            return Err(AuthBoundaryError::UntrustedPermissionTooBroad {
                bar_id: self.bar_id.clone(),
                permission: self.permission_state,
            });
        }

        // Boundary honesty: the surface is labeled embedded and never
        // impersonates native chrome or native-only messaging.
        if !self.labeled_as_embedded {
            return Err(AuthBoundaryError::EmbeddedSurfaceNotLabeled {
                bar_id: self.bar_id.clone(),
            });
        }
        if self.impersonates_native_chrome {
            return Err(AuthBoundaryError::ImpersonatesNativeChrome {
                record_id: self.bar_id.clone(),
            });
        }
        if self.may_show_update_verification {
            return Err(AuthBoundaryError::EmbeddedImpersonatesNativeMessaging {
                bar_id: self.bar_id.clone(),
                messaging: "update_verification",
            });
        }
        if self.may_show_device_permission_prompt {
            return Err(AuthBoundaryError::EmbeddedImpersonatesNativeMessaging {
                bar_id: self.bar_id.clone(),
                messaging: "device_permission_prompt",
            });
        }
        if self.may_show_product_security_messaging {
            return Err(AuthBoundaryError::EmbeddedImpersonatesNativeMessaging {
                bar_id: self.bar_id.clone(),
                messaging: "product_security_messaging",
            });
        }

        // Capability limits are disclosed and always include the not-native-trust
        // limit so embedded content never pretends parity.
        if self.capability_limits.is_empty() {
            return Err(AuthBoundaryError::MissingCapabilityLimits {
                bar_id: self.bar_id.clone(),
            });
        }
        for limit in &self.capability_limits {
            if non_empty(&limit.limit_note).is_none() {
                return Err(AuthBoundaryError::EmptyRequiredField {
                    record_id: self.bar_id.clone(),
                    field: "capability_limits.limit_note",
                });
            }
        }
        if !self
            .capability_limits
            .iter()
            .any(|l| l.limit_class == CapabilityLimitClass::NotNativeTrustChrome)
        {
            return Err(AuthBoundaryError::MissingNotNativeTrustLimit {
                bar_id: self.bar_id.clone(),
            });
        }

        Ok(())
    }

    /// Render a deterministic plaintext block for support exports and previews.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "[{}] {} — owner={} origin={}\n",
            self.bar_id,
            self.headline_label,
            self.owner_class.as_str(),
            self.origin_disclosure.as_str(),
        ));
        out.push_str(&format!(
            "    permission: {} | open_in_browser: {} (available={})\n",
            self.permission_state.as_str(),
            self.open_in_browser.action_ref,
            self.open_in_browser.available,
        ));
        for limit in &self.capability_limits {
            out.push_str(&format!("    limit: {}\n", limit.limit_class.as_str()));
        }
        out
    }
}

/// A bundled set of webview origin bars, one per governed owner class, checked in
/// as canonical M5 auth-boundary source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WebviewOriginBarSet {
    /// Schema version for the bar-set shape.
    pub schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable id for the bar set.
    pub set_id: String,
    /// Reviewer-facing label for the bar set.
    pub set_label: String,
    /// One bar per governed owner class.
    pub bars: Vec<WebviewOriginBar>,
    /// Source contracts this set binds to by id.
    pub source_contract_refs: Vec<String>,
    /// Redaction-class token covering the export boundary.
    pub redaction_class_token: String,
    /// Opaque mint timestamp ref.
    pub minted_at: String,
    /// Frozen contract-doc ref.
    pub contract_doc_ref: String,
}

impl M5WebviewOriginBarSet {
    /// Validate the bar set: every bar validates, every owner class is
    /// represented exactly once, the capability-limit vocabulary is covered
    /// across the set, no two bars share an id, and the source contracts are
    /// present.
    pub fn validate(&self) -> Result<(), AuthBoundaryError> {
        if self.schema_version != M5_WEBVIEW_ORIGIN_BAR_SET_SCHEMA_VERSION {
            return Err(AuthBoundaryError::WrongSetSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_WEBVIEW_ORIGIN_BAR_SET_RECORD_KIND {
            return Err(AuthBoundaryError::WrongSetRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        if non_empty(&self.set_id).is_none()
            || non_empty(&self.set_label).is_none()
            || non_empty(&self.redaction_class_token).is_none()
            || non_empty(&self.minted_at).is_none()
        {
            return Err(AuthBoundaryError::SetIdentityIncomplete);
        }
        if self.contract_doc_ref != M5_AUTH_BOUNDARY_CONTRACT_DOC_REF {
            return Err(AuthBoundaryError::WrongContractDocRef {
                record_id: self.set_id.clone(),
                actual: self.contract_doc_ref.clone(),
            });
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for bar in &self.bars {
            bar.validate()?;
            if !seen.insert(bar.bar_id.as_str()) {
                return Err(AuthBoundaryError::DuplicateRecordId {
                    record_id: bar.bar_id.clone(),
                });
            }
        }

        // Every governed owner class is named exactly once.
        for owner in WebviewOwnerClass::ALL {
            let count = self.bars.iter().filter(|b| b.owner_class == owner).count();
            if count != 1 {
                return Err(AuthBoundaryError::OwnerClassNotNamedOnce {
                    owner,
                    count: count as u32,
                });
            }
        }

        // The capability-limit vocabulary is covered across the set.
        for limit in CapabilityLimitClass::ALL {
            let carried = self
                .bars
                .iter()
                .any(|b| b.capability_limits.iter().any(|l| l.limit_class == limit));
            if !carried {
                return Err(AuthBoundaryError::CapabilityLimitClassMissing { limit });
            }
        }

        let refs: BTreeSet<&str> = self
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        for required in [
            M5_WEBVIEW_ORIGIN_BAR_SCHEMA_REF,
            M5_AUTH_BOUNDARY_CONTRACT_DOC_REF,
            M5_AUTH_BOUNDARY_COMMUNITY_HANDOFF_REF,
            M5_AUTH_BOUNDARY_DEVICE_PERMISSION_REF,
        ] {
            if !refs.contains(required) {
                return Err(AuthBoundaryError::MissingSourceContracts);
            }
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("webview origin bar set serializes"),
        ) {
            return Err(AuthBoundaryError::RawMaterialInExport);
        }
        Ok(())
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("webview origin bar set serializes")
    }

    /// Deterministic CSV: one row per owner class.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "owner_class,origin_disclosure,permission_state,open_in_browser_available,capability_limit_count,labeled_as_embedded,impersonates_native_chrome\n",
        );
        for bar in &self.bars {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                bar.owner_class.as_str(),
                bar.origin_disclosure.as_str(),
                bar.permission_state.as_str(),
                bar.open_in_browser.available,
                bar.capability_limits.len(),
                bar.labeled_as_embedded,
                bar.impersonates_native_chrome,
            ));
        }
        out
    }

    /// Deterministic Markdown governance summary.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 webview origin bars\n\n");
        out.push_str(&format!("Bar set: `{}`\n\n", self.set_id));
        out.push_str("| Owner class | Origin disclosure | Permission | Open in browser | Capability limits |\n");
        out.push_str("| --- | --- | --- | --- | --- |\n");
        for bar in &self.bars {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                bar.owner_class.label(),
                bar.origin_disclosure.label(),
                bar.permission_state.label(),
                bar.open_in_browser.available,
                bar.capability_limits.len(),
            ));
        }
        out.push('\n');
        out.push_str(
            "Every bar is labeled embedded, never impersonates native chrome, holds every ",
        );
        out.push_str(
            "native-only messaging flag false, and discloses that it is not native trust chrome.\n",
        );
        out
    }
}

// ===========================================================================
// Shared helpers and error vocabulary.
// ===========================================================================

/// True when a ref is an opaque token rather than a raw URL, email, or blank.
fn ref_is_opaque(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty()
        && trimmed == value
        && !trimmed.contains("://")
        && !trimmed.contains('@')
        && !trimmed.contains(char::is_whitespace)
}

fn non_empty(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Closed validation-error vocabulary for the auth-boundary contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthBoundaryError {
    WrongCardSchemaVersion {
        card_id: String,
        actual: u32,
    },
    WrongCardRecordKind {
        card_id: String,
        actual: String,
    },
    MalformedCardId {
        card_id: String,
    },
    HandoffReasonMismatch {
        card_id: String,
        kind: BrowserHandoffKind,
        reason: HandoffReasonClass,
    },
    HandoffDataExitMismatch {
        card_id: String,
        kind: BrowserHandoffKind,
        data_exit: DataExitBoundary,
    },
    MissingDeviceCodeDisclosure {
        card_id: String,
    },
    UnexpectedDeviceCodeDisclosure {
        card_id: String,
        kind: BrowserHandoffKind,
    },
    DeviceCodeMissingExpiry {
        card_id: String,
    },
    DeviceCodeTransmitted {
        card_id: String,
    },
    LocalContinuityNotPreserved {
        card_id: String,
    },
    HandoffDoesNotLeaveNativeChrome {
        card_id: String,
    },
    ProviderContentNotLabeled {
        card_id: String,
    },
    HandoffKindNotNamedOnce {
        kind: BrowserHandoffKind,
        count: u32,
    },
    DeviceCodeCardMissing,
    WrongBarSchemaVersion {
        bar_id: String,
        actual: u32,
    },
    WrongBarRecordKind {
        bar_id: String,
        actual: String,
    },
    MalformedBarId {
        bar_id: String,
    },
    OriginDisclosureMismatch {
        bar_id: String,
        owner: WebviewOwnerClass,
        origin: OriginDisclosureClass,
    },
    UntrustedPermissionTooBroad {
        bar_id: String,
        permission: WebviewPermissionState,
    },
    EmbeddedSurfaceNotLabeled {
        bar_id: String,
    },
    EmbeddedImpersonatesNativeMessaging {
        bar_id: String,
        messaging: &'static str,
    },
    MissingCapabilityLimits {
        bar_id: String,
    },
    MissingNotNativeTrustLimit {
        bar_id: String,
    },
    OwnerClassNotNamedOnce {
        owner: WebviewOwnerClass,
        count: u32,
    },
    CapabilityLimitClassMissing {
        limit: CapabilityLimitClass,
    },
    ImpersonatesNativeChrome {
        record_id: String,
    },
    WrongSetSchemaVersion {
        actual: u32,
    },
    WrongSetRecordKind {
        actual: String,
    },
    SetIdentityIncomplete,
    DuplicateRecordId {
        record_id: String,
    },
    MissingSourceContracts,
    RawMaterialInExport,
    WrongContractDocRef {
        record_id: String,
        actual: String,
    },
    EmptyRequiredField {
        record_id: String,
        field: &'static str,
    },
    RawRefLeak {
        record_id: String,
        field: &'static str,
    },
}

impl fmt::Display for AuthBoundaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongCardSchemaVersion { card_id, actual } => write!(
                f,
                "card {card_id} has unsupported browser_handoff_card_schema_version {actual}"
            ),
            Self::WrongCardRecordKind { card_id, actual } => {
                write!(f, "card {card_id} has unsupported record kind {actual}")
            }
            Self::MalformedCardId { card_id } => {
                write!(f, "card id {card_id} must start with browser_handoff_card:")
            }
            Self::HandoffReasonMismatch {
                card_id,
                kind,
                reason,
            } => write!(
                f,
                "card {card_id} handoff kind {} cannot carry reason {}",
                kind.as_str(),
                reason.as_str()
            ),
            Self::HandoffDataExitMismatch {
                card_id,
                kind,
                data_exit,
            } => write!(
                f,
                "card {card_id} handoff kind {} cannot use data exit {}",
                kind.as_str(),
                data_exit.as_str()
            ),
            Self::MissingDeviceCodeDisclosure { card_id } => write!(
                f,
                "card {card_id} is a device-code handoff and must carry a device-code disclosure"
            ),
            Self::UnexpectedDeviceCodeDisclosure { card_id, kind } => write!(
                f,
                "card {card_id} handoff kind {} must not carry a device-code disclosure",
                kind.as_str()
            ),
            Self::DeviceCodeMissingExpiry { card_id } => write!(
                f,
                "card {card_id} device-code disclosure must state a real expiry"
            ),
            Self::DeviceCodeTransmitted { card_id } => write!(
                f,
                "card {card_id} device code must be shown in-app and not transmitted by Aureline"
            ),
            Self::LocalContinuityNotPreserved { card_id } => write!(
                f,
                "card {card_id} must preserve local continuity for a blocked handoff"
            ),
            Self::HandoffDoesNotLeaveNativeChrome { card_id } => write!(
                f,
                "card {card_id} is a browser/device-code handoff and must open outside native chrome"
            ),
            Self::ProviderContentNotLabeled { card_id } => write!(
                f,
                "card {card_id} must label provider-owned content as provider-owned"
            ),
            Self::HandoffKindNotNamedOnce { kind, count } => write!(
                f,
                "handoff kind {} is named {count} times; expected exactly once",
                kind.as_str()
            ),
            Self::DeviceCodeCardMissing => {
                write!(f, "card set must carry at least one device-code handoff card")
            }
            Self::WrongBarSchemaVersion { bar_id, actual } => write!(
                f,
                "bar {bar_id} has unsupported webview_origin_bar_schema_version {actual}"
            ),
            Self::WrongBarRecordKind { bar_id, actual } => {
                write!(f, "bar {bar_id} has unsupported record kind {actual}")
            }
            Self::MalformedBarId { bar_id } => {
                write!(f, "bar id {bar_id} must start with webview_origin_bar:")
            }
            Self::OriginDisclosureMismatch {
                bar_id,
                owner,
                origin,
            } => write!(
                f,
                "bar {bar_id} owner class {} cannot use origin disclosure {}",
                owner.as_str(),
                origin.as_str()
            ),
            Self::UntrustedPermissionTooBroad { bar_id, permission } => write!(
                f,
                "bar {bar_id} untrusted origin cannot hold permission state {}",
                permission.as_str()
            ),
            Self::EmbeddedSurfaceNotLabeled { bar_id } => {
                write!(f, "bar {bar_id} embedded surface must be labeled as embedded")
            }
            Self::EmbeddedImpersonatesNativeMessaging { bar_id, messaging } => write!(
                f,
                "bar {bar_id} embedded surface must not show native {messaging}"
            ),
            Self::MissingCapabilityLimits { bar_id } => {
                write!(f, "bar {bar_id} must disclose at least one capability limit")
            }
            Self::MissingNotNativeTrustLimit { bar_id } => write!(
                f,
                "bar {bar_id} must disclose the not-native-trust-chrome capability limit"
            ),
            Self::OwnerClassNotNamedOnce { owner, count } => write!(
                f,
                "owner class {} is named {count} times; expected exactly once",
                owner.as_str()
            ),
            Self::CapabilityLimitClassMissing { limit } => write!(
                f,
                "bar set never discloses capability limit {}",
                limit.as_str()
            ),
            Self::ImpersonatesNativeChrome { record_id } => {
                write!(f, "record {record_id} must not impersonate native chrome")
            }
            Self::WrongSetSchemaVersion { actual } => {
                write!(f, "set has unsupported schema_version {actual}")
            }
            Self::WrongSetRecordKind { actual } => {
                write!(f, "set has unsupported record kind {actual}")
            }
            Self::SetIdentityIncomplete => write!(f, "set is missing required identity fields"),
            Self::DuplicateRecordId { record_id } => {
                write!(f, "set has duplicate record id {record_id}")
            }
            Self::MissingSourceContracts => {
                write!(f, "set is missing a required source contract ref")
            }
            Self::RawMaterialInExport => write!(f, "export carries forbidden raw material"),
            Self::WrongContractDocRef { record_id, actual } => {
                write!(f, "record {record_id} cites wrong contract doc {actual}")
            }
            Self::EmptyRequiredField { record_id, field } => {
                write!(f, "record {record_id} is missing required field {field}")
            }
            Self::RawRefLeak { record_id, field } => write!(
                f,
                "record {record_id} field {field} contains a raw URL, email, or whitespace; opaque refs only"
            ),
        }
    }
}

impl Error for AuthBoundaryError {}

/// Reads and validates the checked-in stable browser-handoff card set.
pub fn current_stable_m5_browser_handoff_card_set(
) -> Result<M5BrowserHandoffCardSet, Box<dyn Error>> {
    let set: M5BrowserHandoffCardSet = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/help/m5-auth-boundary-proof/browser_handoff_cards.json"
    )))?;
    set.validate()?;
    Ok(set)
}

/// Reads and validates the checked-in stable webview origin bar set.
pub fn current_stable_m5_webview_origin_bar_set() -> Result<M5WebviewOriginBarSet, Box<dyn Error>> {
    let set: M5WebviewOriginBarSet = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/help/m5-auth-boundary-proof/webview_origin_bars.json"
    )))?;
    set.validate()?;
    Ok(set)
}
